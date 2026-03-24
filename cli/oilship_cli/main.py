"""OILSHIP command-line entrypoint."""

from __future__ import annotations

from pathlib import Path

import typer

from . import __version__
from .config import CliConfig
from . import commands as cmds


app = typer.Typer(help="OILSHIP CLI — bridge convoy control surface", no_args_is_help=True)
threat_app = typer.Typer(help="Local risk simulator")
config_app = typer.Typer(help="Configuration")
policy_app = typer.Typer(help="Policy management")
app.add_typer(threat_app, name="threat")
app.add_typer(config_app, name="config")
app.add_typer(policy_app, name="policy")


def _load_cfg() -> CliConfig:
    return CliConfig.load()


@app.callback(invoke_without_command=False)
def root(
    ctx: typer.Context,
    rpc: str = typer.Option(None, "--rpc", help="Override RPC URL"),
    program: str = typer.Option(None, "--program", help="Override program id"),
    plain: bool = typer.Option(False, "--plain", help="Disable colour output"),
    json_output: bool = typer.Option(False, "--json", help="Emit JSON where supported"),
):
    cfg = _load_cfg()
    if rpc:
        cfg.rpc_url = rpc
    if program:
        cfg.program_id = program
    if plain:
        cfg.color = False
    if json_output:
        cfg.json_output = True
    ctx.obj = cfg


@app.command()
def version():
    """Print the CLI version."""
    typer.echo(f"oilship-cli {__version__}")


@app.command()
def status(ctx: typer.Context):
    """Show RPC, program and fleet status."""
    cmds.cmd_status(ctx.obj)


@app.command()
def fleet(ctx: typer.Context, json_out: bool = typer.Option(False, "--json")):
    """List the bridge fleet."""
    cmds.cmd_fleet(ctx.obj, json_out)


@app.command()
def quote(
    ctx: typer.Context,
    sol: float = typer.Argument(..., help="Cargo size in SOL"),
    bridge: str = typer.Option(None, "--bridge", "-b"),
    max_risk: int = typer.Option(None, "--max-risk"),
):
    """Get an escort quote."""
    cmds.cmd_quote(ctx.obj, sol, bridge, max_risk)


@app.command(name="open")
def open_(
    ctx: typer.Context,
    sol: float = typer.Argument(..., help="Cargo size in SOL"),
    bridge: str = typer.Option(None, "--bridge", "-b"),
    hours: int = typer.Option(4, "--hours", "-h"),
):
    """Prepare an open-policy transaction (does not sign)."""
    cmds.cmd_open(ctx.obj, sol, bridge, hours)


@app.command()
def fund(ctx: typer.Context):
    """Show wreck fund + treasury P&L."""
    cmds.cmd_fund(ctx.obj)


@policy_app.command("list")
def policy_list(
    ctx: typer.Context,
    beneficiary: str = typer.Option(None, "--beneficiary"),
):
    """List policies for a beneficiary."""
    cmds.cmd_policy_list(ctx.obj, beneficiary)


@threat_app.command("simulate")
def threat_simulate(
    ctx: typer.Context,
    file: Path = typer.Argument(..., help="JSON file with bridge + anomalies"),
):
    """Compute a provisional risk score from a JSON anomaly file."""
    cmds.cmd_threat_simulate(ctx.obj, file)


@threat_app.command("smooth")
def threat_smooth(
    ctx: typer.Context,
    scores: list[int] = typer.Argument(..., help="Recent scores"),
):
    """EWMA-smooth a series of risk scores."""
    cmds.cmd_threat_smooth(ctx.obj, scores)


@config_app.command("show")
def config_show(ctx: typer.Context):
    """Print the active config."""
    cmds.cmd_config_show(ctx.obj)


@config_app.command("save")
def config_save(ctx: typer.Context):
    """Persist the active config to disk."""
    cmds.cmd_config_save(ctx.obj)


if __name__ == "__main__":
    app()
