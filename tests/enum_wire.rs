#![cfg(feature = "serde")]

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::fmt::Debug;
use vda5050_v3_types::{common, connection, factsheet, order, responses, state, zone_set};

#[derive(Debug, PartialEq, Deserialize)]
struct Nested<T> {
    value: T,
}

fn check<T: Debug + PartialEq + Serialize + DeserializeOwned>(cases: &[(T, &str)]) {
    for (variant, name) in cases {
        let name = *name;
        let string = Value::String(name.into());
        assert_eq!(serde_json::to_value(variant).unwrap(), string);
        let encoded = serde_json::to_string(variant).unwrap();
        assert_eq!(&serde_json::from_str::<T>(&encoded).unwrap(), variant);
        assert_eq!(
            &serde_json::from_slice::<T>(encoded.as_bytes()).unwrap(),
            variant
        );
        assert_eq!(
            &serde_json::from_value::<T>(string.clone()).unwrap(),
            variant
        );
        let nested: Nested<T> = serde_json::from_value(json!({"value": string})).unwrap();
        assert_eq!(&nested.value, variant);

        for invalid in [
            Value::Null,
            json!(false),
            json!(0),
            json!(1.0),
            json!([]),
            json!([name]),
            json!({name: null}),
            json!({name: []}),
            json!({name: {}}),
            json!("UNKNOWN_ENUM_VALUE"),
        ] {
            let encoded = serde_json::to_string(&invalid).unwrap();
            assert!(
                serde_json::from_str::<T>(&encoded).is_err(),
                "{name}: accepted {encoded}"
            );
            assert!(serde_json::from_slice::<T>(encoded.as_bytes()).is_err());
            assert!(serde_json::from_value::<T>(invalid.clone()).is_err());
            assert!(serde_json::from_value::<Nested<T>>(json!({"value": invalid})).is_err());
        }
    }
}

macro_rules! cases {
    ($test:ident, $type:path, $($variant:ident => $name:literal),+ $(,)?) => {
        #[test]
        fn $test() {
            use $type as E;
            check(&[$((E::$variant, $name)),+]);
        }
    };
}

cases!(blocking, common::BlockingType,
    None => "NONE", Single => "SINGLE", Soft => "SOFT", Hard => "HARD");
cases!(connection, connection::ConnectionState,
    Online => "ONLINE", Offline => "OFFLINE", Hibernating => "HIBERNATING",
    ConnectionBroken => "CONNECTION_BROKEN");
cases!(orientation, order::OrientationType, Global => "GLOBAL", Tangential => "TANGENTIAL");
cases!(corridor_reference, order::CorridorReferencePoint,
    KinematicCenter => "KINEMATIC_CENTER", Contour => "CONTOUR");
cases!(corridor_release_loss, order::ReleaseLossBehavior, Stop => "STOP", Return => "RETURN");
cases!(grant, responses::GrantType,
    Granted => "GRANTED", Queued => "QUEUED", Revoked => "REVOKED", Rejected => "REJECTED");
cases!(action_status, state::ActionStatus,
    Waiting => "WAITING", Initializing => "INITIALIZING", Running => "RUNNING",
    Paused => "PAUSED", Retriable => "RETRIABLE", Finished => "FINISHED", Failed => "FAILED");
cases!(operating_mode, state::OperatingMode,
    Startup => "STARTUP", Automatic => "AUTOMATIC", Semiautomatic => "SEMIAUTOMATIC",
    Intervened => "INTERVENED", Manual => "MANUAL", Service => "SERVICE", TeachIn => "TEACH_IN");
cases!(emergency_stop, state::EmergencyStop, Manual => "MANUAL", Remote => "REMOTE", None => "NONE");
cases!(error_level, state::ErrorLevel,
    Warning => "WARNING", Urgent => "URGENT", Critical => "CRITICAL", Fatal => "FATAL");
cases!(info_level, state::InfoLevel, Debug => "DEBUG", Info => "INFO");
cases!(map_status, state::MapStatus, Enabled => "ENABLED", Disabled => "DISABLED");
cases!(zone_set_status, state::ZoneSetStatus, Enabled => "ENABLED", Disabled => "DISABLED");
cases!(zone_request, state::ZoneRequestType, Access => "ACCESS", Replanning => "REPLANNING");
cases!(edge_request, state::EdgeRequestType, Corridor => "CORRIDOR");
cases!(request_status, state::RequestStatus,
    Requested => "REQUESTED", Granted => "GRANTED", Revoked => "REVOKED", Expired => "EXPIRED");
cases!(parameter_support, factsheet::ParameterSupport, Supported => "SUPPORTED", Required => "REQUIRED");
cases!(action_scope, factsheet::ActionScope,
    Instant => "INSTANT", Node => "NODE", Edge => "EDGE", Zone => "ZONE");
cases!(value_data_type, factsheet::ValueDataType,
    Bool => "BOOL", Number => "NUMBER", Integer => "INTEGER", String => "STRING",
    Object => "OBJECT", Array => "ARRAY");
cases!(zone_type, factsheet::ZoneType,
    Blocked => "BLOCKED", LineGuided => "LINE_GUIDED", Release => "RELEASE",
    CoordinatedReplanning => "COORDINATED_REPLANNING", SpeedLimit => "SPEED_LIMIT",
    Action => "ACTION", Priority => "PRIORITY", Penalty => "PENALTY",
    Directed => "DIRECTED", Bidirected => "BIDIRECTED");
cases!(zone_release_loss, zone_set::ReleaseLossBehavior,
    Stop => "STOP", Continue => "CONTINUE", Evacuate => "EVACUATE");
cases!(directed_limitation, zone_set::DirectedLimitation,
    Soft => "SOFT", Restricted => "RESTRICTED", Strict => "STRICT");
cases!(bidirected_limitation, zone_set::BidirectedLimitation,
    Soft => "SOFT", Restricted => "RESTRICTED");

#[cfg(feature = "topic")]
cases!(topic_kind, vda5050_v3_types::topic::TopicKind,
    Order => "order", InstantActions => "instantActions", State => "state",
    Visualization => "visualization", Connection => "connection", Factsheet => "factsheet",
    ZoneSet => "zoneSet", Responses => "responses");
