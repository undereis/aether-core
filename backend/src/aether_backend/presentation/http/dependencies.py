"""FastAPI dependency providers."""

from __future__ import annotations

from typing import Annotated

from fastapi import Depends, Request

from aether_backend.bootstrap.container import Container
from aether_backend.config.settings import Settings


def get_container(request: Request) -> Container:
    """Return the application container for the current request."""

    container = getattr(request.app.state, "container", None)
    if not isinstance(container, Container):
        raise RuntimeError("Aether application container is not initialized.")
    return container


def get_settings(container: Annotated[Container, Depends(get_container)]) -> Settings:
    """Return typed runtime settings."""

    return container.settings
