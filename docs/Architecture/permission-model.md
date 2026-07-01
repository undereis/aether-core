# Permission Model

The Phase 3 permission model is internal and declarative.

It does not implement authentication, users, roles, sessions, tokens, or access
control lists. It only records what a service requests and lets platform
boundaries enforce those declarations.

## Initial Permissions

- `event.publish`
- `event.subscribe`
- `config.read`
- `telemetry.emit`
- `service.command`
- `service.inspect`

## Enforcement

The ASB checks permissions before bus actions. The Service Registry can also
verify whether a service declared a permission before the Kernel allows future
actions.

## Current Limits

Permissions are local model data in Phase 3. Production authorization will
require a future ADR.
