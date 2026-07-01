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

No AI, agents, UI, OS event capture, or business behavior is implemented here.

Run the CLI with:

```bash
cargo run -p aether-cli -- validate
cargo run -p aether-cli -- kernel status
cargo run -p aether-cli -- kernel health
cargo run -p aether-cli -- kernel modules
cargo run -p aether-cli -- kernel capabilities
```
