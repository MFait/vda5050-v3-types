# Upstream provenance and message audits

Inputs are pinned to VDA5050/VDA5050 revision
`0b2ae43154bcd1ab2be4e2445df8b406dcad77b2`, whose Markdown specification
identifies itself as version 3.0.0. The source checkout was clean when audited.
The moving `main` branch is not used by builds or tests.

- [Specification at the pinned revision](https://github.com/VDA5050/VDA5050/blob/0b2ae43154bcd1ab2be4e2445df8b406dcad77b2/VDA5050_EN.md):
  sections 4.2-4.3, 6.1, 6.2.1, 6.4-6.5, 6.7-6.8, and 7.1-7.10.
- [Connection schema at the pinned revision](https://github.com/VDA5050/VDA5050/blob/0b2ae43154bcd1ab2be4e2445df8b406dcad77b2/json_schemas/connection.schema):
  vendored unchanged in [tests/schemas](../tests/schemas/connection.schema).
- [Visualization schema at the pinned revision](https://github.com/VDA5050/VDA5050/blob/0b2ae43154bcd1ab2be4e2445df8b406dcad77b2/json_schemas/visualization.schema):
  vendored unchanged in [tests/schemas](../tests/schemas/visualization.schema).
- [Instant actions schema at the pinned revision](https://github.com/VDA5050/VDA5050/blob/0b2ae43154bcd1ab2be4e2445df8b406dcad77b2/json_schemas/instantActions.schema):
  vendored unchanged in [tests/schemas](../tests/schemas/instantActions.schema).
- [Order schema at the pinned revision](https://github.com/VDA5050/VDA5050/blob/0b2ae43154bcd1ab2be4e2445df8b406dcad77b2/json_schemas/order.schema):
  vendored unchanged in [tests/schemas](../tests/schemas/order.schema).
- [Responses schema at the pinned revision](https://github.com/VDA5050/VDA5050/blob/0b2ae43154bcd1ab2be4e2445df8b406dcad77b2/json_schemas/responses.schema):
  vendored unchanged in [tests/schemas](../tests/schemas/responses.schema).
- [State schema at the pinned revision](https://github.com/VDA5050/VDA5050/blob/0b2ae43154bcd1ab2be4e2445df8b406dcad77b2/json_schemas/state.schema):
  vendored unchanged in [tests/schemas](../tests/schemas/state.schema).
- [Factsheet schema at the pinned revision](https://github.com/VDA5050/VDA5050/blob/0b2ae43154bcd1ab2be4e2445df8b406dcad77b2/json_schemas/factsheet.schema):
  vendored unchanged in [tests/schemas](../tests/schemas/factsheet.schema).
- [Zone-set schema at the pinned revision](https://github.com/VDA5050/VDA5050/blob/0b2ae43154bcd1ab2be4e2445df8b406dcad77b2/json_schemas/zoneSet.schema):
  vendored unchanged in [tests/schemas](../tests/schemas/zoneSet.schema).
- [Upstream license](../tests/schemas/LICENSE.txt): retained with the schemas.

Upstream says that the published VDA PDF takes precedence over the GitHub
Markdown and schemas if they differ. These increments compare the pinned
Markdown and schemas; they do not claim an independent audit against the PDF.
All eight bodies are implemented/audited: `connection`, `visualization`,
`instantActions`, `order`, `responses`, `state`, `factsheet`, and `zoneSet`.

## String-valued enum decoding

String-valued enums accept only their exact wire strings. Serde's default
unit-enum decoder also accepts externally tagged objects such as
`{"ONLINE":null}`; these are not VDA wire strings and are explicitly rejected.
This input-compatibility tightening was approved during the pre-publication API
review. A shared internal macro retains Serde's spelling rules without accepting
its alternate object representation. Regression tests cover every variant of
all 24 string-valued enums, standalone and nested, including `TopicKind`.
The internally tagged `ZoneKind` remains an object with a `zoneType` property.

## Topic-name and layout findings and choices

Sections 4.2-4.3 distinguish the suggested five-level hierarchy from mandatory
message-kind names. The dependency-free `topic` feature exposes
`TopicKind`: `order`, `instantActions`, `state`, `visualization`, `connection`,
`factsheet`, `zoneSet`, and `responses`. Optional message implementation does not
make its name extensible or change capitalization.

`as_str`, Display and FromStr use exactly these spellings; parsing returns the
public `UnknownTopicKind` error without trimming, normalization or interpreting a
complete topic/filter. This error does not retain the input. With `serde` enabled,
the enum uses the same string representation.
Source-cited tests cover all eight names, rejected names/shapes, optional Serde
and use without Serde. CI includes topic-only, topic + serde and topic + json.

The checked `Topic` helper implements only the suggested
`interfaceName/majorVersion/manufacturer/serialNumber/topic` layout.

| Surface | Source / library decision |
| --- | --- |
| Layout | Section 4.2 suggests five levels, allowing cloud adaptations. This helper accepts exactly five, all nonempty (library policy), and is not a universal broker-topic parser. |
| Version | Section 4.2 specifies `v` followed by the major version. Agreed v3-only API: constructor supplies `v3`, parser rejects `v2`, `v4`, `v03`, `v3.0.0` and other spellings. Plain topic construction/parsing does not inspect headers. |
| Characters | Section 4.2 forbids `/` within levels and discourages `+`, `#`, `$`. Reject all four as helper policy; additionally reject MQTT-forbidden NUL and enforce MQTT's 65,535-byte UTF-8 string limit. Rust strings already exclude malformed UTF-8 and surrogate code points. No broader text normalization/restriction is added. |
| Serial | Enforce section 4.2's ASCII letters/digits and `_.:-` alphabet; unlike manufacturer/interface, serial does not permit spaces or non-ASCII letters. |
| Representation | Private owned strings, read-only accessors and a typed kind. Display/FromStr and string conversions preserve exact text. Optional Serde uses a checked string representation, not an object. No dependencies added. |
| Errors | TopicError distinguishes wrong level count, empty level, forbidden character, invalid serial, unsupported version, unknown kind and length. Errors retain rule/level/character context, not the input topic; unknown-kind error exposes its source. |

Tests cover every kind, constructor/parser parity, empty/forbidden fields,
wrong levels/versions, filters, Unicode and whitespace preservation, exact byte
length boundaries and checked Serde shape/round-trips.

Section 4.3's message/schema correspondence also defines the inherent
`topic_kind(&self)` methods on all eight bodies. These const methods are gated
only by `topic`, not Serde/JSON, and return a type-fixed value without inspecting
header contents. This is neither header validation nor a checked topic/body
pairing operation. Tests check availability without Serde, a malformed-header
boundary and all eight fixture mappings without changing their wire bodies.

All eight bodies additionally expose `topic(interface)`, reusing Topic::new with
their fixed kind and unchanged header manufacturer/serial number. The agreed
helper policy requires header.version **exactly `3.0.0`** before construction:
this crate implements that version's models, rather than asserting compatibility
with other minor/patch versions. This is library policy, not an additional
section 4.2/7.2 assertion. Ordinary body JSON decoding and Topic parsing retain
their existing behavior. HeaderTopicError distinguishes unsupported version
from an underlying TopicError and exposes that error as its source.

The helper copies identity without normalization or mutation; the resulting
owned topic is a snapshot, not a binding to mutable body fields. No timestamp,
counter, robot existence or other body validation occurs. Tests cover all eight
fixtures, use without Serde, rejected version spellings, propagated layout/byte
limits, identity preservation, snapshot independence and unchanged JSON parsing.
Independently supplied topic/body consistency is checked separately as below.

`topic::Message` represents section 4.3's eight body kinds with consistently
boxed variants (agreed). Boxing is a Rust storage choice, not a wire envelope.
From<Body> allocates one box; From<Box<Body>> reuses the allocation. Kind/header
access delegates to the selected body, and topic construction shares the same
header/version/layout checks as concrete bodies. Wrapping itself performs no
validation and permits concrete-body access/mutation by matching the variant.
The enum intentionally has no Serde representation; JSON dispatch uses an
external topic kind rather than a discriminator inside the body. Tests cover
all eight variants/conversions without changing concrete wire representations,
shared topic behavior, allocation reuse and availability without Serde.

With both `topic` and `json`, TopicKind::decode_json/decode_json_slice and the
matching Topic methods select section 4.3's corresponding concrete body decoder.
They return the boxed Message and preserve the original serde_json::Error;
topic strings must be parsed separately. There is no fallback, trial decoding,
wrapper envelope or automatic header identity/version check. In particular,
decoding on Topic uses only its kind, unlike header-derived topic construction.
Success does not establish intended payload kind: existing unknown-field rules
allow a JSON object to satisfy more than one body shape. Tests cover all eight
kinds through text/bytes and kind/topic entry points, concrete wire constraints,
error category/location/text, invalid UTF-8, envelope rejection, overlapping
shapes, identity/version acceptance boundaries and float precision.

`TopicMessage` is an opt-in checked pair, not a transport packet or JSON envelope.
The agreed policy compares section 4.3's kind, requires header version exactly
`3.0.0`, then compares section 4.2/7.2 manufacturer and serial strings exactly,
in that order. Exact version compatibility and non-normalizing equality are
library policies; no inference or repair of robot identity is attempted.
The interface has no header counterpart and is not compared. PairingError
identifies the first mismatch without retaining inputs; failed construction
consumes the supplied topic/body.

Private owned fields, read-only access and consuming into_parts preserve the
consistency invariant; there is no mutable accessor or unchecked Serde
representation. Construction does not check the body's numeric/timestamp/graph
validity. With `json`, encoding delegates to the concrete body and returns its
original error, with topic text and payload text/bytes as separate tuple members.
Ordinary Topic decoding remains unchecked until callers explicitly construct
a pair. Tests cover all eight bodies, every cross-kind mismatch, exact identity
and version rules, immutable-access compilation failure, extraction/rechecking,
wire error propagation, separate output shape and float-bit preservation.

`TopicFilter` extends the suggested section 4.2 layout with MQTT whole-level `+`
selectors, not new VDA topic names. Agreed scope keeps interface explicit and
version fixed to `v3`; manufacturer, serial and kind may each be exact or `+`.
The constructor uses Option selectors (None means wildcard); exact levels share
Topic's checks, serial alphabet and total UTF-8 byte limit. Parsing recognizes
only a complete `+` level. Display, string conversions, read-only accessors,
From<&Topic> exact filters and optional checked-string Serde preserve invariants.

Excluding `#`, partial wildcards, wildcard interface/version, shared-subscription
prefixes and arbitrary cloud layouts is deliberate library scope, not a VDA or
MQTT prohibition. There are no matching or broker operations; publishable Topic
remains wildcard-free. Tests cover all selector combinations, all eight exact
kinds, malformed layouts and selectors, Unicode/whitespace preservation, exact
byte boundaries and Serde rejection/round-trips. No dependencies were added.

## Connection findings and choices

| Surface | Schema | Specification / library decision |
| --- | --- | --- |
| `headerId` | Integer without minimum/maximum | Sections 7.2 and 7.7 specify `uint32`; use `u32` and reject negatives/overflow. |
| Integer spelling | JSON Schema treats `1.0` as an integer | Rust/Serde integer decoding rejects floating-point tokens; document and test this stricter wire policy. |
| `timestamp` | `format: date-time`; description shows milliseconds and UTC | Sections 7.2 and 7.7 spell `YYYY-MM-DDTHH:mm:ss.fffZ`; enforce that exact spelling rather than all RFC 3339 variants. Schema format assertions are enabled explicitly in tests. |
| `version` | Unconstrained string, with an old `1.3.2` example | Current bodies use `3.0.0` in fixtures. The example is not a version constraint; content validation is deferred. |
| Header placement | Six required root properties | Section 7.2 says the header is not a JSON object; flatten the shared Rust header. |
| Enum | Four states, including `HIBERNATING` | Matches section 7.7; preserve exact uppercase wire spellings. |
| Additional fields | Not forbidden | Ignore unknown fields, without preserving them. |
| Nullability | All six fields required, with non-null types | Reject missing and explicit-null fields. |

Sections 6.5 and 7.7 describe last wills, retained delivery and connection
behavior. These are not structural schema assertions and are not implemented
as transport logic. In particular, last-will timestamps/counters may be old;
the library must not reject such bodies solely for age or counter history.
Likewise, `HIBERNATING` represents the reported state; decoding it does not
verify that a robot has stopped sending state messages.

Source-cited tests cover both matching schema rules and stricter
specification-derived rules, including schema-valid inputs intentionally
rejected by the library. Successful decoding is not protocol validation.

## Visualization findings and choices

Section 7.9 reuses the section 7.8 state models for all four optional objects.
The Rust models live in `common` for reuse; their presence does not imply
the state/order messages are already implemented.

| Surface | Schema | Specification / library decision |
| --- | --- | --- |
| Required fields | Description says "all fields are optional"; `required` includes header and `referenceStateHeaderId` | Section 7.9 marks only the four body objects optional, using section 7.1's italic convention. Require the header/reference counter; allow a body with none of the four objects. |
| Counters and degree | Counters lack bounds; degree has minimum 1 but no maximum | Sections 7.2/7.8/7.9 specify `uint32`; use `u32` and enforce degree >= 1 on encoding/decoding. |
| Floating-point values | JSON numbers | Section 7.1.4 excludes NaN/infinity. Use `f64` fields, checking finite values on encoding and decoding, including non-JSON Serde input. Prevent `serde_json` from silently encoding non-finite floats as null. |
| Angles | Bounds are decimal `+-3.14159265359`, slightly beyond binary64 pi | Section 7.8 uses `[-Pi, Pi]`; use `std::f64::consts::PI` endpoints, not the schema's rounded extension. Test that deliberate difference. |
| Localization score / deviation | Score `[0,1]`; deviation minimum 0 | Score matches section 7.8. The prose gives no explicit lower bound for deviation; retain the schema's nonnegative constraint and record this schema-only assertion. |
| Control-point weight | Any number; no bounds | Section 7.8 (also 7.3) specifies strictly positive finite `float64`; enforce the prose bound on both wire directions. |
| Waypoint ETA | String without a `format` assertion | Section 7.8 explicitly requires the timestamp format. Reuse the strict header timestamp adapter, with source-cited tests. |
| Optional defaults | Degree, knots, weight may be omitted | Preserve absence using `Option`; do not compute knots or insert degree/weight defaults. Explicit null is not allowed. |
| Object shape | Object types | Each public model owns object-only decoding, equally for standalone, nested and array-element use. Positional arrays are not JSON objects. |
| Cross-field geometry | Counts/multiplicities mentioned in descriptions, not schema assertions | Sections 6.8 and 7.8 require paths starting at current position, base coverage, enough control points and valid NURBS knot relationships. These checks remain deferred, with acceptance-boundary tests. |

Numeric adapters enforce field-level constraints only at the Serde boundary;
public fields remain directly usable `f64`/`Option<f64>` values. Direct
construction/mutation can produce invalid values, which encoding rejects.
No new dependencies are required by this increment.

## Post-increment correctness review

The schema validator enables `serde_json/float_roundtrip` transitively during
library tests. Without that feature in normal builds, an independent consumer
reproduced `8.971135692166839e-150` decoding as `8.97113569216684e-150`.
The normal dependency now enables `float_roundtrip` explicitly, with text/byte
bit-preservation regressions and a separate consumer CI job that does not
inherit the library's dev-dependencies.

Object-only decoding originally lived in parent-field adapters, leaving
standalone shared models permissive. A small private declarative macro now
reuses each handwritten model's field metadata for a map-only deserializer,
without external generation, duplicate maintained definitions, or a second
vector allocation for object arrays. Public field types and wire names are
unchanged. Shared numeric adapters add wire-field context to errors; they do
not provide full nested error paths.

Section 7.8's `localized: false` entails error reporting and stopping automatic
driving. A message type cannot verify or perform these behaviors. Likewise,
the presence/existence of a referenced state, update rate, and robot-specific
path-sharing responsibilities are outside these types. Tests intentionally
show those semantic-invalid combinations remain representable.

## Instant actions findings and choices

Sources: sections 6.2.1, 7.2, 7.3/7.3.1 and 7.4 at the pinned revision.

| Surface | Schema | Specification / library decision |
| --- | --- | --- |
| Header | Unbounded integer; broad `date-time` | Reuse the shared `uint32` and strict millisecond UTC checks from sections 7.2/7.4. |
| Action model | Inline action permits only `NONE` | Section 7.4 references the shared section 7.3 action definition (four blocking types); section 6.2.1 specifically requires `NONE` for instant actions. Reuse `Action`/`BlockingType` and check the instant context during encoding and decoding, reporting the action index. Direct construction remains unchecked. |
| Action names | String, no enum | Standard action names do not exclude vendor functions. Use `String`; do not reject custom action names. |
| Action IDs | String; UUID suggestion | Section 7.3 also only suggests UUID. No UUID dependency or UUID syntax check. Uniqueness is semantic validation and deferred. |
| Parameter value | Required union of array, boolean, number/integer, string, object | Matches section 7.3.1. Checked wrapper rejects top-level null; nested array/object contents are unconstrained, so preserve nested nulls. |
| Numeric parameter values | JSON numbers without size assertions | Delegate representation to `serde_json::Value` and its signed/unsigned integer and float support. Preserve tested integer endpoints and float text round-trips; do not claim arbitrary precision. |
| Actions list | Required, no `minItems` | Section 7.4 gives no explicit minimum count. An empty list remains representable. |
| Optional data | Descriptor and parameters may be absent, not null | Reuse strict optional handling; distinguish absence from an empty parameter list. |
| `retriable` | Missing from the instant schema; additional properties are unconstrained | Section 7.4 reuses section 7.3's action definition, including optional boolean `retriable`. Follow the specification: preserve absence/false/true, reject null and other types. This closes a shared-model omission found during the order audit. |

Action lists, action objects and parameter objects use the same object-only
and non-null rules as existing messages. Extra object properties are ignored,
without affecting arbitrary content inside the parameter's value.
Robot capability checks, parameter meanings, action-ID uniqueness, ordering/
execution and conflicts with active orders remain outside parsing.

## Order findings and choices

Sources: sections 6.1, 7.1.4, 7.2, 7.3 and 7.3.1 at the pinned revision.

| Surface | Schema | Specification / library decision |
| --- | --- | --- |
| Rotation speed name | Edge property `maxRotationSpeed` | Section 7.3 uses `maximumRotationSpeed`. Follow the specification with no decoding alias; the schema spelling is ignored by the normal unknown-field policy. Reported upstream in [PR #671](https://github.com/VDA5050/VDA5050/pull/671); do not change the pinned schema or move the provenance revision. |
| Rotation speed conformance | The specification key is accepted as an unconstrained additional property | Test the specification key's type/finite checks explicitly. Also project the fixture's property onto the schema spelling to check its declared schema type. No blanket claim that the schema validates this field. |
| Integers | Nonnegative counters without upper bounds; degree minimum 1 | Use specification uint32 widths, reusing shared header and degree checks. Reject floating-point integer tokens as for existing messages. |
| Node/edge graphs | Required arrays; no minimum/count/sequence assertions | Preserve required fields and array types. Single-node orders may have empty edges/actions. Nonempty nodes, `nodes = edges + 1`, even/odd continuous sequence and base/horizon rules remain semantic validation. |
| Node position | Optional object, required x/y/mapId; optional theta/tolerances | Keep the order-specific model in `order`; state schema positions omit tolerances and will need their own audit. Preserve exact `allowedDeviationXY` spelling. |
| Node/edge orientation | Schema bounds rounded beyond mathematical pi | Reuse the specification-oriented `[-pi, pi]` check, with explicit discrepancy tests. Edge range is a schema assertion; section 7.3 does not give an explicit edge range. |
| Orientation tolerance | Schema maximum `3.141592654` | Section 7.3 explicitly gives `[0, pi]`. Enforce the mathematical bound; absent means absent, not an inserted zero. |
| Position tolerance ellipse | Nonnegative a/b; theta `[-1.570796327, 1.570796327]` | Section 7.3 describes axis lengths and rotation but no explicit rotation range. Retain the schema's exact decimal bound (not a substituted pi/2). Do not validate cross-field `a >= b`. |
| Corridor | Nonnegative widths; optional closed enums/booleans | Matches section 7.3. Preserve omission rather than materializing KINEMATIC_CENTER, false or STOP defaults; no approval or movement behavior is performed. |
| Speeds/heights/length | Unbounded JSON numbers | Section 7.3 gives float64 without explicit positivity bounds. Enforce finite representation, not invented nonnegative limits. |
| Actions/trajectory | Shared action and NURBS definitions | Reuse `common`, including optional non-null `retriable` and all four blocking types. Retain existing geometry/parameter acceptance boundaries. |
| Legacy edge fields | No startNodeId/endNodeId/rotationAllowed fields | Do not introduce older-version fields. Edges connect nodes through sequence; description text mentioning rotationAllowed is stale, while the actual property is reachOrientationBeforeEntering. |

The vendored order schema was exported from the pinned commit, not the sibling
checkout's later typo-fix branch. No runtime or build depends on that checkout.
Tests cover standalone/nested object consistency, all enum values, required
and optional presence, scalar float and uint32 boundaries in both wire
directions, unknown properties, and explicit semantic-invalid acceptance.

## Responses findings and choices

Sources: sections 7.2 and 7.5 at the pinned revision.

| Surface | Schema | Specification / library decision |
| --- | --- | --- |
| Header ID | Integer with `format: uint32`, without explicit minimum/maximum | Reuse `Header` and enforce the specification's uint32 width. The custom schema format is not relied on to enforce bounds. |
| Header timestamp | Broad `date-time` format | Reuse strict UTC millisecond spelling from sections 7.2/7.5, not every RFC 3339 spelling. |
| Response list | Required array, no minimum length | Represent with `Vec<Response>`; preserve an empty array. |
| Decision | GRANTED, QUEUED, REVOKED, REJECTED | Matches section 7.5. Closed `GrantType` enum, exact uppercase encoding, no unknown-value fallback. |
| Expiry | Optional string, without a `date-time` assertion | Section 7.5 explicitly specifies the timestamp format. Use optional `DateTime<Utc>` and a shared strict timestamp adapter in both directions; no truncation on output, no explicit null on input. |
| Expiry condition | No conditional assertion | Section 7.5 permits expiry only for GRANTED. As already agreed, defer this cross-field validation; test acceptance of other decisions with expiry without claiming protocol validity. |
| Request semantics | String ID | Active-request existence/uniqueness, grant transitions and freshness require external state and remain outside parsing. |

The models use object-only decoding and the existing unknown-field policy.
All four decisions, field presence/null rules, strict optional timestamp
boundaries, integer widths, standalone/nested shapes and JSON helper paths
have source-cited tests. No new dependency is required.

## State findings and choices

Sources: sections 7.1.4, 7.2 and 7.8 at the pinned revision.

| Surface | Schema | Specification / library decision |
| --- | --- | --- |
| Required fields | Header, order/last-node IDs and counters, node/edge arrays, driving, action/instant-action arrays, power supply, operating mode, errors, safety | Matches section 7.8. Required arrays may be empty; no missing-field defaults. Optional collections preserve unknown/absent versus present/empty. |
| Counters | Integers without explicit upper bounds | Follow specification uint32, including edge-request sequence. An empty lastNodeId makes its sequence semantically irrelevant, not exempt from the wire width. |
| Battery health | Number in `[0, 100]` | Section 7.8 says int8 in `[0, 100]`. Use `Option<i8>` with range checks on both encoding and decoding; reject fractional and floating-point tokens. |
| Battery range | Nonnegative number | Section 7.8 says uint32. Use `Option<u32>`; schema-valid fractions and overflow are rejected. |
| Power floats | Charge `[0, 100]`; voltage/current unbounded numbers | Use finite float64 and charge range; preserve signed current without adding positivity rules. Permanently powered charge and charging/availability implications remain semantics. |
| Shared geometry | Same paths, trajectories, robot position and velocity as visualization | Reuse `common` models and checks, including prose-only positive NURBS weights, strict ETA timestamps and deferred geometry constraints. |
| Node position | x/y/mapId, optional theta; no order tolerances | Separate `state::NodePosition`, not the larger order type. Same specification `[-pi, pi]` angle check, stricter than rounded schema bounds. |
| Load | Every field optional; dimensions nonnegative; mass nonnegative | Empty load object is representable. Distinguish absent loads (unknown) from an empty load array (known unloaded). Bounding-box x/y/z/theta are finite; neither source gives theta an explicit angle range. Dimension lower bounds are explicit schema assertions. |
| Zone requests | ACCESS/REPLANNING; optional trajectory | Enum agrees with specification, but the trajectory prose mentions COORDINATED_REPLANNING, which is not listed in either enum. Agreed to retain the two listed values, reject the unlisted value and defer trajectory applicability checks. |
| Closed enums | Map/zone statuses, operating modes, request types/status, action status, severity, info level and emergency stop | Preserve every listed value, including STARTUP, RETRIABLE, URGENT and CRITICAL. No unknown fallbacks. |
| Diagnostic types | errorType/infoType are strings | errorType is explicitly extensible; preserve custom names. Reuse a `Reference` model for identical error/info reference fields. Translation strings are preserved, without an ISO 639-1 validator. |
| Action result | actionResult property | Matches section 7.8's detailed action-state definition. Root descriptions referring to resultDescription/resultDescriptor are stale; do not add aliases. |
| Behavioral descriptions | Some differ from current prose (driving/rotating, instant-action retention/reset) | Types represent booleans and action lists, not execution or retention logic. Do not infer robot behavior from successful decoding. |

The state implementation uses private supporting modules with public
`state::...` re-exports. State-specific models are not prematurely moved into
common or coupled to unimplemented factsheet/zoneSet bodies.

Tests require the full fixture to contain every schema-declared property and
exercise omission/null/type rules recursively through schema references. They
also cover all enum values, direct public object decoding/duplicates, both
serialization directions for numeric limits, independent consumer float
precision, and explicit semantic-invalid acceptance (duplicate enabled maps,
inconsistent traversal and safety/driving reports, and unvalidated requests).
Capability-dependent zoneActionStates presence, unique active IDs, grant/
action transitions and physical safety remain outside deserialization.

## Factsheet findings and choices

Sources: sections 7.1.4, 7.2, 7.8 (referenced load models) and 7.10.

| Surface | Schema | Specification / library decision |
| --- | --- | --- |
| Root | Six required capability objects; configuration optional | Matches section 7.10. Preserve empty permitted sections and omitted optional values; no inferred defaults. |
| Open values | String kinematics, class, localization/navigation and wheel types | Explicitly extensible in prose. Use strings, preserving vendor-defined names. Supported zone types, action scopes, parameter support and value-data types are closed enums. |
| Array limits | Literal dotted property names | Preserve `order.nodes`, `actions.actionsParameters`, `information.infoReferences` and all other keys exactly. Even the unusual `actions.actionsParameters` spelling appears in both sources; do not silently correct it. Use documented Rust field names with explicit wire renames. |
| Limit semantics | Optional nonnegative integers without upper bounds | Section 7.10 specifies uint32. Zero/omission means no explicit limit; preserve the distinction rather than materializing limits. Advertised limits are not enforced against other messages. |
| Timing | Nonnegative numbers | Section 7.10 specifies float32. Use f32, checking sign before narrowing so negative underflow cannot bypass the bound, and rejecting non-finite input/output. Normal float32 rounding is expected, including the shortest decimal spelling of f32::MAX; this is not arbitrary-precision storage. |
| Minimum charging time | Nonnegative number | Specification says uint32 seconds. Reject fractional, floating-point integer tokens and overflow as for existing integer fields. |
| Physical and handling floats | Some minima zero, others unbounded | Enforce stated schema bounds and finite float64, not invented sign/angle restrictions. Nonnegative minima for mass, speed, angular speed, acceleration, handling height and pick/drop time are explicit schema assertions. Deceleration, dimensions of the robot/wheels, displacement, handling depth/tilt and wheel theta have no explicit sign/range bounds. Charging percentages have schema `[0, 100]` bounds. |
| Load geometry | Factsheet dimensions omit the state schema's zero minima | Section 7.10 explicitly references state load dimensions. Reuse the same nonnegative model; schema-valid negative dimensions are deliberately rejected. Move geometry into common while retaining state re-exports, rather than duplicating or coupling factsheet to state internals. |
| Action parameters | Capability definitions with key/valueDataType, not runtime key/value | Use separate factsheet types; share only the existing BlockingType enum. No scope/blocking compatibility or action-parameter semantics are validated. |
| 3D data | Optional unconstrained JSON object | Agreed `Option<serde_json::Map<String, Value>>`: reject root null/arrays, preserve arbitrary nested keys/nulls and raw object shape. This data is intentionally opaque, unlike ignored unknown properties in typed models. |
| Formats and network | Free strings | Do not validate/fetch format-specific data, URLs, IPs or DNS names. No network logic is introduced. |
| Geometry/defaults | Simple closed envelopes and omitted defaults described in prose | No polygon validation, wheel applicability checks, threshold ordering or capability consistency. Preserve absent center displacement/timing defaults rather than inserting values. |

Full/minimal fixtures and source-cited tests cover every schema field, optional
null/type rules, standalone object/duplicate-field behavior, every closed enum,
open extensions, arbitrary object data, numeric input/output bounds, float32
precision and independent consumer behavior. The schema is unchanged at the
original revision. No dependency was added.

## Zone-set findings and choices

Sources: sections 6.4.1-6.4.4, 7.2 and 7.6.

| Surface | Schema | Specification / library decision |
| --- | --- | --- |
| Message nesting | Header and required `zoneSet` object | Preserve nesting with `ZoneSet` and `ZoneSetDefinition`; do not flatten map/set IDs into the header. Required `zones` may be empty. |
| Zone categories | Ten closed values with conditional required properties | Agreed flattened `ZoneKind` enum: only the selected type's parameters are modeled, and all required parameters must be present. Unknown/irrelevant fields are ignored, not preserved or interpreted. This deliberately does not retain arbitrary extra parameters that the schema permits on other types. |
| RELEASE | `releaseLossBehavior` required, STOP/CONTINUE/EVACUATE | Section 7.6 also says required; section 6.4.1 describes STOP if undefined. Follow the former and schema (agreed), reject absence/null; no silent default. |
| ACTION | Three required action arrays, which may be empty | Separate `ZoneAction`: no transmitted actionId, because the robot generates it. Share BlockingType, ActionParameter and checked ActionParameterValue. Optional retriable/default false and non-null rules match both sources. No added blocking or capability restrictions. |
| PRIORITY/PENALTY | Required factors in `[0, 1]` | Matches section 6.4.1.2. Enforce finite float64 and both bounds on encoding/decoding. |
| SPEED_LIMIT | Required number without lower bound | Section 6.4.1.1 specifies float64 and units, but no explicit numeric minimum. Keep finite values without adding nonnegativity, as for existing order speeds. Physical suitability is not validated. |
| DIRECTED/BIDIRECTED | Finite numeric direction, differing closed limitation enums | Match section 6.4.1.2; no angle bound is stated. STRICT exists only for DIRECTED. Preserve both exact enum sets. |
| Polygon | Required vertices, `minItems: 3`, finite x/y | Section 7.6 explicitly requires rejection below three vertices. Enforce count on encoding and decoding (agreed); direct Rust mutation remains unchecked. Reuse common Vertex, preserving factsheet's existing public path. Coordinates have different contextual reference frames. |
| Geometry/identity | No assertions for polygon topology, orientation, IDs or map bounds | Section 7.6 requires simple counterclockwise, implicitly closed polygons; section 6.4.2 requires unique/immutable set IDs, zone IDs and map containment. These remain deferred semantic rules, tested as representable rather than enforced. |

Full/minimal fixtures cover every declared field and all ten conditional shapes.
Tests cover required/optional/null/duplicate/unknown fields, all enum values,
three-vertex boundaries in both directions, numeric boundaries/non-finite output,
strict headers and text/byte precision. Independent consumer checks include zone
parameters and shared vertices. No dependency or transport behavior was added.
