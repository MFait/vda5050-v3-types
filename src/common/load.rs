object_only_deserialize! {
/// Center of the bounding box bottom surface in robot coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BoundingBoxReference {
    /// Finite X-coordinate.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::x"))]
    pub x: f64,
    /// Finite Y-coordinate.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::y"))]
    pub y: f64,
    /// Finite Z-coordinate.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::z"))]
    pub z: f64,
    /// Finite orientation; neither source asserts an angle range here.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::bounding_box_theta::optional"))]
    pub theta: Option<f64>,
}
}

object_only_deserialize! {
/// Load bounding-box dimensions in meters.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct LoadDimensions {
    /// Finite nonnegative length along the robot X-axis.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::load_length"))]
    pub length: f64,
    /// Finite nonnegative width along the robot Y-axis.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::load_width"))]
    pub width: f64,
    /// Finite nonnegative height, if known.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", with = "crate::serde_helpers::load_height::optional"))]
    pub height: Option<f64>,
}
}
