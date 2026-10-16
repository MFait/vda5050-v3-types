use super::{Topic, TopicError, TopicKind};
use crate::common::Header;
use std::fmt;

pub(super) const HEADER_VERSION: &str = "3.0.0";

/// Failure to derive a topic from a message header.
///
/// This helper requires exactly header version `3.0.0`. That is a library
/// compatibility policy, not a restriction imposed by ordinary body decoding.
/// Errors do not retain the header or input interface name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderTopicError {
    /// The header version is not exactly `3.0.0`.
    UnsupportedVersion,
    /// The interface name or header identity cannot form a checked topic.
    InvalidTopic(TopicError),
}

impl fmt::Display for HeaderTopicError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedVersion => {
                formatter.write_str("header.version: expected exactly 3.0.0")
            }
            Self::InvalidTopic(error) => write!(formatter, "cannot derive topic: {error}"),
        }
    }
}

impl std::error::Error for HeaderTopicError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::UnsupportedVersion => None,
            Self::InvalidTopic(error) => Some(error),
        }
    }
}

fn from_header(
    header: &Header,
    interface: impl Into<String>,
    kind: TopicKind,
) -> Result<Topic, HeaderTopicError> {
    if header.version != HEADER_VERSION {
        return Err(HeaderTopicError::UnsupportedVersion);
    }
    Topic::new(interface, &header.manufacturer, &header.serial_number, kind)
        .map_err(HeaderTopicError::InvalidTopic)
}

/// One of the eight VDA 5050 message bodies, independent of a concrete topic.
///
/// Each variant boxes its body to avoid sizing every message for the largest
/// body. Wrapping an owned body through `From` allocates once; matching a variant
/// gives access to its concrete body. Construction does not validate the body.
///
/// This enum has no Serde representation or implicit JSON envelope. With `json`,
/// topic-directed decoding selects the body using the external topic kind.
#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    /// Fleet-control order.
    Order(Box<crate::Order>),
    /// Immediate actions.
    InstantActions(Box<crate::InstantActions>),
    /// Reported robot state.
    State(Box<crate::State>),
    /// Position and planned-path updates.
    Visualization(Box<crate::Visualization>),
    /// Connection status.
    Connection(Box<crate::Connection>),
    /// Robot capabilities and configuration.
    Factsheet(Box<crate::Factsheet>),
    /// Zone definitions.
    ZoneSet(Box<crate::ZoneSet>),
    /// Responses to robot requests.
    Responses(Box<crate::Responses>),
}

impl Message {
    /// Return the contained body's fixed topic kind, without validation.
    pub const fn topic_kind(&self) -> TopicKind {
        match self {
            Self::Order(_) => TopicKind::Order,
            Self::InstantActions(_) => TopicKind::InstantActions,
            Self::State(_) => TopicKind::State,
            Self::Visualization(_) => TopicKind::Visualization,
            Self::Connection(_) => TopicKind::Connection,
            Self::Factsheet(_) => TopicKind::Factsheet,
            Self::ZoneSet(_) => TopicKind::ZoneSet,
            Self::Responses(_) => TopicKind::Responses,
        }
    }

    /// Borrow the contained body's header without changing or validating it.
    pub fn header(&self) -> &Header {
        match self {
            Self::Order(body) => &body.header,
            Self::InstantActions(body) => &body.header,
            Self::State(body) => &body.header,
            Self::Visualization(body) => &body.header,
            Self::Connection(body) => &body.header,
            Self::Factsheet(body) => &body.header,
            Self::ZoneSet(body) => &body.header,
            Self::Responses(body) => &body.header,
        }
    }

    /// Derive an owned topic using the contained body's kind and header.
    ///
    /// Requires header version exactly `3.0.0` and the same layout checks as
    /// the concrete body's `topic` method. Does not otherwise validate the body.
    pub fn topic(&self, interface: impl Into<String>) -> Result<Topic, HeaderTopicError> {
        from_header(self.header(), interface, self.topic_kind())
    }
}

macro_rules! impl_topic_helpers {
    ($($message:ident => $kind:ident),+ $(,)?) => {
        $(
            impl From<crate::$message> for Message {
                fn from(body: crate::$message) -> Self {
                    Self::$kind(Box::new(body))
                }
            }

            impl From<Box<crate::$message>> for Message {
                fn from(body: Box<crate::$message>) -> Self {
                    Self::$kind(body)
                }
            }

            impl crate::$message {
                /// Return this message type's fixed topic kind.
                ///
                /// This does not validate the header or construct a complete topic.
                pub const fn topic_kind(&self) -> TopicKind {
                    TopicKind::$kind
                }

                /// Build a v3 topic using this body's kind and header identity.
                ///
                /// Requires header version exactly `3.0.0`, then checks the
                /// interface, manufacturer and serial number using [`Topic::new`].
                /// Identity strings are copied unchanged; the body is not
                /// mutated or otherwise validated. The returned topic is an
                /// owned snapshot, not a binding to future header changes.
                pub fn topic(&self, interface: impl Into<String>) -> Result<Topic, HeaderTopicError> {
                    from_header(&self.header, interface, self.topic_kind())
                }
            }
        )+
    };
}

impl_topic_helpers! {
    Order => Order,
    InstantActions => InstantActions,
    State => State,
    Visualization => Visualization,
    Connection => Connection,
    Factsheet => Factsheet,
    ZoneSet => ZoneSet,
    Responses => Responses,
}
