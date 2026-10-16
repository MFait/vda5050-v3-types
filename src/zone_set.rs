//! Zone definitions for a map (VDA 5050 v3.0.0, sections 6.4 and 7.6).

#[cfg(feature = "serde")]
mod wire;

pub use crate::common::Vertex;
use crate::common::{ActionParameter, BlockingType, Header};

object_only_deserialize! {
/// Zone-set transfer body. This does not enable or store zones on a robot.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ZoneSet {
    /// Shared header, flattened on the wire.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub header: Header,
    /// Map-specific zone definitions, nested under `zoneSet`.
    pub zone_set: ZoneSetDefinition,
}
}

object_only_deserialize! {
/// A map's zone set; identifier uniqueness and immutability are not validated.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ZoneSetDefinition {
    /// Globally unique map identifier; no map version is carried.
    pub map_id: String,
    /// Globally unique zone-set identifier.
    pub zone_set_id: String,
    /// Human-readable description, not for logical use.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub zone_set_descriptor: Option<String>,
    /// Zones; an empty set is representable.
    pub zones: Vec<Zone>,
}
}

object_only_deserialize! {
/// Polygon and type-specific rules for a zone.
///
/// Serde checks at least three vertices, but not counterclockwise order,
/// intersections, map bounds or interactions with other zones.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Zone {
    /// Identifier unique within its zone set; uniqueness is not checked.
    pub zone_id: String,
    /// Human-readable description, not for logical use.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub zone_descriptor: Option<String>,
    /// Implicitly closed polygon in project coordinates, counterclockwise.
    #[cfg_attr(feature = "serde", serde(with = "wire::vertices"))]
    pub vertices: Vec<Vertex>,
    /// Wire discriminator `zoneType` and its required parameters.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub kind: ZoneKind,
}
}

/// Zone discriminator and parameters; fields serialize beside `zoneType`.
///
/// Parameters belonging to another variant are unknown fields and are ignored
/// when decoding, just like other unknown typed-object fields.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(tag = "zoneType", rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum ZoneKind {
    /// Entry is forbidden.
    Blocked,
    /// Follow predefined edge trajectories.
    LineGuided,
    /// Access requires a grant from fleet control.
    Release {
        /// Required by section 7.6 and the schema, despite section 6.4's default.
        #[cfg_attr(feature = "serde", serde(rename = "releaseLossBehavior"))]
        release_loss_behavior: ReleaseLossBehavior,
    },
    /// Replanning requires permission.
    CoordinatedReplanning,
    /// Maximum speed in meters per second.
    SpeedLimit {
        /// Finite speed; neither source asserts a nonnegative bound.
        #[cfg_attr(
            feature = "serde",
            serde(rename = "maximumSpeed", with = "wire::maximum_speed")
        )]
        maximum_speed: f64,
    },
    /// Actions for entry, traversal and exit; empty arrays mean no actions.
    Action {
        /// Actions triggered on entry.
        #[cfg_attr(feature = "serde", serde(rename = "entryActions"))]
        entry_actions: Vec<ZoneAction>,
        /// Actions executed during traversal.
        #[cfg_attr(feature = "serde", serde(rename = "duringActions"))]
        during_actions: Vec<ZoneAction>,
        /// Actions triggered on exit.
        #[cfg_attr(feature = "serde", serde(rename = "exitActions"))]
        exit_actions: Vec<ZoneAction>,
    },
    /// Prefer this region during planning.
    Priority {
        /// Finite relative preference in [0, 1].
        #[cfg_attr(
            feature = "serde",
            serde(rename = "priorityFactor", with = "wire::priority_factor")
        )]
        priority_factor: f64,
    },
    /// Avoid this region during planning.
    Penalty {
        /// Finite relative penalty in [0, 1].
        #[cfg_attr(
            feature = "serde",
            serde(rename = "penaltyFactor", with = "wire::penalty_factor")
        )]
        penalty_factor: f64,
    },
    /// One preferred direction.
    Directed {
        /// Finite angle in radians; no explicit angle bound is asserted.
        #[cfg_attr(feature = "serde", serde(with = "wire::direction"))]
        direction: f64,
        /// Permitted deviation.
        #[cfg_attr(feature = "serde", serde(rename = "directedLimitation"))]
        directed_limitation: DirectedLimitation,
    },
    /// A direction and its opposite.
    Bidirected {
        /// Finite angle in radians; no explicit angle bound is asserted.
        #[cfg_attr(feature = "serde", serde(with = "wire::direction"))]
        direction: f64,
        /// Permitted deviation.
        #[cfg_attr(feature = "serde", serde(rename = "bidirectedLimitation"))]
        bidirected_limitation: BidirectedLimitation,
    },
}

string_enum! {
/// Response to revoked or expired access while already inside a release zone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum ReleaseLossBehavior {
    /// Stop and report RELEASE_LOST.
    Stop,
    /// Continue the path.
    Continue,
    /// Execute the robot's evacuation behavior.
    Evacuate,
}
}

string_enum! {
/// Permitted deviation from a single preferred direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum DirectedLimitation {
    /// Deviations should be avoided.
    Soft,
    /// Deviations are allowed, but not opposite travel.
    Restricted,
    /// Maintain the direction within robot precision.
    Strict,
}
}

string_enum! {
/// Permitted deviation from two opposite directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum BidirectedLimitation {
    /// Deviations should be avoided.
    Soft,
    /// Other directions are allowed only for obstacle avoidance.
    Restricted,
}
}

object_only_deserialize! {
/// Action template whose action ID is generated by the robot, not sent here.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ZoneAction {
    /// Extensible action function name.
    pub action_type: String,
    /// Human-readable information.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub action_descriptor: Option<String>,
    /// Execution blocking behavior.
    pub blocking_type: BlockingType,
    /// Optional shared checked parameters; absence differs from an empty list.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub action_parameters: Option<Vec<ActionParameter>>,
    /// Optional retry policy; omission preserves the protocol default false.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_helpers::non_null_option"))]
    pub retriable: Option<bool>,
}
}

#[cfg(feature = "json")]
impl_json_helpers!(ZoneSet);
