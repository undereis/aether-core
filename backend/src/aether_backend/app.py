"""FastAPI application factory."""

from __future__ import annotations

import logging

from fastapi import FastAPI

from aether_backend.bootstrap.container import Container
from aether_backend.config.settings import Settings, load_settings
from aether_backend.logging.configuration import configure_logging
from aether_backend.presentation.http.router import api_router


def create_app(settings: Settings | None = None) -> FastAPI:
    """Create a configured FastAPI application instance."""

    resolved_settings = settings if settings is not None else load_settings()
    configure_logging(resolved_settings.log_level)

    docs_url = "/docs" if resolved_settings.docs_enabled else None
    openapi_url = "/openapi.json" if resolved_settings.docs_enabled else None

    app = FastAPI(
        title=resolved_settings.app_name,
        version=resolved_settings.app_version,
        docs_url=docs_url,
        redoc_url=None,
        openapi_url=openapi_url,
    )
    app.state.container = Container(
        settings=resolved_settings,
        logger=logging.getLogger("aether.backend"),
    )
    app.include_router(api_router, prefix=resolved_settings.api_prefix)
    return app
