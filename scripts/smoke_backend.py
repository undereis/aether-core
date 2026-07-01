"""Smoke test the FastAPI application without binding a network port."""

from __future__ import annotations

import asyncio

import httpx

from aether_backend.app import create_app
from aether_backend.config.settings import Settings


async def smoke_backend() -> None:
    app = create_app(Settings(log_level="ERROR"))
    transport = httpx.ASGITransport(app=app)

    async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
        for path in ("/health", "/version"):
            response = await client.get(path)
            response.raise_for_status()
            print(response.json())


def main() -> None:
    asyncio.run(smoke_backend())


if __name__ == "__main__":
    main()
