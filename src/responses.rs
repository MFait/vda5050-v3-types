//! Fleet-control response body (VDA 5050 v3.0.0, section 7.5).

use chrono::{DateTime, Utc};

use crate::common::Header;

object_only_deserialize! {
/// Fleet-control answers to robot requests.
///
/// Parsing does not check active requests, identifier uniqueness, grant
/// transitions or lease freshness. An empty list remains representable.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Responses {
    /// Shared header, flattened into the wire object.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub header: Header,
    /// Required response list.
    pub responses: Vec<Response>,
}
}

#[cfg(feature = "json")]
impl_json_helpers!(Responses);

object_only_deserialize! {
/// Answer to one robot request (section 7.5).
///
/// The specification permits lease expiry only for GRANTED responses.
/// This cross-field rule is not enforced by encoding/decoding.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Response {
    /// Request identifier, unique per robot among active requests.
    pub request_id: String,
    /// Fleet-control decision.
    pub grant_type: GrantType,
    /// Optional UTC expiry, encoded with exactly millisecond precision.
    ///
    /// Absence is omitted; explicit null is rejected. Serialization rejects
    /// sub-millisecond precision and years outside `0000..=9999`.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::timestamp::optional"))]
    pub lease_expiry: Option<DateTime<Utc>>,
}
}

string_enum! {
/// Exact grant decisions defined in section 7.5.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum GrantType {
    /// Permission is granted.
    Granted,
    /// Request is queued; permission is not yet granted.
    Queued,
    /// Previously granted permission is revoked.
    Revoked,
    /// Request is rejected.
    Rejected,
}
}
