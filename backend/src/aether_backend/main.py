"""ASGI entrypoint for the Aether backend."""

from aether_backend.app import create_app

app = create_app()
