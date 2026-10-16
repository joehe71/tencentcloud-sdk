# Test coverage

## Required gate

Workspace production-code **line coverage must be at least 90%**, measured with all features enabled. This is an aggregate workspace requirement, not a claim that every file, function, branch, or feature combination reaches 90%.

```bash
cargo install cargo-llvm-cov --locked
rustup component add llvm-tools-preview
bash scripts/check-coverage.sh
```

The script runs `cargo llvm-cov --workspace --all-features --locked --fail-under-lines 90`. Coverage below 90%, a test failure, or a missing tool fails the command. It does not install tools, change a toolchain, contact cloud APIs, or create GitHub Actions. Cargo may download missing dependencies unless `--offline` is supplied.

The only source exclusions are files below `tests/` and the offline `benchmarks.rs` harness. Internal unit tests live under `crates/core/src/tests/`; they retain access to private implementation details but their code is excluded from the coverage denominator. No production module is excluded or marked as coverage-ignored. The facade has no executable statements to count.

## SSM addition (2026-10-16)

With COS, SES and SSM enabled, workspace production-line coverage is **95.72%** (2,793 / 2,918), function coverage **92.20%** (461 / 500), and region coverage **86.19%** (1,541 / 1,788), under the same Rust 1.85.0/LLVM setup. All measured SSM implementation lines are covered; no production exclusions were added. There are 276 passing offline tests plus five compile-only documentation examples. SSM adds 54 separate model tests, 24 action-routing tests and seven hand-written scenario tests covering secret/SSH redaction, binary pass-through, explicit encryption parameters, no hidden read cache, deletion/error non-replay, page options, bounded bodies and malformed responses. Line coverage is not branch coverage; macro/derive source-mapping caveats remain unchanged.

## SES addition (2026-10-16)

With the SES product enabled, the workspace reports **95.33% production-line coverage** (2,553 / 2,678), **91.08% function coverage** (398 / 437), and **85.68% region coverage** (1,472 / 1,718), using the same Rust 1.85.0/LLVM toolchain. The SES implementation's measured lines are all covered. The 191 passing offline tests include 93 separate SES model tests, 36 separate action-routing tests and four independent SES scenario tests; four documentation examples compile separately. No production exclusions were added. Derived serialization and macro-generated code are subject to LLVM's normal source mapping; exact routing and populated-field fixtures are verified independently of whether those expansions appear as separate coverage rows.

## COS addition (2026-10-16)

After adding the optional COS product and core COS signer, the same Rust 1.85.0/LLVM measurement reports **94.25% production-line coverage** (2,164 / 2,296), **88.06% function coverage** (295 / 335), and **84.71% region coverage** (1,363 / 1,609). COS's own implementation covers 641 / 651 lines (98.46%); the optional core signer covers 166 / 174 (95.40%). No production exclusions were added. Verification includes 58 passing offline tests plus three compile-only documentation examples. Coverage statistics do not include documentation examples.

## Core baseline before COS

Measured on 2026-10-16, macOS ARM, Rust 1.85.0 with matching LLVM 19.1.7 tools and cargo-llvm-cov 0.8.7, using the current locked dependencies:

| Metric | Covered / total | Coverage |
| --- | --- | --- |
| Production lines | 1,357 / 1,471 | **92.25%** |
| Functions | 177 / 205 | 86.34% |
| Regions | 855 / 1,036 | 82.53% |

The comparable production-line baseline before new tests was **86.88%** (1,278 / 1,471). The earlier 89.27% reading counted inline unit-test bodies and must not be used as the production baseline.

Line coverage does not establish branch coverage. Stable-toolchain branch coverage was not measured, and a successful threshold check is not proof that every failure mode is tested. Remaining gaps include platform-specific profile discovery, invalid-Unicode environment inputs, defensive/unreachable failures, and some HTTP/diagnostic paths. Coverage here describes macOS ARM execution, not Windows or Linux validation.

New tests exercise public environment and default-chain entry points in isolated child processes, builder environment selection and shared transport wiring, and SDK/service error formatting, redaction, categories, and source preservation. Child processes receive only explicit fake environment credentials plus the LLVM profile path, never the parent's real credentials or proxy configuration. Existing offline tests cover signing, encoding, response errors, bounds, timeouts, role refresh, cancellation, projected-token rotation and local-loopback HTTP. No real Tencent Cloud endpoint is called.

## Reports and reproducibility

```bash
bash scripts/check-coverage.sh --offline --show-missing-lines
bash scripts/check-coverage.sh --offline --html --output-dir target/coverage
bash scripts/check-coverage.sh --offline --json --summary-only --output-path target/coverage/summary.json
```

With a Homebrew Rust installation lacking LLVM coverage tools, use a matching LLVM toolchain rather than Apple's unrelated system LLVM. This measurement reused an isolated toolchain already installed in this repository; it did not modify the global Rust installation:

```bash
export RUSTC="$PWD/target/msrv/rustup/toolchains/1.85.0-aarch64-apple-darwin/bin/rustc"
export RUSTDOC="$PWD/target/msrv/rustup/toolchains/1.85.0-aarch64-apple-darwin/bin/rustdoc"
export LLVM_COV="$PWD/target/msrv/rustup/toolchains/1.85.0-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/bin/llvm-cov"
export LLVM_PROFDATA="$PWD/target/msrv/rustup/toolchains/1.85.0-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/bin/llvm-profdata"
bash scripts/check-coverage.sh --offline
```

The gate uses source-based instrumentation. Regenerate reports after changes; do not treat these recorded percentages as a permanent guarantee. Maintain the gate locally alongside formatting, Clippy, tests with/without default features, and rustdoc checks.
