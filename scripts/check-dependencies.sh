#!/usr/bin/env bash
set -euo pipefail

# Include build dependencies and all target platforms, but exclude dev edges.
graph=$(cargo tree --locked --edges normal,build --target all --prefix none --format '{p}' "$@")
printf '%s\n' "$graph"
if printf '%s\n' "$graph" | awk '$1 == "jsonschema" { found = 1 } END { exit !found }'; then
    printf '%s\n' 'error: jsonschema must remain dev-only; found in the normal/build dependency graph' >&2
    exit 1
fi
