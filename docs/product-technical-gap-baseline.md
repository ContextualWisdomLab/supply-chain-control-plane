# Product / Technical Gap Baseline

Evidence base: repository main `11f3e0f191d7f5a30e1bb0512d26e0db323f38e2` was a bootstrap README only; the open pull request introduces the first test-first disruption-impact vertical. The PR metadata is the authority for its current exact head, checks, reviews, and merge state.

## Feature specification now implemented

Observed evidence-backed disruption + evidence-backed dependency facts → validate source/node invariants → traverse explicit downstream dependency edges → return unique potential impacts ordered by shortest hop count and semantic key, each with originating event evidence, a deterministic shortest dependency path, and per-edge source evidence. No heuristic score is produced.

```mermaid
flowchart LR
  E[Evidence Registry] -->|event + dependency evidence| I[Disruption Impact]
  N[Network Registry] -->|nodes + dependencies| I
  I --> P[Potential Impact + Path + Evidence]
  P --> H[Human decision / next action]
```

## Commercialization gaps

| Gap | Owner | Evidence | Action | Current status | Next verification |
| --- | --- | --- | --- | --- | --- |
| Durable temporal network/evidence store | this repo | in-memory aggregate only | implement 3NF persistence with semantic keys, temporal validity, immutable evidence and item-level UPSERT/idempotency | open | concurrency + migration + recovery tests |
| Evidence-linked path explanation | this repo | `ImpactRecord.event_evidence`, `dependency_path`, `dependency_evidence`, cycle and equal-length multi-route regression tests | preserve deterministic shortest path and evidence alignment | implemented on writer branch | exact-head central checks + independent review |
| Real source ingestion | adapter boundary; causal source repos if reusable | no connector exists | add EPCIS/ERP/WMS/TMS ACL adapters using real non-synthetic contract fixtures | open | interoperability + malformed-input tests |
| Authn/authz and tenant isolation | this repo + ecosystem identity boundary | no network API | define workspace ownership, authorization and audit before external access | open | security tests + threat model |
| Operability | this repo | no service/container/release | add compose-compatible service only when API exists; health/metrics/backup/restore and resource tuning | open | failure-injection + restore evidence |
| Customer workflow / UX | this repo | no UI | design evidence drill-through and next-action workflow without exposing internal boundaries | open | accessibility/E2E/screenshots + realistic load |
| Quantified severity/recovery scenarios | dedicated validated Rust model boundary | only reachability is justified | select/derive model from authoritative evidence; encode constraints and uncertainty, never rule-of-thumb weights | open | validation dataset + calibration/model tests |
| Source licensing | this repo + product owner | root `LICENSE` and Cargo metadata on writer branch declare Apache-2.0; protected `main` has not integrated them yet | preserve Apache-2.0 source grant and keep future inbound code/assets commercially compatible | implemented on writer branch | exact-head checks/review + protected integration |
| Immutable release / SBOM / provenance | this repo + central workflows | no release or signed artifact exists | define public artifact, SBOM/provenance, checksums/attestation and changelog release gate without treating source licensing as release evidence | open | signed/versioned release evidence |
| Quantitative coverage evidence | this repo | exact-head Product CI enforces 100% line, function, and region coverage on stable Rust plus a nonzero 100% branch denominator on pinned nightly Rust | preserve the version-pinned gates and fail on zero denominators | implemented on writer branch | exact-head central checks + independent review |

## Licensing due diligence

The repository was initialized in ContextualWisdomLab as an independent root commit and the foundation branch contains repository-authored Rust, tests, workflows, and documentation rather than imported/vendored source. The current crate has no third-party runtime dependencies. Under the organization commercial-use policy, the writer branch therefore grants the repository source under Apache License 2.0 and declares `license = "Apache-2.0"` in `Cargo.toml`.

That source grant does not manufacture release, deployment, customer, certification, transfer, SBOM, provenance, or third-party-license evidence. Any future dependency, copied source, generated asset, model artifact, or external adapter must be re-evaluated for inbound provenance and commercial compatibility before incorporation.

## DDD state

Core: Disruption Impact. Supporting: Network Registry, Evidence Registry. Generic: identity/audit/transport/observability. The current aggregate is `SupplyGraph`; entities/value objects are supply nodes, evidence-backed dependency facts, supply events, evidence references and impact records. Domain service behavior is deterministic downstream reachability plus evidence-linked shortest-path explanation. Invariants are documented in PRD/architecture and enforced by tests.
