# Technical Requirements — Supply Chain Control Plane

## Runtime

Rust 1.97.1 is pinned for the domain core. Core graph/scientific computation remains Rust. The current crate has no third-party runtime dependency.

## Domain API

`SupplyGraph` admits typed supply nodes, evidence-backed directed dependencies, and evidence-backed `SupplyEvent` records. `downstream_impacts` performs breadth-first directed reachability, deduplicates visited nodes, records the deterministic shortest dependency path, per-edge dependency evidence, and originating event evidence, then orders results by shortest dependency hops and semantic node key. Ordered maps make equal-length route selection stable and independent of insertion order.

This is a **potential-impact** computation only. It must not be repurposed as severity, probability, lead-time, or recovery scoring.

## Future persistence contract

3NF entities: `supply_node`, `supply_dependency_edge`, `supply_event_record`, `evidence_source_record`, `impact_assessment_record`. Semantic keys replace generic standalone persistence-object names. Command handlers must define item-level UPSERT/idempotency behavior; dependency/event evidence is immutable-by-default; read models may be separated after measured contention/query evidence.

## External boundaries

ERP/WMS/TMS/EPCIS integrations enter through anti-corruption adapters. Source payloads remain source evidence; normalized domain facts retain stable source-record references. Production must never consume synthetic demo events.

## Verification

PR heads must pass local Rust format/lint/test CI plus the ContextualWisdomLab central required workflows and independent review. Behavioral coverage includes cycles, duplicate/unknown references, equal-length routes, evidence alignment, accessors, and operator-facing error rendering. Quantitative coverage measurement, persistence concurrency tests, and realistic load tests become gates when those surfaces exist.
