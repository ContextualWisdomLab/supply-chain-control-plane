# Supply Chain Control Plane

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/ContextualWisdomLab/supply-chain-control-plane)

**Evidence-first disruption impact analysis for auditable supply networks.**

Supply Chain Control Plane turns supplier, facility, item, inventory, shipment, order, and production evidence into an explainable dependency view. Its first bounded product slice answers three questions without inventing unsupported risk scores:

1. **What can this observed disruption affect downstream?**
2. **Which admitted dependency path connects the disruption to each potential impact?**
3. **What source evidence supports the disruption and every dependency edge on that path?**

The current core deliberately reports deterministic **potential impact by graph reachability**, not probability, severity, lead time, recovery time, or optimization advice. Quantitative claims belong behind separately validated models and evidence.

## Why it exists

Supply-chain incident response often fails at the handoff between raw operational facts and action: teams may know that a supplier, facility, shipment, or inventory position changed, but not which downstream obligations depend on it or why a system believes they are exposed.

This repository establishes an evidence-preserving control-plane core where every admitted dependency and disruption remains traceable to a source reference and every downstream result carries its explanation path.

| Need | What the current core provides |
| --- | --- |
| Explainable disruption scope | Deterministic downstream reachability over explicit dependency facts |
| Evidence drill-through | Originating event evidence plus one evidence reference per path edge |
| Replay-safe ingestion boundary | Item upserts insert missing facts, no-op on exact replays, and reject same-key changed immutable content |
| Secret-safe diagnostics | Evidence locators stay available to authorized drill-through code but are redacted from default Rust `Debug` output |
| Stable results | Semantic ordering and deterministic shortest-path selection, including equal-length alternatives |
| Fail-closed graph admission | Blank, duplicate, unknown-reference, direct self-dependency, and replay-conflict rejection |
| Cycle safety | Traversal terminates without re-reporting the disrupted source |
| Commercially usable source | Apache License 2.0 source grant; third-party terms remain separate |

## Quick start

The current implementation is a Rust 1.98 domain library with no third-party runtime dependencies.

```bash
cargo test --all-targets --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo fmt --all -- --check
```

The repository quality contract also measures exact owned coverage in CI. See [`.github/workflows/ci.yml`](.github/workflows/ci.yml) for the current executable gate rather than treating these local commands as a substitute for exact-head hosted evidence.

## Product boundary

Supply Chain Control Plane currently owns the **Disruption Impact** bounded context and its supporting evidence/network admission contracts.

```text
ERP / WMS / TMS / EPCIS / other source systems
                    │
                    │ future versioned anti-corruption adapters
                    ▼
        ┌──────────────────────────────┐
        │  Supply Chain Control Plane  │
        ├──────────────────────────────┤
        │ evidence registry            │
        │ network registry             │
        │ disruption impact            │
        │ deterministic path evidence  │
        └──────────────┬───────────────┘
                       │
                       ▼
          potential impacts + evidence
                       │
                       ▼
             human / downstream action
```

It does **not** replace ERP, WMS, TMS, EPCIS, procurement, inventory, or order systems of record. It also does not currently own probability estimation, severity scoring, recovery prediction, inventory optimization, authenticated external APIs, or customer-facing workflow UI.

External source systems remain authoritative for their own operational facts. Future adapters must enter through explicit versioned anti-corruption boundaries rather than cross-system application-table access or copied source truth.

## Domain model

The current aggregate is `SupplyGraph`.

- **Supply nodes** represent disruption-relevant supplier, facility, item, inventory-position, shipment, and order identities.
- **Dependency facts** are directed and evidence-backed: a downstream node depends on an upstream node.
- **Supply events** identify an observed disruption at one node and retain an immutable evidence reference.
- **Replay-oriented upserts** insert an absent node/dependency/event, no-op only for an exact semantic replay, and fail closed when an existing identity is paired with changed immutable content.
- **Impact records** carry the potentially affected node, hop count, deterministic dependency path, event evidence, and edge-aligned dependency evidence.
- **Evidence references** retain their full locator for explicit drill-through while default `Debug` output replaces the locator with `<redacted>` so ordinary diagnostics do not disclose token-bearing or customer-sensitive source locations.

Admission fails closed for malformed semantic identity, unknown references, duplicate facts/events/nodes, direct self-dependencies, and replay conflicts. The path contract remains deterministic even when multiple shortest routes have equal length.

## What is implemented now

The current writer branch contains the first commercial foundation slice:

- Rust-first, dependency-free domain core;
- evidence-backed disruption events;
- evidence-backed directed dependency facts;
- replay-safe item upserts with explicit inserted/unchanged/conflict semantics;
- evidence-locator debug redaction with a regression test covering token-bearing locators;
- deterministic cycle-safe downstream reachability;
- shortest-path explanation with edge-by-edge evidence;
- originating disruption evidence retained on every impact result;
- equal-length route determinism and evidence-alignment regression tests;
- exact line/function/region coverage and nonzero branch-coverage gates;
- PRD, TRD, DDD/architecture, security, research traceability, and commercialization-gap baselines.

This is **pre-release candidate behavior until the branch integrates through repository governance**. There is no production persistence, authenticated service, deployment image, customer deployment, signed release, or released artifact yet.

## Evidence and claim discipline

A reachable node means only that an admitted dependency path connects it to the disrupted source. It does not mean the node will actually fail, that a shortage is probable, or that a particular mitigation is optimal.

Any future quantitative severity, probability, recovery, or optimization capability must have an explicit mathematical owner, cited model basis, calibrated/validated evidence, uncertainty contract, and Rust implementation where it affects production results. Rule-of-thumb weights are not a substitute for that evidence.

## Architecture and integration

The current architecture separates three subdomain responsibilities:

- **Core — Disruption Impact:** deterministic reachability and explainable impact paths.
- **Supporting — Network Registry:** admitted supply nodes and dependency facts.
- **Supporting — Evidence Registry:** source references attached to facts and events.
- **Generic — identity, audit, transport, observability:** future infrastructure boundaries rather than domain truth.

Persistence, source adapters, tenant/security boundaries, service transport, operability, and UI remain explicit product gaps. The code-current ownership and integration rules are documented in [`ARCHITECTURE.md`](ARCHITECTURE.md), [`docs/PRD.md`](docs/PRD.md), and [`docs/TRD.md`](docs/TRD.md).

## Current maturity

This repository is an early product foundation, not a production or certification claim. The implemented domain behavior and quality gates can be evaluated now; commercialization still requires durable temporal persistence, real source adapters, tenant-safe authorization/audit, realistic interoperability evidence, operability/load/security evidence, user workflow, validated quantitative modeling where justified, and immutable release/provenance evidence.

The exact gap/evidence matrix lives in [`docs/product-technical-gap-baseline.md`](docs/product-technical-gap-baseline.md).

## Documentation map

| Goal | Start here |
| --- | --- |
| Product requirements | [`docs/PRD.md`](docs/PRD.md) |
| Technical requirements | [`docs/TRD.md`](docs/TRD.md) |
| Architecture / DDD boundary | [`ARCHITECTURE.md`](ARCHITECTURE.md) |
| Architecture decision | [`docs/adr/0001-evidence-first-disruption-impact.md`](docs/adr/0001-evidence-first-disruption-impact.md) |
| Research and standards traceability | [`docs/doctoring/research-traceability.md`](docs/doctoring/research-traceability.md) |
| Product / technical gaps | [`docs/product-technical-gap-baseline.md`](docs/product-technical-gap-baseline.md) |
| Security baseline | [`SECURITY.md`](SECURITY.md) |
| Changelog | [`CHANGELOG.md`](CHANGELOG.md) |

## Contributing

Before changing domain behavior, read [`AGENTS.md`](AGENTS.md), [`CLAUDE.md`](CLAUDE.md), the PRD/TRD, and the applicable architecture decision. Keep domain invariants, tests, documentation, and public claims on the same repository revision. Do not add heuristic risk weights or duplicate another system's authority as a convenience shortcut.

## License

Supply Chain Control Plane is licensed under the [Apache License 2.0](LICENSE). The current crate has no third-party runtime dependencies; future dependencies and imported assets must retain their own terms and remain compatible with the repository's commercial-use policy.
