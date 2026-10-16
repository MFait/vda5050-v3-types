//! VDA5050/VDA5050@0b2ae43154bcd1ab2be4e2445df8b406dcad77b2
//! VDA5050_EN.md sections 4.2-4.3 supply the suggested layout and exact names.
//! Wildcard selectors are MQTT filter syntax, not additional VDA topic names.

#![cfg(feature = "topic")]

use vda5050_v3_types::topic::{Topic, TopicError, TopicFilter, TopicKind, TopicLevel};

#[test]
fn all_selector_combinations_round_trip_without_serde() {
    for manufacturer in [None, Some("KIT")] {
        for serial in [None, Some("0001")] {
            for kind in [None, Some(TopicKind::State)] {
                let filter = TopicFilter::new("custom", manufacturer, serial, kind).unwrap();
                let text = format!(
                    "custom/v3/{}/{}/{}",
                    manufacturer.unwrap_or("+"),
                    serial.unwrap_or("+"),
                    kind.map_or("+", TopicKind::as_str)
                );
                assert_eq!(filter.to_string(), text);
                assert_eq!(text.parse::<TopicFilter>().unwrap(), filter);
                assert_eq!(TopicFilter::try_from(text.clone()).unwrap(), filter);
                assert_eq!(String::from(filter.clone()), text);
                assert_eq!(filter.interface_name(), "custom");
                assert_eq!(filter.major_version(), "v3");
                assert_eq!(filter.manufacturer(), manufacturer);
                assert_eq!(filter.serial_number(), serial);
                assert_eq!(filter.kind(), kind);
            }
        }
    }
}

#[test]
fn exact_topics_convert_to_filters_for_all_kinds_without_weakening_publish_checks() {
    for name in [
        "order",
        "instantActions",
        "state",
        "visualization",
        "connection",
        "factsheet",
        "zoneSet",
        "responses",
    ] {
        let text = format!("vda5050/v3/KIT/0001/{name}");
        let topic: Topic = text.parse().unwrap();
        assert_eq!(TopicFilter::from(&topic).to_string(), text);
        assert_eq!(
            text.parse::<TopicFilter>().unwrap().kind(),
            Some(topic.kind())
        );
    }
    assert!("vda5050/v3/+/+/state".parse::<Topic>().is_err());
    assert!("vda5050/v3/KIT/0001/+".parse::<Topic>().is_err());
}

#[test]
fn parser_rejects_wrong_depth_versions_wildcard_scope_and_unknown_kinds() {
    for invalid in [
        "",
        "state",
        "vda5050/v3/KIT/0001",
        "vda5050/v3/KIT/0001/state/extra",
        "$share/group/vda5050/v3/+/+/state",
        "+/v3/+/+/state",
        "vda5050/+/+/+/state",
        "vda5050/v2/+/+/state",
        "vda5050/v03/+/+/state",
        "vda5050/v3/#/+/state",
        "vda5050/v3/+/+/#",
        "vda5050/v3/KIT/0001/#",
        "vda5050/v3/K+/+/state",
        "vda5050/v3/+/robot+/state",
        "vda5050/v3/+/+/sta+",
        "/v3/+/+/state",
        "vda5050/v3//+/state",
        "vda5050/v3/+//state",
        "vda5050/v3/+/+/",
        "vda5050/v3/+/+/STATE",
        "vda5050/v3/+/+/unknown",
        "vda5050/v3/+/+/state ",
        "vda5050/v3/KIT/robot 1/state",
        "vda5050/v3/KIT/\u{00e9}/state",
    ] {
        assert!(invalid.parse::<TopicFilter>().is_err(), "{invalid:?}");
    }
}

#[test]
fn exact_selectors_reuse_topic_field_rules_and_preserve_text() {
    for (index, level) in [
        (0, TopicLevel::InterfaceName),
        (1, TopicLevel::Manufacturer),
        (2, TopicLevel::SerialNumber),
    ] {
        for invalid in ["", "+", "#", "a+b", "a/b", "$SYS", "a\0b"] {
            let mut fields = ["custom", "KIT", "0001"];
            fields[index] = invalid;
            let expected =
                Topic::new(fields[0], fields[1], fields[2], TopicKind::State).unwrap_err();
            assert_eq!(
                TopicFilter::new(
                    fields[0],
                    Some(fields[1]),
                    Some(fields[2]),
                    Some(TopicKind::State)
                ),
                Err(expected)
            );
        }
        let mut fields = ["custom", "KIT", "0001"];
        fields[index] = "";
        assert_eq!(
            TopicFilter::new(fields[0], Some(fields[1]), Some(fields[2]), None),
            Err(TopicError::EmptyLevel(level))
        );
    }
    let filter =
        TopicFilter::new(" custom ", Some("M\u{00fc}ller "), Some("Ab_0.:-"), None).unwrap();
    assert_eq!(filter.to_string(), " custom /v3/M\u{00fc}ller /Ab_0.:-/+");
    assert_eq!(filter.to_string().parse::<TopicFilter>().unwrap(), filter);
}

#[test]
fn filter_limit_counts_utf8_bytes_at_the_exact_boundary() {
    // /v3/+/+/+ occupies nine bytes.
    for bytes in [65_525, 65_526, 65_527] {
        let interface = "a".repeat(bytes);
        let text = format!("{interface}/v3/+/+/+");
        assert_eq!(text.len(), bytes + 9);
        assert_eq!(
            TopicFilter::new(&interface, None, None, None).is_ok(),
            bytes <= 65_526
        );
        assert_eq!(text.parse::<TopicFilter>().is_ok(), bytes <= 65_526);
    }
    let interface = "\u{00e9}".repeat(32_763);
    let filter = TopicFilter::new(&interface, None, None, None).unwrap();
    assert_eq!(filter.to_string().len(), 65_535);
    assert_eq!(filter.to_string().parse::<TopicFilter>().unwrap(), filter);
    assert_eq!(
        TopicFilter::new(format!("{interface}a"), None, None, None),
        Err(TopicError::TooLong)
    );
}

#[cfg(feature = "serde")]
#[test]
fn serde_uses_checked_strings_with_no_object_or_wildcard_bypass() {
    for text in [
        "vda5050/v3/+/+/state",
        "custom/v3/KIT/0001/+",
        "custom/v3/+/+/+",
        "custom/v3/KIT/0001/order",
    ] {
        let filter: TopicFilter = text.parse().unwrap();
        assert_eq!(
            serde_json::to_value(&filter).unwrap(),
            serde_json::json!(text)
        );
        assert_eq!(
            serde_json::from_str::<TopicFilter>(&serde_json::to_string(&filter).unwrap()).unwrap(),
            filter
        );
        assert_eq!(
            serde_json::from_slice::<TopicFilter>(&serde_json::to_vec(&filter).unwrap()).unwrap(),
            filter
        );
    }
    for invalid in [
        serde_json::json!(null),
        serde_json::json!({}),
        serde_json::json!([]),
        serde_json::json!(42),
        serde_json::json!(true),
        serde_json::json!("vda5050/v3/#"),
        serde_json::json!("+/v3/+/+/state"),
        serde_json::json!("vda5050/v3/+/+/STATE"),
    ] {
        assert!(serde_json::from_value::<TopicFilter>(invalid).is_err());
    }
}
