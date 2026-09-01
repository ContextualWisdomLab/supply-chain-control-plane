# Product Requirements — Supply Chain Control Plane

## Problem

Operations, procurement, and supply-risk teams hold dependency facts across ERP, WMS, TMS, spreadsheets, partner messages, and traceability feeds. A disruption is easy to observe locally but hard to connect to downstream facilities, items, shipments, and orders with evidence a human can audit.

## Product job

Given a customer-owned supply-network workspace and a newly observed disruption, surface the set of potentially affected downstream objects, the dependency path basis, and the source evidence for the observation and each dependency fact so a human can decide the next action.

## First commercial vertical

1. Register semantically identified supply nodes.
2. Register explicit directed dependency facts only with source evidence.
3. Admit a disruption event only with source evidence.
4. Compute deterministic downstream reachability without heuristic weights.
5. Return the deterministic shortest admitted dependency path and evidence for every edge.
6. Present results as potential impact, never fabricated certainty.

## Personas

- Supply-risk analyst investigating a disruption.
- Operations planner protecting fulfillment commitments.
- Procurement/operations leader reviewing evidence before intervention.

## Acceptance criteria for this slice

- duplicate nodes, edges, and events fail closed;
- missing referenced nodes fail closed;
- direct self-dependencies fail closed;
- blank evidence identity/locator fails closed;
- dependency facts cannot be admitted without a validated `EvidenceReference`;
- cycles terminate and do not re-report the source node;
- impact output is deterministic by hop count and semantic key;
- every impact result includes an auditable dependency path and edge evidence back to the directly affected node;
- no probability, severity, recovery-time, or optimization claim is created without a validated model contract.

## Explicit non-goals for this slice

No production connector, persistence, authentication, recovery optimizer, synthetic production data, or customer-facing UI is claimed complete.

## Commercial outcome sequence

Evidence ingestion and durable 3NF storage → authenticated impact API → evidence-linked path drill-through and audit → real connector interoperability (prefer EPCIS 2.0 where applicable) → scenario planning with validated constraints → operability/load/security evidence → versioned public release.
