# Product Requirements — Supply Chain Control Plane

## Problem

Operations, procurement, and supply-risk teams hold dependency facts across ERP, WMS, TMS, spreadsheets, partner messages, and traceability feeds. A disruption is easy to observe locally but hard to connect to downstream facilities, items, shipments, and orders with evidence a human can audit.

## Product job

Given a customer-owned supply-network workspace and a newly observed disruption, surface the set of potentially affected downstream objects, the dependency path basis, and the source evidence for the originating observation and each dependency fact so a human can decide the next action.

## First commercial vertical

1. Register semantically identified supply nodes.
2. Register explicit directed dependency facts only with source evidence.
3. Admit a disruption event only with source evidence.
4. Accept exact node/dependency/event ingestion replays idempotently while rejecting same-key semantic conflicts.
5. Compute deterministic downstream reachability without heuristic weights.
6. Return the deterministic shortest admitted dependency path, originating event evidence, and evidence for every dependency edge.
7. Present results as potential impact, never fabricated certainty.

## Personas

- Supply-risk analyst investigating a disruption.
- Operations planner protecting fulfillment commitments.
- Procurement/operations leader reviewing evidence before intervention.

## Acceptance criteria for this slice

- strict add commands reject duplicate nodes, edges, and events rather than silently overwriting them;
- replay-oriented upsert commands return `Inserted` for a new item and `Unchanged` only for an exact semantic replay;
- an upsert that reuses a node, dependency, or event identity with different immutable content fails closed as a conflict;
- missing referenced nodes fail closed for both strict-add and upsert commands;
- direct self-dependencies fail closed;
- blank evidence identity/locator fails closed;
- dependency facts cannot be admitted without a validated `EvidenceReference`;
- cycles terminate and do not re-report the source node;
- impact output is deterministic by hop count and semantic key;
- equal-length routes resolve deterministically rather than by insertion order;
- every impact result includes the originating event evidence plus an auditable dependency path and edge evidence back to the directly affected node;
- no probability, severity, recovery-time, or optimization claim is created without a validated model contract.

## Explicit non-goals for this slice

No production connector, durable persistence, authentication, recovery optimizer, synthetic production data, or customer-facing UI is claimed complete. The in-memory upsert contract specifies replay semantics for future command handlers; it is not evidence of database durability or concurrency safety.

## Commercial outcome sequence

Evidence ingestion and durable 3NF storage → authenticated impact API → evidence-linked path drill-through and audit → real connector interoperability (prefer EPCIS 2.0 where applicable) → scenario planning with validated constraints → operability/load/security evidence → explicit publication license/provenance → versioned public release.
