"""Thin Solana JSON-RPC client used by the CLI."""

from __future__ import annotations

import base64
from dataclasses import dataclass
from typing import Any

import httpx


@dataclass
class AccountInfo:
    address: str
    lamports: int
    owner: str
    executable: bool
    data: bytes


class RpcError(RuntimeError):
    def __init__(self, code: int, message: str):
        super().__init__(f"rpc {code}: {message}")
        self.code = code
        self.message = message


class OilshipRpc:
    def __init__(self, url: str, timeout: float = 15.0):
        self.url = url
        self._client = httpx.Client(timeout=timeout, follow_redirects=False)
        self._next_id = 1

    def close(self) -> None:
        self._client.close()
