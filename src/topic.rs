//! VDA 5050 v3.0.0 topic names and suggested layout (sections 4.2-4.3).
//!
//! Names are independent of a broker's topic hierarchy. [`Topic`] models only
//! the suggested five-level layout, not deployment-specific cloud hierarchies.
//! With `json`, decoding selects a body by topic kind, without header-consistency
//! validation. [`TopicFilter`] builds five-level, single-level-wildcard filters;
//! no matching, broker or subscription operations are provided.

use std::{fmt, str::FromStr};

mod filter;
#[cfg(feature = "json")]
mod json;
mod layout;
mod message;
mod pairing;
pub use filter::TopicFilter;
pub use layout::{Topic, TopicError, TopicLevel};
pub use message::{HeaderTopicError, Message};
pub use pairing::{PairingError, TopicMessage};

string_enum! {
/// One of the eight exact, case-sensitive topic names in section 4.3.
///
/// This represents the message-kind name, not a complete MQTT topic or filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum TopicKind {
    /// Fleet-control orders.
    Order,
    /// Actions to execute immediately.
    InstantActions,
    /// Reported robot state.
    State,
    /// High-frequency position and planned-path updates.
    Visualization,
    /// Robot/broker connection status.
    Connection,
    /// Robot capabilities and configuration.
    Factsheet,
    /// Zone definitions for a map.
    ZoneSet,
    /// Fleet-control responses to robot requests.
    Responses,
}
}

impl TopicKind {
    /// Return the mandatory topic name, without a hierarchy prefix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Order => "order",
            Self::InstantActions => "instantActions",
            Self::State => "state",
            Self::Visualization => "visualization",
            Self::Connection => "connection",
            Self::Factsheet => "factsheet",
            Self::ZoneSet => "zoneSet",
            Self::Responses => "responses",
        }
    }
}

impl fmt::Display for TopicKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A string is not one of the eight exact topic names.
///
/// Parsing does not trim whitespace, normalize capitalization, extract the
/// final level of a full topic or accept wildcard filters. The error does not
/// retain the input string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnknownTopicKind;

impl fmt::Display for UnknownTopicKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("expected an exact VDA 5050 v3.0.0 topic name")
    }
}

impl std::error::Error for UnknownTopicKind {}

impl FromStr for TopicKind {
    type Err = UnknownTopicKind;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "order" => Ok(Self::Order),
            "instantActions" => Ok(Self::InstantActions),
            "state" => Ok(Self::State),
            "visualization" => Ok(Self::Visualization),
            "connection" => Ok(Self::Connection),
            "factsheet" => Ok(Self::Factsheet),
            "zoneSet" => Ok(Self::ZoneSet),
            "responses" => Ok(Self::Responses),
            _ => Err(UnknownTopicKind),
        }
    }
}
