# Workspace Rules

## Language

- README files, AGENTS.md, code comments, Rust documentation comments, package descriptions, and SDK-generated diagnostic messages must be written in English. Preserve official service messages and protocol values verbatim; existing Chinese design documents may remain in Chinese.

## Purpose and scope

- Use this workspace to develop, maintain, and test Tencent Cloud SDK bindings, not for routine cloud resource operations.
- Treat the official Tencent Cloud API documentation as authoritative. Keep application-specific business workflows outside the SDK.
- Follow Rust language rules and idioms throughout design and implementation. Use the Java SDK only as a protocol, field, and feature reference; do not copy its class hierarchy, reflection, getters/setters, or synchronous call structure.

## Tests and live APIs

- Run local tests by default; do not call real Tencent Cloud APIs.
- Use mocks, fixed fixtures, or local simulated services, with fake test credentials.
- Existing Tencent Cloud credentials in the environment do not authorize live API calls.
- Live integration requires separate, explicit user authorization. Explicitly confirm operations that change resources or incur charges.
- Prioritize signing, request serialization, response parsing, error mapping, and pagination. Keep time, randomness, and other test inputs controllable.

## Credential security

- Never hard-code secrets or put real credentials in logs, test fixtures, or Git.

## APIs and error handling

- Explain the impact before changing public APIs, default behavior, or dependency versions. Do not introduce unrelated breaking refactors.
- Preserve Tencent Cloud error codes and RequestId.
- Make retry policies explicit. Do not retry requests with potentially duplicate side effects by default.

## Changes and verification

- Reuse existing implementations and minimize third-party dependencies.
- Avoid unrelated refactors. Do not commit or push changes without user authorization.
- Every Git commit must include a blank line followed by `Co-authored-by: Finch <noreply@finchwork.app>`.
- Report what changed, which tests actually ran, and what remains unverified.
- Workspace production-code line coverage must be at least 90% with all features enabled. Run `bash scripts/check-coverage.sh`; exclude only tests and benchmarks, never production modules. Keep unit tests in separate files under `src/tests/` so test bodies cannot inflate coverage. Report function/region coverage separately; line coverage is not branch coverage.
- Do not present passing local tests as successful live API verification.
