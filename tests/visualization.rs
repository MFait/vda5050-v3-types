//! VDA 5050 v3.0.0 sources pinned to:
//! https://github.com/VDA5050/VDA5050/blob/0b2ae43154bcd1ab2be4e2445df8b406dcad77b2/VDA5050_EN.md
//! Sections 6.7-6.8, 7.1, 7.2, 7.3 (NURBS), 7.8 (shared models), 7.9.
//! Schema: visualization.schema at the same revision, vendored in tests/schemas.

use chrono::{TimeZone, Utc};
use serde_json::Value;
use vda5050_v3_types::{Visualization, common::Header};

mod support;

const FULL: &str = include_str!("fixtures/visualization.json");
const MINIMAL: &str = include_str!("fixtures/visualization-minimal.json");

fn full() -> Value {
    serde_json::from_str(FULL).unwrap()
}

fn schema() -> &'static jsonschema::Validator {
    support::schema("visualization")
}

#[test]
fn visualization_is_constructible_without_optional_features() {
    // Section 7.9: four optional objects; uint32 reference counter is required.
    let value = Visualization {
        header: Header {
            header_id: 0,
            timestamp: Utc.with_ymd_and_hms(2026, 10, 16, 12, 0, 0).unwrap(),
            version: "3.0.0".into(),
            manufacturer: "Example".into(),
            serial_number: "robot-1".into(),
        },
        reference_state_header_id: u32::MAX,
        planned_path: None,
        intermediate_path: None,
        mobile_robot_position: None,
        velocity: None,
    };
    assert_eq!(value.clone(), value);
    assert_eq!(value.reference_state_header_id, u32::MAX);
}

#[test]
fn both_fixtures_satisfy_the_pinned_schema() {
    let validator = schema();
    for value in [full(), serde_json::from_str(MINIMAL).unwrap()] {
        validator.validate(&value).unwrap();
    }
}

#[cfg(feature = "serde")]
mod wire {
    use super::*;
    use chrono::Timelike;
    use serde_json::json;
    use std::f64::consts::PI;
    use support::remove;
    use vda5050_v3_types::common::{ControlPoint, MobileRobotPosition, Velocity};

    fn assert_round_trip(value: Value) -> Visualization {
        let decoded: Visualization = serde_json::from_str(&value.to_string()).unwrap();
        let encoded: Value =
            serde_json::from_slice(&serde_json::to_vec(&decoded).unwrap()).unwrap();
        assert_eq!(encoded, value);
        schema().validate(&encoded).unwrap();
        decoded
    }

    // Schema conformance and message wire layout.
    #[test]
    fn full_and_minimal_round_trip_with_exact_keys_and_flat_header() {
        // Sections 7.2, 7.8, 7.9: all nested models and wire names.
        let decoded = assert_round_trip(full());
        assert_eq!(decoded.reference_state_header_id, 10);
        assert_eq!(decoded.mobile_robot_position.unwrap().x, 1.0);
        assert_eq!(decoded.velocity.unwrap().vy, Some(-0.1));
        assert_eq!(
            decoded.intermediate_path.unwrap().polyline[0]
                .eta
                .timestamp_millis(),
            1_492_256_403_123
        );
        let minimal = assert_round_trip(serde_json::from_str(MINIMAL).unwrap());
        assert!(minimal.mobile_robot_position.is_none());
        assert!(minimal.planned_path.is_none());
        assert!(minimal.intermediate_path.is_none());
        assert!(minimal.velocity.is_none());
        assert!(
            serde_json::to_value(minimal)
                .unwrap()
                .get("header")
                .is_none()
        );
    }

    #[test]
    fn every_required_field_rejects_missing_null_and_wrong_type() {
        // Sections 7.2/7.8/7.9 and each schema.required array.
        let validator = schema();
        support::assert_required::<Visualization>(
            &full(),
            validator,
            &[
                "/headerId",
                "/timestamp",
                "/version",
                "/manufacturer",
                "/serialNumber",
                "/referenceStateHeaderId",
                "/plannedPath/trajectory",
                "/plannedPath/trajectory/controlPoints",
                "/plannedPath/trajectory/controlPoints/0/x",
                "/plannedPath/trajectory/controlPoints/0/y",
                "/intermediatePath/polyline",
                "/intermediatePath/polyline/0/x",
                "/intermediatePath/polyline/0/y",
                "/intermediatePath/polyline/0/eta",
                "/mobileRobotPosition/x",
                "/mobileRobotPosition/y",
                "/mobileRobotPosition/theta",
                "/mobileRobotPosition/mapId",
                "/mobileRobotPosition/localized",
            ],
        );
    }

    #[test]
    fn every_optional_field_distinguishes_omission_presence_and_null() {
        // Section 7.1: optional != nullable; schema types exclude null.
        let validator = schema();
        support::assert_optional::<Visualization>(
            &full(),
            validator,
            &[
                "/plannedPath",
                "/intermediatePath",
                "/mobileRobotPosition",
                "/velocity",
                "/plannedPath/traversedNodes",
                "/plannedPath/trajectory/degree",
                "/plannedPath/trajectory/knotVector",
                "/plannedPath/trajectory/controlPoints/0/weight",
                "/intermediatePath/polyline/0/theta",
                "/mobileRobotPosition/localizationScore",
                "/mobileRobotPosition/deviationRange",
                "/velocity/vx",
                "/velocity/vy",
                "/velocity/omega",
            ],
        );
        assert_round_trip(full());
    }

    #[test]
    fn optional_objects_are_independent_and_empty_velocity_is_valid() {
        // Section 7.9: each body object may be absent independently.
        for field in [
            "plannedPath",
            "intermediatePath",
            "mobileRobotPosition",
            "velocity",
        ] {
            let mut value: Value = serde_json::from_str(MINIMAL).unwrap();
            value[field] = full()[field].clone();
            assert_round_trip(value);
        }
        let mut value = full();
        value["velocity"] = json!({});
        let decoded = assert_round_trip(value);
        assert_eq!(
            decoded.velocity,
            Some(Velocity {
                vx: None,
                vy: None,
                omega: None
            })
        );
    }

    #[test]
    fn uint32_fields_enforce_endpoints_and_reject_overflow_and_fractions() {
        // Sections 7.2/7.9 and 7.8 trajectory.degree: uint32, unlike unbounded schemas.
        let validator = schema();
        for pointer in [
            "/headerId",
            "/referenceStateHeaderId",
            "/plannedPath/trajectory/degree",
        ] {
            let minimum = if pointer.ends_with("degree") { 1 } else { 0 };
            for number in [minimum, u32::MAX] {
                let mut value = full();
                *value.pointer_mut(pointer).unwrap() = json!(number);
                assert_round_trip(value);
            }
            for invalid in [
                json!(-1),
                json!(u64::from(u32::MAX) + 1),
                json!(1.5),
                json!(1.0),
                json!("1"),
            ] {
                let mut value = full();
                *value.pointer_mut(pointer).unwrap() = invalid;
                assert!(
                    serde_json::from_value::<Visualization>(value).is_err(),
                    "{pointer}"
                );
            }
        }
        let mut value = full();
        value["plannedPath"]["trajectory"]["degree"] = json!(0);
        assert!(!validator.is_valid(&value));
        assert!(serde_json::from_value::<Visualization>(value).is_err());
        let mut invalid: Visualization = serde_json::from_str(FULL).unwrap();
        invalid.planned_path.as_mut().unwrap().trajectory.degree = Some(0);
        assert!(serde_json::to_value(invalid).is_err());
    }

    #[test]
    fn float_ranges_check_endpoints_and_just_outside_on_decode_and_encode() {
        // Sections 7.8/7.3: angles [-pi,pi], score/knots [0,1], weight >0.
        type FloatField = fn(&mut Visualization) -> &mut f64;
        let cases: &[(&str, f64, f64, FloatField)] = &[
            ("/mobileRobotPosition/theta", -PI, PI, |v| {
                &mut v.mobile_robot_position.as_mut().unwrap().theta
            }),
            ("/intermediatePath/polyline/0/theta", -PI, PI, |v| {
                v.intermediate_path.as_mut().unwrap().polyline[0]
                    .theta
                    .as_mut()
                    .unwrap()
            }),
            ("/mobileRobotPosition/localizationScore", 0.0, 1.0, |v| {
                v.mobile_robot_position
                    .as_mut()
                    .unwrap()
                    .localization_score
                    .as_mut()
                    .unwrap()
            }),
            ("/plannedPath/trajectory/knotVector/0", 0.0, 1.0, |v| {
                &mut v
                    .planned_path
                    .as_mut()
                    .unwrap()
                    .trajectory
                    .knot_vector
                    .as_mut()
                    .unwrap()[0]
            }),
        ];
        for &(pointer, min, max, field) in cases {
            for number in [min, max] {
                let mut value = full();
                *value.pointer_mut(pointer).unwrap() = json!(number);
                let mut decoded = assert_round_trip(value.clone());
                *field(&mut decoded) = number;
                assert_eq!(serde_json::to_value(decoded).unwrap(), value);
            }
            let below_min = if min == 0.0 {
                -f64::from_bits(1)
            } else {
                f64::from_bits(min.to_bits() + 1)
            };
            let above_max = f64::from_bits(max.to_bits() + 1);
            for number in [below_min, above_max] {
                let mut value = full();
                *value.pointer_mut(pointer).unwrap() = json!(number);
                assert!(
                    serde_json::from_value::<Visualization>(value).is_err(),
                    "{pointer}"
                );
                let mut decoded: Visualization = serde_json::from_str(FULL).unwrap();
                *field(&mut decoded) = number;
                assert!(serde_json::to_value(decoded).is_err(), "{pointer}");
            }
        }
        for (pointer, allowed, rejected) in [
            ("/mobileRobotPosition/deviationRange", 0.0, -0.001),
            (
                "/plannedPath/trajectory/controlPoints/0/weight",
                f64::MIN_POSITIVE,
                0.0,
            ),
        ] {
            let mut value = full();
            *value.pointer_mut(pointer).unwrap() = json!(allowed);
            assert_round_trip(value);
            let mut value = full();
            *value.pointer_mut(pointer).unwrap() = json!(rejected);
            assert!(serde_json::from_value::<Visualization>(value).is_err());
        }
        let mut value: Visualization = serde_json::from_str(FULL).unwrap();
        value
            .mobile_robot_position
            .as_mut()
            .unwrap()
            .deviation_range = Some(-0.001);
        assert!(serde_json::to_value(&value).is_err());
        value
            .mobile_robot_position
            .as_mut()
            .unwrap()
            .deviation_range = None;
        for weight in [0.0, -1.0] {
            value
                .planned_path
                .as_mut()
                .unwrap()
                .trajectory
                .control_points[0]
                .weight = Some(weight);
            assert!(serde_json::to_value(&value).is_err());
        }
    }

    #[test]
    fn nonfinite_values_never_turn_into_null_on_encoding() {
        // Section 7.1.4 forbids IEEE NaN/infinity; serde_json alone encodes them as null.
        for number in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            for field in [
                "x",
                "y",
                "theta",
                "score",
                "deviation",
                "vx",
                "vy",
                "omega",
                "weight",
                "cx",
                "cy",
                "wx",
                "wy",
                "wtheta",
                "knot",
            ] {
                let mut value: Visualization = serde_json::from_str(FULL).unwrap();
                match field {
                    "x" => value.mobile_robot_position.as_mut().unwrap().x = number,
                    "y" => value.mobile_robot_position.as_mut().unwrap().y = number,
                    "theta" => value.mobile_robot_position.as_mut().unwrap().theta = number,
                    "score" => {
                        value
                            .mobile_robot_position
                            .as_mut()
                            .unwrap()
                            .localization_score = Some(number)
                    }
                    "deviation" => {
                        value
                            .mobile_robot_position
                            .as_mut()
                            .unwrap()
                            .deviation_range = Some(number)
                    }
                    "vx" => value.velocity.as_mut().unwrap().vx = Some(number),
                    "vy" => value.velocity.as_mut().unwrap().vy = Some(number),
                    "omega" => value.velocity.as_mut().unwrap().omega = Some(number),
                    "weight" => {
                        value
                            .planned_path
                            .as_mut()
                            .unwrap()
                            .trajectory
                            .control_points[0]
                            .weight = Some(number)
                    }
                    "cx" => {
                        value
                            .planned_path
                            .as_mut()
                            .unwrap()
                            .trajectory
                            .control_points[0]
                            .x = number
                    }
                    "cy" => {
                        value
                            .planned_path
                            .as_mut()
                            .unwrap()
                            .trajectory
                            .control_points[0]
                            .y = number
                    }
                    "wx" => value.intermediate_path.as_mut().unwrap().polyline[0].x = number,
                    "wy" => value.intermediate_path.as_mut().unwrap().polyline[0].y = number,
                    "wtheta" => {
                        value.intermediate_path.as_mut().unwrap().polyline[0].theta = Some(number)
                    }
                    "knot" => {
                        value.planned_path.as_mut().unwrap().trajectory.knot_vector =
                            Some(vec![number])
                    }
                    _ => unreachable!(),
                }
                assert!(serde_json::to_string(&value).is_err(), "{field}");
            }
        }
    }

    #[test]
    fn standalone_shared_models_use_the_same_checked_wire_representation() {
        // Section 7.8: shared definitions must not depend on a parent message.
        let position: MobileRobotPosition =
            serde_json::from_value(full()["mobileRobotPosition"].clone()).unwrap();
        assert_eq!(
            serde_json::to_value(position).unwrap(),
            full()["mobileRobotPosition"]
        );
        let control_point = ControlPoint {
            x: 1.0,
            y: 2.0,
            weight: None,
        };
        assert_eq!(
            serde_json::to_value(control_point).unwrap(),
            json!({"x":1.0,"y":2.0})
        );
        assert!(
            serde_json::from_value::<ControlPoint>(json!({"x":1,"y":2,"weight":null})).is_err()
        );
        let velocity: Velocity = serde_json::from_value(json!({"vx": -2})).unwrap();
        assert_eq!(velocity.vx, Some(-2.0));
    }

    #[test]
    fn waypoint_eta_uses_strict_timestamp_rules_on_both_paths() {
        // Section 7.8 waypoint.eta explicitly uses section 7.2's timestamp spelling.
        let validator = schema();
        for timestamp in [
            "not-a-date",
            "2017-04-15T11:40:03Z",
            "2017-04-15T11:40:03.123+00:00",
        ] {
            let mut value = full();
            value["intermediatePath"]["polyline"][0]["eta"] = json!(timestamp);
            assert!(validator.is_valid(&value)); // Schema omits even a date-time format.
            assert!(serde_json::from_value::<Visualization>(value).is_err());
        }
        let mut value: Visualization = serde_json::from_str(FULL).unwrap();
        let point = &mut value.intermediate_path.as_mut().unwrap().polyline[0];
        point.eta = point.eta.with_nanosecond(1).unwrap();
        assert!(serde_json::to_value(value).is_err());
    }

    // Specification-derived constraints that close gaps in schema assertions.
    #[test]
    fn specification_bounds_close_schema_gaps() {
        let validator = schema();
        for (pointer, invalid) in [
            ("/referenceStateHeaderId", json!(-1)),
            ("/referenceStateHeaderId", json!(u64::from(u32::MAX) + 1)),
            (
                "/plannedPath/trajectory/degree",
                json!(u64::from(u32::MAX) + 1),
            ),
            ("/plannedPath/trajectory/controlPoints/0/weight", json!(0.0)),
            (
                "/plannedPath/trajectory/controlPoints/0/weight",
                json!(-1.0),
            ),
        ] {
            let mut value = full();
            *value.pointer_mut(pointer).unwrap() = invalid;
            assert!(validator.is_valid(&value), "{pointer}");
            assert!(
                serde_json::from_value::<Visualization>(value).is_err(),
                "{pointer}"
            );
        }
        // Schema's decimal pi is slightly larger than f64::consts::PI.
        for pointer in [
            "/mobileRobotPosition/theta",
            "/intermediatePath/polyline/0/theta",
        ] {
            let mut value = full();
            *value.pointer_mut(pointer).unwrap() = json!(PI + 1e-13);
            assert!(validator.is_valid(&value));
            assert!(serde_json::from_value::<Visualization>(value).is_err());
        }
    }

    #[test]
    fn defaults_remain_omitted_and_unknown_nested_fields_are_not_preserved() {
        // Section 7.8: defaults describe protocol interpretation, not required wire fields.
        let mut value = full();
        for pointer in [
            "/plannedPath/trajectory/degree",
            "/plannedPath/trajectory/knotVector",
            "/plannedPath/trajectory/controlPoints/0/weight",
        ] {
            remove(&mut value, pointer);
        }
        let decoded = assert_round_trip(value.clone());
        let trajectory = decoded.planned_path.unwrap().trajectory;
        assert_eq!(trajectory.degree, None);
        assert_eq!(trajectory.knot_vector, None);
        assert_eq!(trajectory.control_points[0].weight, None);
        let mut extended = value.clone();
        for pointer in [
            "",
            "/mobileRobotPosition",
            "/velocity",
            "/plannedPath/trajectory",
            "/intermediatePath/polyline/0",
        ] {
            extended.pointer_mut(pointer).unwrap()["vendorField"] = json!({"value": null});
        }
        schema().validate(&extended).unwrap();
        let decoded: Visualization = serde_json::from_value(extended).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), value);
    }

    // Acceptance boundary, not a semantic-validator test.
    #[test]
    fn cross_field_constraints_are_not_claimed_as_validated() {
        // Sections 6.8/7.8: path starts at robot position, NURBS point counts,
        // clamping/multiplicities and base coverage require semantic validation.
        let mut value = full();
        value["plannedPath"]["trajectory"]["degree"] = json!(5);
        value["plannedPath"]["trajectory"]["knotVector"] = json!([0.7, 0.1]);
        value["intermediatePath"]["polyline"][0]["x"] = json!(999.0);
        value["mobileRobotPosition"]["localized"] = json!(false);
        value["referenceStateHeaderId"] = json!(u32::MAX);
        assert_round_trip(value);
        let mut empty = full();
        empty["plannedPath"]["trajectory"]["controlPoints"] = json!([]);
        empty["intermediatePath"]["polyline"] = json!([]);
        assert_round_trip(empty);
    }

    #[test]
    fn malformed_shapes_and_duplicate_fields_are_errors() {
        for invalid in ["null", "[]", "42", "\"visualization\""] {
            assert!(serde_json::from_str::<Visualization>(invalid).is_err());
        }
        for (pointer, invalid) in [
            ("/velocity", json!([])),
            ("/velocity", json!([0.1, 0.2, 0.3])),
            ("/plannedPath", json!([{"controlPoints": []}])),
            ("/plannedPath/trajectory", json!([1, [], []])),
            ("/plannedPath/trajectory/controlPoints", json!([[1, 2]])),
            ("/intermediatePath", json!([[]])),
            (
                "/intermediatePath/polyline",
                json!([[1, 2, null, "2017-04-15T11:40:03.123Z"]]),
            ),
            ("/mobileRobotPosition", json!([1, 2, 0, "floor-1", true])),
            ("/velocity/vx", json!("0.5")),
            ("/plannedPath/traversedNodes", json!([42])),
            ("/plannedPath/trajectory/knotVector", json!([null])),
            ("/plannedPath/trajectory/controlPoints", json!([null])),
            ("/intermediatePath/polyline", json!([null])),
            ("/mobileRobotPosition/localized", json!(1)),
        ] {
            let mut value = full();
            *value.pointer_mut(pointer).unwrap() = invalid;
            assert!(!schema().is_valid(&value));
            assert!(serde_json::from_value::<Visualization>(value).is_err());
        }
        let duplicate = FULL.replacen(
            "\"referenceStateHeaderId\": 10",
            "\"referenceStateHeaderId\": 10, \"referenceStateHeaderId\": 11",
            1,
        );
        assert!(serde_json::from_str::<Visualization>(&duplicate).is_err());
        let duplicate = FULL.replacen("\"vx\": 0.5", "\"vx\": 0.5, \"vx\": 0.6", 1);
        assert!(serde_json::from_str::<Visualization>(&duplicate).is_err());
    }
}

#[cfg(feature = "json")]
#[test]
fn json_helpers_preserve_bodies_and_errors() {
    use chrono::Timelike;
    for fixture in [FULL, MINIMAL] {
        let decoded = Visualization::from_json(fixture).unwrap();
        assert_eq!(
            Visualization::from_json_slice(fixture.as_bytes()).unwrap(),
            decoded
        );
        assert_eq!(
            Visualization::from_json(&decoded.to_json().unwrap()).unwrap(),
            decoded
        );
        assert_eq!(
            Visualization::from_json_slice(&decoded.to_json_vec().unwrap()).unwrap(),
            decoded
        );
    }
    for payload in ["{", "{}"] {
        assert_eq!(
            Visualization::from_json(payload).unwrap_err().to_string(),
            serde_json::from_str::<Visualization>(payload)
                .unwrap_err()
                .to_string()
        );
    }
    assert!(Visualization::from_json_slice(&[0xff]).is_err());
    let mut value = Visualization::from_json(FULL).unwrap();
    value.velocity.as_mut().unwrap().omega = Some(f64::NAN);
    assert!(value.to_json().is_err());
    assert!(value.to_json_vec().is_err());
    value.velocity = None;
    value.header.timestamp = value.header.timestamp.with_nanosecond(1).unwrap();
    assert!(value.to_json().is_err());
}
