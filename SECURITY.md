# Security Policy

Aether is currently a research and engineering foundation, not a production service or end-user product.

## Supported versions

Security fixes are applied to the current `main` branch. Historical tags are architecture checkpoints and are not maintained as supported production releases.

## Reporting a vulnerability

Do not disclose suspected vulnerabilities, credentials, or sensitive evidence in a public issue.

If private vulnerability reporting is enabled for this repository, use GitHub's **Report a vulnerability** flow. Otherwise, contact the repository owner through an established private channel before sharing technical details.

Include, when possible:

- affected component and commit;
- impact and realistic attack scenario;
- reproduction steps or proof of concept;
- suggested mitigation;
- whether the finding has been disclosed elsewhere.

## Security boundaries

The current repository provides platform foundations and architecture contracts. It does not yet provide production authentication, authorization, tenant isolation, hardened network ingress, secrets management, or a supported production deployment.

## Local development only

The Docker Compose configuration and `.env.example` contain convenience defaults for local development.

- Never reuse example credentials outside an isolated local environment.
- Do not expose PostgreSQL, Redis, Qdrant, or the FastAPI development server directly to untrusted networks.
- Replace all credentials with secrets supplied by an approved secrets manager before any hosted deployment.
- Add authentication, authorization, TLS, network policy, audit logging, backups, and recovery procedures before production use.
- Review third-party images and dependencies before deployment.

## Secure engineering expectations

- Rust `unsafe_code` remains forbidden unless an explicit security review and architectural decision changes the policy.
- Services must communicate through documented contracts and policy boundaries.
- Sensitive inputs and provider outputs must be treated as untrusted until validated.
- New external interfaces require threat modeling, least-privilege permissions, bounded resource use, and failure-safe behavior.
- Security-relevant changes should include tests and an ADR or RFC when they alter durable architecture.
