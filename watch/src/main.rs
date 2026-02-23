//! OILSHIP watch engine — entrypoint.

mod alerts;
mod bridges;
mod config;
mod indexer;
mod metrics;
mod publisher;
mod replay;
mod rpc;
mod scheduler;
mod score;
mod signals;
mod store;
mod types;

use crate::alerts::{Alert, LogSink, SinkSet, WebhookSink};
use crate::bridges::BridgeRegistry;
use crate::config::EngineConfig;
use crate::rpc::RpcClient;
use crate::signals::SignalSet;
use crate::store::Store;
use crate::types::BridgeId;
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{info, warn};

#[derive(Parser)]
#[command(name = "oilship-watch", version, about = "OILSHIP bridge monitor")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Run {
        #[arg(short, long, default_value = "watch.json")]
        config: String,
    },
    Sample {
        bridge: String,
        #[arg(short, long, default_value = "watch.json")]
        config: String,
    },
    PrintConfig,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "oilship_watch=info".into()),
        )
        .init();
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Run { config } => run(&config).await,
        Cmd::Sample { bridge, config } => sample(&config, &bridge).await,
        Cmd::PrintConfig => {
            let cfg = EngineConfig::defaults();
            println!("{}", serde_json::to_string_pretty(&cfg)?);
            Ok(())
        }
    }
}

async fn run(config_path: &str) -> Result<()> {
    let cfg = match EngineConfig::load_from(config_path) {
        Ok(c) => c,
        Err(e) => {
            warn!("could not load config at {}: {} — using defaults", config_path, e);
            EngineConfig::defaults()
        }
    };
    let rpc = RpcClient::new(
        cfg.rpc_table().into_iter().map(|(c, s)| (c, s.to_string())).collect(),
    );
    let registry = BridgeRegistry::default_set();
    let signals = SignalSet::standard();
    let store: Arc<Store> = Store::shared();
    let mut sinks = SinkSet::empty();
    sinks.add(Arc::new(LogSink));
    if let Some(url) = &cfg.alert_webhook {
        sinks.add(Arc::new(WebhookSink::new(url.clone())));
    }
    info!("oilship-watch starting; bridges={}", registry.adapters.len());
    let started = Instant::now();
    let mut tick = 0u64;
    loop {
        tick += 1;
        let cycle_start = Instant::now();
        for adapter in &registry.adapters {
            let id = adapter.id().clone();
            let bcfg = match cfg.lookup(&id) {
                Some(c) => c.clone(),
                None => continue,
            };
            match adapter.sample_blocking(&rpc, &bcfg) {
                Ok(snap) => {
                    let anomalies = signals.evaluate_all(&snap, &bcfg);
                    let assessment = score::compute(id.clone(), &anomalies);
                    let prev_tier = store.last_tier(&id);
                    store.record_sample(snap, assessment.clone());
                    if let Some(mut alert) = Alert::classify(prev_tier, assessment) {
                        alert.anomalies = anomalies;
                        sinks.dispatch(&alert);
                    }
                }
                Err(e) => warn!(bridge = %id, "sample error: {}", e),
            }
        }
        info!("tick {} done in {:?}; uptime {:?}", tick, cycle_start.elapsed(), started.elapsed());
        tokio::time::sleep(Duration::from_secs(cfg.poll_interval_secs)).await;
    }
}

async fn sample(config_path: &str, bridge: &str) -> Result<()> {
    let cfg = EngineConfig::load_from(config_path).unwrap_or_else(|_| EngineConfig::defaults());
    let rpc = RpcClient::new(
        cfg.rpc_table().into_iter().map(|(c, s)| (c, s.to_string())).collect(),
    );
    let registry = BridgeRegistry::default_set();
    let id = BridgeId::new(bridge);
    let adapter = registry.find(&id).ok_or_else(|| anyhow::anyhow!("unknown bridge {bridge}"))?;
    let bcfg = cfg.lookup(&id).cloned().ok_or_else(|| anyhow::anyhow!("bridge {bridge} missing from config"))?;
    let snap = adapter.sample_blocking(&rpc, &bcfg)?;
    let signals = SignalSet::standard();
    let anomalies = signals.evaluate_all(&snap, &bcfg);
    let assessment = score::compute(id, &anomalies);
    println!("snapshot: tvl=${:.0} anomalies={} score={} tier={:?}",
        snap.tvl_usd, anomalies.len(), assessment.score, assessment.tier);
    Ok(())
}
