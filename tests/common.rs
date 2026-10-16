#![cfg(feature = "serde")]
//! Shared-model checks against VDA5050/VDA5050 revision
//! 0b2ae43154bcd1ab2be4e2445df8b406dcad77b2, sections 7.1-7.5 and 7.8.
//! Object shape follows the object types in the pinned schemas; optional
//! does not imply nullable. Numeric bounds follow the cited specification.

use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use vda5050_v3_types::{
    Connection, InstantActions, Order, Responses, State, Visualization,
    common::{
        Action, ActionParameter, ControlPoint, Header, IntermediatePath, MobileRobotPosition,
        PlannedPath, Trajectory, Velocity, Waypoint,
    },
    order::{AllowedDeviationXy, Corridor, Edge, Node, NodePosition},
    responses::Response,
};

fn check_object<T: DeserializeOwned + Serialize>(valid: Value, optional: &[&str]) {
    let model: T = serde_json::from_str(&valid.to_string()).unwrap();
    assert_eq!(serde_json::to_value(model).unwrap(), valid);
    for invalid in [json!(null), json!([]), json!([1, 2]), json!(42)] {
        assert!(
            serde_json::from_value::<T>(invalid.clone()).is_err(),
            "{invalid}"
        );
        assert!(
            serde_json::from_str::<T>(&invalid.to_string()).is_err(),
            "{invalid}"
        );
    }
    for (field, original) in valid.as_object().unwrap() {
        let mut missing = valid.clone();
        missing.as_object_mut().unwrap().remove(field);
        assert_eq!(
            serde_json::from_value::<T>(missing).is_ok(),
            optional.contains(&field.as_str()),
            "{field}"
        );
        let mut null = valid.clone();
        null[field] = Value::Null;
        assert!(serde_json::from_value::<T>(null).is_err(), "{field}");
        let duplicate = format!("{{\"{field}\":{original},{}", &valid.to_string()[1..]);
        assert!(serde_json::from_str::<T>(&duplicate).is_err(), "{field}");
    }
}

#[test]
fn public_objects_have_identical_standalone_and_nested_shape_rules() {
    let full: Value = serde_json::from_str(include_str!("fixtures/visualization.json")).unwrap();
    check_object::<Visualization>(
        full.clone(),
        &[
            "plannedPath",
            "intermediatePath",
            "mobileRobotPosition",
            "velocity",
        ],
    );
    check_object::<Connection>(
        serde_json::from_str(include_str!("fixtures/connection.json")).unwrap(),
        &[],
    );
    let header = json!({
        "headerId": 1, "timestamp": "2017-04-15T11:40:03.123Z",
        "version": "3.0.0", "manufacturer": "Example", "serialNumber": "robot-1"
    });
    check_object::<Header>(header, &[]);
    check_object::<MobileRobotPosition>(
        full["mobileRobotPosition"].clone(),
        &["localizationScore", "deviationRange"],
    );
    check_object::<Velocity>(full["velocity"].clone(), &["vx", "vy", "omega"]);
    check_object::<PlannedPath>(full["plannedPath"].clone(), &["traversedNodes"]);
    check_object::<Trajectory>(
        full["plannedPath"]["trajectory"].clone(),
        &["degree", "knotVector"],
    );
    check_object::<ControlPoint>(
        full["plannedPath"]["trajectory"]["controlPoints"][0].clone(),
        &["weight"],
    );
    check_object::<IntermediatePath>(full["intermediatePath"].clone(), &[]);
    check_object::<Waypoint>(full["intermediatePath"]["polyline"][0].clone(), &["theta"]);
    let order: Value = serde_json::from_str(include_str!("fixtures/order.json")).unwrap();
    check_object::<Order>(order.clone(), &["orderDescription"]);
    check_object::<Node>(
        order["nodes"][0].clone(),
        &["nodeDescriptor", "nodePosition"],
    );
    check_object::<Edge>(
        order["edges"][0].clone(),
        &[
            "edgeDescriptor",
            "maximumSpeed",
            "maximumMobileRobotHeight",
            "minimumLoadHandlingDeviceHeight",
            "orientation",
            "orientationType",
            "direction",
            "reachOrientationBeforeEntering",
            "maximumRotationSpeed",
            "trajectory",
            "length",
            "corridor",
        ],
    );
    check_object::<NodePosition>(
        order["nodes"][0]["nodePosition"].clone(),
        &["theta", "allowedDeviationXY", "allowedDeviationTheta"],
    );
    check_object::<AllowedDeviationXy>(
        order["nodes"][0]["nodePosition"]["allowedDeviationXY"].clone(),
        &[],
    );
    check_object::<Corridor>(
        order["edges"][0]["corridor"].clone(),
        &[
            "corridorReferencePoint",
            "releaseRequired",
            "releaseLossBehavior",
        ],
    );
    check_object::<Action>(
        order["nodes"][0]["actions"][0].clone(),
        &["actionDescriptor", "actionParameters", "retriable"],
    );
    check_object::<ActionParameter>(
        order["nodes"][0]["actions"][0]["actionParameters"][0].clone(),
        &[],
    );
    check_object::<InstantActions>(
        serde_json::from_str(include_str!("fixtures/instant-actions.json")).unwrap(),
        &[],
    );
    let responses: Value = serde_json::from_str(include_str!("fixtures/responses.json")).unwrap();
    check_object::<Responses>(responses.clone(), &[]);
    check_object::<Response>(responses["responses"][0].clone(), &["leaseExpiry"]);
}

#[test]
fn shared_numeric_checks_report_the_wire_field() {
    let error = serde_json::from_value::<Velocity>(json!({"vx": "invalid"})).unwrap_err();
    assert!(error.to_string().contains("vx"));
    let velocity = Velocity {
        vx: None,
        vy: Some(f64::NAN),
        omega: None,
    };
    assert!(
        serde_json::to_string(&velocity)
            .unwrap_err()
            .to_string()
            .contains("vy")
    );
    let point = ControlPoint {
        x: 0.0,
        y: 0.0,
        weight: Some(0.0),
    };
    assert!(
        serde_json::to_string(&point)
            .unwrap_err()
            .to_string()
            .contains("weight")
    );
}

#[test]
fn state_objects_have_identical_standalone_and_nested_rules() {
    use vda5050_v3_types::state::{
        ActionState, BoundingBoxReference, EdgeRequest, EdgeState, Error, Info, Load,
        LoadDimensions, Map, NodePosition, NodeState, PowerSupply, Reference, SafetyState,
        Translation, ZoneRequest, ZoneSetState,
    };
    let full: Value = serde_json::from_str(include_str!("fixtures/state.json")).unwrap();
    check_object::<State>(
        full.clone(),
        &[
            "maps",
            "zoneSets",
            "plannedPath",
            "intermediatePath",
            "mobileRobotPosition",
            "velocity",
            "loads",
            "paused",
            "newBaseRequest",
            "zoneRequests",
            "edgeRequests",
            "distanceSinceLastNode",
            "zoneActionStates",
            "information",
        ],
    );
    check_object::<Map>(full["maps"][0].clone(), &["mapDescriptor"]);
    check_object::<ZoneSetState>(full["zoneSets"][0].clone(), &[]);
    check_object::<NodeState>(
        full["nodeStates"][0].clone(),
        &["nodeDescriptor", "nodePosition"],
    );
    check_object::<NodePosition>(full["nodeStates"][0]["nodePosition"].clone(), &["theta"]);
    check_object::<EdgeState>(
        full["edgeStates"][0].clone(),
        &["edgeDescriptor", "trajectory"],
    );
    check_object::<ZoneRequest>(full["zoneRequests"][0].clone(), &["trajectory"]);
    check_object::<EdgeRequest>(full["edgeRequests"][0].clone(), &[]);
    check_object::<ActionState>(
        full["actionStates"][0].clone(),
        &["actionType", "actionDescriptor", "actionResult"],
    );
    check_object::<Load>(
        full["loads"][0].clone(),
        &[
            "loadId",
            "loadType",
            "loadPosition",
            "boundingBoxReference",
            "loadDimensions",
            "weight",
        ],
    );
    check_object::<BoundingBoxReference>(
        full["loads"][0]["boundingBoxReference"].clone(),
        &["theta"],
    );
    check_object::<LoadDimensions>(full["loads"][0]["loadDimensions"].clone(), &["height"]);
    check_object::<PowerSupply>(
        full["powerSupply"].clone(),
        &["batteryVoltage", "batteryCurrent", "batteryHealth", "range"],
    );
    check_object::<Error>(
        full["errors"][0].clone(),
        &[
            "errorReferences",
            "errorDescription",
            "errorDescriptionTranslations",
            "errorHint",
            "errorHintTranslations",
        ],
    );
    check_object::<Reference>(full["errors"][0]["errorReferences"][0].clone(), &[]);
    check_object::<Translation>(
        full["errors"][0]["errorDescriptionTranslations"][0].clone(),
        &[],
    );
    check_object::<Info>(
        full["information"][0].clone(),
        &["infoReferences", "infoDescriptor"],
    );
    check_object::<SafetyState>(full["safetyState"].clone(), &[]);
}

#[test]
fn float64_text_and_byte_round_trips_preserve_bits() {
    // Section 7.1.4 / 7.8: finite float64 velocity, including subnormal values.
    for value in [
        8.971135692166839e-150,
        f64::MIN,
        f64::MAX,
        f64::MIN_POSITIVE,
        f64::from_bits(1),
        -f64::from_bits(1),
        -0.0,
        0.1,
    ] {
        let model = Velocity {
            vx: Some(value),
            vy: None,
            omega: None,
        };
        let text = serde_json::to_string(&model).unwrap();
        let bytes = serde_json::to_vec(&model).unwrap();
        for decoded in [
            serde_json::from_str::<Velocity>(&text).unwrap(),
            serde_json::from_slice::<Velocity>(&bytes).unwrap(),
        ] {
            assert_eq!(decoded.vx.unwrap().to_bits(), value.to_bits());
        }
    }
}
