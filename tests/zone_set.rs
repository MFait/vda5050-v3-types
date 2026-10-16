//! Pinned VDA5050/VDA5050@0b2ae43154bcd1ab2be4e2445df8b406dcad77b2:
//! VDA5050_EN.md sections 6.4, 7.2, 7.6 and unchanged zoneSet.schema.
//! https://github.com/VDA5050/VDA5050/blob/0b2ae43154bcd1ab2be4e2445df8b406dcad77b2/VDA5050_EN.md

use chrono::{TimeZone, Utc};
use serde_json::Value;
use vda5050_v3_types::{
    ZoneSet,
    common::Header,
    zone_set::{Vertex, Zone, ZoneKind, ZoneSetDefinition},
};

mod support;

const FULL: &str = include_str!("fixtures/zone-set.json");
const MINIMAL: &str = include_str!("fixtures/zone-set-minimal.json");

#[cfg(feature = "serde")]
fn body() -> Value {
    serde_json::from_str(FULL).unwrap()
}

#[test]
fn public_types_work_without_features_and_vertex_paths_are_identical() {
    let vertex: vda5050_v3_types::factsheet::Vertex = Vertex { x: 0.0, y: 0.0 };
    let common: vda5050_v3_types::common::Vertex = vertex;
    let message = ZoneSet {
        header: Header {
            header_id: 0,
            timestamp: Utc.with_ymd_and_hms(2017, 4, 15, 11, 40, 3).unwrap(),
            version: "3.0.0".into(),
            manufacturer: "Example".into(),
            serial_number: "robot-1".into(),
        },
        zone_set: ZoneSetDefinition {
            map_id: "map-1".into(),
            zone_set_id: "zones-1".into(),
            zone_set_descriptor: None,
            zones: vec![Zone {
                zone_id: "blocked".into(),
                zone_descriptor: None,
                vertices: vec![common; 3],
                kind: ZoneKind::Blocked,
            }],
        },
    };
    assert_eq!(message.clone(), message);
}

#[test]
fn fixtures_satisfy_original_pinned_schema() {
    for fixture in [FULL, MINIMAL] {
        support::schema("zoneSet")
            .validate(&serde_json::from_str::<Value>(fixture).unwrap())
            .unwrap();
    }
}

#[cfg(feature = "serde")]
mod wire {
    use super::*;
    use serde_json::json;
    use vda5050_v3_types::{
        common::BlockingType,
        zone_set::{BidirectedLimitation, DirectedLimitation, ReleaseLossBehavior, ZoneAction},
    };

    fn round_trip(value: Value) -> ZoneSet {
        support::schema("zoneSet").validate(&value).unwrap();
        let decoded: ZoneSet = serde_json::from_str(&value.to_string()).unwrap();
        let encoded: Value =
            serde_json::from_slice(&serde_json::to_vec(&decoded).unwrap()).unwrap();
        assert_eq!(encoded, value);
        support::schema("zoneSet").validate(&encoded).unwrap();
        decoded
    }

    #[test]
    fn full_fixture_covers_every_declared_schema_property_and_conditional_branch() {
        let schema: Value = serde_json::from_str(include_str!("schemas/zoneSet.schema")).unwrap();
        let value = body();
        for (definition, sample) in [
            (&schema, &value),
            (&schema["definitions"]["zoneSet"], &value["zoneSet"]),
            (
                &schema["definitions"]["vertex"],
                &value["zoneSet"]["zones"][0]["vertices"][0],
            ),
            (
                &schema["definitions"]["zoneAction"],
                &value["zoneSet"]["zones"][5]["entryActions"][0],
            ),
            (
                &schema["definitions"]["actionParameter"],
                &value["zoneSet"]["zones"][5]["entryActions"][0]["actionParameters"][0],
            ),
        ] {
            for key in definition["properties"].as_object().unwrap().keys() {
                assert!(sample.get(key).is_some(), "{key}");
            }
        }
        let zones = value["zoneSet"]["zones"].as_array().unwrap();
        let definition = &schema["definitions"]["zone"];
        for key in definition["properties"].as_object().unwrap().keys() {
            assert!(zones.iter().any(|zone| zone.get(key).is_some()), "{key}");
        }
        for name in definition["properties"]["zoneType"]["enum"]
            .as_array()
            .unwrap()
        {
            assert!(zones.iter().any(|zone| zone["zoneType"] == *name), "{name}");
        }
        for branch in definition["allOf"].as_array().unwrap() {
            let name = &branch["if"]["properties"]["zoneType"]["const"];
            let sample = zones.iter().find(|zone| zone["zoneType"] == *name).unwrap();
            for key in branch["then"]["properties"].as_object().unwrap().keys() {
                assert!(sample.get(key).is_some(), "{name}/{key}");
            }
        }
    }

    #[test]
    fn all_ten_variants_exact_names_flat_header_and_minimal_body_round_trip() {
        let decoded = round_trip(body());
        assert!(matches!(decoded.zone_set.zones[0].kind, ZoneKind::Blocked));
        assert!(matches!(
            decoded.zone_set.zones[1].kind,
            ZoneKind::LineGuided
        ));
        assert!(matches!(
            decoded.zone_set.zones[2].kind,
            ZoneKind::Release { .. }
        ));
        assert!(matches!(
            decoded.zone_set.zones[3].kind,
            ZoneKind::CoordinatedReplanning
        ));
        assert!(matches!(
            decoded.zone_set.zones[4].kind,
            ZoneKind::SpeedLimit { .. }
        ));
        assert!(matches!(
            decoded.zone_set.zones[5].kind,
            ZoneKind::Action { .. }
        ));
        assert!(matches!(
            decoded.zone_set.zones[6].kind,
            ZoneKind::Priority { .. }
        ));
        assert!(matches!(
            decoded.zone_set.zones[7].kind,
            ZoneKind::Penalty { .. }
        ));
        assert!(matches!(
            decoded.zone_set.zones[8].kind,
            ZoneKind::Directed { .. }
        ));
        assert!(matches!(
            decoded.zone_set.zones[9].kind,
            ZoneKind::Bidirected { .. }
        ));
        let encoded = serde_json::to_value(decoded).unwrap();
        assert!(encoded.get("header").is_none());
        assert!(encoded["zoneSet"]["zones"][4].get("kind").is_none());
        assert!(
            round_trip(serde_json::from_str(MINIMAL).unwrap())
                .zone_set
                .zones
                .is_empty()
        );
    }

    #[test]
    fn all_required_and_conditional_fields_reject_absence_null_and_wrong_types() {
        support::assert_required::<ZoneSet>(
            &body(),
            support::schema("zoneSet"),
            &[
                "/headerId",
                "/timestamp",
                "/version",
                "/manufacturer",
                "/serialNumber",
                "/zoneSet",
                "/zoneSet/mapId",
                "/zoneSet/zoneSetId",
                "/zoneSet/zones",
                "/zoneSet/zones/0/zoneId",
                "/zoneSet/zones/0/zoneType",
                "/zoneSet/zones/0/vertices",
                "/zoneSet/zones/0/vertices/0/x",
                "/zoneSet/zones/0/vertices/0/y",
                "/zoneSet/zones/2/releaseLossBehavior",
                "/zoneSet/zones/4/maximumSpeed",
                "/zoneSet/zones/5/entryActions",
                "/zoneSet/zones/5/duringActions",
                "/zoneSet/zones/5/exitActions",
                "/zoneSet/zones/5/entryActions/0/actionType",
                "/zoneSet/zones/5/entryActions/0/blockingType",
                "/zoneSet/zones/5/entryActions/0/actionParameters/0/key",
                "/zoneSet/zones/6/priorityFactor",
                "/zoneSet/zones/7/penaltyFactor",
                "/zoneSet/zones/8/direction",
                "/zoneSet/zones/8/directedLimitation",
                "/zoneSet/zones/9/direction",
                "/zoneSet/zones/9/bidirectedLimitation",
            ],
        );
    }

    #[test]
    fn every_optional_field_preserves_absence_and_rejects_null() {
        support::assert_optional::<ZoneSet>(
            &body(),
            support::schema("zoneSet"),
            &[
                "/zoneSet/zoneSetDescriptor",
                "/zoneSet/zones/0/zoneDescriptor",
                "/zoneSet/zones/5/entryActions/0/actionDescriptor",
                "/zoneSet/zones/5/entryActions/0/actionParameters",
                "/zoneSet/zones/5/entryActions/0/retriable",
            ],
        );
        for (pointer, invalid) in [
            ("/zoneSet/zoneSetDescriptor", json!(false)),
            ("/zoneSet/zones/0/zoneDescriptor", json!([])),
            ("/zoneSet/zones/5/entryActions/0/actionDescriptor", json!(0)),
            (
                "/zoneSet/zones/5/entryActions/0/actionParameters",
                json!({}),
            ),
            ("/zoneSet/zones/5/entryActions/0/retriable", json!("true")),
        ] {
            let mut value = body();
            *value.pointer_mut(pointer).unwrap() = invalid;
            assert!(!support::schema("zoneSet").is_valid(&value));
            assert!(
                serde_json::from_value::<ZoneSet>(value).is_err(),
                "{pointer}"
            );
        }
    }

    #[test]
    fn objects_are_map_only_standalone_and_nested() {
        for invalid in [json!([]), json!([0, 0]), json!(null), json!(false)] {
            assert!(serde_json::from_value::<ZoneSet>(invalid.clone()).is_err());
            assert!(serde_json::from_value::<ZoneSetDefinition>(invalid.clone()).is_err());
            assert!(serde_json::from_value::<Zone>(invalid.clone()).is_err());
            assert!(serde_json::from_value::<ZoneKind>(invalid.clone()).is_err());
            assert!(serde_json::from_value::<ZoneAction>(invalid.clone()).is_err());
            assert!(serde_json::from_value::<Vertex>(invalid.clone()).is_err());
            for pointer in [
                "/zoneSet",
                "/zoneSet/zones/0",
                "/zoneSet/zones/0/vertices/0",
                "/zoneSet/zones/5/entryActions/0",
                "/zoneSet/zones/5/entryActions/0/actionParameters/0",
            ] {
                let mut value = body();
                *value.pointer_mut(pointer).unwrap() = invalid.clone();
                assert!(
                    serde_json::from_value::<ZoneSet>(value).is_err(),
                    "{pointer}"
                );
            }
        }
    }

    #[test]
    fn duplicate_fields_are_rejected_including_flattened_variant_parameters() {
        for (index, key) in [
            (0, "zoneId"),
            (0, "zoneDescriptor"),
            (0, "vertices"),
            (0, "zoneType"),
            (2, "releaseLossBehavior"),
            (4, "maximumSpeed"),
            (5, "entryActions"),
            (5, "duringActions"),
            (5, "exitActions"),
            (6, "priorityFactor"),
            (7, "penaltyFactor"),
            (8, "direction"),
            (8, "directedLimitation"),
            (9, "bidirectedLimitation"),
        ] {
            let zone = &body()["zoneSet"]["zones"][index];
            let text = serde_json::to_string(zone).unwrap();
            let duplicate = format!("{{\"{key}\":{},{}", zone[key], &text[1..]);
            assert!(serde_json::from_str::<Zone>(&duplicate).is_err(), "{key}");
        }
        for pointer in [
            "/zoneSet",
            "/zoneSet/zones/5/entryActions/0",
            "/zoneSet/zones/0/vertices/0",
        ] {
            let value = body().pointer(pointer).unwrap().clone();
            let text = value.to_string();
            for (key, entry) in value.as_object().unwrap() {
                let duplicate = format!("{{\"{key}\":{entry},{}", &text[1..]);
                let result = match pointer {
                    "/zoneSet" => serde_json::from_str::<ZoneSetDefinition>(&duplicate).map(|_| ()),
                    "/zoneSet/zones/0/vertices/0" => {
                        serde_json::from_str::<Vertex>(&duplicate).map(|_| ())
                    }
                    _ => serde_json::from_str::<ZoneAction>(&duplicate).map(|_| ()),
                };
                assert!(result.is_err(), "{pointer}/{key}");
            }
        }
        for key in [
            "headerId",
            "timestamp",
            "version",
            "manufacturer",
            "serialNumber",
            "zoneSet",
        ] {
            let duplicate = format!("{{\"{key}\":{},{}", body()[key], &FULL.trim()[1..]);
            assert!(
                serde_json::from_str::<ZoneSet>(&duplicate).is_err(),
                "{key}"
            );
        }
    }

    #[test]
    fn closed_enums_accept_all_values_and_reject_extensions() {
        for (index, key, variants) in [
            (
                2,
                "releaseLossBehavior",
                &["STOP", "CONTINUE", "EVACUATE"][..],
            ),
            (
                8,
                "directedLimitation",
                &["SOFT", "RESTRICTED", "STRICT"][..],
            ),
            (9, "bidirectedLimitation", &["SOFT", "RESTRICTED"][..]),
        ] {
            for variant in variants {
                let mut value = body();
                value["zoneSet"]["zones"][index][key] = json!(variant);
                round_trip(value);
            }
            for invalid in ["UNKNOWN", "", "soft"] {
                let mut value = body();
                value["zoneSet"]["zones"][index][key] = json!(invalid);
                assert!(!support::schema("zoneSet").is_valid(&value));
                assert!(serde_json::from_value::<ZoneSet>(value).is_err());
            }
        }
        for (name, expected) in [
            ("STOP", ReleaseLossBehavior::Stop),
            ("CONTINUE", ReleaseLossBehavior::Continue),
            ("EVACUATE", ReleaseLossBehavior::Evacuate),
        ] {
            assert_eq!(
                serde_json::from_value::<ReleaseLossBehavior>(json!(name)).unwrap(),
                expected
            );
        }
        assert_eq!(
            serde_json::from_value::<DirectedLimitation>(json!("STRICT")).unwrap(),
            DirectedLimitation::Strict
        );
        assert_eq!(
            serde_json::from_value::<BidirectedLimitation>(json!("SOFT")).unwrap(),
            BidirectedLimitation::Soft
        );
        assert!(serde_json::from_value::<BidirectedLimitation>(json!("STRICT")).is_err());
        for invalid in ["UNKNOWN", "", "blocked"] {
            let mut value = body();
            value["zoneSet"]["zones"][0]["zoneType"] = json!(invalid);
            assert!(!support::schema("zoneSet").is_valid(&value));
            assert!(serde_json::from_value::<ZoneSet>(value).is_err());
        }
        for blocking in ["NONE", "SINGLE", "SOFT", "HARD"] {
            let mut value = body();
            value["zoneSet"]["zones"][5]["entryActions"][0]["blockingType"] = json!(blocking);
            round_trip(value);
        }
    }

    #[test]
    fn polygons_require_three_vertices_in_both_directions_but_not_geometry() {
        for count in 0..=4 {
            let mut value = body();
            value["zoneSet"]["zones"][0]["vertices"] =
                json!(vec![json!({"x": 0.0, "y": 0.0}); count]);
            assert_eq!(support::schema("zoneSet").is_valid(&value), count >= 3);
            assert_eq!(serde_json::from_value::<ZoneSet>(value).is_ok(), count >= 3);
            let mut decoded: ZoneSet = serde_json::from_str(FULL).unwrap();
            decoded.zone_set.zones[0].vertices = vec![Vertex { x: 0.0, y: 0.0 }; count];
            assert_eq!(serde_json::to_string(&decoded).is_ok(), count >= 3);
        }
        // Degenerate, clockwise and self-intersecting polygons are representable.
        for vertices in [
            json!([{"x":0.0,"y":0.0},{"x":0.0,"y":1.0},{"x":1.0,"y":0.0}]),
            json!([{"x":0.0,"y":0.0},{"x":1.0,"y":1.0},{"x":0.0,"y":1.0},{"x":1.0,"y":0.0}]),
        ] {
            let mut value = body();
            value["zoneSet"]["zones"][0]["vertices"] = vertices;
            round_trip(value);
        }
    }

    #[test]
    fn finite_numeric_bounds_and_text_byte_precision_are_checked_both_ways() {
        for (index, field, bounded) in [
            (4, "maximumSpeed", false),
            (6, "priorityFactor", true),
            (7, "penaltyFactor", true),
            (8, "direction", false),
            (9, "direction", false),
        ] {
            for number in [
                f64::MIN,
                -0.01,
                -0.0,
                0.0,
                0.1,
                1.0,
                1.01,
                f64::MAX,
                8.971135692166839e-150,
                f64::from_bits(1),
            ] {
                let valid = !bounded || (0.0..=1.0).contains(&number);
                let mut value = body();
                value["zoneSet"]["zones"][index][field] = json!(number);
                assert_eq!(support::schema("zoneSet").is_valid(&value), valid);
                assert_eq!(serde_json::from_value::<ZoneSet>(value).is_ok(), valid);
                let mut message: ZoneSet = serde_json::from_str(FULL).unwrap();
                let slot = match &mut message.zone_set.zones[index].kind {
                    ZoneKind::SpeedLimit { maximum_speed } => maximum_speed,
                    ZoneKind::Priority { priority_factor } => priority_factor,
                    ZoneKind::Penalty { penalty_factor } => penalty_factor,
                    ZoneKind::Directed { direction, .. }
                    | ZoneKind::Bidirected { direction, .. } => direction,
                    _ => unreachable!(),
                };
                *slot = number;
                assert_eq!(serde_json::to_string(&message).is_ok(), valid);
                if valid {
                    for decoded in [
                        serde_json::from_str::<ZoneSet>(&serde_json::to_string(&message).unwrap())
                            .unwrap(),
                        serde_json::from_slice::<ZoneSet>(&serde_json::to_vec(&message).unwrap())
                            .unwrap(),
                    ] {
                        let decoded = serde_json::to_value(decoded).unwrap();
                        assert_eq!(
                            decoded["zoneSet"]["zones"][index][field]
                                .as_f64()
                                .unwrap()
                                .to_bits(),
                            number.to_bits()
                        );
                    }
                }
            }
            for nonfinite in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut message: ZoneSet = serde_json::from_str(FULL).unwrap();
                message.zone_set.zones[index].kind = match field {
                    "maximumSpeed" => ZoneKind::SpeedLimit {
                        maximum_speed: nonfinite,
                    },
                    "priorityFactor" => ZoneKind::Priority {
                        priority_factor: nonfinite,
                    },
                    "penaltyFactor" => ZoneKind::Penalty {
                        penalty_factor: nonfinite,
                    },
                    _ => ZoneKind::Directed {
                        direction: nonfinite,
                        directed_limitation: DirectedLimitation::Soft,
                    },
                };
                assert!(serde_json::to_string(&message).is_err());
            }
        }
        for axis in ["x", "y"] {
            for number in [f64::MIN, f64::MAX, -0.0, 8.971135692166839e-150] {
                let mut value = body();
                value["zoneSet"]["zones"][0]["vertices"][0][axis] = json!(number);
                round_trip(value);
            }
            for number in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let mut message: ZoneSet = serde_json::from_str(FULL).unwrap();
                let point = &mut message.zone_set.zones[0].vertices[0];
                if axis == "x" {
                    point.x = number;
                } else {
                    point.y = number;
                }
                assert!(serde_json::to_vec(&message).is_err());
            }
        }
    }

    #[test]
    fn shared_parameters_and_unknown_fields_preserve_the_existing_contract() {
        let pointer = "/zoneSet/zones/5/entryActions/0/actionParameters/0/value";
        let mut missing = body();
        support::remove(&mut missing, pointer);
        let mut null = body();
        *null.pointer_mut(pointer).unwrap() = Value::Null;
        for invalid in [missing, null] {
            assert!(!support::schema("zoneSet").is_valid(&invalid));
            assert!(serde_json::from_value::<ZoneSet>(invalid).is_err());
        }
        for parameter in [
            json!([]),
            json!({}),
            json!("value"),
            json!(true),
            json!(42),
            json!(0.5),
            json!({"nested": [null, {"vendor": true}]}),
        ] {
            let mut value = body();
            value["zoneSet"]["zones"][5]["entryActions"][0]["actionParameters"][0]["value"] =
                parameter;
            round_trip(value);
        }
        let expected = body();
        let mut value = expected.clone();
        for pointer in [
            "",
            "/zoneSet",
            "/zoneSet/zones/0",
            "/zoneSet/zones/4",
            "/zoneSet/zones/0/vertices/0",
            "/zoneSet/zones/5/entryActions/0",
            "/zoneSet/zones/5/entryActions/0/actionParameters/0",
        ] {
            value.pointer_mut(pointer).unwrap()["vendor"] = json!(null);
        }
        // A zone action has no sender-supplied actionId.
        value["zoneSet"]["zones"][5]["entryActions"][0]["actionId"] = json!("ignored");
        // Irrelevant variant fields are unknown, not interpreted.
        value["zoneSet"]["zones"][0]["maximumSpeed"] = json!(null);
        let decoded: ZoneSet = serde_json::from_value(value).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), expected);
        let action: ZoneAction =
            serde_json::from_value(json!({"actionType":"vendor", "blockingType":"NONE"})).unwrap();
        assert_eq!(action.blocking_type, BlockingType::None);
        assert_eq!(action.retriable, None);
    }

    #[test]
    fn shared_header_width_timestamp_and_unvalidated_semantic_boundaries() {
        for number in [json!(0), json!(u32::MAX)] {
            let mut value = body();
            value["headerId"] = number;
            round_trip(value);
        }
        for number in [
            json!(-1),
            json!(u64::from(u32::MAX) + 1),
            json!(1.5),
            json!(1.0),
        ] {
            let mut value = body();
            value["headerId"] = number;
            assert!(serde_json::from_value::<ZoneSet>(value).is_err());
        }
        for timestamp in [
            "2017-04-15T11:40:03Z",
            "2017-04-15T11:40:03.123+00:00",
            "2017-02-30T11:40:03.123Z",
        ] {
            let mut value = body();
            value["timestamp"] = json!(timestamp);
            assert!(serde_json::from_value::<ZoneSet>(value).is_err());
        }
        let mut value = body();
        value["zoneSet"]["zones"][1]["zoneId"] = value["zoneSet"]["zones"][0]["zoneId"].clone();
        value["zoneSet"]["mapId"] = json!("");
        round_trip(value); // Uniqueness, map existence/bounds and activation are not checked.
    }
}

#[cfg(feature = "json")]
#[test]
fn all_json_helpers_round_trip_and_preserve_errors() {
    let message = ZoneSet::from_json(FULL).unwrap();
    assert_eq!(
        ZoneSet::from_json(&message.to_json().unwrap()).unwrap(),
        message
    );
    assert_eq!(
        ZoneSet::from_json_slice(&message.to_json_vec().unwrap()).unwrap(),
        message
    );
    assert!(ZoneSet::from_json_slice(b"\xff").is_err());
    let error = ZoneSet::from_json("{").unwrap_err();
    let original = serde_json::from_str::<ZoneSet>("{").unwrap_err();
    assert_eq!(error.classify(), original.classify());
    assert_eq!(error.to_string(), original.to_string());
    let mut invalid = message;
    invalid.zone_set.zones[0].vertices.clear();
    assert!(invalid.to_json().is_err());
    assert!(invalid.to_json_vec().is_err());
}
