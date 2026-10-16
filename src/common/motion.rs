object_only_deserialize! {
/// Robot position in project-specific map coordinates (section 7.8).
///
/// Numeric ranges are checked on encoding/decoding, not on direct construction.
/// `localized == false` does not trigger robot behavior or error reporting.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct MobileRobotPosition {
    /// X-coordinate in meters; finite.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::x"))]
    pub x: f64,
    /// Y-coordinate in meters; finite.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::y"))]
    pub y: f64,
    /// Absolute orientation in radians, in `[-pi, pi]`.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::theta"))]
    pub theta: f64,
    /// Identifier of the map containing this position.
    pub map_id: String,
    /// Whether the position and orientation can be trusted.
    pub localized: bool,
    /// Localization confidence in `[0, 1]`; for logging/visualization only.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::localization_score::optional"
        )
    )]
    pub localization_score: Option<f64>,
    /// Nonnegative deviation range in meters; for logging/visualization only.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::deviation_range::optional"
        )
    )]
    pub deviation_range: Option<f64>,
}
}

object_only_deserialize! {
/// Velocity in robot coordinates (section 7.8).
///
/// Each component is independently optional. An empty velocity object is valid.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Velocity {
    /// Finite X-direction velocity in meters per second.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::vx::optional"
        )
    )]
    pub vx: Option<f64>,
    /// Finite Y-direction velocity in meters per second.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::vy::optional"
        )
    )]
    pub vy: Option<f64>,
    /// Finite turning speed around the Z-axis in radians per second.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::omega::optional"
        )
    )]
    pub omega: Option<f64>,
}
}
