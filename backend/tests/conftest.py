"""Shared backend test fixtures."""

import pytest
from fastapi import FastAPI

from aether_backend.app import create_app
from aether_backend.config.settings import Settings


@pytest.fixture
def test_settings() -> Settings:
    """Return deterministic settings for tests."""

    return Settings(
        app_version="0.0.0-test",
        docs_enabled=False,
        environment="test",
    )


@pytest.fixture
def app(test_settings: Settings) -> FastAPI:
    """Return a configured FastAPI application for tests."""

    return create_app(settings=test_settings)


@pytest.fixture
def anyio_backend() -> str:
    """Run async tests on asyncio only."""

    return "asyncio"
