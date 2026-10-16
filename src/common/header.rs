use chrono::{DateTime, Utc};

object_only_deserialize! {
/// Protocol header defined by VDA 5050 v3.0.0, section 7.2.
///
/// Messages embed this as `header`, but serialize its fields at the message
/// root. All fields are required; there are no implicit defaults.
///
/// String content (including protocol version and robot identity) and header
/// progression are not semantically validated by header encoding/decoding.
/// With `topic`, explicit body-to-topic helpers and checked pairing apply their
/// separately documented identity/version policies.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Header {
    /// Per-topic message counter, represented as the specified `uint32`.
    pub header_id: u32,
    /// UTC timestamp. On the wire, exactly `YYYY-MM-DDTHH:mm:ss.fffZ`.
    ///
    /// Serialization rejects sub-millisecond precision and years outside
    /// `0000..=9999`, rather than truncating or changing the required format.
    #[cfg_attr(feature = "serde", serde(with = "crate::timestamp"))]
    pub timestamp: DateTime<Utc>,
    /// Protocol version in `Major.Minor.Patch` form (normally `"3.0.0"`).
    pub version: String,
    /// Manufacturer of the mobile robot.
    pub manufacturer: String,
    /// Manufacturer-assigned serial number of the mobile robot.
    pub serial_number: String,
}
}
