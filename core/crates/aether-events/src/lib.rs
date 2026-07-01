#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Internal event contracts and event bus for Aether.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use aether_ids::{IdPrefix, TypedId};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use thiserror::Error;
use time::OffsetDateTime;
use uuid::Uuid;

/// Event metadata map.
pub type Metadata = BTreeMap<String, String>;

/// Event payload represented as structured JSON.
pub type EventPayload = Map<String, Value>;

/// Unique event identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EventId(Uuid);

impl EventId {
    /// Create a time-sortable event identifier.
    #[must_use]
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    /// Return this event identifier using the Phase 2 typed ID prefix strategy.
    #[must_use]
    pub fn typed(&self) -> TypedId {
        match TypedId::new(IdPrefix::Event, self.0.as_simple().to_string()) {
            Ok(id) => id,
            Err(_) => TypedId::generate(IdPrefix::Event),
        }
    }
}

impl fmt::Display for EventId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// Source component that emitted an event.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EventSource(String);

impl EventSource {
    /// Create an event source.
    ///
    /// # Errors
    ///
    /// Returns [`EventError::InvalidSource`] when the value is empty.
    pub fn new(value: impl Into<String>) -> Result<Self, EventError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(EventError::InvalidSource);
        }
        Ok(Self(value))
    }

    /// Return the source as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EventSource {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Base event types supported by Phase 1.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    /// Runtime has started.
    SystemStarted,
    /// Runtime has stopped.
    SystemStopped,
    /// Module has been loaded.
    ModuleLoaded,
    /// Module failed during lifecycle execution.
    ModuleFailed,
    /// Runtime configuration has been loaded.
    ConfigLoaded,
    /// Health check was requested.
    HealthCheckRequested,
    /// Health check completed.
    HealthCheckCompleted,
}

impl EventType {
    /// Return the canonical event type name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemStarted => "system_started",
            Self::SystemStopped => "system_stopped",
            Self::ModuleLoaded => "module_loaded",
            Self::ModuleFailed => "module_failed",
            Self::ConfigLoaded => "config_loaded",
            Self::HealthCheckRequested => "health_check_requested",
            Self::HealthCheckCompleted => "health_check_completed",
        }
    }
}

/// Internal Aether event.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    id: EventId,
    timestamp: OffsetDateTime,
    source: EventSource,
    #[serde(rename = "event_type")]
    kind: EventType,
    payload: EventPayload,
    metadata: Metadata,
}

impl Event {
    /// Create a new event with generated identity and timestamp.
    #[must_use]
    pub fn new(source: EventSource, event_type: EventType) -> Self {
        Self {
            id: EventId::generate(),
            timestamp: OffsetDateTime::now_utc(),
            source,
            kind: event_type,
            payload: EventPayload::new(),
            metadata: Metadata::new(),
        }
    }

    /// Attach a JSON payload value.
    #[must_use]
    pub fn with_payload_value(mut self, key: impl Into<String>, value: Value) -> Self {
        self.payload.insert(key.into(), value);
        self
    }

    /// Attach a metadata value.
    #[must_use]
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Return the event identifier.
    #[must_use]
    pub const fn id(&self) -> EventId {
        self.id
    }

    /// Return the event timestamp.
    #[must_use]
    pub const fn timestamp(&self) -> OffsetDateTime {
        self.timestamp
    }

    /// Return the event source.
    #[must_use]
    pub const fn source(&self) -> &EventSource {
        &self.source
    }

    /// Return the event type.
    #[must_use]
    pub const fn event_type(&self) -> EventType {
        self.kind
    }

    /// Return the event payload.
    #[must_use]
    pub const fn payload(&self) -> &EventPayload {
        &self.payload
    }

    /// Return event metadata.
    #[must_use]
    pub const fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

/// In-memory publish/subscribe event bus.
#[derive(Clone, Debug, Default)]
pub struct EventBus {
    subscribers: Arc<Mutex<Vec<Sender<Event>>>>,
}

/// Event bus abstraction used by runtime and kernel orchestration.
pub trait EventBusPort: Send + Sync {
    /// Subscribe to future events.
    ///
    /// # Errors
    ///
    /// Returns [`EventError`] when the bus cannot create the subscription.
    fn subscribe(&self) -> Result<EventReceiver, EventError>;

    /// Publish an event to active subscribers.
    ///
    /// # Errors
    ///
    /// Returns [`EventError`] when the event cannot be published.
    fn publish(&self, event: &Event) -> Result<usize, EventError>;
}

impl EventBus {
    /// Create an empty event bus.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Subscribe to future events.
    ///
    /// # Errors
    ///
    /// Returns [`EventError::BusUnavailable`] if the bus lock is poisoned.
    pub fn subscribe(&self) -> Result<EventReceiver, EventError> {
        self.subscribe_inner()
    }

    fn subscribe_inner(&self) -> Result<EventReceiver, EventError> {
        let (sender, receiver) = mpsc::channel();
        self.subscribers
            .lock()
            .map_err(|_| EventError::BusUnavailable)?
            .push(sender);
        Ok(EventReceiver { receiver })
    }

    /// Publish an event to active subscribers.
    ///
    /// Returns the number of subscribers that received the event.
    ///
    /// # Errors
    ///
    /// Returns [`EventError::BusUnavailable`] if the bus lock is poisoned.
    pub fn publish(&self, event: &Event) -> Result<usize, EventError> {
        self.publish_inner(event)
    }

    fn publish_inner(&self, event: &Event) -> Result<usize, EventError> {
        let mut subscribers = self
            .subscribers
            .lock()
            .map_err(|_| EventError::BusUnavailable)?;

        let mut delivered = 0usize;
        subscribers.retain(|sender| {
            if sender.send(event.clone()).is_ok() {
                delivered += 1;
                true
            } else {
                false
            }
        });

        Ok(delivered)
    }
}

impl EventBusPort for EventBus {
    fn subscribe(&self) -> Result<EventReceiver, EventError> {
        self.subscribe_inner()
    }

    fn publish(&self, event: &Event) -> Result<usize, EventError> {
        self.publish_inner(event)
    }
}

/// Event subscription receiver.
#[derive(Debug)]
pub struct EventReceiver {
    receiver: Receiver<Event>,
}

impl EventReceiver {
    /// Receive the next event, waiting up to `timeout`.
    ///
    /// # Errors
    ///
    /// Returns [`EventError::ReceiveTimeout`] when no event arrives before timeout
    /// or [`EventError::ReceiverDisconnected`] when the bus is closed.
    pub fn recv_timeout(&self, timeout: Duration) -> Result<Event, EventError> {
        match self.receiver.recv_timeout(timeout) {
            Ok(event) => Ok(event),
            Err(RecvTimeoutError::Timeout) => Err(EventError::ReceiveTimeout),
            Err(RecvTimeoutError::Disconnected) => Err(EventError::ReceiverDisconnected),
        }
    }
}

/// Event subsystem errors.
#[derive(Debug, Error)]
pub enum EventError {
    /// Event source was empty.
    #[error("event source cannot be empty")]
    InvalidSource,
    /// Event bus lock is unavailable.
    #[error("event bus is unavailable")]
    BusUnavailable,
    /// No event was received within the requested timeout.
    #[error("timed out waiting for event")]
    ReceiveTimeout,
    /// Event receiver was disconnected.
    #[error("event receiver disconnected")]
    ReceiverDisconnected,
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use serde_json::json;

    use super::{Event, EventBus, EventBusPort, EventSource, EventType};

    fn publish_with_port(bus: &dyn EventBusPort, event: &Event) -> usize {
        bus.publish(event).expect("publish through port")
    }

    #[test]
    fn publish_event_delivers_to_subscriber() {
        let bus = EventBus::new();
        let subscription = bus.subscribe().expect("subscriber");
        let source = EventSource::new("test").expect("source");
        let event = Event::new(source, EventType::SystemStarted)
            .with_payload_value("status", json!("ok"))
            .with_metadata("trace_id", "phase-1");

        let delivered = publish_with_port(&bus, &event);
        let received_event = subscription
            .recv_timeout(Duration::from_millis(100))
            .expect("receive event");

        assert_eq!(delivered, 1);
        assert_eq!(received_event.event_type(), EventType::SystemStarted);
        assert_eq!(
            received_event.payload().get("status"),
            event.payload().get("status")
        );
        assert_eq!(
            received_event.metadata().get("trace_id"),
            event.metadata().get("trace_id")
        );
    }

    #[test]
    fn invalid_source_is_rejected() {
        assert!(EventSource::new(" ").is_err());
    }

    #[test]
    fn event_id_exposes_typed_prefix() {
        let source = EventSource::new("test").expect("source");
        let event = Event::new(source, EventType::SystemStarted);

        assert!(event.id().typed().as_str().starts_with("evt_"));
    }
}
