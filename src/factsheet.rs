//! Robot capabilities (VDA 5050 v3.0.0, section 7.10).

mod configuration;
mod geometry;
mod limits;
mod loads;
mod protocol;
mod specification;
#[cfg(feature = "serde")]
mod wire;

pub use crate::common::Vertex;
pub use configuration::{BatteryCharging, MobileRobotConfiguration, Network, VersionInfo};
pub use geometry::{Envelope2d, Envelope3d, MobileRobotGeometry, WheelDefinition, WheelPosition};
pub use limits::{MaximumArrayLengths, MaximumStringLengths, ProtocolLimits, Timing};
pub use loads::{LoadSet, LoadSpecification};
pub use protocol::{
    ActionParameter, ActionScope, MobileRobotAction, OptionalParameter, ParameterSupport,
    ProtocolFeatures, ValueDataType,
};
pub use specification::{PhysicalParameters, TypeSpecification, ZoneType};

use crate::common::Header;

object_only_deserialize! {
/// Robot capabilities and implementation limits.
///
/// Parsing checks wire representations and field bounds, not capability
/// consistency, minimum/maximum relationships, geometry, supported parameter
/// names or compliance of other messages with the advertised limits.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Factsheet {
    /// Shared header, flattened on the wire.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub header: Header,
    /// Robot class and capabilities.
    pub type_specification: TypeSpecification,
    /// Basic physical properties.
    pub physical_parameters: PhysicalParameters,
    /// Identifier, collection and timing limits.
    pub protocol_limits: ProtocolLimits,
    /// Supported optional parameters and actions.
    pub protocol_features: ProtocolFeatures,
    /// Wheels and envelopes.
    pub mobile_robot_geometry: MobileRobotGeometry,
    /// Supported load handling.
    pub load_specification: LoadSpecification,
    /// Optional hardware/software and network configuration.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub mobile_robot_configuration: Option<MobileRobotConfiguration>,
}
}

#[cfg(feature = "json")]
impl_json_helpers!(Factsheet);
