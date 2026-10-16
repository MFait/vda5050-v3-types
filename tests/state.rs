//! Pinned VDA5050/VDA5050@0b2ae43154bcd1ab2be4e2445df8b406dcad77b2:
//! https://github.com/VDA5050/VDA5050/blob/0b2ae43154bcd1ab2be4e2445df8b406dcad77b2/VDA5050_EN.md
//! Sections 7.1.4, 7.2 and 7.8. See docs/provenance.md for discrepancies.

use chrono::{TimeZone, Utc};
use serde_json::Value;
use vda5050_v3_types::{
    State,
    common::Header,
    state::{EmergencyStop, OperatingMode, PowerSupply, SafetyState},
};

mod support;

const FULL: &str = include_str!("fixtures/state.json");
const MINIMAL: &str = include_str!("fixtures/state-minimal.json");

fn full() -> Value {
    serde_json::from_str(FULL).unwrap()
}

#[test]
fn state_is_constructible_without_optional_features() {
    let state = State {
        header: Header {
            header_id: 0,
            timestamp: Utc.with_ymd_and_hms(2026, 10, 16, 12, 0, 0).unwrap(),
            version: "3.0.0".into(),
            manufacturer: "Example".into(),
            serial_number: "robot-1".into(),
        },
        maps: None,
        zone_sets: None,
        order_id: String::new(),
        order_update_id: 0,
        last_node_id: String::new(),
        last_node_sequence_id: 0,
        node_states: vec![],
        edge_states: vec![],
        planned_path: None,
        intermediate_path: None,
        mobile_robot_position: None,
        velocity: None,
        loads: None,
        driving: false,
        paused: None,
        new_base_request: None,
        zone_requests: None,
        edge_requests: None,
        distance_since_last_node: None,
        action_states: vec![],
        instant_action_states: vec![],
        zone_action_states: None,
        power_supply: PowerSupply {
            state_of_charge: 100.0,
            battery_voltage: None,
            battery_current: None,
            battery_health: None,
            charging: false,
            range: None,
        },
        operating_mode: OperatingMode::Startup,
        errors: vec![],
        information: None,
        safety_state: SafetyState {
            active_emergency_stop: EmergencyStop::None,
            field_violation: false,
        },
    };
    assert_eq!(state.clone(), state);
}

#[test]
fn fixtures_satisfy_pinned_schema() {
    for value in [full(), serde_json::from_str(MINIMAL).unwrap()] {
        support::schema("state").validate(&value).unwrap();
    }
}

#[cfg(feature = "serde")]
mod wire {
    use super::*;
    use chrono::Timelike;
    use serde_json::json;
    use std::f64::consts::PI;

    fn round_trip(value: Value) -> State {
        support::schema("state").validate(&value).unwrap();
        let state: State = serde_json::from_str(&value.to_string()).unwrap();
        let encoded: Value = serde_json::from_slice(&serde_json::to_vec(&state).unwrap()).unwrap();
        assert_eq!(encoded, value);
        support::schema("state").validate(&encoded).unwrap();
        state
    }

    #[test]
    fn full_and_minimal_round_trip_with_flat_header_and_exact_keys() {
        let state = round_trip(full());
        assert_eq!(state.power_supply.battery_health, Some(95));
        assert_eq!(state.power_supply.range, Some(1000));
        assert_eq!(
            state.node_states[0].node_position.as_ref().unwrap().theta,
            Some(0.0)
        );
        assert!(serde_json::to_value(state).unwrap().get("header").is_none());
        let minimal = round_trip(serde_json::from_str(MINIMAL).unwrap());
        assert!(minimal.loads.is_none());
        assert!(minimal.zone_action_states.is_none());
        assert!(minimal.information.is_none());
    }

    // Traverse only the fixture's schema-declared objects. Requiring every
    // property in the complete fixture makes presence coverage exhaustive.
    fn fields(root: &Value, schema: &Value, value: &Value, path: &str, coverage: &mut Coverage) {
        if let Some(reference) = schema.get("$ref") {
            return fields(
                root,
                root.pointer(reference.as_str().unwrap().strip_prefix('#').unwrap())
                    .unwrap(),
                value,
                path,
                coverage,
            );
        }
        if let Some(values) = schema.get("enum") {
            coverage
                .enums
                .push((path.into(), values.as_array().unwrap().clone()));
        }
        match schema["type"].as_str() {
            Some("object") => {
                coverage.objects.push(path.into());
                for (name, child) in schema["properties"].as_object().unwrap() {
                    let pointer = format!("{path}/{name}");
                    assert!(
                        value.get(name).is_some(),
                        "complete fixture lacks {pointer}"
                    );
                    if schema["required"]
                        .as_array()
                        .is_some_and(|keys| keys.contains(&json!(name)))
                    {
                        coverage.required.push(pointer.clone());
                    } else {
                        coverage.optional.push(pointer.clone());
                    }
                    fields(root, child, &value[name], &pointer, coverage);
                }
            }
            Some("array") => {
                for (index, item) in value.as_array().unwrap().iter().enumerate() {
                    // A second control point intentionally omits optional weight.
                    if index == 0 {
                        fields(
                            root,
                            &schema["items"],
                            item,
                            &format!("{path}/{index}"),
                            coverage,
                        );
                    }
                }
            }
            _ => {}
        }
    }

    struct Coverage {
        required: Vec<String>,
        optional: Vec<String>,
        objects: Vec<String>,
        enums: Vec<(String, Vec<Value>)>,
    }

    fn coverage() -> Coverage {
        let schema: Value = serde_json::from_str(include_str!("schemas/state.schema")).unwrap();
        let mut coverage = Coverage {
            required: vec![],
            optional: vec![],
            objects: vec![],
            enums: vec![],
        };
        fields(&schema, &schema, &full(), "", &mut coverage);
        coverage
    }

    #[test]
    fn every_schema_field_has_required_optional_and_null_coverage() {
        let coverage = coverage();
        let required: Vec<_> = coverage.required.iter().map(String::as_str).collect();
        let optional: Vec<_> = coverage.optional.iter().map(String::as_str).collect();
        support::assert_required::<State>(&full(), support::schema("state"), &required);
        support::assert_optional::<State>(&full(), support::schema("state"), &optional);
        for pointer in required.into_iter().chain(optional) {
            let mut value = full();
            let field = value.pointer_mut(pointer).unwrap();
            *field = if field.is_object() {
                json!([])
            } else {
                json!({})
            };
            assert!(!support::schema("state").is_valid(&value), "{pointer}");
            assert!(serde_json::from_value::<State>(value).is_err(), "{pointer}");
        }
    }

    #[test]
    fn all_nested_objects_reject_arrays_nulls_and_duplicates() {
        let fixture = full();
        for pointer in coverage().objects {
            for invalid in [json!([]), json!([1]), Value::Null, json!(42)] {
                let mut value = fixture.clone();
                *value.pointer_mut(&pointer).unwrap() = invalid;
                assert!(serde_json::from_value::<State>(value).is_err(), "{pointer}");
            }
        }
        let duplicate = FULL.replacen(
            "\"driving\": true",
            "\"driving\": true, \"driving\": false",
            1,
        );
        assert!(serde_json::from_str::<State>(&duplicate).is_err());
    }

    #[test]
    fn all_closed_enum_values_round_trip_and_unknown_values_fail() {
        for (pointer, values) in coverage().enums {
            for valid in &values {
                let mut value = full();
                *value.pointer_mut(&pointer).unwrap() = valid.clone();
                round_trip(value);
            }
            for invalid in [
                json!("UNKNOWN"),
                json!(values[0].as_str().unwrap().to_lowercase()),
                json!(""),
            ] {
                let mut value = full();
                *value.pointer_mut(&pointer).unwrap() = invalid;
                assert!(!support::schema("state").is_valid(&value), "{pointer}");
                assert!(serde_json::from_value::<State>(value).is_err(), "{pointer}");
            }
        }
        let mut value = full();
        value["zoneRequests"][0]["requestType"] = json!("COORDINATED_REPLANNING");
        assert!(serde_json::from_value::<State>(value).is_err());
    }

    #[test]
    fn uint32_fields_follow_spec_widths_instead_of_unbounded_schema_numbers() {
        for pointer in [
            "/headerId",
            "/orderUpdateId",
            "/lastNodeSequenceId",
            "/nodeStates/0/sequenceId",
            "/edgeStates/0/sequenceId",
            "/edgeRequests/0/sequenceId",
            "/powerSupply/range",
            "/edgeStates/0/trajectory/degree",
            "/plannedPath/trajectory/degree",
            "/zoneRequests/0/trajectory/degree",
        ] {
            for valid in [1, u32::MAX] {
                let mut value = full();
                *value.pointer_mut(pointer).unwrap() = json!(valid);
                round_trip(value);
            }
            let mut zero = full();
            *zero.pointer_mut(pointer).unwrap() = json!(0);
            if pointer.ends_with("/degree") {
                assert!(serde_json::from_value::<State>(zero).is_err());
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
                assert!(serde_json::from_value::<State>(value).is_err(), "{pointer}");
            }
        }
        let mut state: State = serde_json::from_str(FULL).unwrap();
        state.power_supply.range = Some(u32::MAX);
        assert_eq!(
            round_trip(serde_json::to_value(state).unwrap())
                .power_supply
                .range,
            Some(u32::MAX)
        );
    }

    #[test]
    fn battery_health_is_spec_int8_in_zero_to_one_hundred() {
        for valid in [0, 100] {
            let mut value = full();
            value["powerSupply"]["batteryHealth"] = json!(valid);
            assert_eq!(round_trip(value).power_supply.battery_health, Some(valid));
        }
        for invalid in [json!(-1), json!(101), json!(128), json!(0.5), json!(1.0)] {
            let mut value = full();
            value["powerSupply"]["batteryHealth"] = invalid;
            assert!(serde_json::from_value::<State>(value).is_err());
        }
        // Schema allows fractional health/range, but section 7.8 does not.
        for field in ["batteryHealth", "range"] {
            let mut value = full();
            value["powerSupply"][field] = json!(0.5);
            assert!(support::schema("state").is_valid(&value));
            assert!(serde_json::from_value::<State>(value).is_err());
        }
        for invalid in [-1, 101, i8::MAX, i8::MIN] {
            let mut state: State = serde_json::from_str(FULL).unwrap();
            state.power_supply.battery_health = Some(invalid);
            assert!(
                serde_json::to_string(&state)
                    .unwrap_err()
                    .to_string()
                    .contains("batteryHealth")
            );
        }
    }

    fn floats(state: &mut State) -> Vec<(&'static str, &mut f64, f64, f64)> {
        let node = state.node_states[0].node_position.as_mut().unwrap();
        let load = &mut state.loads.as_mut().unwrap()[0];
        let bbox = load.bounding_box_reference.as_mut().unwrap();
        let dimensions = load.load_dimensions.as_mut().unwrap();
        let power = &mut state.power_supply;
        vec![
            (
                "/nodeStates/0/nodePosition/x",
                &mut node.x,
                f64::MIN,
                f64::MAX,
            ),
            (
                "/nodeStates/0/nodePosition/y",
                &mut node.y,
                f64::MIN,
                f64::MAX,
            ),
            (
                "/nodeStates/0/nodePosition/theta",
                node.theta.as_mut().unwrap(),
                -PI,
                PI,
            ),
            (
                "/distanceSinceLastNode",
                state.distance_since_last_node.as_mut().unwrap(),
                f64::MIN,
                f64::MAX,
            ),
            (
                "/loads/0/weight",
                load.weight.as_mut().unwrap(),
                0.0,
                f64::MAX,
            ),
            (
                "/loads/0/boundingBoxReference/x",
                &mut bbox.x,
                f64::MIN,
                f64::MAX,
            ),
            (
                "/loads/0/boundingBoxReference/y",
                &mut bbox.y,
                f64::MIN,
                f64::MAX,
            ),
            (
                "/loads/0/boundingBoxReference/z",
                &mut bbox.z,
                f64::MIN,
                f64::MAX,
            ),
            (
                "/loads/0/boundingBoxReference/theta",
                bbox.theta.as_mut().unwrap(),
                f64::MIN,
                f64::MAX,
            ),
            (
                "/loads/0/loadDimensions/length",
                &mut dimensions.length,
                0.0,
                f64::MAX,
            ),
            (
                "/loads/0/loadDimensions/width",
                &mut dimensions.width,
                0.0,
                f64::MAX,
            ),
            (
                "/loads/0/loadDimensions/height",
                dimensions.height.as_mut().unwrap(),
                0.0,
                f64::MAX,
            ),
            (
                "/powerSupply/stateOfCharge",
                &mut power.state_of_charge,
                0.0,
                100.0,
            ),
            (
                "/powerSupply/batteryVoltage",
                power.battery_voltage.as_mut().unwrap(),
                f64::MIN,
                f64::MAX,
            ),
            (
                "/powerSupply/batteryCurrent",
                power.battery_current.as_mut().unwrap(),
                f64::MIN,
                f64::MAX,
            ),
        ]
    }

    #[test]
    fn new_float_fields_enforce_ranges_and_finiteness_in_both_directions() {
        let mut template: State = serde_json::from_str(FULL).unwrap();
        let cases: Vec<_> = floats(&mut template)
            .into_iter()
            .map(|(p, _, min, max)| (p, min, max))
            .collect();
        for (pointer, min, max) in cases {
            for valid in [min, max] {
                let mut value = full();
                *value.pointer_mut(pointer).unwrap() = json!(valid);
                round_trip(value);
                let mut state: State = serde_json::from_str(FULL).unwrap();
                *floats(&mut state)
                    .into_iter()
                    .find(|(p, ..)| *p == pointer)
                    .unwrap()
                    .1 = valid;
                assert_eq!(
                    serde_json::from_str::<State>(&serde_json::to_string(&state).unwrap()).unwrap(),
                    state
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
                    assert!(serde_json::from_value::<State>(value).is_err(), "{pointer}");
                }
                let mut state: State = serde_json::from_str(FULL).unwrap();
                *floats(&mut state)
                    .into_iter()
                    .find(|(p, ..)| *p == pointer)
                    .unwrap()
                    .1 = invalid;
                let error = serde_json::to_string(&state).unwrap_err().to_string();
                assert!(
                    error.contains(pointer.rsplit('/').next().unwrap()),
                    "{pointer}: {error}"
                );
            }
        }
        let mut rounded = full();
        rounded["nodeStates"][0]["nodePosition"]["theta"] = json!(PI + 1e-13);
        assert!(support::schema("state").is_valid(&rounded));
        assert!(serde_json::from_value::<State>(rounded).is_err());
    }

    #[test]
    fn reused_models_keep_their_numeric_and_timestamp_rules() {
        for (pointer, invalid) in [
            ("/mobileRobotPosition/theta", json!(PI + 1e-9)),
            ("/mobileRobotPosition/localizationScore", json!(1.1)),
            ("/mobileRobotPosition/deviationRange", json!(-0.1)),
            ("/edgeStates/0/trajectory/controlPoints/0/weight", json!(0)),
            ("/plannedPath/trajectory/knotVector/0", json!(-0.1)),
            ("/zoneRequests/0/trajectory/knotVector/0", json!(1.1)),
            ("/intermediatePath/polyline/0/theta", json!(PI + 1e-9)),
            (
                "/intermediatePath/polyline/0/eta",
                json!("2017-04-15T11:40:03Z"),
            ),
            ("/timestamp", json!("2017-04-15T11:40:03Z")),
        ] {
            let mut value = full();
            *value.pointer_mut(pointer).unwrap() = invalid;
            assert!(serde_json::from_value::<State>(value).is_err(), "{pointer}");
        }
        let mut state: State = serde_json::from_str(FULL).unwrap();
        state.velocity.as_mut().unwrap().vx = Some(f64::NAN);
        assert!(serde_json::to_string(&state).is_err());
        state.velocity.as_mut().unwrap().vx = Some(8.971135692166839e-150);
        state.header.timestamp = state.header.timestamp.with_nanosecond(1).unwrap();
        assert!(serde_json::to_string(&state).is_err());
        state.header.timestamp = Utc.with_ymd_and_hms(2017, 4, 15, 11, 40, 3).unwrap();
        state.intermediate_path.as_mut().unwrap().polyline[0].eta =
            state.header.timestamp.with_nanosecond(1).unwrap();
        assert!(serde_json::to_string(&state).is_err());
    }

    #[test]
    fn optional_collections_preserve_unknown_empty_and_populated_states() {
        for pointer in [
            "/maps",
            "/zoneSets",
            "/loads",
            "/zoneRequests",
            "/edgeRequests",
            "/zoneActionStates",
            "/information",
        ] {
            let mut value = full();
            *value.pointer_mut(pointer).unwrap() = json!([]);
            round_trip(value);
            let mut missing = full();
            support::remove(&mut missing, pointer);
            round_trip(missing);
        }
        let mut value = full();
        value["loads"] = json!([{}]);
        value["velocity"] = json!({});
        round_trip(value);
    }

    #[test]
    fn unknown_fields_are_ignored_but_open_strings_preserved() {
        let mut value = full();
        value["errors"][0]["errorType"] = json!("vendor.custom");
        value["information"][0]["infoType"] = json!("vendor.custom");
        value["errors"][0]["errorDescriptionTranslations"][0]["translationKey"] =
            json!("not-validated");
        round_trip(value.clone());
        let expected = value.clone();
        for pointer in coverage().objects {
            value
                .pointer_mut(&pointer)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .insert("vendorExtra".into(), json!(true));
        }
        // Order-only tolerances are unknown in the state node position.
        value["nodeStates"][0]["nodePosition"]["allowedDeviationTheta"] = json!(0.1);
        let decoded: State = serde_json::from_value(value).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), expected);
    }

    #[test]
    fn semantic_and_capability_constraints_remain_explicitly_unvalidated() {
        let mut value = full();
        let duplicate_map = value["maps"][0].clone();
        let duplicate_zone_set = value["zoneSets"][0].clone();
        value["maps"].as_array_mut().unwrap().push(duplicate_map);
        value["zoneSets"]
            .as_array_mut()
            .unwrap()
            .push(duplicate_zone_set);
        value["driving"] = json!(true);
        value["paused"] = json!(true);
        value["safetyState"]["activeEmergencyStop"] = json!("MANUAL");
        value["nodeStates"][0]["sequenceId"] = json!(99);
        value["nodeStates"][0]["released"] = json!(false);
        value["edgeStates"][0]["trajectory"]["controlPoints"] = json!([]);
        value["plannedPath"]["trajectory"]["knotVector"] = json!([1.0, 0.0]);
        value["zoneRequests"][0]["requestType"] = json!("ACCESS");
        value["zoneRequests"][0]["requestId"] = value["edgeRequests"][0]["requestId"].clone();
        value["lastNodeId"] = json!("");
        value["lastNodeSequenceId"] = json!(u32::MAX);
        round_trip(value);
    }
}

#[cfg(feature = "json")]
#[test]
fn json_helpers_preserve_bodies_precision_and_errors() {
    let mut state = State::from_json(FULL).unwrap();
    assert_eq!(State::from_json_slice(FULL.as_bytes()).unwrap(), state);
    assert_eq!(State::from_json(&state.to_json().unwrap()).unwrap(), state);
    assert_eq!(
        State::from_json_slice(&state.to_json_vec().unwrap()).unwrap(),
        state
    );
    state.power_supply.battery_current = Some(8.971135692166839e-150);
    for decoded in [
        State::from_json(&state.to_json().unwrap()).unwrap(),
        State::from_json_slice(&state.to_json_vec().unwrap()).unwrap(),
    ] {
        assert_eq!(
            decoded.power_supply.battery_current.unwrap().to_bits(),
            state.power_supply.battery_current.unwrap().to_bits()
        );
    }
    assert_eq!(
        State::from_json("{").unwrap_err().to_string(),
        serde_json::from_str::<State>("{").unwrap_err().to_string()
    );
    assert!(State::from_json_slice(&[0xff]).is_err());
    state.power_supply.state_of_charge = f64::NAN;
    assert!(state.to_json().is_err());
    assert!(state.to_json_vec().is_err());
}
