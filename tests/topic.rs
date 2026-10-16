//! VDA5050/VDA5050@0b2ae43154bcd1ab2be4e2445df8b406dcad77b2:
//! https://github.com/VDA5050/VDA5050/blob/0b2ae43154bcd1ab2be4e2445df8b406dcad77b2/VDA5050_EN.md#43-topics-for-communication
//! Section 4.3 fixes the names, while section 4.2 allows hierarchy adaptations.

#![cfg(feature = "topic")]

use vda5050_v3_types::topic::{HeaderTopicError, Message, Topic, TopicError, TopicLevel};
use vda5050_v3_types::topic::{TopicKind, UnknownTopicKind};

const NAMES: [(TopicKind, &str); 8] = [
    (TopicKind::Order, "order"),
    (TopicKind::InstantActions, "instantActions"),
    (TopicKind::State, "state"),
    (TopicKind::Visualization, "visualization"),
    (TopicKind::Connection, "connection"),
    (TopicKind::Factsheet, "factsheet"),
    (TopicKind::ZoneSet, "zoneSet"),
    (TopicKind::Responses, "responses"),
];

#[test]
fn every_body_exposes_a_mapping_without_serde() {
    use vda5050_v3_types::{
        Connection, Factsheet, InstantActions, Order, Responses, State, Visualization, ZoneSet,
    };

    let _: fn(&Order) -> TopicKind = Order::topic_kind;
    let _: fn(&InstantActions) -> TopicKind = InstantActions::topic_kind;
    let _: fn(&State) -> TopicKind = State::topic_kind;
    let _: fn(&Visualization) -> TopicKind = Visualization::topic_kind;
    let _: fn(&Connection) -> TopicKind = Connection::topic_kind;
    let _: fn(&Factsheet) -> TopicKind = Factsheet::topic_kind;
    let _: fn(&ZoneSet) -> TopicKind = ZoneSet::topic_kind;
    let _: fn(&Responses) -> TopicKind = Responses::topic_kind;

    let message = Connection {
        header: vda5050_v3_types::common::Header {
            header_id: 0,
            timestamp: chrono::DateTime::UNIX_EPOCH,
            version: "not a version".into(),
            manufacturer: "not/a/topic/level".into(),
            serial_number: String::new(),
        },
        connection_state: vda5050_v3_types::connection::ConnectionState::Online,
    };
    assert_eq!(message.topic_kind(), TopicKind::Connection);
}

#[cfg(feature = "serde")]
#[test]
fn all_eight_fixture_bodies_map_to_their_exact_topic_names() {
    use vda5050_v3_types::{
        Connection, Factsheet, InstantActions, Order, Responses, State, Visualization, ZoneSet,
    };

    macro_rules! check {
        ($message:ty, $fixture:literal, $expected:ident, $name:literal) => {
            let message: $message = serde_json::from_str(include_str!($fixture)).unwrap();
            let original = serde_json::to_value(&message).unwrap();
            assert_eq!(message.topic_kind(), TopicKind::$expected);
            assert_eq!(message.topic_kind().as_str(), $name);
            let topic = message.topic("custom-interface").unwrap();
            assert_eq!(topic.kind(), TopicKind::$expected);
            assert_eq!(topic.interface_name(), "custom-interface");
            assert_eq!(topic.major_version(), "v3");
            assert_eq!(topic.manufacturer(), message.header.manufacturer);
            assert_eq!(topic.serial_number(), message.header.serial_number);
            assert_eq!(
                topic.to_string(),
                format!(
                    "custom-interface/v3/{}/{}/{}",
                    message.header.manufacturer, message.header.serial_number, $name
                )
            );
            assert_eq!(topic.to_string().parse::<Topic>().unwrap(), topic);
            assert_eq!(serde_json::to_value(&message).unwrap(), original);
            let wrapped = Message::from(message.clone());
            assert_eq!(wrapped.topic_kind(), TopicKind::$expected);
            assert_eq!(wrapped.header(), &message.header);
            assert_eq!(wrapped.topic("custom-interface").unwrap(), topic);
            assert_eq!(Message::from(Box::new(message.clone())), wrapped);
            assert_eq!(wrapped.clone(), wrapped);
            let Message::$expected(inner) = wrapped else {
                panic!("incorrect message variant");
            };
            assert_eq!(*inner, message);
            assert_eq!(serde_json::to_value(&inner).unwrap(), original);
        };
    }
    check!(Order, "fixtures/order.json", Order, "order");
    check!(
        InstantActions,
        "fixtures/instant-actions.json",
        InstantActions,
        "instantActions"
    );
    check!(State, "fixtures/state.json", State, "state");
    check!(
        Visualization,
        "fixtures/visualization.json",
        Visualization,
        "visualization"
    );
    check!(
        Connection,
        "fixtures/connection.json",
        Connection,
        "connection"
    );
    check!(Factsheet, "fixtures/factsheet.json", Factsheet, "factsheet");
    check!(ZoneSet, "fixtures/zone-set.json", ZoneSet, "zoneSet");
    check!(Responses, "fixtures/responses.json", Responses, "responses");
}

fn connection_for_topic() -> vda5050_v3_types::Connection {
    vda5050_v3_types::Connection {
        header: vda5050_v3_types::common::Header {
            header_id: 0,
            timestamp: chrono::DateTime::UNIX_EPOCH,
            version: "3.0.0".into(),
            manufacturer: "KIT".into(),
            serial_number: "0001".into(),
        },
        connection_state: vda5050_v3_types::connection::ConnectionState::Online,
    }
}

#[test]
fn message_enum_works_without_serde_and_preserves_box_allocation() {
    let body = connection_for_topic();
    let expected = body.topic("custom").unwrap();
    let boxed = Box::new(body.clone());
    let address = std::ptr::from_ref(boxed.as_ref());
    let message = Message::from(boxed);
    assert_eq!(message.topic_kind(), TopicKind::Connection);
    assert_eq!(message.header(), &body.header);
    assert_eq!(message.topic("custom").unwrap(), expected);
    assert_eq!(message.clone(), message);
    let Message::Connection(inner) = message else {
        panic!("incorrect message variant");
    };
    assert_eq!(std::ptr::from_ref(inner.as_ref()), address);
    assert_eq!(*inner, body);
}

#[test]
fn wrapping_a_body_does_not_validate_or_freeze_its_header() {
    let mut body = connection_for_topic();
    body.header.version = "2.0.0".into();
    let mut message = Message::from(body);
    assert_eq!(message.topic_kind(), TopicKind::Connection);
    assert_eq!(message.header().version, "2.0.0");
    assert_eq!(
        message.topic("custom"),
        Err(HeaderTopicError::UnsupportedVersion)
    );
    let Message::Connection(inner) = &mut message else {
        panic!("incorrect message variant");
    };
    inner.header.version = "3.0.0".into();
    inner.header.serial_number = "invalid serial".into();
    assert_eq!(
        message.topic("custom"),
        Err(HeaderTopicError::InvalidTopic(
            TopicError::InvalidSerialNumber { character: ' ' }
        ))
    );
}

#[test]
fn header_derived_topics_work_without_serde_and_are_owned_snapshots() {
    let mut message = connection_for_topic();
    message.header.manufacturer = " M\u{00fc}ller ".into();
    message.header.serial_number = "A_0.:z-".into();
    let original = message.clone();
    let topic = message.topic(String::from("custom")).unwrap();
    assert_eq!(
        topic.to_string(),
        "custom/v3/ M\u{00fc}ller /A_0.:z-/connection"
    );
    assert_eq!(message, original);
    message.header.manufacturer = "changed".into();
    assert_eq!(topic.manufacturer(), original.header.manufacturer);
    assert_eq!(message.topic("custom").unwrap().manufacturer(), "changed");
}

#[test]
fn deriving_topics_requires_exact_header_version_without_changing_other_apis() {
    let mut message = connection_for_topic();
    for version in [
        "",
        "v3",
        "3",
        "3.0",
        "3.0.1",
        "3.1.0",
        "2.0.0",
        "4.0.0",
        "03.0.0",
        "3.0.0-alpha",
        "3.0.0+build",
        " 3.0.0",
        "3.0.0\n",
    ] {
        message.header.version = version.into();
        let original = message.clone();
        assert_eq!(
            message.topic("vda5050"),
            Err(HeaderTopicError::UnsupportedVersion),
            "{version:?}"
        );
        assert_eq!(message.topic_kind(), TopicKind::Connection);
        assert_eq!(message, original);
        #[cfg(feature = "serde")]
        {
            let text = serde_json::to_string(&message).unwrap();
            assert_eq!(
                serde_json::from_str::<vda5050_v3_types::Connection>(&text).unwrap(),
                message
            );
        }
    }
    // Header validation is not added to plain Topic construction or parsing.
    assert!(Topic::new("vda5050", "KIT", "0001", TopicKind::Connection).is_ok());
    assert!("vda5050/v3/KIT/0001/connection".parse::<Topic>().is_ok());
}

#[test]
fn deriving_topics_propagates_existing_layout_checks_without_repair() {
    let base = connection_for_topic();
    for (interface, manufacturer, serial) in [
        ("", "KIT", "0001"),
        ("bad/interface", "KIT", "0001"),
        ("vda5050", "", "0001"),
        ("vda5050", "bad+maker", "0001"),
        ("vda5050", "bad#maker", "0001"),
        ("vda5050", "bad$maker", "0001"),
        ("vda5050", "bad\0maker", "0001"),
        ("vda5050", "KIT", ""),
        ("vda5050", "KIT", "robot 1"),
        ("vda5050", "KIT", "\u{00e9}"),
    ] {
        let mut message = base.clone();
        message.header.manufacturer = manufacturer.into();
        message.header.serial_number = serial.into();
        let original = message.clone();
        let expected =
            Topic::new(interface, manufacturer, serial, TopicKind::Connection).unwrap_err();
        assert_eq!(
            message.topic(interface),
            Err(HeaderTopicError::InvalidTopic(expected))
        );
        assert_eq!(message, original);
    }
    let interface = "a".repeat(65_535);
    assert_eq!(
        base.topic(interface),
        Err(HeaderTopicError::InvalidTopic(TopicError::TooLong))
    );
}

#[test]
fn header_topic_errors_distinguish_version_and_layout_failures() {
    use std::error::Error;
    let error = HeaderTopicError::UnsupportedVersion;
    assert_eq!(error.to_string(), "header.version: expected exactly 3.0.0");
    assert!(error.source().is_none());
    let layout = TopicError::EmptyLevel(TopicLevel::Manufacturer);
    let error = HeaderTopicError::InvalidTopic(layout);
    assert_eq!(error.source().unwrap().to_string(), layout.to_string());
    assert!(error.to_string().contains("manufacturer"));
}

#[test]
fn all_eight_exact_names_parse_and_format_without_serde() {
    const NAME: &str = TopicKind::InstantActions.as_str();
    assert_eq!(NAME, "instantActions");
    for (kind, name) in NAMES {
        assert_eq!(kind.as_str(), name);
        assert_eq!(kind.to_string(), name);
        assert_eq!(name.parse::<TopicKind>().unwrap(), kind);
    }
}

#[test]
fn names_are_not_normalized_and_full_topics_or_filters_are_not_names() {
    for (_, name) in NAMES {
        for invalid in [
            name.to_uppercase(),
            format!(" {name}"),
            format!("{name}\n"),
            format!("/{name}"),
            format!("vda5050/v3/KIT/0001/{name}"),
        ] {
            assert_eq!(invalid.parse::<TopicKind>(), Err(UnknownTopicKind));
        }
    }
    for invalid in [
        "",
        "unknown",
        "instantactions",
        "instant_actions",
        "zoneset",
        "zone_set",
        "response",
        "+",
        "#",
        "$SYS",
        "state/+",
        "state\0",
        "st\u{0430}te",
    ] {
        assert_eq!(
            invalid.parse::<TopicKind>(),
            Err(UnknownTopicKind),
            "{invalid:?}"
        );
    }
}

#[test]
fn parse_errors_implement_standard_error_without_a_source() {
    let error = "invalid".parse::<TopicKind>().unwrap_err();
    let error: &dyn std::error::Error = &error;
    assert_eq!(
        error.to_string(),
        "expected an exact VDA 5050 v3.0.0 topic name"
    );
    assert!(error.source().is_none());
}

#[cfg(feature = "serde")]
#[test]
fn serde_uses_the_same_exact_string_names_and_rejects_other_shapes() {
    for (kind, name) in NAMES {
        assert_eq!(serde_json::to_value(kind).unwrap(), serde_json::json!(name));
        assert_eq!(
            serde_json::from_value::<TopicKind>(serde_json::json!(name)).unwrap(),
            kind
        );
        assert!(
            serde_json::from_value::<TopicKind>(serde_json::json!(name.to_uppercase())).is_err()
        );
    }

    for invalid in [
        serde_json::json!(null),
        serde_json::json!(0),
        serde_json::json!(false),
        serde_json::json!([]),
        serde_json::json!({}),
        serde_json::json!("zoneset"),
        serde_json::json!("state "),
        serde_json::json!("unknown"),
    ] {
        assert!(serde_json::from_value::<TopicKind>(invalid).is_err());
    }
}

#[test]
fn suggested_layout_constructs_parses_and_exposes_all_levels() {
    for (kind, name) in NAMES {
        let topic = Topic::new("vda5050", "KIT", "0001", kind).unwrap();
        let text = format!("vda5050/v3/KIT/0001/{name}");
        assert_eq!(topic.to_string(), text);
        assert_eq!(text.parse::<Topic>().unwrap(), topic);
        assert_eq!(topic.interface_name(), "vda5050");
        assert_eq!(topic.major_version(), "v3");
        assert_eq!(topic.manufacturer(), "KIT");
        assert_eq!(topic.serial_number(), "0001");
        assert_eq!(topic.kind(), kind);
        assert_eq!(Topic::try_from(text.clone()).unwrap(), topic);
        assert_eq!(String::from(topic.clone()), text);
    }
}

#[test]
fn layout_preserves_permitted_text_and_serial_alphabet_without_normalizing() {
    for (interface, manufacturer) in [
        ("custom", "Example"),
        (" interface ", " maker "),
        ("\u{63a5}\u{53e3}", "M\u{00fc}ller"),
    ] {
        let serial = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_.:-";
        let topic = Topic::new(interface, manufacturer, serial, TopicKind::State).unwrap();
        assert_eq!(topic.interface_name(), interface);
        assert_eq!(topic.manufacturer(), manufacturer);
        assert_eq!(topic.to_string().parse::<Topic>().unwrap(), topic);
    }
    for serial in [
        " ",
        "a b",
        "a/b",
        "a+b",
        "a#b",
        "a$b",
        "\0",
        "\n",
        "\u{00e9}",
        "\u{673a}\u{5668}\u{4eba}",
    ] {
        assert!(
            Topic::new("vda5050", "KIT", serial, TopicKind::State).is_err(),
            "{serial:?}"
        );
        assert!(
            format!("vda5050/v3/KIT/{serial}/state")
                .parse::<Topic>()
                .is_err()
        );
    }
}

#[test]
fn constructor_rejects_empty_and_forbidden_levels_with_field_context() {
    for (index, level) in [
        (0, TopicLevel::InterfaceName),
        (1, TopicLevel::Manufacturer),
        (2, TopicLevel::SerialNumber),
    ] {
        let mut fields = ["vda5050", "KIT", "0001"];
        fields[index] = "";
        assert_eq!(
            Topic::new(fields[0], fields[1], fields[2], TopicKind::Order),
            Err(TopicError::EmptyLevel(level))
        );
        for character in ['/', '+', '#', '$', '\0'] {
            let invalid = format!("a{character}b");
            let mut fields = ["vda5050", "KIT", "0001"];
            fields[index] = &invalid;
            assert_eq!(
                Topic::new(fields[0], fields[1], fields[2], TopicKind::Order),
                Err(TopicError::ForbiddenCharacter { level, character })
            );
        }
    }
    assert_eq!(
        Topic::new("vda5050", "KIT", "a b", TopicKind::State),
        Err(TopicError::InvalidSerialNumber { character: ' ' })
    );
}

#[test]
fn parser_rejects_malformed_hierarchies_versions_filters_and_names() {
    for invalid in [
        "",
        "state",
        "vda5050/v3/KIT/0001",
        "/vda5050/v3/KIT/0001/state",
        "vda5050/v3/KIT/0001/state/",
        "prefix/vda5050/v3/KIT/0001/state",
    ] {
        assert_eq!(
            invalid.parse::<Topic>(),
            Err(TopicError::InvalidLevelCount),
            "{invalid:?}"
        );
    }
    for (text, level) in [
        ("/v3/KIT/0001/state", TopicLevel::InterfaceName),
        ("vda5050//KIT/0001/state", TopicLevel::MajorVersion),
        ("vda5050/v3//0001/state", TopicLevel::Manufacturer),
        ("vda5050/v3/KIT//state", TopicLevel::SerialNumber),
        ("vda5050/v3/KIT/0001/", TopicLevel::Kind),
    ] {
        assert_eq!(text.parse::<Topic>(), Err(TopicError::EmptyLevel(level)));
    }
    for version in ["v2", "v4", "V3", "3", "v03", "v3.0.0", " v3", "v3 "] {
        assert_eq!(
            format!("vda5050/{version}/KIT/0001/state").parse::<Topic>(),
            Err(TopicError::UnsupportedMajorVersion)
        );
    }
    for (index, level) in [
        (0, TopicLevel::InterfaceName),
        (1, TopicLevel::MajorVersion),
        (2, TopicLevel::Manufacturer),
        (3, TopicLevel::SerialNumber),
        (4, TopicLevel::Kind),
    ] {
        for character in ['+', '#', '$', '\0'] {
            let invalid = format!("a{character}b");
            let mut fields = ["vda5050", "v3", "KIT", "0001", "state"];
            fields[index] = &invalid;
            assert_eq!(
                fields.join("/").parse::<Topic>(),
                Err(TopicError::ForbiddenCharacter { level, character })
            );
        }
    }
    for kind in ["unknown", "STATE", "instantactions", "zone_set", "state "] {
        assert_eq!(
            format!("vda5050/v3/KIT/0001/{kind}").parse::<Topic>(),
            Err(TopicError::UnknownKind(UnknownTopicKind))
        );
    }
}

#[test]
fn mqtt_string_length_limit_counts_utf8_bytes_not_characters() {
    // /v3/M/S/state is 13 bytes, leaving 65,522 for the interface.
    for length in [65_521, 65_522, 65_523] {
        let interface = "a".repeat(length);
        let text = format!("{interface}/v3/M/S/state");
        assert_eq!(text.len(), length + 13);
        assert_eq!(
            Topic::new(&interface, "M", "S", TopicKind::State).is_ok(),
            length <= 65_522
        );
        assert_eq!(text.parse::<Topic>().is_ok(), length <= 65_522);
    }
    let interface = "\u{00e9}".repeat(32_761);
    let topic = Topic::new(&interface, "M", "S", TopicKind::State).unwrap();
    assert_eq!(topic.to_string().len(), 65_535);
    assert_eq!(topic.to_string().parse::<Topic>().unwrap(), topic);
    assert_eq!(
        Topic::new(format!("{interface}a"), "M", "S", TopicKind::State),
        Err(TopicError::TooLong)
    );
}

#[test]
fn layout_errors_have_useful_messages_and_only_kind_errors_have_sources() {
    use std::error::Error;
    let errors = [
        TopicError::InvalidLevelCount,
        TopicError::EmptyLevel(TopicLevel::Manufacturer),
        TopicError::ForbiddenCharacter {
            level: TopicLevel::InterfaceName,
            character: '\0',
        },
        TopicError::InvalidSerialNumber { character: ' ' },
        TopicError::UnsupportedMajorVersion,
        TopicError::TooLong,
    ];
    for error in errors {
        assert!(!error.to_string().is_empty());
        assert!(error.source().is_none());
    }
    assert!(
        TopicError::EmptyLevel(TopicLevel::Manufacturer)
            .to_string()
            .contains("manufacturer")
    );
    let error = TopicError::UnknownKind(UnknownTopicKind);
    assert_eq!(
        error.source().unwrap().to_string(),
        UnknownTopicKind.to_string()
    );
}

#[cfg(feature = "serde")]
#[test]
fn complete_topics_serialize_as_checked_strings_not_objects() {
    for (_, name) in NAMES {
        let text = format!("vda5050/v3/KIT/0001/{name}");
        let topic: Topic = text.parse().unwrap();
        assert_eq!(
            serde_json::to_value(&topic).unwrap(),
            serde_json::json!(text)
        );
        assert_eq!(
            serde_json::from_str::<Topic>(&serde_json::to_string(&topic).unwrap()).unwrap(),
            topic
        );
        assert_eq!(
            serde_json::from_slice::<Topic>(&serde_json::to_vec(&topic).unwrap()).unwrap(),
            topic
        );
    }
    for invalid in [
        serde_json::json!(null),
        serde_json::json!([]),
        serde_json::json!({}),
        serde_json::json!(true),
        serde_json::json!(42),
        serde_json::json!("vda5050/v2/KIT/0001/state"),
        serde_json::json!("vda5050/v3/KIT/+/state"),
        serde_json::json!("state"),
        serde_json::json!({"interfaceName": "vda5050", "manufacturer": "KIT", "serialNumber": "0001", "kind": "state"}),
    ] {
        assert!(serde_json::from_value::<Topic>(invalid).is_err());
    }
}
