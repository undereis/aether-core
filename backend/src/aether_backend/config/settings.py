"""Typed runtime settings for the Aether backend."""

from __future__ import annotations

from importlib.metadata import PackageNotFoundError, version
from typing import Literal

from pydantic import Field
from pydantic_settings import BaseSettings, SettingsConfigDict

Environment = Literal["local", "test", "development", "staging", "production"]
LogLevel = Literal["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"]


def resolve_package_version(distribution_name: str = "aether-backend") -> str:
    """Resolve the installed package version with a source-tree fallback."""

    try:
        return version(distribution_name)
    except PackageNotFoundError:
        return "0.1.0"


class Settings(BaseSettings):
    """Environment-driven backend settings."""

    app_name: str = Field(default="Aether")
    app_version: str = Field(default_factory=resolve_package_version)
    environment: Environment = Field(default="local")
    log_level: LogLevel = Field(default="INFO")
    api_host: str = Field(default="0.0.0.0")
    api_port: int = Field(default=18000, ge=1, le=65535)
    api_prefix: str = Field(default="")
    docs_enabled: bool = Field(default=True)

    model_config = SettingsConfigDict(
        env_prefix="AETHER_",
        env_file=".env",
        env_file_encoding="utf-8",
        extra="ignore",
    )


def load_settings() -> Settings:
    """Load settings from environment variables and the configured env file."""

    return Settings()
