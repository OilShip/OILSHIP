"""Configuration loader for the OILSHIP CLI."""

from __future__ import annotations

import json
import os
from dataclasses import dataclass, field, asdict
from pathlib import Path
from typing import Any


DEFAULT_CONFIG_PATH = Path.home() / ".config" / "oilship" / "cli.json"
ENV_PREFIX = "OILSHIP_"


@dataclass
class CliConfig:
    rpc_url: str = "https://api.mainnet-beta.solana.com"
    program_id: str = "11111111111111111111111111111111"
    keypair_path: str = str(Path.home() / ".config" / "solana" / "id.json")
    default_bridge: str = "mayan"
    default_lifetime_hours: int = 4
    default_max_risk: int = 60
    color: bool = True
    json_output: bool = False

    @classmethod
    def load(cls, path: Path | None = None) -> "CliConfig":
        cfg = cls()
        path = path or DEFAULT_CONFIG_PATH
        if path.exists():
            try:
                raw = json.loads(path.read_text(encoding="utf-8"))
                cfg.merge(raw)
            except json.JSONDecodeError as exc:
                raise SystemExit(f"oilship: invalid config at {path}: {exc}")
        cfg.apply_env(os.environ)
        return cfg

    def merge(self, raw: dict[str, Any]) -> None:
        for k, v in raw.items():
            if hasattr(self, k):
                setattr(self, k, v)

    def apply_env(self, env: dict[str, str]) -> None:
        mapping = {
            "RPC_URL": "rpc_url",
            "PROGRAM_ID": "program_id",
            "KEYPAIR": "keypair_path",
            "DEFAULT_BRIDGE": "default_bridge",
            "DEFAULT_LIFETIME_HOURS": "default_lifetime_hours",
            "DEFAULT_MAX_RISK": "default_max_risk",
            "COLOR": "color",
            "JSON_OUTPUT": "json_output",
        }
        for key, attr in mapping.items():
            value = env.get(ENV_PREFIX + key)
            if value is None:
                continue
            current = getattr(self, attr)
            if isinstance(current, bool):
                setattr(self, attr, value.lower() not in ("0", "false", "no"))
            elif isinstance(current, int):
                try:
                    setattr(self, attr, int(value))
                except ValueError as exc:
                    raise SystemExit(f"oilship: env {key} must be int") from exc
            else:
                setattr(self, attr, value)

    def save(self, path: Path | None = None) -> None:
        path = path or DEFAULT_CONFIG_PATH
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(asdict(self), indent=2), encoding="utf-8")

    def as_dict(self) -> dict[str, Any]:
        return asdict(self)
