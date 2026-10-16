use super::{Message, Topic, TopicKind};
use std::fmt;

/// A topic and body disagree under the checked-pairing policy.
///
/// Errors identify the failed check without retaining the input topic/body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairingError {
    /// The topic selects a different body kind.
    KindMismatch {
        /// Kind selected by the topic.
        topic_kind: TopicKind,
        /// Kind of the supplied body.
        body_kind: TopicKind,
    },
    /// Header version is not exactly `3.0.0`.
    UnsupportedVersion,
    /// Topic and header manufacturer differ, without normalization.
    ManufacturerMismatch,
    /// Topic and header serial number differ, without normalization.
    SerialNumberMismatch,
}

impl fmt::Display for PairingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KindMismatch {
                topic_kind,
                body_kind,
            } => {
                write!(
                    formatter,
                    "topic kind {topic_kind} does not match body kind {body_kind}"
                )
            }
            Self::UnsupportedVersion => {
                formatter.write_str("header.version: expected exactly 3.0.0")
            }
            Self::ManufacturerMismatch => formatter.write_str("topic/header manufacturer mismatch"),
            Self::SerialNumberMismatch => formatter.write_str("topic/header serialNumber mismatch"),
        }
    }
}

impl std::error::Error for PairingError {}

/// An owned topic/body pair with checked kind, identity and version consistency.
///
/// Construction requires equal kind, header version exactly `3.0.0`, and exact
/// manufacturer/serial equality. No trimming, normalization or repair is done.
/// These are library helper policies, not full protocol or body validation.
///
/// Private fields and read-only access preserve consistency. Consume the pair
/// with [`Self::into_parts`] to edit either value; reconstruct it to recheck.
/// There is no Serde envelope representation.
///
/// ```compile_fail
/// use vda5050_v3_types::topic::{Message, TopicMessage};
///
/// fn change_identity(pair: &mut TopicMessage) {
///     if let Message::Connection(body) = pair.message() {
///         body.header.serial_number = "different".into();
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TopicMessage {
    topic: Topic,
    message: Message,
}

impl TopicMessage {
    /// Own and check a topic/body pair without changing either value.
    ///
    /// Checks kind, version, manufacturer and serial number in that order,
    /// returning the first failure. Inputs are consumed even on failure.
    /// A concrete body, boxed body or [`Message`] can be supplied.
    pub fn new(topic: Topic, message: impl Into<Message>) -> Result<Self, PairingError> {
        let message = message.into();
        if topic.kind() != message.topic_kind() {
            return Err(PairingError::KindMismatch {
                topic_kind: topic.kind(),
                body_kind: message.topic_kind(),
            });
        }
        let header = message.header();
        if header.version != super::message::HEADER_VERSION {
            return Err(PairingError::UnsupportedVersion);
        }
        if topic.manufacturer() != header.manufacturer {
            return Err(PairingError::ManufacturerMismatch);
        }
        if topic.serial_number() != header.serial_number {
            return Err(PairingError::SerialNumberMismatch);
        }
        Ok(Self { topic, message })
    }

    /// Borrow the checked topic, without mutable access.
    pub fn topic(&self) -> &Topic {
        &self.topic
    }

    /// Borrow the checked message, without mutable access to its body.
    pub fn message(&self) -> &Message {
        &self.message
    }

    /// Consume the pair and return its owned topic and message for independent use.
    pub fn into_parts(self) -> (Topic, Message) {
        (self.topic, self.message)
    }
}
