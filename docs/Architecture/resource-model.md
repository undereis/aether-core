# Resource Model

The Phase 3 resource model declares what a service expects from the platform.

It prepares future sandboxing and scheduling without implementing either one in
this phase.

## Resource Fields

- `cpu_class`: `low`, `medium`, or `high`
- `memory_class`: `low`, `medium`, or `high`
- `storage_class`: `none`, `low`, `medium`, or `high`
- `network_access`: boolean, with manifest alias `network`
- `filesystem_access`: `none`, `read_only`, or `read_write`

## Default Profile

The default resource profile is intentionally restricted:

- low CPU
- low memory
- no persistent storage
- no network access
- no filesystem access

## Current Limits

Resource declarations are not enforced by the OS or container runtime in Phase
3. Enforcement belongs to a future sandbox phase.
