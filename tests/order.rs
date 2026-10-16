//! VDA5050/VDA5050@0b2ae43154bcd1ab2be4e2445df8b406dcad77b2:
//! https://github.com/VDA5050/VDA5050/blob/0b2ae43154bcd1ab2be4e2445df8b406dcad77b2/VDA5050_EN.md
//! Sections 6.1 (graphs/base/horizon), 7.1.4 (finite numbers), 7.2 and 7.3.
//! Vendored order.schema is unchanged; see the rotation-speed discrepancy test.

use chrono::{TimeZone, Utc};
use serde_json::Value;
use vda5050_v3_types::{Order, common::Header, order::Node};

mod support;

const FULL: &str = include_str!("fixtures/order.json");
const MINIMAL: &str = include_str!("fixtures/order-minimal.json");

fn full() -> Value {
    serde_json::from_str(FULL).unwrap()
}

#[test]
fn order_is_usable_without_optional_features() {
    let order = Order {
        header: Header {
            header_id: 0,
            timestamp: Utc.with_ymd_and_hms(2026, 10, 16, 12, 0, 0).unwrap(),
            version: "3.0.0".into(),
            manufacturer: "Example".into(),
            serial_number: "robot-1".into(),
        },
        order_id: "order-1".into(),
        order_update_id: 0,
        order_description: None,
        nodes: vec![Node {
            node_id: "station-a".into(),
            sequence_id: 0,
            node_descriptor: None,
            released: true,
            node_position: None,
            actions: vec![],
        }],
        edges: vec![],
    };
    assert_eq!(order.clone(), order);
}

#[test]
fn fixtures_satisfy_pinned_schema() {
    for value in [full(), serde_json::from_str(MINIMAL).unwrap()] {
        support::schema("order").validate(&value).unwrap();
        // The spec name is otherwise an unconstrained additional property in
        // this schema. Project it onto the misspelled key to check its type too.
        let mut projected = value;
        for edge in projected["edges"].as_array_mut().unwrap() {
            if let Some(value) = edge.as_object_mut().unwrap().remove("maximumRotationSpeed") {
                edge["maxRotationSpeed"] = value;
            }
        }
        support::schema("order").validate(&projected).unwrap();
    }
}

#[cfg(feature = "serde")]
mod wire {
    use super::*;
    use serde_json::json;
    use std::f64::consts::PI;
    use vda5050_v3_types::common::BlockingType;

    // The schema asserts this decimal, not an exact pi/2 range.
    #[allow(clippy::approx_constant)]
    const ELLIPSE_ROTATION_LIMIT: f64 = 1.570796327;

    fn round_trip(value: Value) -> Order {
        let decoded: Order = serde_json::from_str(&value.to_string()).unwrap();
        let encoded: Value =
            serde_json::from_slice(&serde_json::to_vec(&decoded).unwrap()).unwrap();
        assert_eq!(encoded, value);
        support::schema("order").validate(&encoded).unwrap();
        decoded
    }

    #[test]
    fn full_and_minimal_round_trip_with_exact_keys() {
        let order = round_trip(full());
        assert_eq!(order.edges[0].maximum_rotation_speed, Some(0.5));
        assert_eq!(order.nodes[0].actions[0].retriable, Some(true));
        let encoded = serde_json::to_value(order).unwrap();
        assert!(encoded.get("header").is_none());
        assert!(encoded["edges"][0].get("maxRotationSpeed").is_none());
        assert!(
            encoded["nodes"][0]["nodePosition"]
                .get("allowedDeviationXy")
                .is_none()
        );
        assert_eq!(
            round_trip(serde_json::from_str(MINIMAL).unwrap())
                .nodes
                .len(),
            1
        );
    }

    #[test]
    fn required_fields_reject_missing_null_and_wrong_types() {
        support::assert_required::<Order>(
            &full(),
            support::schema("order"),
            &[
                "/headerId",
                "/timestamp",
                "/version",
                "/manufacturer",
                "/serialNumber",
                "/orderId",
                "/orderUpdateId",
                "/nodes",
                "/edges",
                "/nodes/0/nodeId",
                "/nodes/0/sequenceId",
                "/nodes/0/released",
                "/nodes/0/actions",
                "/nodes/0/nodePosition/x",
                "/nodes/0/nodePosition/y",
                "/nodes/0/nodePosition/mapId",
                "/nodes/0/nodePosition/allowedDeviationXY/a",
                "/nodes/0/nodePosition/allowedDeviationXY/b",
                "/nodes/0/nodePosition/allowedDeviationXY/theta",
                "/nodes/0/actions/0/actionType",
                "/nodes/0/actions/0/actionId",
                "/nodes/0/actions/0/blockingType",
                "/nodes/0/actions/0/actionParameters/0/key",
                "/edges/0/edgeId",
                "/edges/0/sequenceId",
                "/edges/0/released",
                "/edges/0/actions",
                "/edges/0/trajectory/controlPoints",
                "/edges/0/trajectory/controlPoints/0/x",
                "/edges/0/trajectory/controlPoints/0/y",
                "/edges/0/corridor/leftWidth",
                "/edges/0/corridor/rightWidth",
                "/edges/0/actions/0/actionType",
                "/edges/0/actions/0/actionId",
                "/edges/0/actions/0/blockingType",
            ],
        );
        for invalid in [None, Some(Value::Null)] {
            let mut value = full();
            let pointer = "/nodes/0/actions/0/actionParameters/0/value";
            if let Some(invalid) = invalid {
                *value.pointer_mut(pointer).unwrap() = invalid;
            } else {
                support::remove(&mut value, pointer);
            }
            assert!(!support::schema("order").is_valid(&value));
            assert!(serde_json::from_value::<Order>(value).is_err());
        }
    }

    #[test]
    fn optional_fields_preserve_absence_and_reject_null() {
        support::assert_optional::<Order>(
            &full(),
            support::schema("order"),
            &[
                "/orderDescription",
                "/nodes/0/nodeDescriptor",
                "/nodes/0/nodePosition",
                "/nodes/0/nodePosition/theta",
                "/nodes/0/nodePosition/allowedDeviationXY",
                "/nodes/0/nodePosition/allowedDeviationTheta",
                "/nodes/0/actions/0/actionDescriptor",
                "/nodes/0/actions/0/actionParameters",
                "/nodes/0/actions/0/retriable",
                "/edges/0/edgeDescriptor",
                "/edges/0/maximumSpeed",
                "/edges/0/maximumMobileRobotHeight",
                "/edges/0/minimumLoadHandlingDeviceHeight",
                "/edges/0/orientation",
                "/edges/0/orientationType",
                "/edges/0/direction",
                "/edges/0/reachOrientationBeforeEntering",
                "/edges/0/trajectory",
                "/edges/0/trajectory/degree",
                "/edges/0/trajectory/knotVector",
                "/edges/0/trajectory/controlPoints/0/weight",
                "/edges/0/length",
                "/edges/0/corridor",
                "/edges/0/corridor/corridorReferencePoint",
                "/edges/0/corridor/releaseRequired",
                "/edges/0/corridor/releaseLossBehavior",
            ],
        );
        // §7.3 name differs from the schema; test it independently, not via
        // schema additional-property permissiveness.
        let mut missing = full();
        support::remove(&mut missing, "/edges/0/maximumRotationSpeed");
        assert!(
            round_trip(missing).edges[0]
                .maximum_rotation_speed
                .is_none()
        );
        for invalid in [Value::Null, json!("fast"), json!({})] {
            let mut value = full();
            value["edges"][0]["maximumRotationSpeed"] = invalid;
            assert!(serde_json::from_value::<Order>(value).is_err());
        }
    }

    #[test]
    fn rotation_speed_uses_spec_name_without_aliasing_schema_typo() {
        let mut value = full();
        value["edges"][0]["maxRotationSpeed"] = json!(99.0);
        let decoded: Order = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(decoded.edges[0].maximum_rotation_speed, Some(0.5));
        support::remove(&mut value, "/edges/0/maximumRotationSpeed");
        let decoded: Order = serde_json::from_value(value).unwrap();
        assert!(decoded.edges[0].maximum_rotation_speed.is_none());
        assert!(
            serde_json::to_value(decoded).unwrap()["edges"][0]
                .get("maxRotationSpeed")
                .is_none()
        );
        // The pinned schema does not validate the specification's property.
        let mut wrong = full();
        wrong["edges"][0]["maximumRotationSpeed"] = json!("invalid");
        assert!(support::schema("order").is_valid(&wrong));
        assert!(serde_json::from_value::<Order>(wrong).is_err());
    }

    #[test]
    fn uint32_fields_enforce_boundaries() {
        for pointer in [
            "/headerId",
            "/orderUpdateId",
            "/nodes/0/sequenceId",
            "/edges/0/sequenceId",
            "/edges/0/trajectory/degree",
        ] {
            for valid in [1, u32::MAX] {
                let mut value = full();
                *value.pointer_mut(pointer).unwrap() = json!(valid);
                round_trip(value);
            }
            let mut zero = full();
            *zero.pointer_mut(pointer).unwrap() = json!(0);
            if pointer.ends_with("/degree") {
                assert!(serde_json::from_value::<Order>(zero).is_err());
            } else {
                round_trip(zero);
            }
            for invalid in [
                json!(-1),
                json!(u64::from(u32::MAX) + 1),
                json!(0.5),
                json!(1.0),
            ] {
                let mut value = full();
                *value.pointer_mut(pointer).unwrap() = invalid;
                assert!(serde_json::from_value::<Order>(value).is_err(), "{pointer}");
            }
        }
    }

    // References to all scalar floats in the complete fixture allow the same
    // range cases to exercise direct-construction serialization as decoding.
    fn floats(order: &mut Order) -> Vec<(&'static str, &mut f64)> {
        let position = order.nodes[0].node_position.as_mut().unwrap();
        let ellipse = position.allowed_deviation_xy.as_mut().unwrap();
        let edge = &mut order.edges[0];
        let corridor = edge.corridor.as_mut().unwrap();
        let trajectory = edge.trajectory.as_mut().unwrap();
        let point = &mut trajectory.control_points[0];
        vec![
            ("/nodes/0/nodePosition/x", &mut position.x),
            ("/nodes/0/nodePosition/y", &mut position.y),
            (
                "/nodes/0/nodePosition/theta",
                position.theta.as_mut().unwrap(),
            ),
            (
                "/nodes/0/nodePosition/allowedDeviationTheta",
                position.allowed_deviation_theta.as_mut().unwrap(),
            ),
            ("/nodes/0/nodePosition/allowedDeviationXY/a", &mut ellipse.a),
            ("/nodes/0/nodePosition/allowedDeviationXY/b", &mut ellipse.b),
            (
                "/nodes/0/nodePosition/allowedDeviationXY/theta",
                &mut ellipse.theta,
            ),
            (
                "/edges/0/maximumSpeed",
                edge.maximum_speed.as_mut().unwrap(),
            ),
            (
                "/edges/0/maximumMobileRobotHeight",
                edge.maximum_mobile_robot_height.as_mut().unwrap(),
            ),
            (
                "/edges/0/minimumLoadHandlingDeviceHeight",
                edge.minimum_load_handling_device_height.as_mut().unwrap(),
            ),
            ("/edges/0/orientation", edge.orientation.as_mut().unwrap()),
            (
                "/edges/0/maximumRotationSpeed",
                edge.maximum_rotation_speed.as_mut().unwrap(),
            ),
            ("/edges/0/length", edge.length.as_mut().unwrap()),
            ("/edges/0/corridor/leftWidth", &mut corridor.left_width),
            ("/edges/0/corridor/rightWidth", &mut corridor.right_width),
            ("/edges/0/trajectory/controlPoints/0/x", &mut point.x),
            ("/edges/0/trajectory/controlPoints/0/y", &mut point.y),
            (
                "/edges/0/trajectory/controlPoints/0/weight",
                point.weight.as_mut().unwrap(),
            ),
            (
                "/edges/0/trajectory/knotVector/0",
                &mut trajectory.knot_vector.as_mut().unwrap()[0],
            ),
        ]
    }

    #[test]
    fn float_bounds_are_checked_in_both_directions() {
        let mut template: Order = serde_json::from_str(FULL).unwrap();
        let pointers: Vec<_> = floats(&mut template)
            .iter()
            .map(|(pointer, _)| *pointer)
            .collect();
        for pointer in pointers {
            let (min, max) = match pointer {
                "/nodes/0/nodePosition/theta" | "/edges/0/orientation" => (-PI, PI),
                "/nodes/0/nodePosition/allowedDeviationTheta" => (0.0, PI),
                "/nodes/0/nodePosition/allowedDeviationXY/theta" => {
                    (-ELLIPSE_ROTATION_LIMIT, ELLIPSE_ROTATION_LIMIT)
                }
                "/nodes/0/nodePosition/allowedDeviationXY/a"
                | "/nodes/0/nodePosition/allowedDeviationXY/b"
                | "/edges/0/corridor/leftWidth"
                | "/edges/0/corridor/rightWidth" => (0.0, f64::MAX),
                "/edges/0/trajectory/controlPoints/0/weight" => (f64::from_bits(1), f64::MAX),
                "/edges/0/trajectory/knotVector/0" => (0.0, 1.0),
                _ => (f64::MIN, f64::MAX),
            };
            for valid in [min, max] {
                let mut value = full();
                *value.pointer_mut(pointer).unwrap() = json!(valid);
                round_trip(value);
                let mut order: Order = serde_json::from_str(FULL).unwrap();
                *floats(&mut order)
                    .into_iter()
                    .find(|(p, _)| *p == pointer)
                    .unwrap()
                    .1 = valid;
                let encoded = serde_json::to_string(&order).unwrap();
                assert_eq!(
                    serde_json::from_str::<Order>(&encoded).unwrap(),
                    order,
                    "{pointer}"
                );
            }
            for invalid in [
                min - 1e-9,
                max + 1e-9,
                f64::NAN,
                f64::INFINITY,
                f64::NEG_INFINITY,
            ] {
                if invalid.is_finite() && (min..=max).contains(&invalid) {
                    continue;
                }
                if invalid.is_finite() {
                    let mut value = full();
                    *value.pointer_mut(pointer).unwrap() = json!(invalid);
                    assert!(
                        serde_json::from_value::<Order>(value).is_err(),
                        "{pointer}: {invalid}"
                    );
                }
                let mut order: Order = serde_json::from_str(FULL).unwrap();
                *floats(&mut order)
                    .into_iter()
                    .find(|(p, _)| *p == pointer)
                    .unwrap()
                    .1 = invalid;
                let error = serde_json::to_string(&order).unwrap_err().to_string();
                let field = pointer.rsplit('/').next().unwrap();
                assert!(
                    error.contains(if field == "0" { "knots" } else { field }),
                    "{pointer}: {error}"
                );
            }
        }
        // Strictly positive weight rejects exactly zero as well.
        let mut value = full();
        value["edges"][0]["trajectory"]["controlPoints"][0]["weight"] = json!(0);
        assert!(serde_json::from_value::<Order>(value).is_err());
        let mut order: Order = serde_json::from_str(FULL).unwrap();
        order.edges[0].trajectory.as_mut().unwrap().control_points[0].weight = Some(0.0);
        assert!(serde_json::to_string(&order).is_err());
        order.edges[0].trajectory.as_mut().unwrap().control_points[0].weight = None;
        order.edges[0].trajectory.as_mut().unwrap().degree = Some(0);
        assert!(serde_json::to_string(&order).is_err());
    }

    #[test]
    fn rounded_schema_angles_do_not_extend_specification_bounds() {
        for pointer in [
            "/nodes/0/nodePosition/theta",
            "/nodes/0/nodePosition/allowedDeviationTheta",
            "/edges/0/orientation",
        ] {
            let mut value = full();
            *value.pointer_mut(pointer).unwrap() = json!(PI + 1e-13);
            assert!(support::schema("order").is_valid(&value), "{pointer}");
            assert!(serde_json::from_value::<Order>(value).is_err(), "{pointer}");
            let mut order: Order = serde_json::from_str(FULL).unwrap();
            *floats(&mut order)
                .into_iter()
                .find(|(p, _)| *p == pointer)
                .unwrap()
                .1 = PI + 1e-13;
            assert!(serde_json::to_string(&order).is_err(), "{pointer}");
        }
        // Ellipse rotation, conversely, has only an explicit schema bound:
        // preserve its decimal endpoint, including its tiny extension past pi/2.
        let mut value = full();
        value["nodes"][0]["nodePosition"]["allowedDeviationXY"]["theta"] =
            json!(ELLIPSE_ROTATION_LIMIT);
        round_trip(value);
    }

    #[test]
    fn enums_preserve_every_wire_value_and_reject_unknowns() {
        for (pointer, values) in [
            ("/edges/0/orientationType", &["GLOBAL", "TANGENTIAL"][..]),
            (
                "/edges/0/corridor/corridorReferencePoint",
                &["KINEMATIC_CENTER", "CONTOUR"][..],
            ),
            (
                "/edges/0/corridor/releaseLossBehavior",
                &["STOP", "RETURN"][..],
            ),
            (
                "/nodes/0/actions/0/blockingType",
                &["NONE", "SINGLE", "SOFT", "HARD"][..],
            ),
            (
                "/edges/0/actions/0/blockingType",
                &["NONE", "SINGLE", "SOFT", "HARD"][..],
            ),
        ] {
            for value in values {
                let mut fixture = full();
                *fixture.pointer_mut(pointer).unwrap() = json!(value);
                round_trip(fixture);
            }
            for invalid in ["UNKNOWN", "", &values[0].to_lowercase()] {
                let mut fixture = full();
                *fixture.pointer_mut(pointer).unwrap() = json!(invalid);
                assert!(!support::schema("order").is_valid(&fixture));
                assert!(serde_json::from_value::<Order>(fixture).is_err());
            }
        }
        let mut fixture = full();
        fixture["nodes"][0]["actions"][0]["blockingType"] = json!("HARD");
        assert_eq!(
            round_trip(fixture).nodes[0].actions[0].blocking_type,
            BlockingType::Hard
        );
    }

    #[test]
    fn malformed_nested_objects_and_array_elements_are_rejected() {
        for pointer in [
            "/nodes/0",
            "/edges/0",
            "/nodes/0/nodePosition",
            "/nodes/0/nodePosition/allowedDeviationXY",
            "/nodes/0/actions/0",
            "/nodes/0/actions/0/actionParameters/0",
            "/edges/0/corridor",
            "/edges/0/trajectory",
            "/edges/0/trajectory/controlPoints/0",
        ] {
            for invalid in [json!([]), json!([1]), Value::Null, json!(false)] {
                let mut value = full();
                *value.pointer_mut(pointer).unwrap() = invalid;
                assert!(serde_json::from_value::<Order>(value).is_err(), "{pointer}");
            }
        }
        let text = FULL.replacen(
            "\"orderId\": \"order-1\"",
            "\"orderId\": \"a\", \"orderId\": \"b\"",
            1,
        );
        assert!(serde_json::from_str::<Order>(&text).is_err());
    }

    #[test]
    fn defaults_and_false_remain_distinct_and_unknown_fields_are_ignored() {
        let mut value = full();
        for pointer in [
            "/nodes/0/actions/0/retriable",
            "/edges/0/reachOrientationBeforeEntering",
            "/edges/0/corridor/releaseRequired",
        ] {
            *value.pointer_mut(pointer).unwrap() = json!(false);
        }
        round_trip(value.clone());
        let expected = value.clone();
        value["vendorField"] = json!(true);
        value["nodes"][0]["vendorField"] = json!(true);
        value["nodes"][0]["nodePosition"]["vendorField"] = json!(true);
        value["nodes"][0]["nodePosition"]["allowedDeviationXY"]["vendorField"] = json!(true);
        value["edges"][0]["vendorField"] = json!(true);
        value["edges"][0]["corridor"]["vendorField"] = json!(true);
        let decoded: Order = serde_json::from_value(value).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), expected);
        let mut omitted = full();
        for pointer in [
            "/nodes/0/nodePosition/allowedDeviationXY",
            "/nodes/0/nodePosition/allowedDeviationTheta",
            "/nodes/0/actions/0/retriable",
            "/edges/0/orientationType",
            "/edges/0/reachOrientationBeforeEntering",
            "/edges/0/corridor/corridorReferencePoint",
            "/edges/0/corridor/releaseRequired",
            "/edges/0/corridor/releaseLossBehavior",
            "/edges/0/trajectory/degree",
            "/edges/0/trajectory/knotVector",
            "/edges/0/trajectory/controlPoints/0/weight",
        ] {
            support::remove(&mut omitted, pointer);
        }
        round_trip(omitted);
    }

    #[test]
    fn semantic_graph_geometry_and_robot_checks_are_not_claimed() {
        let mut empty = full();
        empty["nodes"] = json!([]);
        empty["edges"] = json!([]);
        round_trip(empty);
        let mut inconsistent = full();
        inconsistent["nodes"][0]["sequenceId"] = json!(19);
        inconsistent["nodes"][1]["sequenceId"] = json!(19);
        inconsistent["nodes"][0]["released"] = json!(false);
        inconsistent["edges"][0]["trajectory"]["controlPoints"] = json!([]);
        inconsistent["edges"][0]["trajectory"]["knotVector"] = json!([1.0, 0.0]);
        inconsistent["nodes"][0]["nodePosition"]["allowedDeviationXY"]["a"] = json!(0.01);
        inconsistent["edges"][0]["actions"][0]["actionId"] = json!("signal-1");
        inconsistent["edges"][0]["direction"] = json!("vendor-defined");
        round_trip(inconsistent);
        // No positivity bounds are asserted for these fields by either source.
        let mut negative = full();
        for field in [
            "maximumSpeed",
            "maximumMobileRobotHeight",
            "minimumLoadHandlingDeviceHeight",
            "maximumRotationSpeed",
            "length",
        ] {
            negative["edges"][0][field] = json!(-1.0);
        }
        round_trip(negative);
    }

    #[test]
    fn strict_header_timestamp_rules_are_reused_in_both_directions() {
        use chrono::Timelike;
        for invalid in [
            "2017-04-15T11:40:03Z",
            "2017-04-15T11:40:03.123+00:00",
            "2017-02-30T11:40:03.123Z",
        ] {
            let mut value = full();
            value["timestamp"] = json!(invalid);
            assert!(serde_json::from_value::<Order>(value).is_err());
        }
        let mut order: Order = serde_json::from_str(FULL).unwrap();
        order.header.timestamp = order.header.timestamp.with_nanosecond(1).unwrap();
        assert!(serde_json::to_string(&order).is_err());
    }
}

#[cfg(feature = "json")]
#[test]
fn json_helpers_round_trip_and_preserve_errors() {
    let order = Order::from_json(FULL).unwrap();
    assert_eq!(Order::from_json_slice(FULL.as_bytes()).unwrap(), order);
    assert_eq!(Order::from_json(&order.to_json().unwrap()).unwrap(), order);
    assert_eq!(
        Order::from_json_slice(&order.to_json_vec().unwrap()).unwrap(),
        order
    );
    assert_eq!(
        Order::from_json("{").unwrap_err().to_string(),
        serde_json::from_str::<Order>("{").unwrap_err().to_string()
    );
    assert!(Order::from_json_slice(&[0xff]).is_err());
    let mut invalid = order;
    invalid.edges[0].maximum_rotation_speed = Some(f64::NAN);
    assert!(invalid.to_json().is_err());
    assert!(invalid.to_json_vec().is_err());
}
