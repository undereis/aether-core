"""HTTP response schemas."""

from __future__ import annotations

from typing import Literal

from pydantic import BaseModel, ConfigDict


class HealthResponse(BaseModel):
    """Health endpoint response."""

    model_config = ConfigDict(frozen=True)

    status: Literal["ok"]
    service: str
    version: str
    environment: str


class VersionResponse(BaseModel):
    """Version endpoint response."""

    model_config = ConfigDict(frozen=True)

    service: str
    version: str
    environment: str
