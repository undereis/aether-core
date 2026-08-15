#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Official service map and base system services for Aether.

use std::sync::Arc;

use aether_core::{Capability, LifecycleStatus};
use aether_kernel::{AetherKernel, KernelError};
use aether_service::{ServiceDescriptor, ServiceError, ServiceHealthStatus, ServiceManifest};
use aether_service_bus::{
    CommandHandler, ServiceBusClient, ServiceBusError, ServiceCommand, ServiceReply,
};
use serde_json::json;
use thiserror::Error;

/// Number of core system services introduced in Phase 4.
pub const CORE_SYSTEM_SERVICE_COUNT: usize = 5;

const TELEMETRY_SERVICE_MANIFEST: &str =
    include_str!("../../../services/telemetry-service/service.toml");
const CONFIGURATION_SERVICE_MANIFEST: &str =
    include_str!("../../../services/configuration-service/service.toml");
const HEALTH_SERVICE_MANIFEST: &str = include_str!("../../../services/health-service/service.toml");
const EVENT_SERVICE_MANIFEST: &str = include_str!("../../../services/event-service/service.toml");
const SERVICE_INSPECTOR_MANIFEST: &str =
    include_str!("../../../services/service-inspector/service.toml");

/// Official service map layer.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ServiceLayer {
    /// Foundation services required by the platform itself.
    Foundation,
    /// Cognitive services planned for memory, context, knowledge, and reasoning.
    Cognitive,
    /// AI services planned for model, prompt, inference, agent, and skill runtime.
    Ai,
    /// Interaction services planned for conversation, voice, vision, and notification.
    Interaction,
    /// Device services planned for supervised local device access.
    Device,
    /// Automation services planned for workflow and controlled automation.
    Automation,
    /// Enterprise services planned for commercial and organizational scenarios.
    Enterprise,
}

impl ServiceLayer {
    /// Return the canonical layer name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Foundation => "Foundation Services",
            Self::Cognitive => "Cognitive Services",
            Self::Ai => "AI Services",
            Self::Interaction => "Interaction Services",
            Self::Device => "Device Services",
            Self::Automation => "Automation Services",
            Self::Enterprise => "Enterprise Services",
        }
    }
}

/// Official service classification.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ServiceType {
    /// System-level service.
    System,
    /// Platform-level service.
    Platform,
    /// Cognitive service.
    Cognitive,
    /// AI service.
    Ai,
    /// Device service.
    Device,
    /// Automation service.
    Automation,
    /// Enterprise service.
    Enterprise,
    /// External integration service.
    External,
}

impl ServiceType {
    /// Return the canonical service type name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::System => "System Service",
            Self::Platform => "Platform Service",
            Self::Cognitive => "Cognitive Service",
            Self::Ai => "AI Service",
            Self::Device => "Device Service",
            Self::Automation => "Automation Service",
            Self::Enterprise => "Enterprise Service",
            Self::External => "External Service",
        }
    }
}

/// Official service definition used by the service map.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OfficialServiceDefinition {
    /// Human-readable service name.
    pub name: &'static str,
    /// Service layer.
    pub layer: ServiceLayer,
    /// Service classification.
    pub service_type: ServiceType,
    /// Permanent platform responsibility.
    pub responsibility: &'static str,
    /// Capabilities provided by the service.
    pub capabilities_provided: &'static [&'static str],
    /// Capabilities required by the service.
    pub capabilities_required: &'static [&'static str],
    /// Probable permissions for the service.
    pub probable_permissions: &'static [&'static str],
    /// Probable resources for the service.
    pub probable_resources: &'static str,
    /// Service priority.
    pub priority: &'static str,
    /// Probable future implementation phase.
    pub probable_phase: &'static str,
    /// Primary risks.
    pub risks: &'static str,
    /// Dependencies allowed by architecture.
    pub allowed_dependencies: &'static [&'static str],
}

/// Official Aether service map.
pub const OFFICIAL_SERVICE_MAP: &[OfficialServiceDefinition] = &[
    OfficialServiceDefinition {
        name: "Kernel Service",
        layer: ServiceLayer::Foundation,
        service_type: ServiceType::System,
        responsibility: "Boot, shutdown, lifecycle orchestration, and supervision.",
        capabilities_provided: &["kernel.boot", "kernel.shutdown", "kernel.supervise"],
        capabilities_required: &["telemetry.emit", "events.publish"],
        probable_permissions: &["telemetry.emit", "event.publish", "service.inspect"],
        probable_resources: "cpu=low; memory=low; storage=none; network=false",
        priority: "P0",
        probable_phase: "Phase 4+",
        risks: "Incorrect supervision can destabilize every service.",
        allowed_dependencies: &["Telemetry Service", "Event Service", "Health Service"],
    },
    OfficialServiceDefinition {
        name: "Service Registry Service",
        layer: ServiceLayer::Foundation,
        service_type: ServiceType::Platform,
        responsibility: "Register, discover, and inspect service descriptors.",
        capabilities_provided: &["service.registry.read", "service.registry.write"],
        capabilities_required: &["telemetry.emit"],
        probable_permissions: &["service.inspect", "telemetry.emit"],
        probable_resources: "cpu=low; memory=low; storage=low; network=false",
        priority: "P0",
        probable_phase: "Phase 4+",
        risks: "Bad discovery data can create invalid runtime decisions.",
        allowed_dependencies: &["Telemetry Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Event Service",
        layer: ServiceLayer::Foundation,
        service_type: ServiceType::System,
        responsibility: "Internal event publication, subscription, and routing.",
        capabilities_provided: &["events.publish", "events.subscribe", "events.route"],
        capabilities_required: &["telemetry.emit"],
        probable_permissions: &["event.publish", "event.subscribe", "telemetry.emit"],
        probable_resources: "cpu=low; memory=low; storage=none; network=false",
        priority: "P0",
        probable_phase: "Phase 4",
        risks: "Event storms, ordering ambiguity, and unbounded subscribers.",
        allowed_dependencies: &["Telemetry Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Configuration Service",
        layer: ServiceLayer::Foundation,
        service_type: ServiceType::System,
        responsibility: "Local configuration, environment values, and providers.",
        capabilities_provided: &["config.read", "config.validate", "config.provider.local"],
        capabilities_required: &["telemetry.emit"],
        probable_permissions: &["config.read", "telemetry.emit"],
        probable_resources: "cpu=low; memory=low; storage=low; network=false; filesystem=read_only",
        priority: "P0",
        probable_phase: "Phase 4",
        risks: "Configuration drift and accidental exposure of sensitive values.",
        allowed_dependencies: &["Telemetry Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Permission Service",
        layer: ServiceLayer::Foundation,
        service_type: ServiceType::System,
        responsibility: "Internal permission declaration and enforcement contracts.",
        capabilities_provided: &["permission.check", "permission.declare"],
        capabilities_required: &["audit.write", "telemetry.emit"],
        probable_permissions: &["service.inspect", "telemetry.emit"],
        probable_resources: "cpu=low; memory=low; storage=low; network=false",
        priority: "P0",
        probable_phase: "Phase 5+",
        risks: "Weak enforcement can allow uncontrolled service actions.",
        allowed_dependencies: &["Audit Service", "Telemetry Service"],
    },
    OfficialServiceDefinition {
        name: "Resource Manager Service",
        layer: ServiceLayer::Foundation,
        service_type: ServiceType::System,
        responsibility: "Declare, inspect, and later enforce service resource limits.",
        capabilities_provided: &["resource.declare", "resource.inspect", "resource.enforce"],
        capabilities_required: &["service.inspect", "telemetry.emit"],
        probable_permissions: &["service.inspect", "telemetry.emit"],
        probable_resources: "cpu=low; memory=low; storage=low; network=false",
        priority: "P0",
        probable_phase: "Phase 5+",
        risks: "Incorrect limits can starve critical services or fail to sandbox risky services.",
        allowed_dependencies: &["Service Registry Service", "Telemetry Service"],
    },
    OfficialServiceDefinition {
        name: "Telemetry Service",
        layer: ServiceLayer::Foundation,
        service_type: ServiceType::System,
        responsibility: "Logs, metrics, traces, and operational telemetry contracts.",
        capabilities_provided: &["telemetry.emit", "telemetry.query", "telemetry.health"],
        capabilities_required: &["events.publish"],
        probable_permissions: &["telemetry.emit", "event.publish"],
        probable_resources: "cpu=low; memory=low; storage=none; network=false",
        priority: "P0",
        probable_phase: "Phase 4",
        risks: "Telemetry gaps make incidents opaque and hard to diagnose.",
        allowed_dependencies: &["Event Service"],
    },
    OfficialServiceDefinition {
        name: "Health Service",
        layer: ServiceLayer::Foundation,
        service_type: ServiceType::System,
        responsibility: "Health checks, aggregation, and controlled degradation.",
        capabilities_provided: &["health.report", "health.aggregate", "health.degradation"],
        capabilities_required: &["service.inspect", "telemetry.emit"],
        probable_permissions: &["service.inspect", "telemetry.emit", "event.publish"],
        probable_resources: "cpu=low; memory=low; storage=none; network=false",
        priority: "P0",
        probable_phase: "Phase 4",
        risks: "False health signals can hide failures or cause unnecessary shutdowns.",
        allowed_dependencies: &[
            "Service Registry Service",
            "Telemetry Service",
            "Event Service",
        ],
    },
    OfficialServiceDefinition {
        name: "Audit Service",
        layer: ServiceLayer::Foundation,
        service_type: ServiceType::System,
        responsibility: "Record critical platform actions for accountability.",
        capabilities_provided: &["audit.write", "audit.query"],
        capabilities_required: &["storage.write", "telemetry.emit"],
        probable_permissions: &["telemetry.emit", "event.subscribe"],
        probable_resources: "cpu=low; memory=low; storage=medium; network=false",
        priority: "P1",
        probable_phase: "Phase 5+",
        risks: "Incomplete audit trails can weaken security and compliance.",
        allowed_dependencies: &["Storage Service", "Telemetry Service"],
    },
    OfficialServiceDefinition {
        name: "Scheduler Service",
        layer: ServiceLayer::Foundation,
        service_type: ServiceType::Platform,
        responsibility: "Future execution, jobs, and scheduled tasks.",
        capabilities_provided: &["schedule.create", "schedule.run", "schedule.cancel"],
        capabilities_required: &["events.publish", "telemetry.emit"],
        probable_permissions: &["event.publish", "telemetry.emit", "service.command"],
        probable_resources: "cpu=low; memory=low; storage=low; network=false",
        priority: "P2",
        probable_phase: "Phase 6+",
        risks: "Unsafe scheduling can create runaway or duplicated work.",
        allowed_dependencies: &["Event Service", "Telemetry Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Storage Service",
        layer: ServiceLayer::Foundation,
        service_type: ServiceType::Platform,
        responsibility: "Persistence abstractions for platform storage.",
        capabilities_provided: &["storage.read", "storage.write", "storage.migrate"],
        capabilities_required: &["telemetry.emit", "config.read"],
        probable_permissions: &["config.read", "telemetry.emit"],
        probable_resources: "cpu=low; memory=medium; storage=high; network=false",
        priority: "P1",
        probable_phase: "Phase 5+",
        risks: "Persistence failures can corrupt platform state.",
        allowed_dependencies: &[
            "Configuration Service",
            "Telemetry Service",
            "Audit Service",
        ],
    },
    OfficialServiceDefinition {
        name: "Memory Service",
        layer: ServiceLayer::Cognitive,
        service_type: ServiceType::Cognitive,
        responsibility: "Raw, processed, and retrievable memories.",
        capabilities_provided: &["memory.write", "memory.read", "memory.retrieve"],
        capabilities_required: &["storage.write", "embedding.search", "audit.write"],
        probable_permissions: &["service.command", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=high; network=false",
        priority: "P1",
        probable_phase: "Phase 5+",
        risks: "Privacy, retention, and retrieval quality errors.",
        allowed_dependencies: &["Storage Service", "Embedding Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Knowledge Graph Service",
        layer: ServiceLayer::Cognitive,
        service_type: ServiceType::Cognitive,
        responsibility: "Entities, relations, and cognitive graph operations.",
        capabilities_provided: &["knowledge.entity.write", "knowledge.relation.query"],
        capabilities_required: &["storage.write", "memory.retrieve"],
        probable_permissions: &["service.command", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=high; storage=high; network=false",
        priority: "P2",
        probable_phase: "Phase 6+",
        risks: "Incorrect relations can pollute reasoning and memory retrieval.",
        allowed_dependencies: &["Storage Service", "Memory Service", "Telemetry Service"],
    },
    OfficialServiceDefinition {
        name: "Context Service",
        layer: ServiceLayer::Cognitive,
        service_type: ServiceType::Cognitive,
        responsibility: "Current user, project, and session context.",
        capabilities_provided: &["context.read", "context.update", "context.snapshot"],
        capabilities_required: &["memory.retrieve", "service.inspect"],
        probable_permissions: &["service.inspect", "telemetry.emit"],
        probable_resources: "cpu=low; memory=medium; storage=medium; network=false",
        priority: "P1",
        probable_phase: "Phase 5+",
        risks: "Stale context can mislead cognitive workflows.",
        allowed_dependencies: &[
            "Memory Service",
            "Service Registry Service",
            "Telemetry Service",
        ],
    },
    OfficialServiceDefinition {
        name: "Embedding Service",
        layer: ServiceLayer::Cognitive,
        service_type: ServiceType::Cognitive,
        responsibility: "Embeddings and vector search contracts.",
        capabilities_provided: &["embedding.create", "embedding.search", "vector.query"],
        capabilities_required: &["storage.write", "model.registry.read"],
        probable_permissions: &["service.command", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=high; network=false",
        priority: "P1",
        probable_phase: "Phase 5+",
        risks: "Embedding drift and vector index inconsistency.",
        allowed_dependencies: &[
            "Storage Service",
            "Model Registry Service",
            "Telemetry Service",
        ],
    },
    OfficialServiceDefinition {
        name: "Reasoning Service",
        layer: ServiceLayer::Cognitive,
        service_type: ServiceType::Cognitive,
        responsibility: "Reasoning, response synthesis, and evidence-based output.",
        capabilities_provided: &["reasoning.execute", "reasoning.summarize"],
        capabilities_required: &["context.read", "inference.run", "memory.retrieve"],
        probable_permissions: &["service.command", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=high; storage=low; network=false",
        priority: "P2",
        probable_phase: "Phase 6+",
        risks: "Unverifiable reasoning can produce unreliable conclusions.",
        allowed_dependencies: &["Context Service", "Inference Service", "Memory Service"],
    },
    OfficialServiceDefinition {
        name: "Learning Service",
        layer: ServiceLayer::Cognitive,
        service_type: ServiceType::Cognitive,
        responsibility: "Recurring patterns and behavioral learning signals.",
        capabilities_provided: &["learning.observe", "learning.patterns.query"],
        capabilities_required: &["memory.read", "audit.write"],
        probable_permissions: &["telemetry.emit", "service.inspect"],
        probable_resources: "cpu=medium; memory=medium; storage=high; network=false",
        priority: "P3",
        probable_phase: "Phase 7+",
        risks: "Learning from noisy or sensitive behavior can reduce trust.",
        allowed_dependencies: &["Memory Service", "Audit Service", "Telemetry Service"],
    },
    OfficialServiceDefinition {
        name: "Planning Service",
        layer: ServiceLayer::Cognitive,
        service_type: ServiceType::Cognitive,
        responsibility: "Plans, tasks, and decomposition of objectives.",
        capabilities_provided: &["planning.create", "planning.decompose", "planning.track"],
        capabilities_required: &["reasoning.execute", "context.read"],
        probable_permissions: &["service.command", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=medium; network=false",
        priority: "P3",
        probable_phase: "Phase 7+",
        risks: "Poor decomposition can cause unsafe automation chains.",
        allowed_dependencies: &["Reasoning Service", "Context Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Decision Service",
        layer: ServiceLayer::Cognitive,
        service_type: ServiceType::Cognitive,
        responsibility: "Decision support based on context and evidence.",
        capabilities_provided: &["decision.support", "decision.compare"],
        capabilities_required: &["reasoning.execute", "knowledge.relation.query"],
        probable_permissions: &["service.command", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=low; network=false",
        priority: "P3",
        probable_phase: "Phase 7+",
        risks: "Recommendations without traceable evidence can be unsafe.",
        allowed_dependencies: &[
            "Reasoning Service",
            "Knowledge Graph Service",
            "Audit Service",
        ],
    },
    OfficialServiceDefinition {
        name: "LLM Service",
        layer: ServiceLayer::Ai,
        service_type: ServiceType::Ai,
        responsibility: "Communication with local and external language models.",
        capabilities_provided: &["llm.chat", "llm.complete", "llm.stream"],
        capabilities_required: &["model.registry.read", "policy.evaluate"],
        probable_permissions: &["telemetry.emit", "network.request"],
        probable_resources: "cpu=medium; memory=medium; storage=low; network=conditional",
        priority: "P2",
        probable_phase: "Phase 6+",
        risks: "External data exposure, latency, and provider instability.",
        allowed_dependencies: &[
            "Model Registry Service",
            "Policy Service",
            "Telemetry Service",
        ],
    },
    OfficialServiceDefinition {
        name: "Model Registry Service",
        layer: ServiceLayer::Ai,
        service_type: ServiceType::Ai,
        responsibility: "Register available local and external models.",
        capabilities_provided: &["model.registry.read", "model.registry.write"],
        capabilities_required: &["config.read", "telemetry.emit"],
        probable_permissions: &["config.read", "telemetry.emit"],
        probable_resources: "cpu=low; memory=low; storage=medium; network=false",
        priority: "P2",
        probable_phase: "Phase 6+",
        risks: "Incorrect model metadata can route work to unsafe models.",
        allowed_dependencies: &["Configuration Service", "Telemetry Service"],
    },
    OfficialServiceDefinition {
        name: "Prompt Service",
        layer: ServiceLayer::Ai,
        service_type: ServiceType::Ai,
        responsibility: "Prompt templates, policies, and prompt versioning.",
        capabilities_provided: &["prompt.render", "prompt.version", "prompt.policy"],
        capabilities_required: &["storage.read", "policy.evaluate"],
        probable_permissions: &["telemetry.emit", "service.inspect"],
        probable_resources: "cpu=low; memory=low; storage=medium; network=false",
        priority: "P2",
        probable_phase: "Phase 6+",
        risks: "Prompt drift can change behavior without code changes.",
        allowed_dependencies: &["Storage Service", "Policy Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Inference Service",
        layer: ServiceLayer::Ai,
        service_type: ServiceType::Ai,
        responsibility: "Standardized inference execution.",
        capabilities_provided: &["inference.run", "inference.stream"],
        capabilities_required: &["llm.chat", "prompt.render", "telemetry.emit"],
        probable_permissions: &["service.command", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=low; network=conditional",
        priority: "P2",
        probable_phase: "Phase 6+",
        risks: "Unbounded inference can increase cost and latency.",
        allowed_dependencies: &["LLM Service", "Prompt Service", "Telemetry Service"],
    },
    OfficialServiceDefinition {
        name: "Agent Runtime Service",
        layer: ServiceLayer::Ai,
        service_type: ServiceType::Ai,
        responsibility: "Safe execution of agents.",
        capabilities_provided: &["agent.run", "agent.stop", "agent.sandbox"],
        capabilities_required: &["permission.check", "resource.enforce", "audit.write"],
        probable_permissions: &["service.command", "telemetry.emit"],
        probable_resources: "cpu=high; memory=high; storage=medium; network=conditional",
        priority: "P4",
        probable_phase: "Phase 8+",
        risks: "Unsafe agents can execute actions beyond declared permissions.",
        allowed_dependencies: &[
            "Permission Service",
            "Resource Manager Service",
            "Audit Service",
        ],
    },
    OfficialServiceDefinition {
        name: "Agent Factory Service",
        layer: ServiceLayer::Ai,
        service_type: ServiceType::Ai,
        responsibility: "Suggest and create specialized agents.",
        capabilities_provided: &["agent.factory.suggest", "agent.factory.create"],
        capabilities_required: &["agent.run", "skill.query", "policy.evaluate"],
        probable_permissions: &["service.command", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=medium; network=false",
        priority: "P4",
        probable_phase: "Phase 8+",
        risks: "Agent generation without policy can produce unsafe behavior.",
        allowed_dependencies: &["Agent Runtime Service", "Skill Service", "Policy Service"],
    },
    OfficialServiceDefinition {
        name: "Skill Service",
        layer: ServiceLayer::Ai,
        service_type: ServiceType::Ai,
        responsibility: "Reusable skills and skill metadata.",
        capabilities_provided: &["skill.query", "skill.execute", "skill.register"],
        capabilities_required: &["storage.read", "permission.check"],
        probable_permissions: &["service.command", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=medium; network=conditional",
        priority: "P3",
        probable_phase: "Phase 7+",
        risks: "Unverified skills can create security and quality issues.",
        allowed_dependencies: &["Storage Service", "Permission Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Chat Service",
        layer: ServiceLayer::Interaction,
        service_type: ServiceType::Platform,
        responsibility: "Conversational interface contract.",
        capabilities_provided: &["chat.session", "chat.message"],
        capabilities_required: &["context.read", "reasoning.execute"],
        probable_permissions: &["service.command", "telemetry.emit"],
        probable_resources: "cpu=low; memory=medium; storage=medium; network=false",
        priority: "P3",
        probable_phase: "Phase 7+",
        risks: "Conversation history may include sensitive content.",
        allowed_dependencies: &["Context Service", "Reasoning Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Voice Service",
        layer: ServiceLayer::Interaction,
        service_type: ServiceType::Platform,
        responsibility: "Voice input and output.",
        capabilities_provided: &["voice.input", "voice.output"],
        capabilities_required: &["microphone.read", "telemetry.emit"],
        probable_permissions: &["device.microphone", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=low; network=conditional",
        priority: "P4",
        probable_phase: "Phase 8+",
        risks: "Voice capture creates privacy and consent risks.",
        allowed_dependencies: &["Microphone Service", "Permission Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Vision Service",
        layer: ServiceLayer::Interaction,
        service_type: ServiceType::Platform,
        responsibility: "Visual analysis contracts.",
        capabilities_provided: &["vision.analyze", "vision.describe"],
        capabilities_required: &["screen.read", "camera.read", "inference.run"],
        probable_permissions: &["device.screen", "device.camera", "telemetry.emit"],
        probable_resources: "cpu=high; memory=high; storage=medium; network=conditional",
        priority: "P4",
        probable_phase: "Phase 8+",
        risks: "Visual data can reveal private user information.",
        allowed_dependencies: &["Screen Service", "Camera Service", "Inference Service"],
    },
    OfficialServiceDefinition {
        name: "OCR Service",
        layer: ServiceLayer::Interaction,
        service_type: ServiceType::Platform,
        responsibility: "Text extraction from images and documents.",
        capabilities_provided: &["ocr.extract", "ocr.layout"],
        capabilities_required: &["filesystem.read", "vision.analyze"],
        probable_permissions: &["filesystem.read", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=low; network=false",
        priority: "P4",
        probable_phase: "Phase 8+",
        risks: "Extracted text can include sensitive data.",
        allowed_dependencies: &["Filesystem Service", "Vision Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Notification Service",
        layer: ServiceLayer::Interaction,
        service_type: ServiceType::Platform,
        responsibility: "Internal and external notifications.",
        capabilities_provided: &["notification.send", "notification.schedule"],
        capabilities_required: &["policy.evaluate", "telemetry.emit"],
        probable_permissions: &["service.command", "telemetry.emit"],
        probable_resources: "cpu=low; memory=low; storage=low; network=conditional",
        priority: "P3",
        probable_phase: "Phase 7+",
        risks: "Noisy or unauthorized notifications reduce user trust.",
        allowed_dependencies: &["Policy Service", "Telemetry Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Translation Service",
        layer: ServiceLayer::Interaction,
        service_type: ServiceType::Platform,
        responsibility: "Translation and language adaptation.",
        capabilities_provided: &["translation.translate", "translation.localize"],
        capabilities_required: &["inference.run", "model.registry.read"],
        probable_permissions: &["service.command", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=low; network=conditional",
        priority: "P4",
        probable_phase: "Phase 8+",
        risks: "Poor translation can distort meaning or intent.",
        allowed_dependencies: &[
            "Inference Service",
            "Model Registry Service",
            "Telemetry Service",
        ],
    },
    OfficialServiceDefinition {
        name: "Filesystem Service",
        layer: ServiceLayer::Device,
        service_type: ServiceType::Device,
        responsibility: "Supervised file access.",
        capabilities_provided: &["filesystem.read", "filesystem.write", "filesystem.watch"],
        capabilities_required: &["permission.check", "audit.write"],
        probable_permissions: &["filesystem.read", "filesystem.write", "telemetry.emit"],
        probable_resources: "cpu=low; memory=low; storage=conditional; network=false; filesystem=conditional",
        priority: "P2",
        probable_phase: "Phase 6+",
        risks: "File access can expose or modify sensitive user data.",
        allowed_dependencies: &["Permission Service", "Audit Service", "Telemetry Service"],
    },
    OfficialServiceDefinition {
        name: "Clipboard Service",
        layer: ServiceLayer::Device,
        service_type: ServiceType::Device,
        responsibility: "Supervised clipboard access.",
        capabilities_provided: &["clipboard.read", "clipboard.write"],
        capabilities_required: &["permission.check", "audit.write"],
        probable_permissions: &["device.clipboard", "telemetry.emit"],
        probable_resources: "cpu=low; memory=low; storage=none; network=false",
        priority: "P3",
        probable_phase: "Phase 7+",
        risks: "Clipboard contents commonly include secrets and personal data.",
        allowed_dependencies: &["Permission Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Screen Service",
        layer: ServiceLayer::Device,
        service_type: ServiceType::Device,
        responsibility: "Supervised screen reading.",
        capabilities_provided: &["screen.read", "screen.region.read"],
        capabilities_required: &["permission.check", "audit.write"],
        probable_permissions: &["device.screen", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=none; network=false",
        priority: "P4",
        probable_phase: "Phase 8+",
        risks: "Screen capture can expose private application data.",
        allowed_dependencies: &["Permission Service", "Audit Service", "Telemetry Service"],
    },
    OfficialServiceDefinition {
        name: "Camera Service",
        layer: ServiceLayer::Device,
        service_type: ServiceType::Device,
        responsibility: "Supervised camera use.",
        capabilities_provided: &["camera.read", "camera.status"],
        capabilities_required: &["permission.check", "audit.write"],
        probable_permissions: &["device.camera", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=none; network=false",
        priority: "P4",
        probable_phase: "Phase 8+",
        risks: "Camera use requires explicit consent and strong auditing.",
        allowed_dependencies: &["Permission Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Microphone Service",
        layer: ServiceLayer::Device,
        service_type: ServiceType::Device,
        responsibility: "Supervised microphone use.",
        capabilities_provided: &["microphone.read", "microphone.status"],
        capabilities_required: &["permission.check", "audit.write"],
        probable_permissions: &["device.microphone", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=none; network=false",
        priority: "P4",
        probable_phase: "Phase 8+",
        risks: "Audio capture has high privacy sensitivity.",
        allowed_dependencies: &["Permission Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Network Service",
        layer: ServiceLayer::Device,
        service_type: ServiceType::Device,
        responsibility: "Network state and network access policies.",
        capabilities_provided: &["network.status", "network.policy"],
        capabilities_required: &["permission.check", "audit.write"],
        probable_permissions: &["network.inspect", "telemetry.emit"],
        probable_resources: "cpu=low; memory=low; storage=low; network=true",
        priority: "P3",
        probable_phase: "Phase 7+",
        risks: "Network policy errors can leak data or block critical work.",
        allowed_dependencies: &["Permission Service", "Audit Service", "Telemetry Service"],
    },
    OfficialServiceDefinition {
        name: "Browser Context Service",
        layer: ServiceLayer::Device,
        service_type: ServiceType::Device,
        responsibility: "Authorized browser context access.",
        capabilities_provided: &["browser.context.read", "browser.context.snapshot"],
        capabilities_required: &["permission.check", "audit.write"],
        probable_permissions: &["browser.context", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=medium; network=conditional",
        priority: "P4",
        probable_phase: "Phase 8+",
        risks: "Browser context may include credentials and private browsing data.",
        allowed_dependencies: &[
            "Permission Service",
            "Audit Service",
            "Browser Automation Service",
        ],
    },
    OfficialServiceDefinition {
        name: "Workflow Service",
        layer: ServiceLayer::Automation,
        service_type: ServiceType::Automation,
        responsibility: "Workflow definitions and execution coordination.",
        capabilities_provided: &["workflow.create", "workflow.run", "workflow.inspect"],
        capabilities_required: &["permission.check", "scheduler.run"],
        probable_permissions: &["service.command", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=medium; network=conditional",
        priority: "P4",
        probable_phase: "Phase 8+",
        risks: "Workflows can chain actions with broad side effects.",
        allowed_dependencies: &["Permission Service", "Scheduler Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Macro Service",
        layer: ServiceLayer::Automation,
        service_type: ServiceType::Automation,
        responsibility: "Local macros.",
        capabilities_provided: &["macro.record", "macro.run"],
        capabilities_required: &["permission.check", "desktop.automation"],
        probable_permissions: &["service.command", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=medium; network=false",
        priority: "P4",
        probable_phase: "Phase 8+",
        risks: "Macros can replay unsafe local actions.",
        allowed_dependencies: &[
            "Permission Service",
            "Desktop Automation Service",
            "Audit Service",
        ],
    },
    OfficialServiceDefinition {
        name: "Browser Automation Service",
        layer: ServiceLayer::Automation,
        service_type: ServiceType::Automation,
        responsibility: "Authorized browser automation.",
        capabilities_provided: &["browser.automation.run", "browser.automation.inspect"],
        capabilities_required: &["browser.context.read", "permission.check"],
        probable_permissions: &["browser.automation", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=medium; network=conditional",
        priority: "P4",
        probable_phase: "Phase 8+",
        risks: "Browser automation can perform user-visible external actions.",
        allowed_dependencies: &[
            "Browser Context Service",
            "Permission Service",
            "Audit Service",
        ],
    },
    OfficialServiceDefinition {
        name: "Desktop Automation Service",
        layer: ServiceLayer::Automation,
        service_type: ServiceType::Automation,
        responsibility: "Authorized desktop automation.",
        capabilities_provided: &["desktop.automation", "desktop.action.run"],
        capabilities_required: &["permission.check", "screen.read"],
        probable_permissions: &["desktop.automation", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=low; network=false",
        priority: "P4",
        probable_phase: "Phase 8+",
        risks: "Desktop automation can affect arbitrary user applications.",
        allowed_dependencies: &["Permission Service", "Screen Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Integration Service",
        layer: ServiceLayer::Automation,
        service_type: ServiceType::External,
        responsibility: "Integrations with external APIs.",
        capabilities_provided: &["integration.call", "integration.registry"],
        capabilities_required: &["network.policy", "permission.check"],
        probable_permissions: &["network.request", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=medium; network=true",
        priority: "P4",
        probable_phase: "Phase 8+",
        risks: "External API calls can leak data or create cost exposure.",
        allowed_dependencies: &["Network Service", "Permission Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "API Gateway Service",
        layer: ServiceLayer::Enterprise,
        service_type: ServiceType::Enterprise,
        responsibility: "External API entrypoint for future hosted deployments.",
        capabilities_provided: &["api.gateway.route", "api.gateway.limit"],
        capabilities_required: &["policy.evaluate", "audit.write"],
        probable_permissions: &["network.request", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=low; network=true",
        priority: "P5",
        probable_phase: "Enterprise future",
        risks: "Public ingress expands attack surface.",
        allowed_dependencies: &["Policy Service", "Audit Service", "Telemetry Service"],
    },
    OfficialServiceDefinition {
        name: "Multi-Tenant Service",
        layer: ServiceLayer::Enterprise,
        service_type: ServiceType::Enterprise,
        responsibility: "Tenant isolation and tenant-scoped metadata.",
        capabilities_provided: &["tenant.resolve", "tenant.isolate"],
        capabilities_required: &["organization.read", "policy.evaluate"],
        probable_permissions: &["service.inspect", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=high; network=true",
        priority: "P5",
        probable_phase: "Enterprise future",
        risks: "Tenant isolation failures can expose cross-tenant data.",
        allowed_dependencies: &["Organization Service", "Policy Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Organization Service",
        layer: ServiceLayer::Enterprise,
        service_type: ServiceType::Enterprise,
        responsibility: "Organizations, users, and organizational metadata.",
        capabilities_provided: &["organization.read", "organization.write"],
        capabilities_required: &["storage.write", "audit.write"],
        probable_permissions: &["telemetry.emit", "service.inspect"],
        probable_resources: "cpu=medium; memory=medium; storage=high; network=true",
        priority: "P5",
        probable_phase: "Enterprise future",
        risks: "Organization data errors can impact access and billing boundaries.",
        allowed_dependencies: &["Storage Service", "Audit Service", "Policy Service"],
    },
    OfficialServiceDefinition {
        name: "Billing Service",
        layer: ServiceLayer::Enterprise,
        service_type: ServiceType::Enterprise,
        responsibility: "Billing and usage accounting.",
        capabilities_provided: &["billing.usage", "billing.invoice"],
        capabilities_required: &["organization.read", "audit.write"],
        probable_permissions: &["network.request", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=high; network=true",
        priority: "P5",
        probable_phase: "Enterprise future",
        risks: "Billing errors create financial and customer trust issues.",
        allowed_dependencies: &[
            "Organization Service",
            "Audit Service",
            "Integration Service",
        ],
    },
    OfficialServiceDefinition {
        name: "Licensing Service",
        layer: ServiceLayer::Enterprise,
        service_type: ServiceType::Enterprise,
        responsibility: "License state and entitlement checks.",
        capabilities_provided: &["license.check", "license.activate"],
        capabilities_required: &["organization.read", "policy.evaluate"],
        probable_permissions: &["network.request", "telemetry.emit"],
        probable_resources: "cpu=low; memory=low; storage=medium; network=true",
        priority: "P5",
        probable_phase: "Enterprise future",
        risks: "License failures can block legitimate use or allow unauthorized use.",
        allowed_dependencies: &["Organization Service", "Policy Service", "Audit Service"],
    },
    OfficialServiceDefinition {
        name: "Policy Service",
        layer: ServiceLayer::Enterprise,
        service_type: ServiceType::Enterprise,
        responsibility: "Policy evaluation across platform and enterprise contexts.",
        capabilities_provided: &["policy.evaluate", "policy.read", "policy.write"],
        capabilities_required: &["audit.write", "storage.read"],
        probable_permissions: &["telemetry.emit", "service.inspect"],
        probable_resources: "cpu=medium; memory=medium; storage=medium; network=false",
        priority: "P4",
        probable_phase: "Phase 8+",
        risks: "Incorrect policy decisions can allow unsafe actions.",
        allowed_dependencies: &["Audit Service", "Storage Service", "Organization Service"],
    },
    OfficialServiceDefinition {
        name: "Compliance Service",
        layer: ServiceLayer::Enterprise,
        service_type: ServiceType::Enterprise,
        responsibility: "Compliance evidence and reporting.",
        capabilities_provided: &["compliance.report", "compliance.evidence"],
        capabilities_required: &["audit.query", "policy.read"],
        probable_permissions: &["service.inspect", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=high; network=conditional",
        priority: "P5",
        probable_phase: "Enterprise future",
        risks: "Incomplete evidence can fail compliance audits.",
        allowed_dependencies: &["Audit Service", "Policy Service", "Storage Service"],
    },
    OfficialServiceDefinition {
        name: "Plugin Marketplace Service",
        layer: ServiceLayer::Enterprise,
        service_type: ServiceType::Enterprise,
        responsibility: "Plugin discovery, distribution, and policy gatekeeping.",
        capabilities_provided: &["plugin.marketplace.search", "plugin.marketplace.install"],
        capabilities_required: &["license.check", "policy.evaluate", "audit.write"],
        probable_permissions: &["network.request", "telemetry.emit"],
        probable_resources: "cpu=medium; memory=medium; storage=high; network=true",
        priority: "P5",
        probable_phase: "Enterprise future",
        risks: "Untrusted plugins can compromise platform integrity.",
        allowed_dependencies: &["Licensing Service", "Policy Service", "Audit Service"],
    },
];

/// Core system service contract.
pub trait SystemService {
    /// Return the service descriptor.
    fn descriptor(&self) -> &ServiceDescriptor;

    /// Return the command route exposed on the ASB.
    fn route(&self) -> &'static str;

    /// Return the service health.
    fn health(&self) -> ServiceHealthStatus;

    /// Register ASB handlers exposed by this service.
    ///
    /// # Errors
    ///
    /// Returns [`SystemServicesError`] when the Kernel cannot register handlers.
    fn register_handlers(&self, kernel: &mut AetherKernel) -> Result<(), SystemServicesError>;
}

/// Core system service loaded from a manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreSystemService {
    route: &'static str,
    descriptor: ServiceDescriptor,
}

impl CoreSystemService {
    /// Return the source manifest.
    #[must_use]
    pub const fn manifest(&self) -> &ServiceManifest {
        &self.descriptor.manifest
    }
}

impl SystemService for CoreSystemService {
    fn descriptor(&self) -> &ServiceDescriptor {
        &self.descriptor
    }

    fn route(&self) -> &'static str {
        self.route
    }

    fn health(&self) -> ServiceHealthStatus {
        self.descriptor.health_status
    }

    fn register_handlers(&self, kernel: &mut AetherKernel) -> Result<(), SystemServicesError> {
        register_system_service_handler(kernel, self)?;
        Ok(())
    }
}

/// Load all Phase 4 core system services from manifests.
///
/// # Errors
///
/// Returns [`SystemServicesError`] when any manifest is invalid.
pub fn load_core_system_services() -> Result<Vec<CoreSystemService>, SystemServicesError> {
    CORE_SYSTEM_SERVICE_MANIFESTS
        .iter()
        .map(load_core_system_service)
        .collect()
}

/// Register all Phase 4 core system services with the Kernel and ASB.
///
/// # Errors
///
/// Returns [`SystemServicesError`] when service registration or handler registration fails.
pub fn register_core_system_services(
    kernel: &mut AetherKernel,
) -> Result<Vec<ServiceBusClient>, SystemServicesError> {
    let mut clients = Vec::with_capacity(CORE_SYSTEM_SERVICE_COUNT);
    for service in load_core_system_services()? {
        let client = kernel.register_service(service.descriptor().clone())?;
        service.register_handlers(kernel)?;
        clients.push(client);
    }
    Ok(clients)
}

/// Return whether the provided services avoid direct service coupling.
#[must_use]
pub fn core_services_use_service_bus_only(services: &[CoreSystemService]) -> bool {
    services.iter().all(|service| {
        service.route().starts_with("system.")
            && !service.route().contains("svc_")
            && capabilities_do_not_reference_service_ids(
                service.descriptor().capabilities.requires(),
            )
    })
}

/// Errors raised while loading or registering system services.
#[derive(Debug, Error)]
pub enum SystemServicesError {
    /// Service model failed.
    #[error("system service model failed: {0}")]
    Service(#[from] ServiceError),
    /// Service bus failed.
    #[error("system service bus failed: {0}")]
    ServiceBus(#[from] ServiceBusError),
    /// Kernel registration failed.
    #[error("system service kernel registration failed: {0}")]
    Kernel(#[from] KernelError),
}

struct CoreSystemServiceManifest {
    route: &'static str,
    toml: &'static str,
}

const CORE_SYSTEM_SERVICE_MANIFESTS: &[CoreSystemServiceManifest] = &[
    CoreSystemServiceManifest {
        route: "system.telemetry",
        toml: TELEMETRY_SERVICE_MANIFEST,
    },
    CoreSystemServiceManifest {
        route: "system.configuration",
        toml: CONFIGURATION_SERVICE_MANIFEST,
    },
    CoreSystemServiceManifest {
        route: "system.health",
        toml: HEALTH_SERVICE_MANIFEST,
    },
    CoreSystemServiceManifest {
        route: "system.events",
        toml: EVENT_SERVICE_MANIFEST,
    },
    CoreSystemServiceManifest {
        route: "system.inspector",
        toml: SERVICE_INSPECTOR_MANIFEST,
    },
];

fn load_core_system_service(
    definition: &CoreSystemServiceManifest,
) -> Result<CoreSystemService, SystemServicesError> {
    let manifest = ServiceManifest::from_toml_str(definition.toml)?;
    let descriptor = ServiceDescriptor::from_manifest(manifest)?
        .with_lifecycle_status(LifecycleStatus::Running)
        .with_health_status(ServiceHealthStatus::Healthy);
    Ok(CoreSystemService {
        route: definition.route,
        descriptor,
    })
}

fn register_system_service_handler(
    kernel: &mut AetherKernel,
    service: &CoreSystemService,
) -> Result<(), KernelError> {
    let route = service.route();
    let handler = SystemServiceCommandHandler {
        service_name: service.descriptor().name.clone(),
        service_id: service.descriptor().id.to_string(),
        route,
        health: service.health(),
    };
    kernel.register_service_command_handler(&service.descriptor().id, route, Arc::new(handler))
}

fn capabilities_do_not_reference_service_ids(capabilities: &[Capability]) -> bool {
    capabilities
        .iter()
        .all(|capability| !capability.name().starts_with("svc_"))
}

#[derive(Debug)]
struct SystemServiceCommandHandler {
    service_name: String,
    service_id: String,
    route: &'static str,
    health: ServiceHealthStatus,
}

impl CommandHandler for SystemServiceCommandHandler {
    fn handle(&self, command: &ServiceCommand) -> Result<ServiceReply, ServiceBusError> {
        Ok(ServiceReply::accepted()
            .with_payload_value("service", json!(self.service_name))
            .with_payload_value("service_id", json!(self.service_id))
            .with_payload_value("route", json!(self.route))
            .with_payload_value("command", json!(command.name()))
            .with_payload_value("health", json!(format!("{:?}", self.health))))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use aether_config::AetherConfig;
    use aether_events::EventBus;
    use aether_ids::KernelId;
    use aether_kernel::AetherKernel;
    use aether_logging::{MemoryLogSink, StructuredLogger};
    use aether_permissions::Permission;
    use aether_resources::{FilesystemAccess, StorageClass};
    use aether_service_bus::{AetherServiceBus, ServiceCommand};
    use aether_telemetry::{MemoryTelemetrySink, TelemetryEmitter};

    use super::{
        CORE_SYSTEM_SERVICE_COUNT, OFFICIAL_SERVICE_MAP, ServiceLayer, ServiceType, SystemService,
        core_services_use_service_bus_only, load_core_system_services,
        register_core_system_services,
    };

    const SERVICE_MAP_DOC: &str =
        include_str!("../../../../docs/Architecture/aether-official-service-map.md");

    fn test_kernel() -> AetherKernel {
        let config = AetherConfig::default();
        let log_sink = Arc::new(MemoryLogSink::new());
        let logger = StructuredLogger::new(config.runtime.log_level, log_sink);
        let telemetry_sink = Arc::new(MemoryTelemetrySink::new());
        let telemetry = TelemetryEmitter::new(config.runtime.log_level, telemetry_sink);

        AetherKernel::new(
            KernelId::generate(),
            config,
            EventBus::new(),
            logger,
            telemetry,
        )
    }

    #[test]
    fn validates_official_service_map() {
        assert_eq!(OFFICIAL_SERVICE_MAP.len(), 52);
        assert!(
            OFFICIAL_SERVICE_MAP
                .iter()
                .any(|service| service.name == "Kernel Service"
                    && service.layer == ServiceLayer::Foundation)
        );
        assert!(
            OFFICIAL_SERVICE_MAP
                .iter()
                .any(|service| service.name == "Memory Service"
                    && service.service_type == ServiceType::Cognitive)
        );
        assert!(
            OFFICIAL_SERVICE_MAP
                .iter()
                .any(|service| service.service_type == ServiceType::External)
        );
    }

    #[test]
    fn official_service_map_document_mentions_every_service() {
        assert!(SERVICE_MAP_DOC.contains("Engineering Rule #002"));
        for service in OFFICIAL_SERVICE_MAP {
            assert!(
                SERVICE_MAP_DOC.contains(service.name),
                "missing service in docs: {}",
                service.name
            );
        }
    }

    #[test]
    fn loads_base_service_manifests() {
        let services = load_core_system_services().expect("services");

        assert_eq!(services.len(), CORE_SYSTEM_SERVICE_COUNT);
        assert!(
            services
                .iter()
                .all(|service| service.descriptor().owner == "neuroforge-labs")
        );
    }

    #[test]
    fn registers_telemetry_service() {
        assert_can_register_service("telemetry-service");
    }

    #[test]
    fn registers_configuration_service() {
        assert_can_register_service("configuration-service");
    }

    #[test]
    fn registers_health_service() {
        assert_can_register_service("health-service");
    }

    #[test]
    fn registers_event_service() {
        assert_can_register_service("event-service");
    }

    #[test]
    fn registers_service_inspector_service() {
        assert_can_register_service("service-inspector");
    }

    #[test]
    fn queries_base_service_capabilities() {
        let mut kernel = test_kernel();
        register_core_system_services(&mut kernel).expect("register services");

        let providers = kernel
            .service_registry()
            .providers_for_capability_name("telemetry.emit")
            .expect("providers");

        assert!(!providers.is_empty());
        assert!(
            providers
                .iter()
                .any(|service| service.name == "telemetry-service")
        );
    }

    #[test]
    fn validates_base_service_permissions() {
        let services = load_core_system_services().expect("services");
        let inspector = services
            .iter()
            .find(|service| service.descriptor().name == "service-inspector")
            .expect("service inspector");

        assert!(
            inspector
                .descriptor()
                .permissions
                .contains(&Permission::service_command())
        );
    }

    #[test]
    fn validates_base_service_resources() {
        let services = load_core_system_services().expect("services");
        let configuration = services
            .iter()
            .find(|service| service.descriptor().name == "configuration-service")
            .expect("configuration service");

        assert_eq!(
            configuration.descriptor().resources.storage_class,
            StorageClass::Low
        );
        assert_eq!(
            configuration.descriptor().resources.filesystem_access,
            FilesystemAccess::ReadOnly
        );
    }

    #[test]
    fn aggregates_base_service_health() {
        let mut kernel = test_kernel();
        register_core_system_services(&mut kernel).expect("register services");

        let health = kernel.service_health();

        assert_eq!(health.total_services, CORE_SYSTEM_SERVICE_COUNT);
        assert_eq!(health.running_services, CORE_SYSTEM_SERVICE_COUNT);
        assert_eq!(health.failed_services, 0);
    }

    #[test]
    fn core_services_do_not_call_each_other_directly() {
        let services = load_core_system_services().expect("services");

        assert!(core_services_use_service_bus_only(&services));
    }

    #[test]
    fn core_services_expose_commands_through_asb() {
        let mut kernel = test_kernel();
        let clients = register_core_system_services(&mut kernel).expect("register services");
        let source = kernel
            .services()
            .into_iter()
            .find(|service| service.name == "service-inspector")
            .expect("source service");
        let client = clients
            .iter()
            .find(|client| client.service_id() == &source.id)
            .expect("bound service client");

        let reply = kernel
            .route_service_command(
                client,
                &ServiceCommand::new("system.health", "status").expect("command"),
            )
            .expect("reply");

        assert!(reply.is_accepted());
        assert_eq!(
            reply.payload().get("service"),
            Some(&serde_json::json!("health-service"))
        );
    }

    fn assert_can_register_service(name: &str) {
        let mut kernel = test_kernel();
        let service = load_core_system_services()
            .expect("services")
            .into_iter()
            .find(|service| service.descriptor().name == name)
            .expect("service");

        kernel
            .register_service(service.descriptor().clone())
            .expect("register service");

        assert_eq!(kernel.services().len(), 1);
        assert_eq!(kernel.services()[0].name, name);
    }
}
