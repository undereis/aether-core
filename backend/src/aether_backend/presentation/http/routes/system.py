"""System-level HTTP endpoints."""

from __future__ import annotations

from typing import Annotated

from fastapi import APIRouter, Depends

from aether_backend.config.settings import Settings
from aether_backend.presentation.http.dependencies import get_settings
from aether_backend.presentation.http.schemas import HealthResponse, VersionResponse

router = APIRouter(tags=["system"])


@router.get("/health", response_model=HealthResponse, summary="Service health")
def health(settings: Annotated[Settings, Depends(get_settings)]) -> HealthResponse:
    """Report process-level health."""

    return HealthResponse(
        status="ok",
        service=settings.app_name,
        version=settings.app_version,
        environment=settings.environment,
    )


@router.get("/version", response_model=VersionResponse, summary="Service version")
def version(settings: Annotated[Settings, Depends(get_settings)]) -> VersionResponse:
    """Report service version metadata."""

    return VersionResponse(
        service=settings.app_name,
        version=settings.app_version,
        environment=settings.environment,
    )
