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
