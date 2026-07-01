"""Application dependency container."""

from __future__ import annotations

from dataclasses import dataclass
from logging import Logger

from aether_backend.config.settings import Settings


@dataclass(frozen=True, slots=True)
class Container:
    """Immutable dependencies shared by the FastAPI application."""

    settings: Settings
    logger: Logger
