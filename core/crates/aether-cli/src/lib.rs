#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Minimal command-line interface for validating the Aether core runtime.

use std::io::Write;
use std::sync::Arc;

use aether_config::{AetherConfig, StaticConfigProvider};
use aether_events::EventBus;
use aether_ids::KernelId;
use aether_kernel::{AetherKernel, KernelError};
use aether_logging::{MemoryLogSink, StructuredLogger};
use aether_runtime::{AetherRuntime, RuntimeError};
use aether_service_bus::{AetherServiceBus, ServiceBusError};
use aether_telemetry::{MemoryTelemetrySink, TelemetryEmitter};
use thiserror::Error;

/// Successful process exit code.
pub const EXIT_SUCCESS: i32 = 0;

/// Invalid command process exit code.
pub const EXIT_USAGE: i32 = 2;

/// Runtime failure process exit code.
pub const EXIT_FAILURE: i32 = 1;

/// Run the CLI with explicit arguments and output.
///
/// # Errors
///
/// Returns [`CliError`] when writing output or validating the runtime fails.
pub fn run<I, S, W>(args: I, writer: &mut W) -> Result<i32, CliError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
    W: Write,
{
    let args = args
        .into_iter()
        .map(|value| value.as_ref().to_owned())
        .collect::<Vec<_>>();
    let command = args.get(1).map_or("validate", String::as_str);

    match command {
        "validate" => validate(writer),
        "kernel" => kernel_command(args.get(2).map(String::as_str), writer),
        "service" => service_command(args.get(2).map(String::as_str), writer),
        "bus" => bus_command(args.get(2).map(String::as_str), writer),
        "version" => {
            writeln!(writer, "aether-cli {}", env!("CARGO_PKG_VERSION"))?;
            Ok(EXIT_SUCCESS)
        }
        "help" | "--help" | "-h" => {
            write_usage(writer)?;
            Ok(EXIT_SUCCESS)
        }
        _ => {
            writeln!(writer, "Unknown command: {command}")?;
            write_usage(writer)?;
            Ok(EXIT_USAGE)
        }
    }
}

fn validate<W>(writer: &mut W) -> Result<i32, CliError>
where
    W: Write,
{
    let config = AetherConfig::default();
    let event_bus = EventBus::new();
    let sink = Arc::new(MemoryLogSink::new());
    let logger = StructuredLogger::new(config.runtime.log_level, sink.clone());
    let mut runtime = AetherRuntime::new(config, event_bus, logger);

    runtime.start()?;
    let health = runtime.health_check()?;
    runtime.stop()?;

    writeln!(
        writer,
        "Aether core validation completed: running={}, modules={}, healthy={}, logs={}",
        health.running,
        health.module_count,
        health.healthy,
        sink.records()?.len()
    )?;
    Ok(EXIT_SUCCESS)
}

fn kernel_command<W>(command: Option<&str>, writer: &mut W) -> Result<i32, CliError>
where
    W: Write,
{
    match command.unwrap_or("status") {
        "status" => kernel_status(writer),
        "health" => kernel_health(writer),
        "modules" => kernel_modules(writer),
        "capabilities" => kernel_capabilities(writer),
        "help" | "--help" | "-h" => {
            writeln!(
                writer,
                "Usage: aether kernel [status|health|modules|capabilities]"
            )?;
            Ok(EXIT_SUCCESS)
        }
        unknown => {
            writeln!(writer, "Unknown kernel command: {unknown}")?;
            writeln!(
                writer,
                "Usage: aether kernel [status|health|modules|capabilities]"
            )?;
            Ok(EXIT_USAGE)
        }
    }
}

fn kernel_status<W>(writer: &mut W) -> Result<i32, CliError>
where
    W: Write,
{
    let mut kernel = local_kernel()?;
    kernel.start()?;
    writeln!(
        writer,
        "kernel status: id={}, status={}, runtime_running={}, modules={}",
        kernel.id(),
        kernel.status(),
        kernel.runtime_is_running(),
        kernel.modules().len()
    )?;
    kernel.shutdown()?;
    Ok(EXIT_SUCCESS)
}

fn kernel_health<W>(writer: &mut W) -> Result<i32, CliError>
where
    W: Write,
{
    let mut kernel = local_kernel()?;
    kernel.start()?;
    let health = kernel.health()?;
    writeln!(
        writer,
        "kernel health: id={}, status={}, healthy={}, runtime_running={}, modules={}",
        health.kernel_id,
        health.status,
        health.healthy,
        health.runtime.running,
        health.modules.len()
    )?;
    kernel.shutdown()?;
    Ok(EXIT_SUCCESS)
}

fn kernel_modules<W>(writer: &mut W) -> Result<i32, CliError>
where
    W: Write,
{
    let kernel = local_kernel()?;
    let modules = kernel.modules();
    writeln!(writer, "kernel modules: count={}", modules.len())?;
    for module in modules {
        writeln!(
            writer,
            "- id={}, name={}, version={}, status={}",
            module.descriptor().id(),
            module.descriptor().name(),
            module.descriptor().version(),
            module.status()
        )?;
    }
    Ok(EXIT_SUCCESS)
}

fn kernel_capabilities<W>(writer: &mut W) -> Result<i32, CliError>
where
    W: Write,
{
    let kernel = local_kernel()?;
    let capabilities = kernel.capabilities();
    writeln!(writer, "kernel capabilities: count={}", capabilities.len())?;
    for capability in capabilities {
        writeln!(
            writer,
            "- module_id={}, capability_id={}, capability={}",
            capability.module_id,
            capability.capability.id(),
            capability.capability.name()
        )?;
    }
    Ok(EXIT_SUCCESS)
}

fn service_command<W>(command: Option<&str>, writer: &mut W) -> Result<i32, CliError>
where
    W: Write,
{
    match command.unwrap_or("list") {
        "list" => service_list(writer),
        "inspect" => service_inspect(writer),
        "capabilities" => service_capabilities(writer),
        "health" => service_health(writer),
        "help" | "--help" | "-h" => {
            writeln!(
                writer,
                "Usage: aether service [list|inspect|capabilities|health]"
            )?;
            Ok(EXIT_SUCCESS)
        }
        unknown => {
            writeln!(writer, "Unknown service command: {unknown}")?;
            writeln!(
                writer,
                "Usage: aether service [list|inspect|capabilities|health]"
            )?;
            Ok(EXIT_USAGE)
        }
    }
}

fn service_list<W>(writer: &mut W) -> Result<i32, CliError>
where
    W: Write,
{
    let kernel = local_kernel()?;
    let services = kernel.services();
    writeln!(writer, "service list: count={}", services.len())?;
    for service in services {
        writeln!(
            writer,
            "- id={}, name={}, version={}, status={:?}, health={:?}",
            service.id,
            service.name,
            service.version,
            service.lifecycle_status,
            service.health_status
        )?;
    }
    Ok(EXIT_SUCCESS)
}

fn service_inspect<W>(writer: &mut W) -> Result<i32, CliError>
where
    W: Write,
{
    let kernel = local_kernel()?;
    let services = kernel.services();
    if services.is_empty() {
        writeln!(writer, "service inspect: no services registered")?;
        return Ok(EXIT_SUCCESS);
    }

    for service in services {
        writeln!(
            writer,
            "service inspect: id={}, name={}, owner={}, provides={}, requires={}, permissions={}, resources=declared",
            service.id,
            service.name,
            service.owner,
            service.capabilities.provides().len(),
            service.capabilities.requires().len(),
            service.permissions.requested().len()
        )?;
    }
    Ok(EXIT_SUCCESS)
}

fn service_capabilities<W>(writer: &mut W) -> Result<i32, CliError>
where
    W: Write,
{
    let kernel = local_kernel()?;
    let services = kernel.services();
    let provided = services
        .iter()
        .map(|service| service.capabilities.provides().len())
        .sum::<usize>();
    let required = services
        .iter()
        .map(|service| service.capabilities.requires().len())
        .sum::<usize>();
    writeln!(
        writer,
        "service capabilities: services={}, provided={}, required={}",
        services.len(),
        provided,
        required
    )?;
    Ok(EXIT_SUCCESS)
}

fn service_health<W>(writer: &mut W) -> Result<i32, CliError>
where
    W: Write,
{
    let kernel = local_kernel()?;
    let health = kernel.service_health();
    writeln!(
        writer,
        "service health: total={}, running={}, degraded={}, failed={}, capabilities={}, permissions={}, resources={}",
        health.total_services,
        health.running_services,
        health.degraded_services,
        health.failed_services,
        health.capabilities_available.len(),
        health.permissions_requested.len(),
        health.resources_declared
    )?;
    Ok(EXIT_SUCCESS)
}

fn bus_command<W>(command: Option<&str>, writer: &mut W) -> Result<i32, CliError>
where
    W: Write,
{
    match command.unwrap_or("status") {
        "status" => bus_status(writer),
        "help" | "--help" | "-h" => {
            writeln!(writer, "Usage: aether bus [status]")?;
            Ok(EXIT_SUCCESS)
        }
        unknown => {
            writeln!(writer, "Unknown bus command: {unknown}")?;
            writeln!(writer, "Usage: aether bus [status]")?;
            Ok(EXIT_USAGE)
        }
    }
}

fn bus_status<W>(writer: &mut W) -> Result<i32, CliError>
where
    W: Write,
{
    let kernel = local_kernel()?;
    let status = kernel.service_bus().status()?;
    writeln!(
        writer,
        "bus status: event_bus={}, notification_subscribers={}, request_handlers={}, command_handlers={}, permission_entries={}",
        status.event_bus,
        status.notification_subscribers,
        status.request_handlers,
        status.command_handlers,
        status.permission_entries
    )?;
    Ok(EXIT_SUCCESS)
}

fn local_kernel() -> Result<AetherKernel, CliError> {
    let config = AetherConfig::default();
    let provider = StaticConfigProvider::new(config.clone());
    let event_bus = EventBus::new();
    let sink = Arc::new(MemoryLogSink::new());
    let logger = StructuredLogger::new(config.runtime.log_level, sink);
    let telemetry_sink = Arc::new(MemoryTelemetrySink::new());
    let telemetry = TelemetryEmitter::new(config.runtime.log_level, telemetry_sink);

    Ok(AetherKernel::from_config_provider(
        &provider,
        KernelId::generate(),
        event_bus,
        logger,
        telemetry,
    )?)
}

fn write_usage<W>(writer: &mut W) -> Result<(), std::io::Error>
where
    W: Write,
{
    writeln!(
        writer,
        "Usage: aether [validate|version|help|kernel <status|health|modules|capabilities>|service <list|inspect|capabilities|health>|bus <status>]"
    )
}

/// CLI errors.
#[derive(Debug, Error)]
pub enum CliError {
    /// Runtime validation failed.
    #[error("runtime validation failed: {0}")]
    Runtime(#[from] RuntimeError),
    /// Kernel validation failed.
    #[error("kernel validation failed: {0}")]
    Kernel(#[from] KernelError),
    /// Service Bus validation failed.
    #[error("service bus validation failed: {0}")]
    ServiceBus(#[from] ServiceBusError),
    /// Logging validation failed.
    #[error("logging validation failed: {0}")]
    Logging(#[from] aether_logging::LoggingError),
    /// Output writing failed.
    #[error("failed to write CLI output: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::{EXIT_SUCCESS, EXIT_USAGE, run};

    #[test]
    fn validate_command_runs_runtime() {
        let mut output = Vec::new();
        let code = run(["aether", "validate"], &mut output).expect("cli");
        let output = String::from_utf8(output).expect("utf8");

        assert_eq!(code, EXIT_SUCCESS);
        assert!(output.contains("Aether core validation completed"));
        assert!(output.contains("healthy=true"));
    }

    #[test]
    fn version_command_prints_version() {
        let mut output = Vec::new();
        let code = run(["aether", "version"], &mut output).expect("cli");
        let output = String::from_utf8(output).expect("utf8");

        assert_eq!(code, EXIT_SUCCESS);
        assert!(output.contains("aether-cli"));
    }

    #[test]
    fn unknown_command_returns_usage_code() {
        let mut output = Vec::new();
        let code = run(["aether", "unknown"], &mut output).expect("cli");

        assert_eq!(code, EXIT_USAGE);
    }

    #[test]
    fn kernel_status_command_runs_kernel() {
        let mut output = Vec::new();
        let code = run(["aether", "kernel", "status"], &mut output).expect("cli");
        let output = String::from_utf8(output).expect("utf8");

        assert_eq!(code, EXIT_SUCCESS);
        assert!(output.contains("kernel status"));
        assert!(output.contains("status=running"));
    }

    #[test]
    fn kernel_health_command_reports_healthy_kernel() {
        let mut output = Vec::new();
        let code = run(["aether", "kernel", "health"], &mut output).expect("cli");
        let output = String::from_utf8(output).expect("utf8");

        assert_eq!(code, EXIT_SUCCESS);
        assert!(output.contains("kernel health"));
        assert!(output.contains("healthy=true"));
    }

    #[test]
    fn kernel_modules_command_lists_empty_registry() {
        let mut output = Vec::new();
        let code = run(["aether", "kernel", "modules"], &mut output).expect("cli");
        let output = String::from_utf8(output).expect("utf8");

        assert_eq!(code, EXIT_SUCCESS);
        assert!(output.contains("kernel modules: count=0"));
    }

    #[test]
    fn kernel_capabilities_command_lists_empty_registry() {
        let mut output = Vec::new();
        let code = run(["aether", "kernel", "capabilities"], &mut output).expect("cli");
        let output = String::from_utf8(output).expect("utf8");

        assert_eq!(code, EXIT_SUCCESS);
        assert!(output.contains("kernel capabilities: count=0"));
    }

    #[test]
    fn service_list_command_lists_services() {
        let mut output = Vec::new();
        let code = run(["aether", "service", "list"], &mut output).expect("cli");
        let output = String::from_utf8(output).expect("utf8");

        assert_eq!(code, EXIT_SUCCESS);
        assert!(output.contains("service list: count=0"));
    }

    #[test]
    fn service_health_command_reports_aggregation() {
        let mut output = Vec::new();
        let code = run(["aether", "service", "health"], &mut output).expect("cli");
        let output = String::from_utf8(output).expect("utf8");

        assert_eq!(code, EXIT_SUCCESS);
        assert!(output.contains("service health: total=0"));
    }

    #[test]
    fn bus_status_command_reports_bus() {
        let mut output = Vec::new();
        let code = run(["aether", "bus", "status"], &mut output).expect("cli");
        let output = String::from_utf8(output).expect("utf8");

        assert_eq!(code, EXIT_SUCCESS);
        assert!(output.contains("bus status: event_bus=in-memory"));
    }
}
