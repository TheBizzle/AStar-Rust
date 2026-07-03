#!/usr/bin/env bash
set -euo pipefail

echo "=== rustfmt ==="
cargo fmt --check

echo "=== Compile ==="
cargo check

echo "=== Clippy ==="
cargo clippy -- -D warnings

echo "=== Tests ==="
cargo test

echo ""
echo "All checks passed."
