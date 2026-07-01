"""Tests for system-level endpoints."""

import httpx
import pytest
from fastapi import FastAPI


@pytest.mark.anyio
async def test_health_endpoint(app: FastAPI) -> None:
    transport = httpx.ASGITransport(app=app)

    async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
        response = await client.get("/health")

    assert response.status_code == 200
    assert response.json() == {
        "status": "ok",
        "service": "Aether",
        "version": "0.0.0-test",
        "environment": "test",
    }


@pytest.mark.anyio
async def test_version_endpoint(app: FastAPI) -> None:
    transport = httpx.ASGITransport(app=app)

    async with httpx.AsyncClient(transport=transport, base_url="http://testserver") as client:
        response = await client.get("/version")

    assert response.status_code == 200
    assert response.json() == {
        "service": "Aether",
        "version": "0.0.0-test",
        "environment": "test",
    }
