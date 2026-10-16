# Contributing

Keep changes focused on transport-independent VDA 5050 v3.0.0 models and the
agreed topic helpers. MQTT clients, network behavior and fleet-control logic
are outside this library's scope.

## Changes and source evidence

- Discuss new APIs, dependencies and validation policies with the maintainer
  before implementation.
- Use the pinned sources in [the provenance audit](https://github.com/MFait/vda5050-v3-types/blob/main/docs/provenance.md).
  Do not silently update vendored schemas to a moving upstream branch.
- Document schema/specification discrepancies and distinguish wire checks from
  semantic/protocol validation. Preserve upstream attribution.
- Add regression tests for affected messages, exact wire names, optional/null
  behavior, numeric boundaries and feature gates as applicable.
- Update public documentation and examples with behavior changes.
  Feature-specific README snippets mirror the runnable files in `examples/`;
  `tests/readme_examples.rs` checks that they match and executes them when their
  required features are enabled. Keep these snippets free of rustdoc hidden lines
  so they also display correctly on GitHub and crates.io.
- Keep dependencies minimal and compatible with Rust 1.85. Schema validation
  tooling belongs only in dev-dependencies.

## Local checks

From a repository checkout, run formatting and the checks relevant to your change:

```sh
cargo fmt --all --check
cargo fmt --manifest-path tests/consumer/Cargo.toml --check
cargo test --locked --all-features
cargo clippy --locked --all-targets --all-features -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --locked --no-deps --all-features
bash scripts/check-dependencies.sh --all-features
cargo run --locked --manifest-path tests/consumer/Cargo.toml
cargo clippy --locked --manifest-path tests/consumer/Cargo.toml -- -D warnings
```

CI runs tests, Clippy and rustdoc on stable and Rust 1.85 with default features,
no default features, `serde`, `json`, `topic`, `topic,serde`, `topic,json` and all
features. Preserve those combinations; an all-features run alone does not
verify feature isolation. The independent consumer avoids library dev-dependency
feature unification that can hide JSON precision regressions.

The dependency check excludes dev edges and checks normal/build edges across
all target platforms. It rejects `jsonschema` even when introduced transitively;
it is not a general dependency allowlist or license audit.

## Licensing

Project code is licensed under the [MIT license](LICENSE). Contributions are
intended to be provided under that same license; do not contribute material
you are not entitled to license. Retain applicable third-party notices.

Vendored upstream schemas retain their separate
[upstream copyright and permission notice](tests/schemas/LICENSE.txt); the
project license does not replace it.
