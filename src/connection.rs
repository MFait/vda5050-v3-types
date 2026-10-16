//! Connection message body (VDA 5050 v3.0.0, sections 6.5 and 7.7).
//!
//! Retained delivery and last-will configuration belong to the caller's MQTT
//! implementation, not these types.

use crate::common::Header;

object_only_deserialize! {
/// Connection status message body. All six wire fields are required.
///
/// Unknown fields are ignored on decoding and not preserved on encoding.
/// Decoding does not check freshness, counter progression, or robot behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Connection {
    /// Shared protocol header, flattened into the JSON root.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub header: Header,
    /// Reported connection state.
    pub connection_state: ConnectionState,
}
}

string_enum! {
/// Exact connection-state values from section 7.7.
///
/// Unknown wire values are rejected rather than mapped to a fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum ConnectionState {
    /// Robot and broker are connected.
    Online,
    /// The connection was closed in a coordinated way.
    Offline,
    /// The robot remains connected but stops sending state messages.
    Hibernating,
    /// The connection ended unexpectedly; used in the last-will body.
    ConnectionBroken,
}
}

#[cfg(feature = "json")]
impl_json_helpers!(Connection);
