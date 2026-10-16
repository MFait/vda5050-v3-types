# vda5050-v3-types

[![CI](https://github.com/MFait/vda5050-v3-types/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/MFait/vda5050-v3-types/actions/workflows/ci.yml)

Rust message types for **VDA 5050 v3.0.0**, without MQTT
clients or fleet-control logic.

The `connection`, `visualization`,
`instantActions`, `order`, `responses`, `state`, `factsheet` and `zoneSet` bodies
and their shared models are implemented. The optional `topic` feature provides
exact names, checked topics and filters, body mappings, and checked topic/body
pairing. With `json`, it also provides body dispatch and paired payload encoding.
Rust 1.85 or newer is required.

- [Installation and features](#installation-and-features)
- [Examples](#examples)
- [Message models](#message-models)
- [Topic helper policies](#topic-helper-policies)
- [Wire guarantees and validation limits](#wire-guarantees-and-validation-limits)
- [Sources and testing](#sources-and-testing)

## Installation and features

Select the features your application needs:

```toml
[dependencies]
vda5050-v3-types = { version = "1", features = ["json", "topic"] }
```

| Feature | Capability |
| --- | --- |
| Default (empty feature set) | Construct and use all message types |
| `serde` | Serde implementations for message bodies and their supporting wire models |
| `json` | Enables `serde`; adds JSON string/byte helpers to all eight bodies |
| `topic` | Names, topics, filters, body mappings and checked pairs; no added dependencies |
| `topic` + `serde` | Also serialize/deserialize topic names, topics and filters as checked strings |
| `topic` + `json` | Also dispatch bodies by topic kind and encode checked topic/body pairs |

Core dependencies are Chrono (typed UTC timestamps, without clock/local-time
features) and `serde_json` (used for action-parameter values and envelope data).
No optional features are enabled by default.
`serde_json`'s `float_roundtrip` feature is enabled explicitly so finite
float64 values survive JSON text/byte round-trips without relying on
dev-dependency feature unification.

The `serde` feature gates this crate's implementations, not Serde support
provided by dependencies such as `serde_json::Value`. `topic::Message` and
`topic::TopicMessage` deliberately have no Serde representation or JSON envelope.
Use topic-directed decoding and paired encoding with `topic,json` instead.

## Examples

Feature-specific snippets below are tested through the matching files in
[examples](https://github.com/MFait/vda5050-v3-types/tree/main/examples), with
feature requirements declared in Cargo rather than hidden in the displayed code.

### No features: construct a message

All bodies and shared models are available without optional features. Add a
direct `chrono` dependency if your application constructs timestamps with its API.

```rust
use chrono::{TimeZone, Utc};
use vda5050_v3_types::{
    common::Header, connection::ConnectionState, Connection,
};

let connection = Connection {
    header: Header {
        header_id: 0,
        timestamp: Utc.with_ymd_and_hms(2026, 10, 16, 12, 0, 0).unwrap(),
        version: "3.0.0".into(),
        manufacturer: "Example".into(),
        serial_number: "robot-1".into(),
    },
    connection_state: ConnectionState::Online,
};
assert_eq!(connection.header.header_id, 0);
```

### `serde`: use a serializer directly

Enable `serde` and add your chosen serializer as a direct dependency. This example
uses `serde_json`; the crate's `json` convenience feature is not required.

```rust,ignore
use vda5050_v3_types::Connection;

fn main() -> Result<(), serde_json::Error> {
    let body = r#"{
        "headerId": 0,
        "timestamp": "2017-04-15T11:40:03.123Z",
        "version": "3.0.0",
        "manufacturer": "Example",
        "serialNumber": "robot-1",
        "connectionState": "ONLINE"
    }"#;
    let message: Connection = serde_json::from_str(body)?;
    let value = serde_json::to_value(&message)?;
    assert_eq!(value["connectionState"], "ONLINE");
    assert!(value.get("header").is_none());
    Ok(())
}
```

### `json`: decode and encode payloads

All eight bodies provide `from_json`, `from_json_slice`, `to_json`, and
`to_json_vec`, returning the original `serde_json::Error`.

```rust,ignore
use vda5050_v3_types::Connection;

fn main() -> Result<(), serde_json::Error> {
    let body = r#"{
        "headerId": 0,
        "timestamp": "2017-04-15T11:40:03.123Z",
        "version": "3.0.0",
        "manufacturer": "Example",
        "serialNumber": "robot-1",
        "connectionState": "ONLINE"
    }"#;
    let message = Connection::from_json(body)?;
    let payload = message.to_json_vec()?;
    assert_eq!(Connection::from_json_slice(&payload)?, message);
    Ok(())
}
```

### `topic`: names, topics and subscription filters

These helpers construct checked text, without a broker connection or JSON.

```rust,ignore
use vda5050_v3_types::topic::{Topic, TopicFilter, TopicKind};

fn main() -> Result<(), vda5050_v3_types::topic::TopicError> {
    assert_eq!(TopicKind::InstantActions.as_str(), "instantActions");
    let topic = Topic::new("vda5050", "KIT", "0001", TopicKind::Order)?;
    assert_eq!(topic.to_string(), "vda5050/v3/KIT/0001/order");
    assert_eq!(topic.to_string().parse::<Topic>()?, topic);

    let states = TopicFilter::new("vda5050", None, None, Some(TopicKind::State))?;
    assert_eq!(states.to_string(), "vda5050/v3/+/+/state");
    Ok(())
}
```

### `topic` + `serde`: checked topic strings

```rust,ignore
use vda5050_v3_types::topic::Topic;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let topic: Topic = serde_json::from_str(r#""vda5050/v3/KIT/0001/state""#)?;
    assert_eq!(
        serde_json::to_string(&topic)?,
        r#""vda5050/v3/KIT/0001/state""#
    );
    assert!(serde_json::from_str::<Topic>(r#""vda5050/v2/KIT/0001/state""#).is_err());
    Ok(())
}
```

### `topic` + `json`: dispatch and checked pairing

```rust,ignore
use vda5050_v3_types::topic::{Message, Topic, TopicMessage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let topic: Topic = "vda5050/v3/KIT/0001/responses".parse()?;
    let payload = br#"{
        "headerId": 0,
        "timestamp": "2017-04-15T11:40:03.123Z",
        "version": "3.0.0",
        "manufacturer": "KIT",
        "serialNumber": "0001",
        "responses": []
    }"#;
    let message = topic.decode_json_slice(payload)?;
    assert!(matches!(&message, Message::Responses(_)));

    // Dispatch selects the body type; pairing separately checks header consistency.
    let pair = TopicMessage::new(topic, message)?;
    let (topic_text, payload_bytes) = pair.to_json_vec()?;
    assert_eq!(topic_text, "vda5050/v3/KIT/0001/responses");
    assert_eq!(
        pair.topic().decode_json_slice(&payload_bytes)?,
        *pair.message()
    );
    Ok(())
}
```

## Message models

Every body embeds `common::Header`, flattened into the wire object. Supporting
types live in the corresponding public module; shared models live in `common`.

| Body | Module and supporting models |
| --- | --- |
| `Connection` | `connection`: connection state |
| `Visualization` | `visualization`: shared motion and path models from `common` |
| `InstantActions` | `instant_actions`: shared `common::Action` and checked parameters |
| `Order` | `order`: nodes, edges, positions, deviations and corridors |
| `Responses` | `responses`: grant decisions and optional UTC lease expiry |
| `State` | `state`: graph/action/request status, loads, power and diagnostics |
| `Factsheet` | `factsheet`: capabilities, limits, geometry, loads and configuration |
| `ZoneSet` | `zone_set`: `ZoneSetDefinition`, polygons and typed `ZoneKind` rules |

Some similarly named types have intentionally different roles:

- `state::NodePosition` omits order tolerances; `order::NodePosition` includes them.
- `state::ZoneSetState` reports stored status, not the `ZoneSet` transfer body.
- `state::Error` is a reported robot error, not a Rust parsing error.
- `factsheet::ActionParameter` describes capabilities;
  `common::ActionParameter` carries runtime values.
- `ZoneAction` omits the sender-supplied `actionId` used by `common::Action`.

Shared load geometry is available through `common` and re-exported from `state`.
`common::Vertex`, `factsheet::Vertex` and `zone_set::Vertex` are the same type;
coordinates are robot-relative in envelopes and project-relative in zones.
`ZoneKind` serializes as `zoneType` plus parameters beside it, without a `kind`
wrapper. Factsheet dotted limits such as `order.nodes` are literal JSON keys.

## Topic helper policies

### Names and layout

`TopicKind` represents the eight case-sensitive names from section 4.3.
Parsing returns `UnknownTopicKind` for any nonmatching string; it does not trim,
normalize or extract names from complete topics. With `serde`, names use the
same exact strings.

`Topic` implements the suggested section 4.2 layout, restricted to `v3`.
Topic fields are private, with read-only accessors. Construction supplies `v3`;
parsing rejects other versions and requires exactly five nonempty levels. Levels
reject `/`, `+`, `#`, `$` and NUL; serial numbers permit only ASCII letters,
digits, `_`, `.`, `:` and `-`. The full topic cannot exceed 65,535 UTF-8 bytes.
Other text is preserved without trimming, escaping or normalization. Nonempty
levels, v3-only support and rejection of the standard's discouraged characters
are intentional helper policies, not claims that all other broker layouts are
invalid.

`TopicError` identifies the failed rule and, where applicable, level/character.
With `serde`, `Topic` encodes as a string and decoding uses the same checks.
Custom/cloud hierarchies are not supported by `Topic`; use `TopicKind` separately
there. No MQTT connection or network behavior is provided.

### Body mappings and header-derived topics

All eight message bodies expose a `const` `topic_kind(&self)` method with `topic`
enabled, independently of Serde or JSON.
The result depends only on the message type; it does not inspect or validate
header identity/version, mutate the body or construct a complete topic.

Each body also provides `topic(interface)` without requiring Serde or JSON.
For example, `order.topic("vda5050")?` builds a topic whose kind is
`TopicKind::Order` and whose manufacturer/serial number come from `order.header`.

This helper first requires header version **exactly `3.0.0`** (an intentional
library compatibility policy), then copies manufacturer and serial number
unchanged into a checked v3 topic. `HeaderTopicError::UnsupportedVersion`
distinguishes a version failure from `HeaderTopicError::InvalidTopic`, which
wraps the existing `TopicError` and exposes it as its error source.
The body is not mutated or otherwise validated; ordinary JSON parsing still
does not validate version content. A returned topic is an owned snapshot:
subsequent header mutations do not update it or check its consistency.

### Message dispatch and checked pairs

For consumers handling several kinds, `topic::Message` holds any of the eight
bodies in a typed, boxed variant. Matching a variant gives access to its body.
`From<Body>` allocates a box; `From<Box<Body>>` reuses an existing box.
`Message` provides `topic_kind()`, `header()` and the same checked
`topic(interface)` operation, without requiring Serde or JSON. Wrapping does not
validate a body or bind it to an existing topic. Variants permit access to and
mutation of the concrete bodies. The enum has no Serde representation or JSON
envelope.

With both `topic` and `json`, `TopicKind` and `Topic` provide `decode_json` and
`decode_json_slice`, returning `Result<Message, serde_json::Error>`.

Parse a raw topic separately to keep `TopicError` distinct from JSON decoding
errors. Dispatch selects exactly one concrete body decoder and returns its
original error; it does not try other kinds or unwrap an envelope. Only the
topic kind is used: header manufacturer, serial number and version are not
validated or compared. Success is not proof of the payload's intended kind:
unknown fields are ignored, and a payload may satisfy multiple body shapes.
Pairing is a separate, explicit operation, available even without `json`.

`TopicMessage::new(topic, body_or_message)` requires matching kind, header version
exactly `3.0.0`, and exact manufacturer and serial-number equality. It checks
those rules in that order, returning `PairingError` without normalization or
repair. Construction consumes its inputs, including on failure. The interface
name has no corresponding header field and is not compared.

The pair owns private fields with read-only `topic()`/`message()` accessors;
editing requires consuming `into_parts()` and constructing a new pair.
It has no Serde envelope or unchecked deserializer. With `json`, `to_json()`
returns `(String, String)` and `to_json_vec()` returns `(String, Vec<u8>)`:
topic and body payload stay separate. Encoding returns the concrete body's
original JSON error if its wire constraints fail. Pair consistency does not
guarantee body/protocol validity or intended payload kind; ordinary dispatch
retains its existing unchecked behavior.

### Subscription-filter helpers

`TopicFilter` is separate from publishable `Topic`. It uses the same explicit
interface and fixed `v3` layout, with `None` selecting a whole-level `+` for
manufacturer, serial number or message kind. For example:

| Selectors | Filter |
| --- | --- |
| Any robot, state only | `vda5050/v3/+/+/state` |
| All messages from one robot | `vda5050/v3/KIT/0001/+` |
| All messages from one manufacturer | `vda5050/v3/KIT/+/+` |

Exact selectors follow the existing topic rules, including nonempty values,
serial alphabet and the full 65,535-byte UTF-8 limit. `Some("+")` is invalid:
use `None` for a wildcard. Read-only accessors return `Option` for selectors.
`TopicFilter::from(&topic)` creates an exact filter for an existing topic.
With `serde`, filters use checked strings rather than objects.

These helpers deliberately support only five levels and `+` in the final three
positions, not `#`, partial wildcards, wildcard interface/version, shared
subscriptions or deployment-specific hierarchies. This is a helper scope choice,
not a claim that those MQTT features are invalid. No matching or broker
subscription logic is implemented, and publish topics still reject wildcards.

## Wire guarantees and validation limits

### Wire representation

- Required fields, Rust numeric widths (`headerId` is `u32`), and exact enum
  spellings are enforced. String-valued enums reject object/array alternatives;
  `ZoneKind` remains an object tagged by `zoneType`.
  There are no defaults for missing required fields.
  Integer fields reject floating-point JSON tokens such as `1.0`, even though
  JSON Schema treats mathematically integral numbers as integers.
- Timestamps follow section 7.2: `YYYY-MM-DDTHH:mm:ss.fffZ`, UTC, exactly three
  fractional digits. Invalid dates and other wire spellings are rejected.
  Serialization rejects years outside `0000..=9999` and sub-millisecond
  precision instead of silently truncating; callers must explicitly convert
  higher-precision values before encoding.
- Unknown fields in typed objects are ignored and lost on re-encoding. Opaque
  action-parameter values and 3D-envelope data preserve their nested keys.
- Message bodies and supporting wire-object structs reject positional arrays
  in standalone and nested decoding. JSON-valued fields retain their documented
  shapes; topic names, topics and filters serialize as strings, not objects.
- Wire bounds are checked by this crate's Serde implementations, not by direct
  Rust construction or field mutation. Optional non-null fields omit `None`,
  reject explicit `null`, and preserve absence instead of inserting defaults.
- All current connection fields are required and non-nullable.
- Visualization requires its header and `referenceStateHeaderId`. Its four
  body objects are independently optional. Optional fields omit `None` when
  encoding, and reject explicit `null` when decoding. Missing trajectory
  degree/knots/control-point weight remain absent rather than inserting the
  protocol defaults.
- Shared floating-point fields are `f64`/`Option<f64>` for logic-code use;
  factsheet timing is `f32`. Encoding and decoding reject non-finite numbers
  in typed numeric fields, bounded orientation angles outside `[-pi, pi]`,
  scores/knots outside `[0, 1]`, negative deviation ranges, nonpositive weights,
  and degree zero. Direct Rust field assignment is not validated; invalid
  constructed values return serialization errors, never silent `null` values.
  Numeric errors identify the offending wire field (not a full nested path).
- Instant actions require `actions` (an empty array is representable), with
  `blockingType: "NONE"` on every item. The shared `Action` type also supports
  `SINGLE`, `SOFT`, and `HARD`, but instant-message encoding/decoding rejects them.
  Action names are strings, not a closed enum. Parameter values use checked
  `ActionParameterValue`: top-level null is rejected, nested JSON nulls are
  preserved. Numeric representation follows `serde_json::Value` (no
  arbitrary-precision guarantee).
- Shared actions preserve optional `retriable`, including on instant actions:
  section 7.4 reuses the section 7.3 definition even though the instant schema
  omits this property. `None` stays absent; the protocol default is false.
- Orders use uint32 update/sequence IDs, exact closed enums and optional
  non-null fields. Ellipse axes and corridor widths are nonnegative; orientation
  tolerance is `[0, pi]`. Ellipse rotation retains the schema's decimal bound
  `[-1.570796327, 1.570796327]`, which is not stated in the prose. Other
  numeric limits follow their documented schema/specification constraints;
  finite edge speeds, heights and length have no added positivity constraint.
  `maximumRotationSpeed` follows the specification, not the pinned schema's
  `maxRotationSpeed` typo. The typo is ignored as an unknown field, not aliased.
  See [the audit](https://github.com/MFait/vda5050-v3-types/blob/main/docs/provenance.md) and [upstream fix PR](https://github.com/VDA5050/VDA5050/pull/671).

**Body deserialization is not full schema or protocol validation.** It does not
validate version-string content, robot identity, timestamp freshness, counter
progression or robot behavior. Explicit header-derived topic construction and
checked pairing apply the separate identity/version policies documented above.
The standard's retained delivery, last-will, and
hibernation rules remain the caller's responsibility. An old timestamp in a
last-will body is representable intentionally (section 6.5).

### Message-specific constraints and deferred semantics

Cross-field semantic validation is deferred. Visualization does not check
whether the referenced state exists, path start positions/base coverage,
NURBS control-point counts, or knot-vector length/order/multiplicities.
`localized: false` does not verify robot behavior or emit localization errors.
Instant action support, ID uniqueness, parameter semantics and conflicts with
the current order are not checked. UUID action IDs are suggested, not required.
Orders do not validate graph counts (including a nonempty node list), sequence
continuity/parity, base/horizon consistency, ellipse axis ordering, action
execution or compatibility with the robot/current order. Omitted defaults
remain absent, including corridor behavior and trajectory defaults.
Responses do not validate active request identity, uniqueness, grant transitions
or expiry freshness. Section 7.5 permits `leaseExpiry` only with `GRANTED`;
that cross-field rule is deferred too, so other decisions with an expiry remain
representable. Optional expiry rejects explicit null and omits `None`.
State checks all required/optional fields and closed enum spellings, including
`RETRIABLE` and `STARTUP`. Extensible error types and information types remain
strings. Battery health is specification `int8` in `[0, 100]`; battery range is
`uint32`, despite their broader schema `number` types. Load mass/dimensions are
nonnegative, charge is `[0, 100]`, and other new floats are finite. Bounding-box
orientation has no added angle range. Optional load reporting distinguishes
absence (unknown) from an empty array (known to be unloaded).

State does not check capability-dependent field presence, uniqueness of enabled
maps/zone sets, action/request lifecycles, graph consistency, translation language
codes or consistency between safety, driving and operating mode. Zone request
types are `ACCESS`/`REPLANNING`; conflicting prose mentioning
`COORDINATED_REPLANNING` is documented in the audit. Trajectory applicability is
not enforced. Reported safety data is not a safety-control implementation.

Factsheet extensible kinematics, robot classes, localization/navigation types and
wheel types remain strings; the listed closed enums are checked. Optional 3D
envelope data uses a JSON map: the root must be an object, while arbitrary nested
keys and nulls are preserved. Data format, URLs and network-address content are
not validated or fetched. Unknown fields outside that data object are still lost.
Numeric limits follow the audited schema/specification, without adding unstated
physical constraints. Timing is rounded to float32; decoding checks sign before
narrowing and rejects overflow. Encoding rejects non-finite values.

Capability relationships, minimum/maximum ordering, simple/closed polygon
geometry and compliance of other messages with advertised limits are not
validated. Omitted defaults remain absent. See the audit for schema-only bounds
and the stricter shared load-dimension and charging-time checks.

Zone sets enforce all ten zone types' required parameters, exact closed enums,
finite numbers and priority/penalty factors in `[0, 1]`. Encoding and decoding
require at least three vertices. These checks do not prevent invalid values from
being constructed or directly mutated in Rust. Neither source gives a speed
minimum or direction-angle bound here, so no additional bound is invented.
`releaseLossBehavior` is required by section 7.6 and the schema; the conflicting
STOP-if-absent prose in section 6.4.1 is documented, not materialized as a default.
Parameters irrelevant to a zone variant are ignored and lost as unknown fields.

Zone polygons are implicitly closed; simplicity, counterclockwise ordering, map
bounds, unique IDs, zone interactions and immutable set identity are not checked.
No grant, activation, action execution or zone-transfer behavior is implemented.

## Sources and testing

See [provenance and discrepancies](https://github.com/MFait/vda5050-v3-types/blob/main/docs/provenance.md) for pinned upstream
inputs and the differences between schema assertions and specification rules.
Tests use the vendored schema offline, independently of a sibling checkout.
`jsonschema` is dev-only, with HTTP and file resolution disabled.
The independent [consumer check](https://github.com/MFait/vda5050-v3-types/tree/main/tests/consumer) runs without this library's
dev-dependencies, checking float precision through both JSON helper paths.
Run it from a repository checkout:

```sh
cargo run --locked --manifest-path tests/consumer/Cargo.toml
```

[vda5050-types-rs](https://github.com/kKdH/vda5050-types-rs) is v2 prior art,
not the source of truth for v3. This crate is licensed under [MIT](./LICENSE),
Copyright (c) 2026 Michael Fait; vendored upstream material retains its
[upstream notice](tests/schemas/LICENSE.txt).
See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution scope and local checks.
CI asserts that `jsonschema` stays outside the normal/build dependency graph
for every tested feature configuration, including transitive dependencies.
