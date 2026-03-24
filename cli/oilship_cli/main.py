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

