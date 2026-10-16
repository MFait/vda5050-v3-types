# VDA 5050 v3 Rust types: decisions and implementation plan

Goal: a small, open-source, transport-independent library covering all eight
VDA 5050 v3.0.0 message bodies, implemented incrementally.

Agreed decisions below describe the intended design, not completed coverage.
Checkboxes in the work sections track implementation and verification.

## Agreed design

- Crate: `vda5050-v3-types`; MIT; edition 2024; MSRV Rust 1.85.
  A license does not guarantee freedom from liability.
- Handwritten public types, one module per message, top-level message types
  re-exported at the crate root, genuinely shared models in `common`.
  No redundant `v3` namespace.
- Schemas and specification are design inputs, not build/runtime dependencies.
  Pin provenance and vendor test inputs with attribution; builds/tests must
  not require a sibling checkout. Flag discrepancies rather than hiding them.
  The published VDA PDF takes precedence according to upstream; existing
  audits compare the Markdown and schemas, not an independently checked PDF.
- Shared `common::Header` is embedded in Rust and flattened on the wire.
- Chrono `DateTime<Utc>` timestamps; wire format follows section 7.2:
  `YYYY-MM-DDTHH:mm:ss.fffZ`. Reject nonconforming input/unrepresentable output
  rather than silently losing precision.
- Match specification integer widths. Use public `f64`/`Option<f64>` fields
  with finite/range checks at the Serde boundary; direct mutation is not
  validated. Factsheet timing uses specification `f32`. Numeric and timestamp
  tests cite pinned specification sections.
- Ignore unknown object fields without preserving them; reject unknown closed
  enum values and missing required fields. Check explicit extension rules
  per message. Do not hide required-field omissions with defaults.
- Optional does not mean nullable: accept `null` only when explicitly allowed.
  Optional non-nullable fields map absence to `None`, reject explicit `null`,
  and omit `None` on encoding. Preserve omitted protocol defaults.
- Shared `ActionParameterValue` wraps `serde_json::Value` with a private
  inner value, checked construction rejecting top-level `null`, read-only
  access and consuming extraction, but no mutable invariant bypass.
  Preserve permitted nested values, including nested nulls, and serialize
  without a wrapper object. Reuse across order, instantActions, and zoneSet.
- Shared actions preserve optional non-null `retriable`, following section
  7.3 and section 7.4's reference to it despite the instant schema omission.
  Order rotation speed uses specification `maximumRotationSpeed`, not the
  pinned schema typo `maxRotationSpeed`; no decoding alias.
- Zone sets use a flattened `ZoneKind` enum with required variant parameters
  (agreed). Check at least three vertices on encoding/decoding, but defer
  polygon geometry and protocol semantics. Require release loss behavior per
  section 7.6/schema and document section 6.4's conflicting absent-value default.
  ZoneAction omits actionId; reuse shared blocking types/parameters and Vertex.
- Keep dependencies minimal and discuss each addition. Chrono and `serde_json`
  are approved core dependencies; `jsonschema` is dev-only, with HTTP/file
  resolution disabled. Zero dependencies is not a goal at usability's expense.
- `serde` gates our model implementations; dependency-owned Serde support
  remains available. `json` enables `serde` and gates inherent helpers:
  `from_json`, `from_json_slice`, `to_json`, `to_json_vec`, returning original
  `serde_json::Error` values. No optional features are enabled by default.
- Initial scope is types, wire encoding/decoding, and agreed value-level checks.
  Cross-field semantic validation needs a separately agreed API.
  Tests document acceptance boundaries without claiming protocol validity.
- Exclude MQTT clients, network behavior, subscription operations, QoS/session
  management, async runtimes and fleet-control logic. Topic-filter text helpers
  are explicitly included below; message builders remain deferred.

## Current implementation

- [x] Convert the binary scaffold into a library with feature-gated Serde/JSON.
- [x] Implement connection and shared header; fixture, source-cited tests and
      schema/specification discrepancy audit.
- [x] Implement visualization with shared position, velocity, trajectory,
      control-point, planned/intermediate-path and waypoint models; full/minimal
      fixtures, numeric/null checks and source-cited tests.
- [x] Vendor all eight implemented message schemas at upstream revision
      `0b2ae43154bcd1ab2be4e2445df8b406dcad77b2` with upstream notice.
      See [provenance and audits](docs/provenance.md).
- [x] Establish [CI](.github/workflows/ci.yml): formatting, Clippy, tests/doctests
      and rustdoc with warnings denied, stable and Rust 1.85, default,
      no-default, serde, json and all-feature configurations.
- [x] Add README examples, feature/coverage documentation and docs.rs settings.

Current baseline (all message bodies and topic helpers): 1 unit test,
152 integration tests and 18 doctests pass with all features, plus the independent
consumer regression.
Stable and Rust 1.85 pass all implemented feature configurations, Clippy and
rustdoc.

## Completed: correctness and refactoring before a third message

### Priority 1: float precision and consumer coverage

- [x] Explicitly enable `serde_json/float_roundtrip` for normal builds.
      Review reproduced a text encode/decode precision change in an independent
      consumer: `8.971135692166839e-150` became `8.97113569216684e-150`.
      The dev-only schema validator previously enabled this feature during
      tests, masking consumer behavior through feature unification.
- [x] Add a regression using that value and representative float64 boundaries
      through actual JSON text/bytes, not only `serde_json::Value`.
- [x] Add an independent consumer check in CI without this crate's
      dev-dependencies, so test-only dependency features cannot mask behavior.
      Run with `cargo run --locked --manifest-path tests/consumer/Cargo.toml`;
      the consumer has its own lockfile and stable/MSRV CI job.

### Priority 2: consistent object decoding

- [x] Make public object types own their object-shape decoding rules, so
      standalone and nested decoding agree. Review confirmed
      `serde_json::from_str::<Velocity>("[]")` previously succeeded, while the
      same array nested under visualization was rejected. A small private
      `object_only_deserialize!` macro reuses handwritten fields/attributes for type-owned
      map-only decoding, without maintaining duplicate model definitions.
- [x] Test standalone and nested object models against positional arrays,
      nulls, required-field omissions, duplicates and valid object input.
- [x] Once child types enforce their own shape, simplify parent adapters and
      remove the temporary wrapper-vector/second allocation in `object_vec`.
      Preserve public types, wire names and agreed null behavior.

### Priority 3: scalable tests and diagnostics

- [x] Extract small shared test helpers for schema loading, pointer mutation,
      required fields and optional/null behavior; cache compiled validators.
      Avoid a generator or elaborate testing framework.
- [x] Add direct shared-model/adapter regressions in `tests/common.rs` and
      unit tests; reuse field-presence helpers across message tests. Retain
      existing message-level numeric cases as end-to-end integration coverage.
- [x] Clearly separate schema assertions, stricter specification-derived rules
      and intentionally unenforced semantics. Keep pinned source citations.
- [x] Improve numeric failure context with wire-field names, without a new
      dependency. Full nested error paths are not provided.
- [x] Re-run the full feature/MSRV matrix, formatting, Clippy and rustdoc after
      refactoring, preserving connection and visualization behavior except for
      the documented correctness fixes.

## Remaining messages: repeat this workflow per increment

For each message: compare the pinned schema with matching specification
sections, discuss new design choices, record discrepancies, implement shared
models where appropriate, vendor inputs with notices, and add fixtures/tests.
Do not assume a schema alone defines the entire protocol.

- [x] instantActions: shared Action/BlockingType/ActionParameter, checked
      non-null JSON wrapper, all helper APIs, pinned schema/fixture and
      source-cited tests. Shared blocking types are reused, but instant-message
      encoding/decoding requires NONE (agreed after the section 6.2.1 audit).
      Action names remain open strings; uniqueness/support/conflict checks
      stay deferred. No new dependency.
- [x] order: required node/edge graph, positions/tolerance ellipses, corridors,
      enums and all optional fields; shared header/actions/trajectory, full and
      minimal fixtures, source-cited wire/numeric/semantic-boundary tests.
      Pinned original schema retained; specification rotation-speed spelling
      chosen and typo reported in upstream PR #671. No new dependency.
- [x] responses: shared header, response list and four grant decisions;
      optional non-null strict expiry timestamps, pinned schema/fixture,
      source-cited wire tests and explicit deferred expiry/grant relationship.
      No new dependency.
- [x] state: all root/nested fields, shared motion/path models, distinct state
      node position, loads, maps/zone statuses, requests, action states, power,
      diagnostics/translations and safety. Specification int8 battery health
      and uint32 range; listed ACCESS/REPLANNING request types despite conflicting
      trajectory prose (agreed). Full/minimal fixtures, pinned schema, complete
      field/enum coverage, numeric boundaries and explicit semantic limits.
      No new dependency.
- [x] factsheet: all capability sections/configuration, explicit dotted limits,
      float32 timing and uint32 charging time, open/closed enums, preserved
      arbitrary JSON-object envelope data (agreed), shared load geometry with
      existing state paths preserved. Full/minimal fixtures, pinned schema,
      exhaustive presence/enum tests, numeric bounds and semantic limits.
      No new dependency.
- [x] zoneSet: nested set definition, ten typed zone variants (agreed), exact
      conditional parameters, closed enums, separate action templates with
      shared parameters/blocking types, shared Vertex with factsheet path
      preserved. Three-vertex minimum in both Serde directions, numeric bounds,
      full/minimal fixtures, pinned schema, field/branch coverage and explicit
      deferred geometry/protocol semantics. No new dependency.
- [x] Implement shared action parameters/value wrapper when first needed;
      keep the public model usable without optional features.

Every increment must cover wire names, enum values, nested variants,
required/optional fields, null handling, unknown fields, numeric boundaries,
schema-valid serialized output and text/byte round-trips. Compare JSON values,
not object-key order. Preserve the CI feature/MSRV checks.

Specification-derived cases include order graph counts/sequence continuity,
base/horizon consistency, NURBS counts/knot relationships, path start/base
coverage and `leaseExpiry` only with `GRANTED`. Until semantic validation is
agreed, tests document what remains representable rather than asserting that
these rules are enforced.

## Optional topic support: all eight planned helpers implemented

- Bodies remain independent. Model topic/body pairs, not MQTT packets or an
  invented JSON envelope; encoding returns topic text and payload separately.
- Optional `topic` feature adds no dependencies. Provide `TopicKind` with exact
  wire names and `Topic` for the suggested five-level layout from section 4.2:
  `interfaceName/majorVersion/manufacturer/serialNumber/topic`.
  Brokers may adapt this layout; final topic names are mandatory.
- Use typed message variants and checked construction to prevent topic-kind/
  body mismatches. Private topic fields preserve syntax invariants.
- Helper scope agreed below; API names/examples remain proposals until finalized.
  Subscription-filter builders expand the earlier exclusion of filters, but do
  not add an MQTT client or subscription/network behavior.

### Planned helpers

- [x] Exact topic names: `TopicKind` for all eight names, `as_str`, `Display`
      and `FromStr`; reject unknown names and incorrect capitalization.
      Usable independently of a deployment's topic hierarchy. Public unit error
      `UnknownTopicKind` implements Display/Error without retaining the input.
      Optional `topic` feature adds no dependencies or implicit features;
      with `serde`, encode/decode the same exact string names. No normalization,
      full-topic parsing or wildcard handling. Source-cited tests and CI cover
      topic alone, topic + serde and topic + json on stable/Rust 1.85.
- [x] Suggested-layout topics: checked `Topic::new(interface, manufacturer,
      serial, kind)`, FromStr, Display, read-only level accessors and string
      conversions. Private fields preserve invariants. Exactly five nonempty
      levels; fixed `v3` supplied by construction, other versions rejected on
      parsing (agreed). Reject `/`, `+`, `#`, `$`, NUL, enforce section 4.2's ASCII
      serial alphabet and MQTT's 65,535 UTF-8 byte limit. Preserve other text.
      `TopicError`/`TopicLevel` identify failures without retaining input.
      With Serde, encode/decode a checked string, not an object. Document helper
      policies separately from specification requirements and do not claim
      support for arbitrary deployment-specific hierarchies.
- [x] Body-to-kind mapping: all eight message bodies expose inherent
      `pub const fn topic_kind(&self) -> TopicKind` behind `topic`, without Serde
      or JSON. Type-only mapping does not inspect/validate headers or mutate
      bodies. Topic-only API checks and all-eight fixture mapping regressions.
- [x] Header-derived topic construction: all eight bodies expose
      `topic(interface) -> Result<Topic, HeaderTopicError>` behind `topic` alone.
      Require header.version exactly `3.0.0` (agreed), then reuse Topic::new
      with fixed kind/v3 and unchanged manufacturer/serial. Distinguish
      UnsupportedVersion from InvalidTopic(TopicError), preserving error sources.
      No body mutation, general validation or change to ordinary JSON parsing.
      Returned topic is an owned snapshot; future header edits are not tracked.
      Test all eight mappings, invalid versions/identity/interface, byte limit,
      no normalization, error sources and topic-only availability.
- [x] Message-body enum: `topic::Message` with all eight consistently boxed
      variants (agreed), Debug/Clone/PartialEq, From<Body> and From<Box<Body>>.
      Owned conversion allocates once; boxed conversion preserves allocation.
      Expose topic_kind(), header() and checked topic(interface), requiring
      only `topic`. No validation on wrapping, Serde representation or invented
      JSON envelope. Matching variants accesses the concrete mutable bodies.
      Test all eight fixture conversions, kind/header/topic correspondence,
      unchanged concrete wire bodies, clone/equality, box reuse and topic-only use.
- [x] Topic-directed JSON decoding: TopicKind::decode_json/decode_json_slice
      and matching Topic methods return Result<Message, serde_json::Error>
      with both `topic` and `json` (agreed). Select only the kind's concrete
      decoder, preserving its wire checks and original errors; no fallback,
      envelope or header identity/version checks. Parse raw topics separately.
      Success does not detect intended kind: unknown fields permit overlapping
      valid body shapes. Test all eight bodies through all four entry points,
      malformed JSON/UTF-8, error fidelity, value constraints, no envelope,
      unchecked identity/version and float precision.
- [x] Checked topic/body pairing: TopicMessage::new(topic, impl Into<Message>)
      owns private fields, checking kind, exact header.version `3.0.0`, exact
      manufacturer and exact serial equality in that order (agreed). No
      normalization/repair; PairingError identifies the first failure. Inputs
      are consumed even on failure. Read-only topic()/message(), Clone/PartialEq
      and consuming into_parts(); no mutable invariant bypass or Serde envelope.
      With `json`, to_json()/to_json_vec() return separate topic text and
      body text/bytes, propagating original body encoding errors. Pairing alone
      is not wire/protocol validation. Test all variants/kind mismatches, exact
      identity/version checks, extraction/rechecking, error context, immutable
      access compile failure, output shape, encoding errors and float precision.
- [x] Subscription-filter builders: separate checked TopicFilter, explicit
      interface and fixed v3; manufacturer/serial/kind Option selectors use
      None for whole-level + (agreed). TopicFilter::new, parsing/formatting,
      read-only accessors, string conversions and From<&Topic> exact filters.
      Exact selectors reuse topic character/serial rules and UTF-8 byte limit.
      With Serde, use checked strings. No #, partial wildcards, wildcard
      interface/version, shared prefixes, matching or broker operations.
      Test selector combinations, all kinds, malformed scope/depth/version,
      constructor/parser rules, Unicode preservation, byte limits and Serde.

### Design and verification

- [x] Finalize constructors, errors and helper signatures before implementing.
      All eight helper increments discussed and implemented.
- [x] Agree topic/header manufacturer, serial-number and version consistency
      policy separately from topic syntax and JSON parsing.
      Header-derived construction and independently supplied pair policies
      agreed/implemented above. Ordinary dispatch remains unchanged.
- [x] Test topic alone, topic + serde, topic + json, exact wire names, malformed
      levels, body mappings, header-derived topics, pairing, dispatch
      errors/round-trips, filter wildcard placement and agreed consistency checks.
      Preserve feature/MSRV, Clippy and rustdoc coverage.
- [x] Update README/API examples, feature docs and pinned provenance audit.

Deployment-specific cloud-layout frameworks and automatic topic/header
validation during ordinary body JSON parsing remain outside the agreed scope.

## Later maintenance and release readiness

- [x] Split `common` into private action/header/motion/path files, preserving
      public `common::...` paths through explicit re-exports.
- [x] Reuse the private optional-adapter macro for numeric and timestamp
      adapters; preserve omitted/nonnull behavior and existing value checks.
- [x] Turn normal-dependency inspection into a CI assertion: reusable
      scripts/check-dependencies.sh rejects jsonschema in normal/build edges,
      including transitive dependencies and all target platforms, for each
      feature/MSRV CI configuration. This is not a general package allowlist.
- [x] Add the project's MIT license text, Copyright (c) 2026 Michael Fait
      (agreed), and CONTRIBUTING.md with scope, source policy and local checks.
      Retain the vendored upstream copyright/permission notice unchanged and
      link both notices from README.
- [ ] Set the repository URL once the GitHub repository exists.
- [x] Review README/API examples, coverage and validation limits before publication.
      Correct stale coverage/action-sharing docs; distinguish body wire structs,
      opaque JSON and string-shaped topic models, and document integer-token
      strictness and the Message/TopicMessage Serde exclusions. Run standalone
      dispatch/filter examples as doctests and check rustdoc links. Continue
      keeping documentation aligned with later changes.
      Reorganized README around installation/features, small examples for each
      capability, a model guide, topic policies and wire/validation limits.
- [x] Make string-valued enum decoding reject Serde's alternate object form.
      API review confirmed that `ConnectionState` accepts `{"ONLINE":null}` and
      `TopicKind` accepts `{"state":null}`, although their wire forms are strings.
      Approved input-compatibility tightening: all 24 string-valued enums now
      reject non-string inputs, with every variant tested standalone and nested.
      Tagged `ZoneKind` is unchanged.
      Retain [vda5050-types-rs](https://github.com/kKdH/vda5050-types-rs)
      as v2 prior art, not a v3 source of truth.
- [ ] Add accurate CI/crates.io/docs.rs/license badges only when targets exist.
- [ ] Finalize publishing metadata and inspect packaged contents with
      `cargo package`; verify `cargo publish --dry-run` before publication.
- [ ] Agree versioning and release workflow before the first publication.
