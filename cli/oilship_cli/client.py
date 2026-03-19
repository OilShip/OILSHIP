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

    def __enter__(self) -> "OilshipRpc":
        return self

    def __exit__(self, *exc: Any) -> None:
        self.close()

    def _call(self, method: str, params: list[Any]) -> Any:
        payload = {
            "jsonrpc": "2.0",
            "id": self._next_id,
            "method": method,
            "params": params,
        }
        self._next_id += 1
        try:
            r = self._client.post(self.url, json=payload)
        except httpx.HTTPError as exc:
            raise RpcError(-1, f"transport: {exc}") from exc
        if r.status_code != 200:
            raise RpcError(r.status_code, r.text)
        body = r.json()
        if "error" in body and body["error"]:
            err = body["error"]
            raise RpcError(int(err.get("code", -1)), str(err.get("message", "rpc error")))
        return body.get("result")

    def get_slot(self) -> int:
        return int(self._call("getSlot", []))

    def get_balance(self, address: str) -> int:
        result = self._call("getBalance", [address])
        return int(result.get("value", 0))

    def get_account_info(self, address: str) -> AccountInfo | None:
        result = self._call(
            "getAccountInfo",
            [address, {"encoding": "base64", "commitment": "confirmed"}],
        )
        value = result.get("value") if isinstance(result, dict) else None
        if not value:
            return None
        data_b64, _enc = value["data"]
        return AccountInfo(
            address=address,
            lamports=int(value["lamports"]),
            owner=str(value["owner"]),
            executable=bool(value["executable"]),
            data=base64.b64decode(data_b64),
        )

    def get_program_accounts(self, program_id: str) -> list[AccountInfo]:
        result = self._call(
            "getProgramAccounts",
            [program_id, {"encoding": "base64", "commitment": "confirmed"}],
        )
        out: list[AccountInfo] = []
        if not isinstance(result, list):
            return out
        for item in result:
            pubkey = item.get("pubkey")
            account = item.get("account") or {}
            data = account.get("data")
            if not pubkey or not data:
                continue
            data_b64, _enc = data
            out.append(
                AccountInfo(
                    address=pubkey,
                    lamports=int(account.get("lamports", 0)),
                    owner=str(account.get("owner", "")),
                    executable=bool(account.get("executable", False)),
                    data=base64.b64decode(data_b64),
                )
            )
        return out

    def get_signatures_for_address(self, address: str, limit: int = 50) -> list[dict[str, Any]]:
        return list(self._call("getSignaturesForAddress", [address, {"limit": limit}]) or [])

    def get_block_height(self) -> int:
        return int(self._call("getBlockHeight", []))

    def health(self) -> str:
        try:
            return str(self._call("getHealth", []))
        except RpcError:
            return "unhealthy"

    def airdrop(self, address: str, lamports: int) -> str:
        return str(self._call("requestAirdrop", [address, lamports]))


def lamports_to_sol(lamports: int) -> float:
    return lamports / 1e9


def sol_to_lamports(sol: float) -> int:
    if sol < 0:
        raise ValueError("sol must be non-negative")
    return int(round(sol * 1e9))


_B58_ALPHABET = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"


def _b58encode(buf: bytes) -> str:
    n = int.from_bytes(buf, "big")
    out = ""
    while n > 0:
        n, rem = divmod(n, 58)
        out = _B58_ALPHABET[rem] + out
    pad = 0
    for byte in buf:
        if byte == 0:
            pad += 1
        else:
            break
    return _B58_ALPHABET[0] * pad + out


def decode_pubkey(blob: bytes) -> str:
    if len(blob) != 32:
        raise ValueError("pubkey must be 32 bytes")
    return _b58encode(blob)
