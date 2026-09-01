# Changelog

All notable changes to Supply Chain Control Plane are documented here.

## [Unreleased]

### Added
- Test-first Rust disruption-impact domain core with evidence-backed events and cycle-safe deterministic downstream reachability.
- Deterministic shortest dependency-path explanation for every reported potential impact.
- Immutable source evidence on every admitted dependency edge and edge-by-edge evidence returned with impact paths.
- Originating disruption-event evidence returned with every impact record for end-to-end auditability.
- Equal-length multi-route determinism, evidence-alignment, accessor, and operator-error regression coverage.
- Replay-safe item upserts for supply nodes, dependency facts, and disruption events with explicit inserted/unchanged outcomes and conflict-on-change semantics.
- Regression coverage for exact retries, immutable-evidence conflicts, node-kind conflicts, event-content conflicts, and upsert invariant failures.
- Test-first evidence-locator debug redaction so routine `Debug` output retains the semantic source-record key while never emitting the potentially sensitive locator.
- Product, technical, architecture, security, research, and commercialization baselines.
- Pinned Rust toolchain and pull-request quality workflow.
- Apache License 2.0 source grant with matching Cargo package metadata.
- Product-first README with purpose, quickstart, product/integration boundary, maturity, documentation map, contribution guidance, and licensing.

### Changed
- Evidence locators remain available to authorized drill-through code through the explicit accessor, but default Rust debug output now emits `<redacted>` instead of locator content.
- Future persistence is now required to preserve the tested item-level replay/no-op/conflict contract rather than relying on last-write-wins behavior.
- Commercialization tracking now separates the implemented domain idempotency contract from still-open durable temporal persistence, concurrency, migration, and recovery evidence.
- Commercialization tracking continues to separate the resolved source-license decision from the still-open immutable release, SBOM, signing, and provenance work.
