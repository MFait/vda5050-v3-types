object_only_deserialize! {
/// Two-dimensional polygon vertex in the containing model's coordinate system.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Vertex {
    /// Finite X-coordinate in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::x"))]
    pub x: f64,
    /// Finite Y-coordinate in meters.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_helpers::y"))]
    pub y: f64,
}
}
