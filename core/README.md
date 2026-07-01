# Core

Rust workspace for the Aether core runtime.

Phase 1 introduced the native runtime foundation:

- internal events
- local configuration
- structured logging
- module contracts
- runtime bootstrap
- validation CLI

Phase 2 introduces the Aether Kernel:

- controlled runtime startup and shutdown
- module registry
- lifecycle state tracking
- dependency validation
- capability declaration and discovery
- kernel health checks
- telemetry abstraction
- typed ID strategy

Phase 3 introduces the Aether Service Platform:

- Service Model and Service Manifest
- Service Registry
- Aether Service Bus
- Permission Model
- Resource Model
- service health aggregation
- service and bus inspection CLI

Phase 4 introduces the official service map and core system service base:

- `aether-system-services`
- service manifests under `core/services`
- Telemetry Service
- Configuration Service
- Health Service
- Event Service
- Service Inspector Service
- service map and system service inspection CLI

No AI, agents, UI, OS event capture, or business behavior is implemented here.

Run the CLI with:

```bash
cargo run -p aether-cli -- validate
cargo run -p aether-cli -- kernel status
cargo run -p aether-cli -- kernel health
cargo run -p aether-cli -- kernel modules
cargo run -p aether-cli -- kernel capabilities
cargo run -p aether-cli -- service list
cargo run -p aether-cli -- service inspect
cargo run -p aether-cli -- service capabilities
cargo run -p aether-cli -- service health
cargo run -p aether-cli -- service map
cargo run -p aether-cli -- system services
cargo run -p aether-cli -- system health
cargo run -p aether-cli -- system inspect
cargo run -p aether-cli -- bus status
```
