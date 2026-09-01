# ADR-0001: Evidence-first disruption impact boundary

- Status: Accepted
- Date: 2026-09-01

## Context

The repository began with only a bootstrap README while its product responsibility is to connect supply events and calculate disruption impact. The highest-leverage bounded slice is an auditable causal reachability core, not a speculative optimization engine.

## Decision

Implement the first domain core in Rust. Model explicit directed dependencies and require immutable source evidence on both dependency facts and admitted disruption events. Report graph-reachable downstream nodes as **potential impacts** only. Every impact carries a deterministic shortest dependency path plus the evidence reference for each edge. Do not attach heuristic probability, severity, recovery time, or weights.

Keep external ERP/WMS/TMS/EPCIS semantics behind anti-corruption layers. Keep future relational persistence in normalized named entities and make event/dependency evidence history auditable and immutable-by-default.

## Consequences

The initial feature is explainable, deterministic, cycle-safe, and small enough to verify. It intentionally leaves durable storage, temporal validity, multi-tier quantities/capacity, scenario optimization, connectors, authentication, deployment, and customer-facing UX as explicit commercialization gaps.
