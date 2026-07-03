#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Aether Service Bus for internal service communication.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use aether_events::{Event, EventBus, EventError, EventReceiver};
use aether_ids::ServiceId;
use aether_permissions::{Permission, PermissionSet};
use aether_service::ServiceDescriptor;
use aether_service::ServiceError;
use aether_service::ServiceRegistry;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use thiserror::Error;

/// Service Bus payload map.
pub type BusPayload = Map<String, Value>;

/// Typed bus contract used by the Contract Bus base.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BusContract {
    subject: String,
    version: String,
}

impl BusContract {
    /// Create a typed bus contract.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError::InvalidContract`] when subject or version is empty.
    pub fn new(
        subject: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<Self, ServiceBusError> {
        let subject = subject.into();
        let version = version.into();
        if subject.trim().is_empty() || version.trim().is_empty() {
            return Err(ServiceBusError::InvalidContract);
        }
        Ok(Self { subject, version })
    }

    /// Return the contract subject.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Return the contract version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    fn route_key(&self) -> String {
        format!("{}@{}", self.subject, self.version)
    }
}

/// Contract request routed through the Contract Bus base.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractRequest {
    contract: BusContract,
    payload: BusPayload,
}

impl ContractRequest {
    /// Create a contract request.
    #[must_use]
    pub fn new(contract: BusContract) -> Self {
        Self {
            contract,
            payload: BusPayload::new(),
        }
    }

    /// Attach a JSON payload value.
    #[must_use]
    pub fn with_payload_value(mut self, key: impl Into<String>, value: Value) -> Self {
        self.payload.insert(key.into(), value);
        self
    }

    /// Return the typed contract.
    #[must_use]
    pub const fn contract(&self) -> &BusContract {
        &self.contract
    }

    /// Return the request payload.
    #[must_use]
    pub const fn payload(&self) -> &BusPayload {
        &self.payload
    }
}

/// Contract reply returned by the Contract Bus base.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractReply {
    accepted: bool,
    payload: BusPayload,
}

impl ContractReply {
    /// Create an accepted contract reply.
    #[must_use]
    pub fn accepted() -> Self {
        Self {
            accepted: true,
            payload: BusPayload::new(),
        }
    }

    /// Attach a JSON payload value.
    #[must_use]
    pub fn with_payload_value(mut self, key: impl Into<String>, value: Value) -> Self {
        self.payload.insert(key.into(), value);
        self
    }

    /// Return whether the reply accepted the contract request.
    #[must_use]
    pub const fn is_accepted(&self) -> bool {
        self.accepted
    }

    /// Return the reply payload.
    #[must_use]
    pub const fn payload(&self) -> &BusPayload {
        &self.payload
    }
}

/// Contract handler used by typed contract routing.
pub trait ContractHandler: Send + Sync {
    /// Handle a typed contract request.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError`] when request handling fails.
    fn handle(&self, request: &ContractRequest) -> Result<ContractReply, ServiceBusError>;
}

impl<F> ContractHandler for F
where
    F: Fn(&ContractRequest) -> Result<ContractReply, ServiceBusError> + Send + Sync,
{
    fn handle(&self, request: &ContractRequest) -> Result<ContractReply, ServiceBusError> {
        self(request)
    }
}

/// Request handler used by local request/reply routing.
pub trait RequestHandler: Send + Sync {
    /// Handle a local service request.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError`] when request handling fails.
    fn handle(&self, request: &ServiceRequest) -> Result<ServiceReply, ServiceBusError>;
}

impl<F> RequestHandler for F
where
    F: Fn(&ServiceRequest) -> Result<ServiceReply, ServiceBusError> + Send + Sync,
{
    fn handle(&self, request: &ServiceRequest) -> Result<ServiceReply, ServiceBusError> {
        self(request)
    }
}

/// Command handler used by service command routing.
pub trait CommandHandler: Send + Sync {
    /// Handle a local service command.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError`] when command handling fails.
    fn handle(&self, command: &ServiceCommand) -> Result<ServiceReply, ServiceBusError>;
}

impl<F> CommandHandler for F
where
    F: Fn(&ServiceCommand) -> Result<ServiceReply, ServiceBusError> + Send + Sync,
{
    fn handle(&self, command: &ServiceCommand) -> Result<ServiceReply, ServiceBusError> {
        self(command)
    }
}

/// Local service request.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceRequest {
    subject: String,
    payload: BusPayload,
}

impl ServiceRequest {
    /// Create a request for a subject.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError::InvalidSubject`] when the subject is empty.
    pub fn new(subject: impl Into<String>) -> Result<Self, ServiceBusError> {
        let subject = subject.into();
        if subject.trim().is_empty() {
            return Err(ServiceBusError::InvalidSubject);
        }
        Ok(Self {
            subject,
            payload: BusPayload::new(),
        })
    }

    /// Attach a JSON payload value.
    #[must_use]
    pub fn with_payload_value(mut self, key: impl Into<String>, value: Value) -> Self {
        self.payload.insert(key.into(), value);
        self
    }

    /// Return the request subject.
    #[must_use]
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Return the request payload.
    #[must_use]
    pub const fn payload(&self) -> &BusPayload {
        &self.payload
    }
}

/// Local service reply.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceReply {
    accepted: bool,
    payload: BusPayload,
}

impl ServiceReply {
    /// Create an accepted reply.
    #[must_use]
    pub fn accepted() -> Self {
        Self {
            accepted: true,
            payload: BusPayload::new(),
        }
    }

    /// Attach a JSON payload value.
    #[must_use]
    pub fn with_payload_value(mut self, key: impl Into<String>, value: Value) -> Self {
        self.payload.insert(key.into(), value);
        self
    }

    /// Return whether the reply accepted the request.
    #[must_use]
    pub const fn is_accepted(&self) -> bool {
        self.accepted
    }

    /// Return the reply payload.
    #[must_use]
    pub const fn payload(&self) -> &BusPayload {
        &self.payload
    }
}

/// Service command routed through the bus.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCommand {
    route: String,
    name: String,
    payload: BusPayload,
}

impl ServiceCommand {
    /// Create a service command.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError::InvalidCommand`] when the route or command name is empty.
    pub fn new(route: impl Into<String>, name: impl Into<String>) -> Result<Self, ServiceBusError> {
        let route = route.into();
        let name = name.into();
        if route.trim().is_empty() || name.trim().is_empty() {
            return Err(ServiceBusError::InvalidCommand);
        }
        Ok(Self {
            route,
            name,
            payload: BusPayload::new(),
        })
    }

    /// Attach a JSON payload value.
    #[must_use]
    pub fn with_payload_value(mut self, key: impl Into<String>, value: Value) -> Self {
        self.payload.insert(key.into(), value);
        self
    }

    /// Return the command route.
    #[must_use]
    pub fn route(&self) -> &str {
        &self.route
    }

    /// Return the command name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the command payload.
    #[must_use]
    pub const fn payload(&self) -> &BusPayload {
        &self.payload
    }
}

/// Service notification broadcast through the bus.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceNotification {
    source_service: ServiceId,
    topic: String,
    payload: BusPayload,
}

impl ServiceNotification {
    /// Create a service notification.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError::InvalidSubject`] when the topic is empty.
    pub fn new(
        source_service: ServiceId,
        topic: impl Into<String>,
    ) -> Result<Self, ServiceBusError> {
        let topic = topic.into();
        if topic.trim().is_empty() {
            return Err(ServiceBusError::InvalidSubject);
        }
        Ok(Self {
            source_service,
            topic,
            payload: BusPayload::new(),
        })
    }

    /// Attach a JSON payload value.
    #[must_use]
    pub fn with_payload_value(mut self, key: impl Into<String>, value: Value) -> Self {
        self.payload.insert(key.into(), value);
        self
    }

    /// Return the source service identifier.
    #[must_use]
    pub const fn source_service(&self) -> &ServiceId {
        &self.source_service
    }

    /// Return the notification topic.
    #[must_use]
    pub fn topic(&self) -> &str {
        &self.topic
    }
}

/// Service Bus abstraction.
pub trait AetherServiceBus: Send + Sync {
    /// Register service permissions with the bus.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError`] when the permission index is unavailable.
    fn register_permissions(
        &self,
        service_id: &ServiceId,
        permissions: PermissionSet,
    ) -> Result<(), ServiceBusError>;

    /// Publish an event through the bus.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError`] when publication fails or permission is missing.
    fn publish_event(&self, source: &ServiceId, event: &Event) -> Result<usize, ServiceBusError>;

    /// Subscribe to events through the bus.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError`] when subscription fails or permission is missing.
    fn subscribe_events(&self, source: &ServiceId) -> Result<EventReceiver, ServiceBusError>;

    /// Register a local request handler.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError`] when the handler cannot be registered.
    fn register_request_handler(
        &self,
        subject: impl Into<String>,
        handler: Arc<dyn RequestHandler>,
    ) -> Result<(), ServiceBusError>;

    /// Execute local request/reply through the bus.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError`] when request routing fails or permission is missing.
    fn request(
        &self,
        source: &ServiceId,
        request: &ServiceRequest,
    ) -> Result<ServiceReply, ServiceBusError>;

    /// Register a service command handler.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError`] when the command handler cannot be registered.
    fn register_command_handler(
        &self,
        route: impl Into<String>,
        handler: Arc<dyn CommandHandler>,
    ) -> Result<(), ServiceBusError>;

    /// Route a service command through the bus.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError`] when command routing fails or permission is missing.
    fn command(
        &self,
        source: &ServiceId,
        command: &ServiceCommand,
    ) -> Result<ServiceReply, ServiceBusError>;

    /// Subscribe to service notifications.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError`] when subscription fails.
    fn subscribe_notifications(&self) -> Result<ServiceNotificationReceiver, ServiceBusError>;

    /// Publish a service notification.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError`] when notification delivery fails or permission is missing.
    fn notify(&self, notification: &ServiceNotification) -> Result<usize, ServiceBusError>;

    /// Return bus status.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError`] when internal indexes are unavailable.
    fn status(&self) -> Result<ServiceBusStatus, ServiceBusError>;
}

/// Contract Bus abstraction for typed internal service contracts.
pub trait AetherContractBus: Send + Sync {
    /// Register a typed contract handler.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError`] when the handler cannot be registered.
    fn register_contract_handler(
        &self,
        contract: BusContract,
        handler: Arc<dyn ContractHandler>,
    ) -> Result<(), ServiceBusError>;

    /// Execute a typed contract request.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError`] when routing fails or permission is missing.
    fn request_contract(
        &self,
        source: &ServiceId,
        request: &ContractRequest,
    ) -> Result<ContractReply, ServiceBusError>;
}

/// In-memory Aether Service Bus implementation.
#[derive(Clone, Default)]
pub struct InMemoryAetherServiceBus {
    event_bus: EventBus,
    notification_subscribers: Arc<Mutex<Vec<Sender<ServiceNotification>>>>,
    request_handlers: Arc<Mutex<BTreeMap<String, Arc<dyn RequestHandler>>>>,
    command_handlers: Arc<Mutex<BTreeMap<String, Arc<dyn CommandHandler>>>>,
    contract_handlers: Arc<Mutex<BTreeMap<String, Arc<dyn ContractHandler>>>>,
    permissions: Arc<Mutex<BTreeMap<ServiceId, PermissionSet>>>,
}

impl InMemoryAetherServiceBus {
    /// Create an empty in-memory bus.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn ensure_permission(
        &self,
        service_id: &ServiceId,
        permission: &Permission,
    ) -> Result<(), ServiceBusError> {
        let permissions = self
            .permissions
            .lock()
            .map_err(|_| ServiceBusError::BusUnavailable)?;
        let Some(permission_set) = permissions.get(service_id) else {
            return Err(ServiceBusError::PermissionDenied {
                service_id: service_id.to_string(),
                permission: permission.to_string(),
            });
        };
        if permission_set.contains(permission) {
            Ok(())
        } else {
            Err(ServiceBusError::PermissionDenied {
                service_id: service_id.to_string(),
                permission: permission.to_string(),
            })
        }
    }
}

impl fmt::Debug for InMemoryAetherServiceBus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InMemoryAetherServiceBus")
            .finish_non_exhaustive()
    }
}

impl AetherServiceBus for InMemoryAetherServiceBus {
    fn register_permissions(
        &self,
        service_id: &ServiceId,
        permissions: PermissionSet,
    ) -> Result<(), ServiceBusError> {
        self.permissions
            .lock()
            .map_err(|_| ServiceBusError::BusUnavailable)?
            .insert(service_id.clone(), permissions);
        Ok(())
    }

    fn publish_event(&self, source: &ServiceId, event: &Event) -> Result<usize, ServiceBusError> {
        self.ensure_permission(source, &Permission::event_publish())?;
        Ok(self.event_bus.publish(event)?)
    }

    fn subscribe_events(&self, source: &ServiceId) -> Result<EventReceiver, ServiceBusError> {
        self.ensure_permission(source, &Permission::event_subscribe())?;
        Ok(self.event_bus.subscribe()?)
    }

    fn register_request_handler(
        &self,
        subject: impl Into<String>,
        handler: Arc<dyn RequestHandler>,
    ) -> Result<(), ServiceBusError> {
        let subject = subject.into();
        if subject.trim().is_empty() {
            return Err(ServiceBusError::InvalidSubject);
        }
        self.request_handlers
            .lock()
            .map_err(|_| ServiceBusError::BusUnavailable)?
            .insert(subject, handler);
        Ok(())
    }

    fn request(
        &self,
        source: &ServiceId,
        request: &ServiceRequest,
    ) -> Result<ServiceReply, ServiceBusError> {
        self.ensure_permission(source, &Permission::service_command())?;
        let handlers = self
            .request_handlers
            .lock()
            .map_err(|_| ServiceBusError::BusUnavailable)?;
        let handler = handlers
            .get(request.subject())
            .ok_or_else(|| ServiceBusError::NoRoute {
                route: request.subject().to_owned(),
            })?;
        handler.handle(request)
    }

    fn register_command_handler(
        &self,
        route: impl Into<String>,
        handler: Arc<dyn CommandHandler>,
    ) -> Result<(), ServiceBusError> {
        let route = route.into();
        if route.trim().is_empty() {
            return Err(ServiceBusError::InvalidCommand);
        }
        self.command_handlers
            .lock()
            .map_err(|_| ServiceBusError::BusUnavailable)?
            .insert(route, handler);
        Ok(())
    }

    fn command(
        &self,
        source: &ServiceId,
        command: &ServiceCommand,
    ) -> Result<ServiceReply, ServiceBusError> {
        self.ensure_permission(source, &Permission::service_command())?;
        let handlers = self
            .command_handlers
            .lock()
            .map_err(|_| ServiceBusError::BusUnavailable)?;
        let handler = handlers
            .get(command.route())
            .ok_or_else(|| ServiceBusError::NoRoute {
                route: command.route().to_owned(),
            })?;
        handler.handle(command)
    }

    fn subscribe_notifications(&self) -> Result<ServiceNotificationReceiver, ServiceBusError> {
        let (sender, receiver) = mpsc::channel();
        self.notification_subscribers
            .lock()
            .map_err(|_| ServiceBusError::BusUnavailable)?
            .push(sender);
        Ok(ServiceNotificationReceiver { receiver })
    }

    fn notify(&self, notification: &ServiceNotification) -> Result<usize, ServiceBusError> {
        self.ensure_permission(notification.source_service(), &Permission::event_publish())?;
        let mut subscribers = self
            .notification_subscribers
            .lock()
            .map_err(|_| ServiceBusError::BusUnavailable)?;

        let mut delivered = 0usize;
        subscribers.retain(|sender| {
            if sender.send(notification.clone()).is_ok() {
                delivered += 1;
                true
            } else {
                false
            }
        });
        Ok(delivered)
    }

    fn status(&self) -> Result<ServiceBusStatus, ServiceBusError> {
        Ok(ServiceBusStatus {
            event_bus: "in-memory".to_owned(),
            notification_subscribers: self
                .notification_subscribers
                .lock()
                .map_err(|_| ServiceBusError::BusUnavailable)?
                .len(),
            request_handlers: self
                .request_handlers
                .lock()
                .map_err(|_| ServiceBusError::BusUnavailable)?
                .len(),
            command_handlers: self
                .command_handlers
                .lock()
                .map_err(|_| ServiceBusError::BusUnavailable)?
                .len(),
            contract_handlers: self
                .contract_handlers
                .lock()
                .map_err(|_| ServiceBusError::BusUnavailable)?
                .len(),
            permission_entries: self
                .permissions
                .lock()
                .map_err(|_| ServiceBusError::BusUnavailable)?
                .len(),
        })
    }
}

impl AetherContractBus for InMemoryAetherServiceBus {
    fn register_contract_handler(
        &self,
        contract: BusContract,
        handler: Arc<dyn ContractHandler>,
    ) -> Result<(), ServiceBusError> {
        self.contract_handlers
            .lock()
            .map_err(|_| ServiceBusError::BusUnavailable)?
            .insert(contract.route_key(), handler);
        Ok(())
    }

    fn request_contract(
        &self,
        source: &ServiceId,
        request: &ContractRequest,
    ) -> Result<ContractReply, ServiceBusError> {
        self.ensure_permission(source, &Permission::service_command())?;
        let handlers = self
            .contract_handlers
            .lock()
            .map_err(|_| ServiceBusError::BusUnavailable)?;
        let route = request.contract().route_key();
        let handler = handlers
            .get(&route)
            .ok_or(ServiceBusError::NoRoute { route })?;
        handler.handle(request)
    }
}

/// Service notification receiver.
#[derive(Debug)]
pub struct ServiceNotificationReceiver {
    receiver: Receiver<ServiceNotification>,
}

impl ServiceNotificationReceiver {
    /// Receive the next notification, waiting up to `timeout`.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceBusError`] when no notification arrives or the receiver is disconnected.
    pub fn recv_timeout(&self, timeout: Duration) -> Result<ServiceNotification, ServiceBusError> {
        match self.receiver.recv_timeout(timeout) {
            Ok(notification) => Ok(notification),
            Err(RecvTimeoutError::Timeout) => Err(ServiceBusError::ReceiveTimeout),
            Err(RecvTimeoutError::Disconnected) => Err(ServiceBusError::ReceiverDisconnected),
        }
    }
}

/// Service Bus status snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServiceBusStatus {
    /// Event bus implementation.
    pub event_bus: String,
    /// Number of notification subscribers.
    pub notification_subscribers: usize,
    /// Number of request handlers.
    pub request_handlers: usize,
    /// Number of command handlers.
    pub command_handlers: usize,
    /// Number of typed contract handlers.
    pub contract_handlers: usize,
    /// Number of permission entries.
    pub permission_entries: usize,
}

/// Service Bus errors.
#[derive(Debug, Error)]
pub enum ServiceBusError {
    /// Event subsystem failed.
    #[error("event subsystem failed: {0}")]
    Events(#[from] EventError),
    /// Bus internal lock is unavailable.
    #[error("service bus is unavailable")]
    BusUnavailable,
    /// Subject is invalid.
    #[error("invalid service bus subject")]
    InvalidSubject,
    /// Command is invalid.
    #[error("invalid service command")]
    InvalidCommand,
    /// Contract is invalid.
    #[error("invalid service contract")]
    InvalidContract,
    /// Route has no handler.
    #[error("no route registered for {route}")]
    NoRoute {
        /// Missing route.
        route: String,
    },
    /// Service lacks the required permission.
    #[error("service {service_id} lacks permission {permission}")]
    PermissionDenied {
        /// Service identifier.
        service_id: String,
        /// Required permission.
        permission: String,
    },
    /// No notification was received before timeout.
    #[error("timed out waiting for service notification")]
    ReceiveTimeout,
    /// Notification receiver was disconnected.
    #[error("service notification receiver disconnected")]
    ReceiverDisconnected,
}

/// Register a service descriptor with a registry and bus permission index.
///
/// # Errors
///
/// Returns [`ServiceBusError`] when the bus permission index is unavailable, or
/// [`ServiceError`] when the service cannot be registered.
pub fn register_service_with_bus(
    registry: &mut ServiceRegistry,
    bus: &InMemoryAetherServiceBus,
    descriptor: ServiceDescriptor,
) -> Result<(), RegisterServiceError> {
    let service_id = descriptor.id.clone();
    let permissions = descriptor.permissions.clone();
    registry.register(descriptor)?;
    bus.register_permissions(&service_id, permissions)?;
    Ok(())
}

/// Error raised while registering a service with the bus.
#[derive(Debug, Error)]
pub enum RegisterServiceError {
    /// Service registry failed.
    #[error("service registry failed: {0}")]
    Service(#[from] ServiceError),
    /// Service bus failed.
    #[error("service bus failed: {0}")]
    Bus(#[from] ServiceBusError),
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use aether_events::{Event, EventSource, EventType};
    use aether_permissions::{Permission, PermissionSet};
    use aether_service::{ServiceDescriptor, ServiceManifest};
    use serde_json::json;

    use super::{
        AetherContractBus, AetherServiceBus, BusContract, ContractReply, ContractRequest,
        InMemoryAetherServiceBus, ServiceBusError, ServiceCommand, ServiceNotification,
        ServiceReply, ServiceRequest,
    };

    fn service_id() -> aether_ids::ServiceId {
        aether_ids::ServiceId::new("telemetry-service").expect("service id")
    }

    fn permissions() -> PermissionSet {
        PermissionSet::new(vec![
            Permission::event_publish(),
            Permission::event_subscribe(),
            Permission::service_command(),
        ])
    }

    fn event() -> Event {
        Event::new(
            EventSource::new("service-bus-test").expect("source"),
            EventType::SystemStarted,
        )
    }

    #[test]
    fn asb_publishes_event() {
        let bus = InMemoryAetherServiceBus::new();
        let service_id = service_id();
        bus.register_permissions(&service_id, permissions())
            .expect("permissions");
        let subscription = bus.subscribe_events(&service_id).expect("subscribe");

        let delivered = bus.publish_event(&service_id, &event()).expect("publish");

        assert_eq!(delivered, 1);
        assert!(
            subscription
                .recv_timeout(Duration::from_millis(100))
                .is_ok()
        );
    }

    #[test]
    fn asb_rejects_publish_without_permission() {
        let bus = InMemoryAetherServiceBus::new();
        let service_id = service_id();

        let result = bus.publish_event(&service_id, &event());

        assert!(matches!(
            result,
            Err(ServiceBusError::PermissionDenied { .. })
        ));
    }

    #[test]
    fn asb_handles_request_reply() {
        let bus = InMemoryAetherServiceBus::new();
        let service_id = service_id();
        bus.register_permissions(&service_id, permissions())
            .expect("permissions");
        bus.register_request_handler(
            "telemetry.echo",
            Arc::new(|request: &ServiceRequest| {
                Ok(
                    ServiceReply::accepted()
                        .with_payload_value("subject", json!(request.subject())),
                )
            }),
        )
        .expect("handler");

        let reply = bus
            .request(
                &service_id,
                &ServiceRequest::new("telemetry.echo").expect("request"),
            )
            .expect("reply");

        assert!(reply.is_accepted());
        assert_eq!(
            reply.payload().get("subject"),
            Some(&json!("telemetry.echo"))
        );
    }

    #[test]
    fn asb_routes_service_command() {
        let bus = InMemoryAetherServiceBus::new();
        let source = service_id();
        bus.register_permissions(&source, permissions())
            .expect("permissions");
        bus.register_command_handler(
            "telemetry.commands",
            Arc::new(|command: &ServiceCommand| {
                Ok(ServiceReply::accepted().with_payload_value("command", json!(command.name())))
            }),
        )
        .expect("handler");

        let reply = bus
            .command(
                &source,
                &ServiceCommand::new("telemetry.commands", "refresh").expect("command"),
            )
            .expect("reply");

        assert_eq!(reply.payload().get("command"), Some(&json!("refresh")));
    }

    #[test]
    fn asb_delivers_service_notification() {
        let bus = InMemoryAetherServiceBus::new();
        let service_id = service_id();
        bus.register_permissions(&service_id, permissions())
            .expect("permissions");
        let receiver = bus.subscribe_notifications().expect("subscriber");

        let delivered = bus
            .notify(
                &ServiceNotification::new(service_id, "telemetry.updated")
                    .expect("notification")
                    .with_payload_value("ok", json!(true)),
            )
            .expect("notify");
        let notification = receiver
            .recv_timeout(Duration::from_millis(100))
            .expect("notification");

        assert_eq!(delivered, 1);
        assert_eq!(notification.topic(), "telemetry.updated");
    }

    #[test]
    fn asb_status_reports_routes() {
        let bus = InMemoryAetherServiceBus::new();
        let service_id = service_id();
        bus.register_permissions(&service_id, permissions())
            .expect("permissions");
        bus.register_request_handler(
            "telemetry.echo",
            Arc::new(|_request: &ServiceRequest| Ok(ServiceReply::accepted())),
        )
        .expect("handler");

        let status = bus.status().expect("status");

        assert_eq!(status.event_bus, "in-memory");
        assert_eq!(status.request_handlers, 1);
        assert_eq!(status.contract_handlers, 0);
        assert_eq!(status.permission_entries, 1);
    }

    #[test]
    fn contract_bus_routes_typed_contract_request() {
        let bus = InMemoryAetherServiceBus::new();
        let service_id = service_id();
        let contract = BusContract::new("system.inspect", "v1").expect("contract");
        bus.register_permissions(&service_id, permissions())
            .expect("permissions");
        bus.register_contract_handler(
            contract.clone(),
            Arc::new(|request: &ContractRequest| {
                Ok(ContractReply::accepted()
                    .with_payload_value("subject", json!(request.contract().subject()))
                    .with_payload_value("version", json!(request.contract().version())))
            }),
        )
        .expect("handler");

        let reply = bus
            .request_contract(&service_id, &ContractRequest::new(contract))
            .expect("reply");
        let status = bus.status().expect("status");

        assert!(reply.is_accepted());
        assert_eq!(
            reply.payload().get("subject"),
            Some(&json!("system.inspect"))
        );
        assert_eq!(reply.payload().get("version"), Some(&json!("v1")));
        assert_eq!(status.contract_handlers, 1);
    }

    #[test]
    fn service_manifest_permissions_can_register_with_bus() {
        let manifest = ServiceManifest::from_toml_str(
            r#"
            [service]
            name = "telemetry-service"
            version = "0.1.0"
            description = "Base telemetry service"
            owner = "neuroforge-labs"

            [capabilities]
            provides = ["telemetry.emit"]
            requires = []

            [permissions]
            requested = ["event.publish", "event.subscribe"]

            [resources]
            cpu_class = "low"
            memory_class = "low"
            storage_class = "none"
            network = false
            "#,
        )
        .expect("manifest");
        let descriptor = ServiceDescriptor::from_manifest(manifest).expect("descriptor");
        let bus = InMemoryAetherServiceBus::new();

        bus.register_permissions(&descriptor.id, descriptor.permissions)
            .expect("permissions");

        assert_eq!(bus.status().expect("status").permission_entries, 1);
    }
}
