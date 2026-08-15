"""Security-sensitive backend setting tests."""

from aether_backend.config.settings import Settings


def test_backend_defaults_to_loopback() -> None:
    """The development API must not bind to every interface by default."""

    assert Settings().api_host == "127.0.0.1"
