# Test Strategy — Supply Chain Control Plane

## 1. Quality contract

All behavior changes are test-first. Tests protect domain meaning, scientific validity, security boundaries, interoperability, operability, and customer workflow rather than merely executing lines.

Owned production code requires:

- statement coverage 100%;
- branch coverage 100%;
- function and relevant region coverage 100%;
- public module/type/trait/function/method/endpoint/schema/error documentation 100%;
- no skipped or expected-failure test that hides an unmet required behavior;
- no source rewriting, denominator reduction, fixture deletion, or unrealistic data reduction to satisfy a gate.

## 2. Test layers

### Unit tests

Use known, minimal, deterministic cases for:

- semantic identity and validation;
- exact replay and conflict;
- temporal interval and knowledge cutoff;
- graph cycles, equal-length routes, path evidence, and limits;
- state machines and invalid transitions;
- authorization policy decisions;
- quantity/unit/numeric validation;
- model/solver termination semantics;
- serialization redaction.

Synthetic data is permitted for unit and known-truth scientific tests.

### Property tests

Properties include:

- identical input snapshot and seed produce identical output;
- graph node insertion order does not change deterministic path selection;
- exact replay never changes state;
- correction never mutates original event;
- debit/credit or conservation constraints balance where applicable;
- scenario hard constraints are never violated in a `feasible` result;
- cross-tenant reference construction always fails;
- writeback retry produces at most one external effect under a conforming adapter.

### Fuzz tests

Fuzz targets include:

- EPCIS and JSON/JSON-LD parsing;
- CSV, MHTML-derived, and adapter payload parsing;
- URI, unit, identifier, timestamp, and interval handling;
- graph admission and traversal;
- RFC 9457 problem serialization;
- event envelope validation;
- untrusted model structured output;
- compressed and nested artifact metadata.

### Integration tests

Use real services in isolated containers or equivalent environments:

- PostgreSQL migrations, RLS, concurrency, locks, conflict, and restore;
- object storage digest, encryption context, retention, and restore;
- event outbox/inbox, replay, dead letter, and ordering;
- `keyverse`-compatible OIDC and service identity;
- adapter sandbox APIs;
- OpenTelemetry collector acceptance;
- container startup, readiness, drain, and shutdown.

### Contract tests

- OpenAPI request/response and problem types
- AsyncAPI and CloudEvents envelopes
- context-graph-contracts canonical references and truth status
- EPCIS 2.0.1 normative schemas and representative examples
- released CWL owner clients and schemas
- adapter capability and idempotency contracts
- event producer/consumer compatibility

### End-to-end tests

The first buyer journey uses sanitized realistic source data and standards-conformant fixtures:

```text
shipment delay observed
→ identity and evidence admission
→ affected item/site/order path
→ quantified shortage assessment
→ two feasible and one infeasible recovery scenarios
→ approval
→ sandbox ERP writeback
→ duplicate retry
→ outcome observation
→ post-action review
```

The test verifies values, provenance, uncertainty, authorization, audit, writeback idempotency, user-visible states, and exports.

## 3. Current foundation tests

The existing Rust foundation retains regression coverage for:

- evidence-backed dependencies and events;
- unknown references and duplicate rejection;
- direct self-dependency rejection;
- cycle-safe traversal;
- deterministic shortest path and equal-length alternatives;
- event and edge evidence alignment;
- exact replay and changed-content conflict;
- evidence-locator debug redaction;
- blank dependency keys;
- exact line/function/region/branch coverage.

Future refactoring must preserve these tests or replace them with demonstrably equivalent or stronger tests before deleting the original behavior.

## 4. Temporal tests

Required scenarios:

- event occurred before the cutoff and was available before the cutoff;
- event occurred before the cutoff but was available after it;
- retrospective report of an earlier event;
- corrected event arriving after a decision;
- overlapping and open-ended business-valid intervals;
- source clock skew and timezone normalization;
- late event replay into a historical projection;
- daylight-saving and leap-second-tolerant parsing according to the selected timestamp contract;
- event and system-time order disagreement.

No historical assessment may include `available_at > knowledge_cutoff`.

## 5. Persistence and concurrency tests

- clean database install;
- supported N-1 upgrade;
- schema-ahead and schema-behind startup failure;
- two concurrent exact replays;
- exact replay racing changed-content conflict;
- overlapping validity constraint;
- outbox atomicity under transaction rollback;
- inbox duplicate delivery;
- connection interruption before/after commit;
- pool exhaustion and bounded wait;
- partition rollover and pruning;
- restore and replay without duplicate effect.

## 6. Scientific and quantitative validation

Each model class defines a validation protocol before production implementation.

### Demand and lead-time models

- rolling-origin backtest;
- RMSE, MAE, calibration, interval coverage, and bias;
- segment and horizon breakdown;
- missingness and data-revision sensitivity;
- baseline and ablation comparison;
- out-of-distribution and abstention tests.

### Inventory and allocation

- conservation and capacity invariants;
- comparison with known analytic cases;
- small-instance enumeration oracle;
- feasible/infeasible classification;
- optimality or bound verification;
- sensitivity to unit conversion, rounding, and tier boundary.

### Disruption simulation

- known parameter recovery;
- deterministic seed manifest;
- Monte Carlo standard error and interval-aware acceptance;
- rare-event behavior;
- CPU/GPU parity;
- reproducibility across supported hardware profiles.

### Supplier assessment psychometrics

- construct and criterion evidence;
- rater severity and language/group effects where applicable;
- reliability, calibration, and validity against operational outcomes;
- no use in high-stakes decisions outside the validated scope.

## 7. CPU, multithreading, and GPU tests

- CPU `f64` is the reference result.
- Thread-count changes preserve result within declared tolerance.
- GPU/WGPU/CUDA/MLX backends execute rather than skip on supported CI profiles.
- Peak memory, transfer bytes, kernel time, fallback reason, and numerical error are recorded.
- OOM reduces batch or falls back according to policy without returning a partial authoritative result.
- Thread oversubscription and nondeterministic reduction are tested.

## 8. Security tests

- token signature, audience, issuer, type, expiry, and delegation;
- cross-tenant ID guessing and RLS bypass;
- field-level and purpose-based access;
- protected evidence locator in `Debug`, logs, traces, metrics, errors, and exports;
- prompt injection and model tool escalation;
- SSRF, redirect, DNS rebinding, and metadata service access;
- malformed, oversized, nested, compressed, and polyglot artifacts;
- event replay and changed-payload conflict;
- graph path explosion and resource budget;
- stale approval, self-approval, and segregation conflict;
- writeback timeout, duplicate retry, ambiguous result, and reconciliation;
- dependency, SBOM, signature, and provenance verification.

## 9. Performance and load tests

Use k6 or an equivalent reproducible harness for APIs and browser journeys, plus Rust and database benchmarks for compute and persistence.

Measure:

- p50, p95, p99, error, timeout, and cancellation;
- queue wait and service time separately;
- cold and warm state explicitly;
- CPU, RSS, GPU/VRAM, heap, GC, database connections, locks, WAL, I/O, network, and object bytes;
- DOM nodes, JavaScript bundle, hydration, main-thread work, and interaction latency;
- graph size, fan-out, depth, path count, and event volume;
- tenant fairness and noisy-neighbor behavior.

A performance claim includes hardware, software versions, data shape, concurrency, duration, cache state, and exclusions.

## 10. UI and accessibility tests

Storybook isolates:

```text
normal
loading
empty
partial_data
stale_data
error
access_denied
conflict
offline_recovery
mobile
intermediate_width
large_dataset
```

Automated and manual checks cover:

- keyboard navigation and visible focus;
- screen-reader labels and state announcements;
- WCAG 2.2 AA contrast and target size;
- graph equivalent exact-value table;
- no color-only status meaning;
- zoom and text expansion;
- Korean, English, Japanese, Simplified Chinese, Vietnamese, Spanish, German, and French layouts;
- CJK fallback and line breaking;
- print/PDF/SVG/CSV/JSON exports;
- loading, error, retry, cancel, and permission flows;
- no nonfunctional control or decorative interaction.

## 11. Release verification

Before a merge-ready or release-ready claim:

1. re-fetch exact head and base;
2. run owned tests and coverage on exact head;
3. verify central security/review workflows on exact head;
4. confirm no unresolved review thread;
5. verify migration and contract compatibility;
6. verify SBOM, provenance, signatures, licenses, and artifact digests;
7. run restore and rollback/forward-fix rehearsal for release candidates;
8. execute the realistic buyer journey;
9. verify documentation, gap baseline, and changelog against the same revision;
10. re-check branch protection and independent approval before integration.
