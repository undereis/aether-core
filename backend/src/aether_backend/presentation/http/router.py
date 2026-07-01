"""HTTP router composition."""

from fastapi import APIRouter

from aether_backend.presentation.http.routes import system

api_router = APIRouter()
api_router.include_router(system.router)
