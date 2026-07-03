# AEP-0010: Drivers Touch The Outside World

## Objective

Reserve native, device, filesystem, network, browser, desktop, and external IO
access for Drivers or explicit future driver-mediated contracts.

## Motivation

External access is where privacy, security, portability, and reliability risks
concentrate. It must not be scattered across services or managers.

## Context

Phase 4.5 introduced Driver contracts only. No real filesystem, clipboard,
screen, camera, microphone, browser, network, or IPC driver exists yet.

## Rules

- Services must not perform unmanaged external IO.
- Managers must not implement device adapters.
- Drivers must declare capabilities, resources, provider, version, and health.
- Driver access must be constrained by Policies and Resources.
- Real Drivers require architecture review and tests.

## Mandatory Flow

1. Identify the external surface.
2. Define driver manifest and descriptor.
3. Define policy and resource constraints.
4. Define mediated service contract.
5. Validate with security and portability tests.

## Correct Examples

- A future Filesystem Driver declaring `filesystem.read`.
- A service requesting a filesystem capability through an approved contract.

## Incorrect Examples

- A service reading arbitrary local files directly.
- The Kernel invoking OS APIs for screen capture.

## Violation Detection

- Direct OS, filesystem, browser, network, camera, or microphone calls outside
  driver boundaries.
- Services with hidden external dependencies.
- Resource declarations missing for external access.

## Violation Correction

Move external IO behind a Driver contract and require policy/resource checks
before use.

## Relationship With Other AEPs

Depends on AEP-0011 and AEP-0013. Protects AEP-0007 and AEP-0009.
