# ADR 0002 — Supply Chain Decision and Resilience Control Plane, Not an ERP Replacement

- Status: Proposed
- Date: 2026-09-02
- Decision owners: Supply Chain Control Plane product and architecture owners

## Context

Enterprise buyers already operate ERP, PLM, MES, QMS, WMS, TMS, SRM, carrier, planning, and partner systems. Those products own transactions and operational facts within their domains. Reimplementing all of them would expand the repository into an unbounded suite, duplicate authoritative data, and obscure the product's strongest capability: evidence-linked cross-system decisions.

The buyer problem is not the absence of another transaction system. It is the inability to reconstruct, at decision speed, what changed, which commitments depend on it, what was knowable at the time, which responses are feasible, who approved an action, and whether the action worked.

## Decision

This repository owns a **Supply Chain Decision and Resilience Control Plane**.

It owns:

- cross-system supply-network identity and relationship mappings;
- evidence, event, disruption-case, impact, scenario, decision, action, and outcome records;
- model, solver, snapshot, uncertainty, approval, and writeback provenance; and
- the buyer workflow that closes analysis into controlled execution and review.

It does not own:

- legal trading-party or material masters;
- purchase or sales orders;
- physical inventory adjustment;
- production execution;
- shipment execution;
- quality disposition;
- accounting journals; or
- identity credentials.

Those facts remain with their external domain owners. The control plane consumes and writes through versioned anti-corruption adapters, never cross-system SQL or copied application tables.

## Alternatives considered

### Build a complete ERP/SCM suite

Rejected. The scope would be too broad, would duplicate customer systems of record, and would delay the first defensible buyer workflow.

### Build only a supplier-risk dashboard

Rejected as the full product boundary. It is a useful acquisition wedge, but it stops before constrained response, approval, writeback, and outcome verification.

### Build a generic enterprise graph

Rejected. A generic graph does not own the supply-chain invariants, temporal evidence, decision states, or controlled operational actions required by the product.

## Consequences

- Integrations are first-class product work rather than data-copy shortcuts.
- The first commercial vertical is disruption impact and recovery.
- Source authority and truth status appear in every material result.
- Operational writeback requires explicit approval, idempotency, and reconciliation.
- ERP replacement remains a separate future product decision, not an implied roadmap promise.
