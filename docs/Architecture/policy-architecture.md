# Policy Architecture

Policies define behavioral constraints for the platform.

Phase 4.5 creates the policy model only. It does not implement authentication,
authorization, sandboxing, or runtime enforcement.

## Policies Introduced

- `SecurityPolicy`;
- `FilesystemPolicy`;
- `TelemetryPolicy`;
- `MemoryPolicy`;
- `PrivacyPolicy`.

## Contracts

The policy layer defines:

- `Policy`;
- `PolicyKind`;
- `PolicyManifest`;
- `PolicyDescriptor`;
- `PolicyEvaluationContext`;
- `PolicyDecision`;
- `PolicyRegistry`.

Policy evaluation currently returns `NotApplicable` by default. This makes the
contract explicit without pretending enforcement exists.

## Future Responsibility

Policies will eventually constrain:

- service permissions;
- driver access;
- filesystem access;
- telemetry retention;
- memory retention;
- privacy-sensitive operations.

Policies should be evaluated by Managers or future enforcement components, not
by the Kernel directly.
