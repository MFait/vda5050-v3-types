//! VDA5050/VDA5050@0b2ae43154bcd1ab2be4e2445df8b406dcad77b2:
//! https://github.com/VDA5050/VDA5050/blob/0b2ae43154bcd1ab2be4e2445df8b406dcad77b2/VDA5050_EN.md
//! Sections 7.1.4, 7.2, 7.8 (shared loads), and 7.10.

use chrono::{TimeZone, Utc};
use serde_json::Value;
use vda5050_v3_types::{
    Factsheet,
    common::Header,
    factsheet::{
        LoadSpecification, MaximumArrayLengths, MaximumStringLengths, MobileRobotGeometry,
        PhysicalParameters, ProtocolFeatures, ProtocolLimits, Timing, TypeSpecification,
    },
};

mod support;

const FULL: &str = include_str!("fixtures/factsheet.json");
const MINIMAL: &str = include_str!("fixtures/factsheet-minimal.json");

fn full() -> Value {
    serde_json::from_str(FULL).unwrap()
}

#[test]
fn types_are_usable_without_optional_features() {
    let sheet = Factsheet {
        header: Header {
            header_id: 0,
            timestamp: Utc.with_ymd_and_hms(2026, 10, 16, 12, 0, 0).unwrap(),
            version: "3.0.0".into(),
            manufacturer: "Example".into(),
            serial_number: "robot-1".into(),
        },
        type_specification: TypeSpecification {
            series_name: "Example".into(),
            series_description: None,
            mobile_robot_kinematics: "vendor-kinematics".into(),
            mobile_robot_class: "vendor-class".into(),
            maximum_load_mass: 0.0,
            localization_types: vec![],
            navigation_types: vec![],
            supported_zones: None,
        },
        physical_parameters: PhysicalParameters {
            minimum_speed: 0.0,
            maximum_speed: 1.0,
            minimum_angular_speed: None,
            maximum_angular_speed: None,
            maximum_acceleration: 0.5,
            maximum_deceleration: 0.5,
            minimum_height: 0.3,
            maximum_height: 0.3,
            width: 0.8,
            length: 1.2,
        },
        protocol_limits: ProtocolLimits {
            maximum_string_lengths: MaximumStringLengths {
                maximum_message_length: None,
                maximum_topic_serial_length: None,
                maximum_topic_element_length: None,
                maximum_id_length: None,
                id_numerical_only: None,
                maximum_load_id_length: None,
            },
            maximum_array_lengths: MaximumArrayLengths {
                order_nodes: None,
                order_edges: None,
                node_actions: None,
                edge_actions: None,
                actions_parameters: None,
                instant_actions: None,
                trajectory_knot_vector: None,
                trajectory_control_points: None,
                zone_set_zones: None,
                state_node_states: None,
                state_edge_states: None,
                state_loads: None,
                state_action_states: None,
                state_instant_action_states: None,
                state_zone_action_states: None,
                state_errors: None,
                state_information: None,
                error_references: None,
                information_references: None,
            },
            timing: Timing {
                minimum_order_interval: 0.125,
                minimum_state_interval: 0.25,
                default_state_interval: None,
                visualization_interval: None,
            },
        },
        protocol_features: ProtocolFeatures {
            optional_parameters: vec![],
            mobile_robot_actions: vec![],
        },
        mobile_robot_geometry: MobileRobotGeometry {
            wheel_definitions: None,
            envelopes2d: None,
            envelopes3d: None,
        },
        load_specification: LoadSpecification {
            load_positions: None,
            load_sets: None,
        },
        mobile_robot_configuration: None,
    };
    assert_eq!(sheet.clone(), sheet);
}

#[test]
fn fixtures_satisfy_pinned_schema() {
    for value in [full(), serde_json::from_str(MINIMAL).unwrap()] {
        support::schema("factsheet").validate(&value).unwrap();
    }
}

#[cfg(feature = "serde")]
mod wire {
    use super::*;
    use serde::{Serialize, de::DeserializeOwned};
    use serde_json::json;
    use vda5050_v3_types::factsheet::*;

    fn round_trip(value: Value) -> Factsheet {
        support::schema("factsheet").validate(&value).unwrap();
        let model: Factsheet = serde_json::from_str(&value.to_string()).unwrap();
        let encoded: Value = serde_json::from_slice(&serde_json::to_vec(&model).unwrap()).unwrap();
        assert_eq!(encoded, value);
        support::schema("factsheet").validate(&encoded).unwrap();
        model
    }

    // Every declared property must appear in the full fixture. Arbitrary
    // envelope data has no declared properties and is deliberately opaque.
    fn fields(schema: &Value, value: &Value, path: &str, out: &mut Vec<(String, Value, bool)>) {
        if let Some(properties) = schema["properties"].as_object() {
            for (name, child) in properties {
                assert!(
                    value.get(name).is_some(),
                    "full fixture lacks {path}/{name}"
                );
                let pointer = format!("{path}/{name}");
                let required = schema["required"]
                    .as_array()
                    .is_some_and(|r| r.contains(&json!(name)));
                out.push((pointer.clone(), child.clone(), required));
                fields(child, &value[name], &pointer, out);
            }
        } else if schema["type"] == "array" {
            let item = value
                .as_array()
                .unwrap()
                .first()
                .expect("full fixture array must be populated");
            let pointer = format!("{path}/0");
            out.push((pointer.clone(), schema["items"].clone(), true));
            fields(&schema["items"], item, &pointer, out);
        }
    }

    fn coverage() -> Vec<(String, Value, bool)> {
        let schema = serde_json::from_str(include_str!("schemas/factsheet.schema")).unwrap();
        let mut out = vec![];
        fields(&schema, &full(), "", &mut out);
        out
    }

    #[test]
    fn exact_wire_names_flat_header_and_minimal_omissions_round_trip() {
        let sheet = round_trip(full());
        assert_eq!(
            sheet
                .protocol_limits
                .maximum_array_lengths
                .actions_parameters,
            Some(10)
        );
        assert_eq!(
            sheet.mobile_robot_geometry.wheel_definitions.unwrap()[0].r#type,
            "DRIVE"
        );
        let encoded = serde_json::to_value(round_trip(full())).unwrap();
        assert!(encoded.get("header").is_none());
        assert!(
            encoded["protocolLimits"]["maximumArrayLengths"]
                .get("order")
                .is_none()
        );
        assert_eq!(
            encoded["protocolLimits"]["maximumArrayLengths"]["order.nodes"],
            100
        );
        let minimal = round_trip(serde_json::from_str(MINIMAL).unwrap());
        assert!(minimal.mobile_robot_geometry.envelopes3d.is_none());
        assert!(minimal.mobile_robot_configuration.is_none());
    }

    #[test]
    fn all_fields_reject_null_and_wrong_types_and_preserve_optional_absence() {
        let fields = coverage();
        let mut required = vec![];
        let mut optional = vec![];
        for (pointer, schema, is_required) in &fields {
            // Array item positions are not removable object properties.
            if pointer.ends_with("/0") {
                continue;
            }
            if *is_required {
                required.push(pointer.as_str());
            } else {
                optional.push(pointer.as_str());
            }
            let mut value = full();
            *value.pointer_mut(pointer).unwrap() = if schema["type"] == "object" {
                json!([])
            } else {
                json!({})
            };
            assert!(!support::schema("factsheet").is_valid(&value), "{pointer}");
            assert!(
                serde_json::from_value::<Factsheet>(value).is_err(),
                "{pointer}"
            );
        }
        support::assert_required::<Factsheet>(&full(), support::schema("factsheet"), &required);
        support::assert_optional::<Factsheet>(&full(), support::schema("factsheet"), &optional);
    }

    fn object<T: DeserializeOwned + Serialize>(value: Value) {
        assert_eq!(
            serde_json::to_value(serde_json::from_value::<T>(value.clone()).unwrap()).unwrap(),
            value
        );
        for invalid in [json!([]), json!([1]), Value::Null, json!(42)] {
            assert!(serde_json::from_value::<T>(invalid).is_err());
        }
        for (field, original) in value.as_object().unwrap() {
            let duplicate = format!("{{\"{field}\":{original},{}", &value.to_string()[1..]);
            assert!(serde_json::from_str::<T>(&duplicate).is_err(), "{field}");
        }
    }

    #[test]
    fn all_public_objects_reject_arrays_and_duplicate_fields() {
        let v = full();
        object::<Factsheet>(v.clone());
        object::<TypeSpecification>(v["typeSpecification"].clone());
        object::<PhysicalParameters>(v["physicalParameters"].clone());
        object::<ProtocolLimits>(v["protocolLimits"].clone());
        object::<MaximumStringLengths>(v["protocolLimits"]["maximumStringLengths"].clone());
        object::<MaximumArrayLengths>(v["protocolLimits"]["maximumArrayLengths"].clone());
        object::<Timing>(v["protocolLimits"]["timing"].clone());
        object::<ProtocolFeatures>(v["protocolFeatures"].clone());
        object::<OptionalParameter>(v["protocolFeatures"]["optionalParameters"][0].clone());
        object::<MobileRobotAction>(v["protocolFeatures"]["mobileRobotActions"][0].clone());
        object::<ActionParameter>(
            v["protocolFeatures"]["mobileRobotActions"][0]["actionParameters"][0].clone(),
        );
        object::<MobileRobotGeometry>(v["mobileRobotGeometry"].clone());
        object::<WheelDefinition>(v["mobileRobotGeometry"]["wheelDefinitions"][0].clone());
        object::<WheelPosition>(
            v["mobileRobotGeometry"]["wheelDefinitions"][0]["position"].clone(),
        );
        object::<Envelope2d>(v["mobileRobotGeometry"]["envelopes2d"][0].clone());
        object::<Vertex>(v["mobileRobotGeometry"]["envelopes2d"][0]["vertices"][0].clone());
        object::<Envelope3d>(v["mobileRobotGeometry"]["envelopes3d"][0].clone());
        object::<LoadSpecification>(v["loadSpecification"].clone());
        object::<LoadSet>(v["loadSpecification"]["loadSets"][0].clone());
        object::<MobileRobotConfiguration>(v["mobileRobotConfiguration"].clone());
        object::<VersionInfo>(v["mobileRobotConfiguration"]["versions"][0].clone());
        object::<Network>(v["mobileRobotConfiguration"]["network"].clone());
        object::<BatteryCharging>(v["mobileRobotConfiguration"]["batteryCharging"].clone());
        for (pointer, schema, _) in coverage() {
            if schema["type"] == "object" {
                let mut value = full();
                *value.pointer_mut(&pointer).unwrap() = json!([]);
                assert!(
                    serde_json::from_value::<Factsheet>(value).is_err(),
                    "{pointer}"
                );
            }
        }
    }

    #[test]
    fn every_closed_enum_accepts_all_values_and_rejects_unknowns() {
        for (pointer, schema, _) in coverage() {
            if let Some(values) = schema["enum"].as_array() {
                for valid in values {
                    let mut value = full();
                    *value.pointer_mut(&pointer).unwrap() = valid.clone();
                    round_trip(value);
                }
                for invalid in ["UNKNOWN", "", &values[0].as_str().unwrap().to_lowercase()] {
                    let mut value = full();
                    *value.pointer_mut(&pointer).unwrap() = json!(invalid);
                    assert!(
                        serde_json::from_value::<Factsheet>(value).is_err(),
                        "{pointer}"
                    );
                }
            }
        }
    }

    #[test]
    fn extensible_values_and_envelope_data_are_preserved_but_unknown_fields_are_not() {
        let mut value = full();
        for pointer in [
            "/typeSpecification/mobileRobotKinematics",
            "/typeSpecification/mobileRobotClass",
            "/typeSpecification/localizationTypes/0",
            "/typeSpecification/navigationTypes/0",
            "/mobileRobotGeometry/wheelDefinitions/0/type",
        ] {
            *value.pointer_mut(pointer).unwrap() = json!("VENDOR_CUSTOM");
        }
        value["mobileRobotGeometry"]["envelopes3d"][0]["data"] = json!({
            "vendor": [null, true, {"nested": "retained"}], "integer": u64::MAX
        });
        round_trip(value.clone());
        let expected = value.clone();
        value["vendorExtra"] = json!(true);
        for (pointer, schema, _) in coverage() {
            if schema.get("properties").is_some() {
                value
                    .pointer_mut(&pointer)
                    .unwrap()
                    .as_object_mut()
                    .unwrap()
                    .insert("vendorExtra".into(), json!(true));
            }
        }
        let decoded: Factsheet = serde_json::from_value(value).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), expected);
        for invalid in [Value::Null, json!([]), json!(true), json!("mesh")] {
            let mut value = full();
            value["mobileRobotGeometry"]["envelopes3d"][0]["data"] = invalid;
            assert!(serde_json::from_value::<Factsheet>(value).is_err());
        }
        let mut empty = full();
        empty["mobileRobotGeometry"]["envelopes3d"][0]["data"] = json!({});
        round_trip(empty);
    }

    #[test]
    fn uint32_limits_and_charging_time_follow_specification_widths() {
        let mut pointers: Vec<_> = coverage()
            .into_iter()
            .filter(|(_, s, _)| s["type"] == "integer")
            .map(|(p, _, _)| p)
            .collect();
        pointers.push("/mobileRobotConfiguration/batteryCharging/minimumChargingTime".into());
        for pointer in pointers {
            for valid in [0, u32::MAX] {
                let mut value = full();
                *value.pointer_mut(&pointer).unwrap() = json!(valid);
                round_trip(value);
            }
            for invalid in [
                json!(-1),
                json!(u64::from(u32::MAX) + 1),
                json!(0.5),
                json!(1.0),
            ] {
                let mut value = full();
                *value.pointer_mut(&pointer).unwrap() = invalid;
                assert!(
                    serde_json::from_value::<Factsheet>(value).is_err(),
                    "{pointer}"
                );
            }
        }
        let mut value = full();
        value["mobileRobotConfiguration"]["batteryCharging"]["minimumChargingTime"] = json!(0.5);
        assert!(support::schema("factsheet").is_valid(&value));
        assert!(serde_json::from_value::<Factsheet>(value).is_err());
    }

    #[test]
    fn every_schema_numeric_bound_is_enforced_on_decode() {
        for (pointer, schema, _) in coverage() {
            if schema["type"] != "number" {
                continue;
            }
            if pointer.ends_with("minimumChargingTime") {
                continue;
            }
            let max = if pointer.starts_with("/protocolLimits/timing/") {
                f64::from(f32::MAX)
            } else {
                schema["maximum"].as_f64().unwrap_or(f64::MAX)
            };
            let min = schema["minimum"].as_f64().unwrap_or(f64::MIN);
            for valid in [min, max] {
                // Shared load dimensions additionally inherit the state model's
                // nonnegative bounds through section 7.10's explicit reference.
                if valid < 0.0 && pointer.contains("/loadDimensions/") {
                    continue;
                }
                let mut value = full();
                *value.pointer_mut(&pointer).unwrap() = json!(valid);
                let decoded: Factsheet = serde_json::from_value(value).unwrap();
                assert!(serde_json::to_string(&decoded).is_ok(), "{pointer}");
            }
            for invalid in [min - 0.01, max + 0.01] {
                if !invalid.is_finite() || (min..=max).contains(&invalid) {
                    continue;
                }
                let mut value = full();
                *value.pointer_mut(&pointer).unwrap() = json!(invalid);
                assert!(
                    serde_json::from_value::<Factsheet>(value).is_err(),
                    "{pointer}"
                );
            }
        }
        for field in ["length", "width", "height"] {
            let mut value = full();
            value["loadSpecification"]["loadSets"][0]["loadDimensions"][field] = json!(-1);
            assert!(support::schema("factsheet").is_valid(&value));
            assert!(serde_json::from_value::<Factsheet>(value).is_err());
        }
    }

    fn float64s(sheet: &mut Factsheet) -> Vec<(&mut f64, f64, f64)> {
        let p = &mut sheet.physical_parameters;
        let g = &mut sheet.mobile_robot_geometry;
        let w = &mut g.wheel_definitions.as_mut().unwrap()[0];
        let vertex = &mut g.envelopes2d.as_mut().unwrap()[0].vertices[0];
        let l = &mut sheet.load_specification.load_sets.as_mut().unwrap()[0];
        let b = l.bounding_box_reference.as_mut().unwrap();
        let d = l.load_dimensions.as_mut().unwrap();
        let c = sheet
            .mobile_robot_configuration
            .as_mut()
            .unwrap()
            .battery_charging
            .as_mut()
            .unwrap();
        vec![
            (
                &mut sheet.type_specification.maximum_load_mass,
                0.0,
                f64::MAX,
            ),
            (&mut p.minimum_speed, 0.0, f64::MAX),
            (&mut p.maximum_speed, 0.0, f64::MAX),
            (p.minimum_angular_speed.as_mut().unwrap(), 0.0, f64::MAX),
            (p.maximum_angular_speed.as_mut().unwrap(), 0.0, f64::MAX),
            (&mut p.maximum_acceleration, 0.0, f64::MAX),
            (&mut p.maximum_deceleration, f64::MIN, f64::MAX),
            (&mut p.minimum_height, f64::MIN, f64::MAX),
            (&mut p.maximum_height, f64::MIN, f64::MAX),
            (&mut p.width, f64::MIN, f64::MAX),
            (&mut p.length, f64::MIN, f64::MAX),
            (&mut w.position.x, f64::MIN, f64::MAX),
            (&mut w.position.y, f64::MIN, f64::MAX),
            (w.position.theta.as_mut().unwrap(), f64::MIN, f64::MAX),
            (&mut w.diameter, f64::MIN, f64::MAX),
            (&mut w.width, f64::MIN, f64::MAX),
            (w.center_displacement.as_mut().unwrap(), f64::MIN, f64::MAX),
            (&mut vertex.x, f64::MIN, f64::MAX),
            (&mut vertex.y, f64::MIN, f64::MAX),
            (&mut b.x, f64::MIN, f64::MAX),
            (&mut b.y, f64::MIN, f64::MAX),
            (&mut b.z, f64::MIN, f64::MAX),
            (b.theta.as_mut().unwrap(), f64::MIN, f64::MAX),
            (&mut d.length, 0.0, f64::MAX),
            (&mut d.width, 0.0, f64::MAX),
            (d.height.as_mut().unwrap(), 0.0, f64::MAX),
            (l.maximum_weight.as_mut().unwrap(), 0.0, f64::MAX),
            (
                l.minimum_loadhandling_height.as_mut().unwrap(),
                0.0,
                f64::MAX,
            ),
            (
                l.maximum_loadhandling_height.as_mut().unwrap(),
                0.0,
                f64::MAX,
            ),
            (
                l.minimum_loadhandling_depth.as_mut().unwrap(),
                f64::MIN,
                f64::MAX,
            ),
            (
                l.maximum_loadhandling_depth.as_mut().unwrap(),
                f64::MIN,
                f64::MAX,
            ),
            (
                l.minimum_loadhandling_tilt.as_mut().unwrap(),
                f64::MIN,
                f64::MAX,
            ),
            (
                l.maximum_loadhandling_tilt.as_mut().unwrap(),
                f64::MIN,
                f64::MAX,
            ),
            (l.maximum_speed.as_mut().unwrap(), 0.0, f64::MAX),
            (l.maximum_acceleration.as_mut().unwrap(), 0.0, f64::MAX),
            (l.maximum_deceleration.as_mut().unwrap(), f64::MIN, f64::MAX),
            (l.pick_time.as_mut().unwrap(), 0.0, f64::MAX),
            (l.drop_time.as_mut().unwrap(), 0.0, f64::MAX),
            (c.critical_low_charging_level.as_mut().unwrap(), 0.0, 100.0),
            (
                c.minimum_desired_charging_level.as_mut().unwrap(),
                0.0,
                100.0,
            ),
            (
                c.maximum_desired_charging_level.as_mut().unwrap(),
                0.0,
                100.0,
            ),
        ]
    }

    #[test]
    fn every_float64_field_checks_bounds_and_nonfinite_output() {
        let mut template: Factsheet = serde_json::from_str(FULL).unwrap();
        let count = float64s(&mut template).len();
        for index in 0..count {
            let (_, min, max) = float64s(&mut template).swap_remove(index);
            for valid in [min, max] {
                let mut sheet: Factsheet = serde_json::from_str(FULL).unwrap();
                *float64s(&mut sheet).swap_remove(index).0 = valid;
                let text = serde_json::to_string(&sheet).unwrap();
                assert_eq!(serde_json::from_str::<Factsheet>(&text).unwrap(), sheet);
            }
            for invalid in [
                min - 0.01,
                max + 0.01,
                f64::NAN,
                f64::INFINITY,
                f64::NEG_INFINITY,
            ] {
                if invalid.is_finite() && (min..=max).contains(&invalid) {
                    continue;
                }
                let mut sheet: Factsheet = serde_json::from_str(FULL).unwrap();
                *float64s(&mut sheet).swap_remove(index).0 = invalid;
                assert!(serde_json::to_string(&sheet).is_err(), "field {index}");
            }
        }
    }

    fn float32s(timing: &mut Timing) -> [&mut f32; 4] {
        [
            &mut timing.minimum_order_interval,
            &mut timing.minimum_state_interval,
            timing.default_state_interval.as_mut().unwrap(),
            timing.visualization_interval.as_mut().unwrap(),
        ]
    }

    #[test]
    fn timing_uses_finite_nonnegative_float32_in_both_directions() {
        for (index, field) in [
            "minimumOrderInterval",
            "minimumStateInterval",
            "defaultStateInterval",
            "visualizationInterval",
        ]
        .iter()
        .enumerate()
        {
            for valid in [0.0, f32::MAX, f32::MIN_POSITIVE, f32::from_bits(1), 0.1] {
                let mut sheet: Factsheet = serde_json::from_str(FULL).unwrap();
                *float32s(&mut sheet.protocol_limits.timing)[index] = valid;
                let text = serde_json::to_string(&sheet).unwrap();
                let mut decoded: Factsheet = serde_json::from_str(&text).unwrap();
                assert_eq!(
                    float32s(&mut decoded.protocol_limits.timing)[index].to_bits(),
                    valid.to_bits()
                );
            }
            for invalid in [-1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
                let mut sheet: Factsheet = serde_json::from_str(FULL).unwrap();
                *float32s(&mut sheet.protocol_limits.timing)[index] = invalid;
                assert!(
                    serde_json::to_string(&sheet)
                        .unwrap_err()
                        .to_string()
                        .contains(field)
                );
            }
            for invalid in [f64::MAX, f64::from(f32::MAX) * 1.000001, -1e-100] {
                let mut value = full();
                value["protocolLimits"]["timing"][field] = json!(invalid);
                if invalid >= 0.0 {
                    assert!(support::schema("factsheet").is_valid(&value));
                }
                assert!(serde_json::from_value::<Factsheet>(value).is_err());
            }
        }
    }

    #[test]
    fn header_timestamp_and_shared_load_paths_preserve_existing_behavior() {
        use chrono::Timelike;
        let mut value = full();
        value["timestamp"] = json!("2017-04-15T11:40:03Z");
        assert!(serde_json::from_value::<Factsheet>(value).is_err());
        let mut sheet: Factsheet = serde_json::from_str(FULL).unwrap();
        sheet.header.timestamp = sheet.header.timestamp.with_nanosecond(1).unwrap();
        assert!(serde_json::to_string(&sheet).is_err());
        fn same_type(
            value: vda5050_v3_types::state::LoadDimensions,
        ) -> vda5050_v3_types::common::LoadDimensions {
            value
        }
        let dimensions = sheet
            .load_specification
            .load_sets
            .unwrap()
            .remove(0)
            .load_dimensions
            .unwrap();
        assert_eq!(same_type(dimensions), dimensions);
    }

    #[test]
    fn semantic_relationships_defaults_and_external_data_are_not_validated() {
        let mut value = full();
        value["physicalParameters"]["minimumSpeed"] = json!(10.0);
        value["physicalParameters"]["maximumSpeed"] = json!(1.0);
        value["protocolLimits"]["maximumStringLengths"]["maximumIdLength"] = json!(0);
        value["protocolLimits"]["maximumArrayLengths"]["order.nodes"] = json!(0);
        value["mobileRobotConfiguration"]["batteryCharging"]["minimumDesiredChargingLevel"] =
            json!(90.0);
        value["mobileRobotConfiguration"]["batteryCharging"]["maximumDesiredChargingLevel"] =
            json!(10.0);
        value["mobileRobotConfiguration"]["network"]["localIpAddress"] = json!("not an IP address");
        value["mobileRobotGeometry"]["envelopes3d"][0]["url"] = json!("not a URL");
        value["mobileRobotGeometry"]["envelopes2d"][0]["vertices"] = json!([]);
        value["protocolFeatures"]["mobileRobotActions"][0]["actionScopes"] = json!(["INSTANT"]);
        value["protocolFeatures"]["mobileRobotActions"][0]["blockingTypes"] = json!(["HARD"]);
        round_trip(value);
    }
}

#[cfg(feature = "json")]
#[test]
fn json_helpers_round_trip_text_bytes_and_original_errors() {
    let sheet = Factsheet::from_json(FULL).unwrap();
    assert_eq!(Factsheet::from_json_slice(FULL.as_bytes()).unwrap(), sheet);
    assert_eq!(
        Factsheet::from_json(&sheet.to_json().unwrap()).unwrap(),
        sheet
    );
    assert_eq!(
        Factsheet::from_json_slice(&sheet.to_json_vec().unwrap()).unwrap(),
        sheet
    );
    assert_eq!(
        Factsheet::from_json("{").unwrap_err().to_string(),
        serde_json::from_str::<Factsheet>("{")
            .unwrap_err()
            .to_string()
    );
    assert!(Factsheet::from_json_slice(&[0xff]).is_err());
    let mut invalid = sheet;
    invalid.protocol_limits.timing.minimum_order_interval = f32::NAN;
    assert!(invalid.to_json().is_err());
    assert!(invalid.to_json_vec().is_err());
}
