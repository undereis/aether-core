# Docker

Docker-specific support files belong here.

The default Compose entrypoint is the repository root `compose.yaml` so local
commands remain predictable:

```bash
make docker-up
make docker-validate
```

