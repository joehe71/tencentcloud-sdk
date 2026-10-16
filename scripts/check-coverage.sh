#!/usr/bin/env bash
# Enforce workspace production-line coverage without counting tests or benchmarks.
set -euo pipefail
cd "$(dirname "$0")/.."
cargo llvm-cov --workspace --all-features --locked \
  --ignore-filename-regex '(/tests/|/benchmarks\.rs$)' \
  --fail-under-lines 90 "$@"
