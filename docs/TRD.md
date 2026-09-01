# Technical Requirements — Supply Chain Control Plane

## Runtime

Rust 1.97.1 is pinned for the domain core. Core graph/scientific computation remains Rust. The current crate has no third-party runtime dependency.

## Domain API

`SupplyGraph` admits typed supply nodes, directed dependencies, and evidence-backed `SupplyEvent` records. `downstream_impacts` performs breadth-first directed reachability, deduplicates visited nodes, records the deterministic shortest dependency path, and orders results by shortest dependency hops then semantic node key.

This is a **potential-impact** computation only. It must not be repurposed as severity, probability, lead-time, or recovery scoring.

## Future persistence contract

3NF entities: `supply_node`, `supply_dependency_edge`, `supply_event_record`, `evidence_source_record`, `impact_assessment_record`. Semantic keys replace generic standalone persistence-object names. Command handlers must define item-level UPSERT/idempotency behavior; read models may be separated after measured contention/query evidence.

## External boundaries

ERP/WMS/TMS/EPCIS integrations enter through anti-corruption adapters. Source payloads remain source evidence; normalized domain facts retain stable source-record references. Production must never consume synthetic demo events.

## Verification

PR heads must pass local Rust format/lint/test CI plus the ContextualWisdomLab central required workflows and independent review. Coverage measurement, fuzz/property testing, persistence concurrency tests, and realistic load tests become gates when those surfaces exist.
