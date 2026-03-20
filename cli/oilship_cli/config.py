"""Configuration loader for the OILSHIP CLI."""

from __future__ import annotations

import json
import os
from dataclasses import dataclass, field, asdict
from pathlib import Path
from typing import Any


DEFAULT_CONFIG_PATH = Path.home() / ".config" / "oilship" / "cli.json"
ENV_PREFIX = "OILSHIP_"
