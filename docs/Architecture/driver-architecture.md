# Driver Architecture

Drivers are the future boundary between Aether and native or external execution
surfaces.

Phase 4.5 creates contracts only. No real driver is implemented.

## Contracts

The driver layer defines:

- `Driver`;
- `DriverManifest`;
- `DriverCapability`;
- `DriverHealth`;
- `DriverRegistry`;
- `DriverDescriptor`.

Drivers will eventually represent controlled adapters such as filesystem,
clipboard, screen, camera, microphone, browser, network, local IPC, or device
integrations.

## Future Flow

Future drivers should be registered through `DriverManager`.

They must declare:

- capabilities;
- resource requirements;
- health;
- provider;
- version.

Driver usage must be constrained by Policies and Resources. Services should not
call drivers directly unless a future phase defines an explicit mediated
contract for that interaction.

## Non-Goals

Phase 4.5 does not implement filesystem access, clipboard access, screen
capture, browser automation, camera, microphone, network adapters, or IPC.
