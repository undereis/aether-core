# Aether Official Service Map

Status: Approved baseline for Phase 4.

This document defines the planned service map for Aether as a cognitive
operating system. Services are permanent platform responsibilities, not product
features or application screens.

## Engineering Rule #002

No service may know or call another service directly.

All service communication must pass through the Aether Service Bus. Service
dependencies are expressed as capabilities, permissions, resources, and bus
routes. Direct service references, direct service method calls, or shared
service-owned channels are architecture violations.

## Classification

Services are classified as:

- System Service
- Platform Service
- Cognitive Service
- AI Service
- Device Service
- Automation Service
- Enterprise Service
- External Service

## Layer 1: Foundation Services

| Service | Type | Responsibility | Provides | Requires | Probable Permissions | Probable Resources | Priority | Probable Phase | Risks | Allowed Dependencies |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Kernel Service | System Service | Boot, shutdown, lifecycle, and supervision. | `kernel.boot`, `kernel.shutdown`, `kernel.supervise` | `telemetry.emit`, `events.publish` | `telemetry.emit`, `event.publish`, `service.inspect` | CPU low; memory low; storage none; network false | P0 | Phase 4+ | Incorrect supervision destabilizes the platform. | Telemetry Service, Event Service, Health Service |
| Service Registry Service | Platform Service | Register, discover, and inspect services. | `service.registry.read`, `service.registry.write` | `telemetry.emit` | `service.inspect`, `telemetry.emit` | CPU low; memory low; storage low; network false | P0 | Phase 4+ | Bad discovery data can create invalid runtime decisions. | Telemetry Service, Audit Service |
| Event Service | System Service | Internal event publication, subscription, and routing. | `events.publish`, `events.subscribe`, `events.route` | `telemetry.emit` | `event.publish`, `event.subscribe`, `telemetry.emit` | CPU low; memory low; storage none; network false | P0 | Phase 4 | Event storms and ordering ambiguity. | Telemetry Service, Audit Service |
| Configuration Service | System Service | Local configuration, environment, and providers. | `config.read`, `config.validate`, `config.provider.local` | `telemetry.emit` | `config.read`, `telemetry.emit` | CPU low; memory low; storage low; network false; filesystem read-only | P0 | Phase 4 | Configuration drift and accidental secret exposure. | Telemetry Service, Audit Service |
| Permission Service | System Service | Internal permission declaration and enforcement contracts. | `permission.check`, `permission.declare` | `audit.write`, `telemetry.emit` | `service.inspect`, `telemetry.emit` | CPU low; memory low; storage low; network false | P0 | Phase 5+ | Weak enforcement allows uncontrolled service actions. | Audit Service, Telemetry Service |
| Resource Manager Service | System Service | Declare, inspect, and later enforce service resource limits. | `resource.declare`, `resource.inspect`, `resource.enforce` | `service.inspect`, `telemetry.emit` | `service.inspect`, `telemetry.emit` | CPU low; memory low; storage low; network false | P0 | Phase 5+ | Incorrect limits can starve critical services. | Service Registry Service, Telemetry Service |
| Telemetry Service | System Service | Logs, metrics, traces, and operational telemetry contracts. | `telemetry.emit`, `telemetry.query`, `telemetry.health` | `events.publish` | `telemetry.emit`, `event.publish` | CPU low; memory low; storage none; network false | P0 | Phase 4 | Telemetry gaps make incidents opaque. | Event Service |
| Health Service | System Service | Health checks, aggregation, and controlled degradation. | `health.report`, `health.aggregate`, `health.degradation` | `service.inspect`, `telemetry.emit` | `service.inspect`, `telemetry.emit`, `event.publish` | CPU low; memory low; storage none; network false | P0 | Phase 4 | False health signals can hide failures. | Service Registry Service, Telemetry Service, Event Service |
| Audit Service | System Service | Record critical platform actions. | `audit.write`, `audit.query` | `storage.write`, `telemetry.emit` | `telemetry.emit`, `event.subscribe` | CPU low; memory low; storage medium; network false | P1 | Phase 5+ | Incomplete audit trails weaken security. | Storage Service, Telemetry Service |
| Scheduler Service | Platform Service | Future execution, jobs, and scheduled tasks. | `schedule.create`, `schedule.run`, `schedule.cancel` | `events.publish`, `telemetry.emit` | `event.publish`, `telemetry.emit`, `service.command` | CPU low; memory low; storage low; network false | P2 | Phase 6+ | Unsafe scheduling can create runaway work. | Event Service, Telemetry Service, Audit Service |
| Storage Service | Platform Service | Persistence abstractions for platform storage. | `storage.read`, `storage.write`, `storage.migrate` | `telemetry.emit`, `config.read` | `config.read`, `telemetry.emit` | CPU low; memory medium; storage high; network false | P1 | Phase 5+ | Persistence failures can corrupt platform state. | Configuration Service, Telemetry Service, Audit Service |

## Layer 2: Cognitive Services

| Service | Type | Responsibility | Provides | Requires | Probable Permissions | Probable Resources | Priority | Probable Phase | Risks | Allowed Dependencies |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Memory Service | Cognitive Service | Raw, processed, and retrievable memories. | `memory.write`, `memory.read`, `memory.retrieve` | `storage.write`, `embedding.search`, `audit.write` | `service.command`, `telemetry.emit` | CPU medium; memory medium; storage high; network false | P1 | Phase 5+ | Privacy, retention, and retrieval quality errors. | Storage Service, Embedding Service, Audit Service |
| Knowledge Graph Service | Cognitive Service | Entities, relations, and cognitive graph operations. | `knowledge.entity.write`, `knowledge.relation.query` | `storage.write`, `memory.retrieve` | `service.command`, `telemetry.emit` | CPU medium; memory high; storage high; network false | P2 | Phase 6+ | Incorrect relations pollute reasoning. | Storage Service, Memory Service, Telemetry Service |
| Context Service | Cognitive Service | Current user, project, and session context. | `context.read`, `context.update`, `context.snapshot` | `memory.retrieve`, `service.inspect` | `service.inspect`, `telemetry.emit` | CPU low; memory medium; storage medium; network false | P1 | Phase 5+ | Stale context misleads workflows. | Memory Service, Service Registry Service, Telemetry Service |
| Embedding Service | Cognitive Service | Embeddings and vector search contracts. | `embedding.create`, `embedding.search`, `vector.query` | `storage.write`, `model.registry.read` | `service.command`, `telemetry.emit` | CPU medium; memory medium; storage high; network false | P1 | Phase 5+ | Embedding drift and vector index inconsistency. | Storage Service, Model Registry Service, Telemetry Service |
| Reasoning Service | Cognitive Service | Reasoning, response synthesis, and evidence-based output. | `reasoning.execute`, `reasoning.summarize` | `context.read`, `inference.run`, `memory.retrieve` | `service.command`, `telemetry.emit` | CPU medium; memory high; storage low; network false | P2 | Phase 6+ | Unverifiable reasoning produces unreliable conclusions. | Context Service, Inference Service, Memory Service |
| Learning Service | Cognitive Service | Recurring patterns and behavioral learning signals. | `learning.observe`, `learning.patterns.query` | `memory.read`, `audit.write` | `telemetry.emit`, `service.inspect` | CPU medium; memory medium; storage high; network false | P3 | Phase 7+ | Learning from noisy or sensitive behavior reduces trust. | Memory Service, Audit Service, Telemetry Service |
| Planning Service | Cognitive Service | Plans, tasks, and decomposition of objectives. | `planning.create`, `planning.decompose`, `planning.track` | `reasoning.execute`, `context.read` | `service.command`, `telemetry.emit` | CPU medium; memory medium; storage medium; network false | P3 | Phase 7+ | Poor decomposition can cause unsafe automation chains. | Reasoning Service, Context Service, Audit Service |
| Decision Service | Cognitive Service | Decision support based on context and evidence. | `decision.support`, `decision.compare` | `reasoning.execute`, `knowledge.relation.query` | `service.command`, `telemetry.emit` | CPU medium; memory medium; storage low; network false | P3 | Phase 7+ | Recommendations without evidence can be unsafe. | Reasoning Service, Knowledge Graph Service, Audit Service |

## Layer 3: AI Services

| Service | Type | Responsibility | Provides | Requires | Probable Permissions | Probable Resources | Priority | Probable Phase | Risks | Allowed Dependencies |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| LLM Service | AI Service | Communication with local and external language models. | `llm.chat`, `llm.complete`, `llm.stream` | `model.registry.read`, `policy.evaluate` | `telemetry.emit`, `network.request` | CPU medium; memory medium; storage low; network conditional | P2 | Phase 6+ | External data exposure, latency, provider instability. | Model Registry Service, Policy Service, Telemetry Service |
| Model Registry Service | AI Service | Register available local and external models. | `model.registry.read`, `model.registry.write` | `config.read`, `telemetry.emit` | `config.read`, `telemetry.emit` | CPU low; memory low; storage medium; network false | P2 | Phase 6+ | Incorrect model metadata routes work unsafely. | Configuration Service, Telemetry Service |
| Prompt Service | AI Service | Prompt templates, policies, and versions. | `prompt.render`, `prompt.version`, `prompt.policy` | `storage.read`, `policy.evaluate` | `telemetry.emit`, `service.inspect` | CPU low; memory low; storage medium; network false | P2 | Phase 6+ | Prompt drift changes behavior without code changes. | Storage Service, Policy Service, Audit Service |
| Inference Service | AI Service | Standardized inference execution. | `inference.run`, `inference.stream` | `llm.chat`, `prompt.render`, `telemetry.emit` | `service.command`, `telemetry.emit` | CPU medium; memory medium; storage low; network conditional | P2 | Phase 6+ | Unbounded inference increases cost and latency. | LLM Service, Prompt Service, Telemetry Service |
| Agent Runtime Service | AI Service | Safe execution of agents. | `agent.run`, `agent.stop`, `agent.sandbox` | `permission.check`, `resource.enforce`, `audit.write` | `service.command`, `telemetry.emit` | CPU high; memory high; storage medium; network conditional | P4 | Phase 8+ | Unsafe agents can execute beyond declared permissions. | Permission Service, Resource Manager Service, Audit Service |
| Agent Factory Service | AI Service | Suggest and create specialized agents. | `agent.factory.suggest`, `agent.factory.create` | `agent.run`, `skill.query`, `policy.evaluate` | `service.command`, `telemetry.emit` | CPU medium; memory medium; storage medium; network false | P4 | Phase 8+ | Agent generation without policy can be unsafe. | Agent Runtime Service, Skill Service, Policy Service |
| Skill Service | AI Service | Reusable skills and skill metadata. | `skill.query`, `skill.execute`, `skill.register` | `storage.read`, `permission.check` | `service.command`, `telemetry.emit` | CPU medium; memory medium; storage medium; network conditional | P3 | Phase 7+ | Unverified skills create security and quality risks. | Storage Service, Permission Service, Audit Service |

## Layer 4: Interaction Services

| Service | Type | Responsibility | Provides | Requires | Probable Permissions | Probable Resources | Priority | Probable Phase | Risks | Allowed Dependencies |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Chat Service | Platform Service | Conversational interface contract. | `chat.session`, `chat.message` | `context.read`, `reasoning.execute` | `service.command`, `telemetry.emit` | CPU low; memory medium; storage medium; network false | P3 | Phase 7+ | Conversation history may include sensitive content. | Context Service, Reasoning Service, Audit Service |
| Voice Service | Platform Service | Voice input and output. | `voice.input`, `voice.output` | `microphone.read`, `telemetry.emit` | `device.microphone`, `telemetry.emit` | CPU medium; memory medium; storage low; network conditional | P4 | Phase 8+ | Voice capture creates privacy and consent risk. | Microphone Service, Permission Service, Audit Service |
| Vision Service | Platform Service | Visual analysis contracts. | `vision.analyze`, `vision.describe` | `screen.read`, `camera.read`, `inference.run` | `device.screen`, `device.camera`, `telemetry.emit` | CPU high; memory high; storage medium; network conditional | P4 | Phase 8+ | Visual data can reveal private information. | Screen Service, Camera Service, Inference Service |
| OCR Service | Platform Service | Text extraction from images and documents. | `ocr.extract`, `ocr.layout` | `filesystem.read`, `vision.analyze` | `filesystem.read`, `telemetry.emit` | CPU medium; memory medium; storage low; network false | P4 | Phase 8+ | Extracted text can include sensitive data. | Filesystem Service, Vision Service, Audit Service |
| Notification Service | Platform Service | Internal and external notifications. | `notification.send`, `notification.schedule` | `policy.evaluate`, `telemetry.emit` | `service.command`, `telemetry.emit` | CPU low; memory low; storage low; network conditional | P3 | Phase 7+ | Noisy or unauthorized notifications reduce trust. | Policy Service, Telemetry Service, Audit Service |
| Translation Service | Platform Service | Translation and language adaptation. | `translation.translate`, `translation.localize` | `inference.run`, `model.registry.read` | `service.command`, `telemetry.emit` | CPU medium; memory medium; storage low; network conditional | P4 | Phase 8+ | Poor translation can distort intent. | Inference Service, Model Registry Service, Telemetry Service |

## Layer 5: Device Services

| Service | Type | Responsibility | Provides | Requires | Probable Permissions | Probable Resources | Priority | Probable Phase | Risks | Allowed Dependencies |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Filesystem Service | Device Service | Supervised file access. | `filesystem.read`, `filesystem.write`, `filesystem.watch` | `permission.check`, `audit.write` | `filesystem.read`, `filesystem.write`, `telemetry.emit` | CPU low; memory low; storage conditional; network false; filesystem conditional | P2 | Phase 6+ | File access can expose or modify sensitive user data. | Permission Service, Audit Service, Telemetry Service |
| Clipboard Service | Device Service | Supervised clipboard access. | `clipboard.read`, `clipboard.write` | `permission.check`, `audit.write` | `device.clipboard`, `telemetry.emit` | CPU low; memory low; storage none; network false | P3 | Phase 7+ | Clipboard contents commonly include secrets. | Permission Service, Audit Service |
| Screen Service | Device Service | Supervised screen reading. | `screen.read`, `screen.region.read` | `permission.check`, `audit.write` | `device.screen`, `telemetry.emit` | CPU medium; memory medium; storage none; network false | P4 | Phase 8+ | Screen capture can expose private application data. | Permission Service, Audit Service, Telemetry Service |
| Camera Service | Device Service | Supervised camera use. | `camera.read`, `camera.status` | `permission.check`, `audit.write` | `device.camera`, `telemetry.emit` | CPU medium; memory medium; storage none; network false | P4 | Phase 8+ | Camera use requires explicit consent and auditing. | Permission Service, Audit Service |
| Microphone Service | Device Service | Supervised microphone use. | `microphone.read`, `microphone.status` | `permission.check`, `audit.write` | `device.microphone`, `telemetry.emit` | CPU medium; memory medium; storage none; network false | P4 | Phase 8+ | Audio capture has high privacy sensitivity. | Permission Service, Audit Service |
| Network Service | Device Service | Network state and policies. | `network.status`, `network.policy` | `permission.check`, `audit.write` | `network.inspect`, `telemetry.emit` | CPU low; memory low; storage low; network true | P3 | Phase 7+ | Policy errors can leak data or block critical work. | Permission Service, Audit Service, Telemetry Service |
| Browser Context Service | Device Service | Authorized browser context access. | `browser.context.read`, `browser.context.snapshot` | `permission.check`, `audit.write` | `browser.context`, `telemetry.emit` | CPU medium; memory medium; storage medium; network conditional | P4 | Phase 8+ | Browser context may include credentials and private data. | Permission Service, Audit Service, Browser Automation Service |

## Layer 6: Automation Services

| Service | Type | Responsibility | Provides | Requires | Probable Permissions | Probable Resources | Priority | Probable Phase | Risks | Allowed Dependencies |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Workflow Service | Automation Service | Workflow definitions and execution coordination. | `workflow.create`, `workflow.run`, `workflow.inspect` | `permission.check`, `scheduler.run` | `service.command`, `telemetry.emit` | CPU medium; memory medium; storage medium; network conditional | P4 | Phase 8+ | Workflows can chain actions with broad side effects. | Permission Service, Scheduler Service, Audit Service |
| Macro Service | Automation Service | Local macros. | `macro.record`, `macro.run` | `permission.check`, `desktop.automation` | `service.command`, `telemetry.emit` | CPU medium; memory medium; storage medium; network false | P4 | Phase 8+ | Macros can replay unsafe local actions. | Permission Service, Desktop Automation Service, Audit Service |
| Browser Automation Service | Automation Service | Authorized browser automation. | `browser.automation.run`, `browser.automation.inspect` | `browser.context.read`, `permission.check` | `browser.automation`, `telemetry.emit` | CPU medium; memory medium; storage medium; network conditional | P4 | Phase 8+ | Browser automation can perform external actions. | Browser Context Service, Permission Service, Audit Service |
| Desktop Automation Service | Automation Service | Authorized desktop automation. | `desktop.automation`, `desktop.action.run` | `permission.check`, `screen.read` | `desktop.automation`, `telemetry.emit` | CPU medium; memory medium; storage low; network false | P4 | Phase 8+ | Desktop automation can affect arbitrary applications. | Permission Service, Screen Service, Audit Service |
| Integration Service | External Service | Integrations with external APIs. | `integration.call`, `integration.registry` | `network.policy`, `permission.check` | `network.request`, `telemetry.emit` | CPU medium; memory medium; storage medium; network true | P4 | Phase 8+ | External API calls can leak data or create cost exposure. | Network Service, Permission Service, Audit Service |

## Layer 7: Enterprise Services

| Service | Type | Responsibility | Provides | Requires | Probable Permissions | Probable Resources | Priority | Probable Phase | Risks | Allowed Dependencies |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| API Gateway Service | Enterprise Service | External API entrypoint for future hosted deployments. | `api.gateway.route`, `api.gateway.limit` | `policy.evaluate`, `audit.write` | `network.request`, `telemetry.emit` | CPU medium; memory medium; storage low; network true | P5 | Enterprise future | Public ingress expands attack surface. | Policy Service, Audit Service, Telemetry Service |
| Multi-Tenant Service | Enterprise Service | Tenant isolation and tenant-scoped metadata. | `tenant.resolve`, `tenant.isolate` | `organization.read`, `policy.evaluate` | `service.inspect`, `telemetry.emit` | CPU medium; memory medium; storage high; network true | P5 | Enterprise future | Tenant isolation failures expose cross-tenant data. | Organization Service, Policy Service, Audit Service |
| Organization Service | Enterprise Service | Organizations, users, and organizational metadata. | `organization.read`, `organization.write` | `storage.write`, `audit.write` | `telemetry.emit`, `service.inspect` | CPU medium; memory medium; storage high; network true | P5 | Enterprise future | Organization data errors impact access boundaries. | Storage Service, Audit Service, Policy Service |
| Billing Service | Enterprise Service | Billing and usage accounting. | `billing.usage`, `billing.invoice` | `organization.read`, `audit.write` | `network.request`, `telemetry.emit` | CPU medium; memory medium; storage high; network true | P5 | Enterprise future | Billing errors create financial and trust issues. | Organization Service, Audit Service, Integration Service |
| Licensing Service | Enterprise Service | License state and entitlement checks. | `license.check`, `license.activate` | `organization.read`, `policy.evaluate` | `network.request`, `telemetry.emit` | CPU low; memory low; storage medium; network true | P5 | Enterprise future | License failures block legitimate use or allow unauthorized use. | Organization Service, Policy Service, Audit Service |
| Policy Service | Enterprise Service | Policy evaluation across platform and enterprise contexts. | `policy.evaluate`, `policy.read`, `policy.write` | `audit.write`, `storage.read` | `telemetry.emit`, `service.inspect` | CPU medium; memory medium; storage medium; network false | P4 | Phase 8+ | Incorrect policy decisions can allow unsafe actions. | Audit Service, Storage Service, Organization Service |
| Compliance Service | Enterprise Service | Compliance evidence and reporting. | `compliance.report`, `compliance.evidence` | `audit.query`, `policy.read` | `service.inspect`, `telemetry.emit` | CPU medium; memory medium; storage high; network conditional | P5 | Enterprise future | Incomplete evidence can fail compliance audits. | Audit Service, Policy Service, Storage Service |
| Plugin Marketplace Service | Enterprise Service | Plugin discovery, distribution, and policy gatekeeping. | `plugin.marketplace.search`, `plugin.marketplace.install` | `license.check`, `policy.evaluate`, `audit.write` | `network.request`, `telemetry.emit` | CPU medium; memory medium; storage high; network true | P5 | Enterprise future | Untrusted plugins can compromise platform integrity. | Licensing Service, Policy Service, Audit Service |

## Phase 4 Implementation Boundary

Phase 4 implements only the base for these Foundation services:

- Telemetry Service
- Configuration Service
- Health Service
- Event Service
- Service Inspector Service

The remaining services are architectural commitments and are not implemented in
Phase 4.
