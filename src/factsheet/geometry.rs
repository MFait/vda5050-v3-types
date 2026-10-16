object_only_deserialize! {
/// Robot wheel arrangement and envelopes.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct MobileRobotGeometry {
    /// Wheel definitions, if reported.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub wheel_definitions: Option<Vec<WheelDefinition>>,
    /// Two-dimensional envelopes, if reported.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub envelopes2d: Option<Vec<Envelope2d>>,
    /// Three-dimensional envelopes, if reported.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub envelopes3d: Option<Vec<Envelope3d>>,
}
}

object_only_deserialize! {
/// Wheel geometry; applicability constraints are not checked.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct WheelDefinition {
    /// Extensible type, e.g. DRIVE, CASTER, FIXED or MECANUM.
    pub r#type: String,
    /// Whether actively driven.
    pub is_active_driven: bool,
    /// Whether actively steered.
    pub is_active_steered: bool,
    /// Position in robot coordinates.
    pub position: WheelPosition,
    /// Finite nominal diameter in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::factsheet::wire::diameter"))]
    pub diameter: f64,
    /// Finite nominal width in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::factsheet::wire::width"))]
    pub width: f64,
    /// Finite center displacement in meters; omitted default zero is preserved.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::factsheet::wire::center_displacement::optional"))]
    pub center_displacement: Option<f64>,
    /// Manufacturer-defined constraints.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub constraints: Option<String>,
}
}

object_only_deserialize! {
/// Wheel position; no explicit angle range is asserted for theta.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct WheelPosition {
    /// Finite X-coordinate in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::x"))]
    pub x: f64,
    /// Finite Y-coordinate in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::y"))]
    pub y: f64,
    /// Finite orientation in radians, if reported.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::bounding_box_theta::optional"))]
    pub theta: Option<f64>,
}
}

object_only_deserialize! {
/// Closed polygon envelope; simplicity and point count are not validated.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Envelope2d {
    /// Envelope identifier.
    pub envelope2d_id: String,
    /// Polygon points, implicitly closed.
    pub vertices: Vec<Vertex>,
    /// Free-form explanation.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub description: Option<String>,
}
}

use crate::common::Vertex;

object_only_deserialize! {
/// Format-specific three-dimensional envelope.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Envelope3d {
    /// Envelope identifier.
    pub envelope3d_id: String,
    /// Data format, e.g. DXF; content is not validated.
    pub format: String,
    /// Arbitrary format-specific object; nested nulls and keys are preserved.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub data: Option<serde_json::Map<String, serde_json::Value>>,
    /// Download location; no URL validation or fetching is performed.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub url: Option<String>,
    /// Free-form explanation.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub description: Option<String>,
}
}
