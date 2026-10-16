use std::sync::OnceLock;

use serde_json::Value;

pub fn schema(name: &str) -> &'static jsonschema::Validator {
    static CONNECTION: OnceLock<jsonschema::Validator> = OnceLock::new();
    static VISUALIZATION: OnceLock<jsonschema::Validator> = OnceLock::new();
    static INSTANT_ACTIONS: OnceLock<jsonschema::Validator> = OnceLock::new();
    static ORDER: OnceLock<jsonschema::Validator> = OnceLock::new();
    static RESPONSES: OnceLock<jsonschema::Validator> = OnceLock::new();
    static STATE: OnceLock<jsonschema::Validator> = OnceLock::new();
    static FACTSHEET: OnceLock<jsonschema::Validator> = OnceLock::new();
    static ZONE_SET: OnceLock<jsonschema::Validator> = OnceLock::new();
    let (cache, source) = match name {
        "connection" => (&CONNECTION, include_str!("../schemas/connection.schema")),
        "visualization" => (
            &VISUALIZATION,
            include_str!("../schemas/visualization.schema"),
        ),
        "instantActions" => (
            &INSTANT_ACTIONS,
            include_str!("../schemas/instantActions.schema"),
        ),
        "order" => (&ORDER, include_str!("../schemas/order.schema")),
        "responses" => (&RESPONSES, include_str!("../schemas/responses.schema")),
        "state" => (&STATE, include_str!("../schemas/state.schema")),
        "factsheet" => (&FACTSHEET, include_str!("../schemas/factsheet.schema")),
        "zoneSet" => (&ZONE_SET, include_str!("../schemas/zoneSet.schema")),
        _ => panic!("unknown test schema: {name}"),
    };
    cache.get_or_init(|| {
        jsonschema::options()
            .should_validate_formats(true)
            .build(&serde_json::from_str::<Value>(source).unwrap())
            .unwrap()
    })
}

#[cfg(feature = "serde")]
pub fn remove(value: &mut Value, pointer: &str) {
    let (parent, key) = pointer.rsplit_once('/').unwrap();
    assert!(
        value
            .pointer_mut(parent)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .remove(key)
            .is_some(),
        "missing fixture field: {pointer}"
    );
}

#[cfg(feature = "serde")]
pub fn assert_required<T: serde::de::DeserializeOwned>(
    fixture: &Value,
    validator: &jsonschema::Validator,
    pointers: &[&str],
) {
    for pointer in pointers {
        let mut missing = fixture.clone();
        remove(&mut missing, pointer);
        assert!(!validator.is_valid(&missing), "{pointer}");
        assert!(serde_json::from_value::<T>(missing).is_err(), "{pointer}");
        let wrong_type = if fixture.pointer(pointer).unwrap().is_object() {
            serde_json::json!([])
        } else {
            serde_json::json!({})
        };
        for invalid in [Value::Null, wrong_type] {
            let mut value = fixture.clone();
            *value.pointer_mut(pointer).unwrap() = invalid;
            assert!(!validator.is_valid(&value), "{pointer}");
            assert!(serde_json::from_value::<T>(value).is_err(), "{pointer}");
        }
    }
}

#[cfg(feature = "serde")]
#[allow(dead_code)] // Integration test binaries use different subsets of these helpers.
pub fn assert_optional<T: serde::de::DeserializeOwned + serde::Serialize>(
    fixture: &Value,
    validator: &jsonschema::Validator,
    pointers: &[&str],
) {
    for pointer in pointers {
        let mut missing = fixture.clone();
        remove(&mut missing, pointer);
        validator.validate(&missing).unwrap();
        let decoded: T = serde_json::from_str(&missing.to_string()).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), missing, "{pointer}");
        let mut null = fixture.clone();
        *null.pointer_mut(pointer).unwrap() = Value::Null;
        assert!(!validator.is_valid(&null), "{pointer}");
        assert!(serde_json::from_value::<T>(null).is_err(), "{pointer}");
    }
}
