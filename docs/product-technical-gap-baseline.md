# Product / Technical Gap Baseline

Evidence base: repository main `11f3e0f191d7f5a30e1bb0512d26e0db323f38e2` was a bootstrap README only; this writer branch introduces the first test-first disruption-impact vertical. Status must be revalidated against the PR's current exact head before merge.

## Feature specification now implemented

Observed evidence-backed disruption → validate source/node invariants → traverse explicit downstream dependency edges → return unique potential impacts ordered by shortest hop count and semantic key. No heuristic score is produced.

```mermaid
flowchart LR
  E[Evidence Registry] -->|observed event| I[Disruption Impact]
  N[Network Registry] -->|nodes + dependencies| I
  I --> P[Potential Impact Result]
  P --> H[Human decision / next action]
```

## Commercialization gaps

| Gap | Owner | Evidence | Action | Current status | Next verification |
| --- | --- | --- | --- | --- | --- |
| Durable temporal network/evidence store | this repo | in-memory aggregate only | implement 3NF persistence with semantic keys, temporal validity, immutable evidence and item-level UPSERT/idempotency | open | concurrency + migration + recovery tests |
| Path-level explanation | this repo | result currently contains node + hop count | return supporting dependency path/evidence without duplicating source truth | open | cycle/multi-path property tests |
| Real source ingestion | adapter boundary; causal source repos if reusable | no connector exists | add EPCIS/ERP/WMS/TMS ACL adapters using real non-synthetic contract fixtures | open | interoperability + malformed-input tests |
| Authn/authz and tenant isolation | this repo + ecosystem identity boundary | no network API | define workspace ownership, authorization and audit before external access | open | security tests + threat model |
| Operability | this repo | no service/container/release | add compose-compatible service only when API exists; health/metrics/backup/restore and resource tuning | open | failure-injection + restore evidence |
| Customer workflow / UX | this repo | no UI | design evidence drill-through and next-action workflow without exposing internal boundaries | open | accessibility/E2E/screenshots + realistic load |
| Quantified severity/recovery scenarios | dedicated validated Rust model boundary | only reachability is justified | select/derive model from authoritative evidence; encode constraints and uncertainty, never rule-of-thumb weights | open | validation dataset + calibration/model tests |
| Release/provenance | this repo + central workflows | no release | define public artifact, SBOM/provenance and changelog release gate | open | signed/versioned release evidence |
| Quantitative coverage evidence | this repo | behavioral tests exist; no coverage artifact yet | adopt the org's pinned Rust coverage toolchain before claiming 100% | open | exact-head coverage artifact |

## DDD state

Core: Disruption Impact. Supporting: Network Registry, Evidence Registry. Generic: identity/audit/transport/observability. The current aggregate is `SupplyGraph`; entities/value objects are supply nodes, dependency facts, supply events and evidence references. Domain service behavior is deterministic downstream reachability. Invariants are documented in PRD/architecture and enforced by tests.
