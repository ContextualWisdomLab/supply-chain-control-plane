# ADR-0001: Evidence-first disruption impact boundary

- Status: Accepted
- Date: 2026-09-01

## Context

The repository began with only a bootstrap README while its product responsibility is to connect supply events and calculate disruption impact. The highest-leverage bounded slice is an auditable causal reachability core, not a speculative optimization engine. Real ERP/WMS/TMS/EPCIS ingestion is retry-prone, so a commercial command boundary also needs deterministic replay behavior before durable adapters are introduced.

## Decision

Implement the first domain core in Rust. Model explicit directed dependencies and require immutable source evidence on both dependency facts and admitted disruption events. Report graph-reachable downstream nodes as **potential impacts** only. Every impact carries a deterministic shortest dependency path plus the evidence reference for each edge. Do not attach heuristic probability, severity, recovery time, or weights.

Keep external ERP/WMS/TMS/EPCIS semantics behind anti-corruption layers. Preserve strict duplicate-rejecting commands for callers that require uniqueness assertions, and expose separate replay-oriented item upserts for nodes, dependency facts, and events. An upsert inserts an absent semantic item, returns unchanged only for an exact replay, and rejects same-key changed immutable content as a conflict. In particular, dependency and event evidence must not be rewritten by a retry.

Keep future relational persistence in normalized named entities and make event/dependency evidence history auditable and immutable-by-default. A durable writer must preserve the same replay/no-op/conflict semantics under concurrency rather than using last-write-wins.

## Consequences

The initial feature is explainable, deterministic, cycle-safe, replay-safe at the domain command boundary, and small enough to verify. It intentionally leaves durable storage, temporal validity, database concurrency/recovery, multi-tier quantities/capacity, scenario optimization, connectors, authentication, deployment, and customer-facing UX as explicit commercialization gaps. The in-memory upsert contract does not claim database durability.
