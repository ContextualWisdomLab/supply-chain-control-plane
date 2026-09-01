# Technical Requirements — Supply Chain Control Plane

## Runtime

Rust 1.98.0 is pinned for the domain core. Core graph/scientific computation remains Rust. The current crate has no third-party runtime dependency.

## Domain API

`SupplyGraph` admits typed supply nodes, evidence-backed directed dependencies, and evidence-backed `SupplyEvent` records. Strict `add_*`/`record_event` commands reject duplicate identities so callers cannot silently overwrite state. Replay-oriented `upsert_node`, `upsert_dependency`, and `upsert_event` commands implement the item-level idempotency contract: a new semantic item returns `UpsertOutcome::Inserted`, an exact replay returns `UpsertOutcome::Unchanged`, and same-key changed content fails closed with an explicit conflict. Dependency and event evidence therefore remain immutable across retries.

`downstream_impacts` performs breadth-first directed reachability, deduplicates visited nodes, records the deterministic shortest dependency path, per-edge dependency evidence, and originating event evidence, then orders results by shortest dependency hops and semantic node key. Ordered maps make equal-length route selection stable and independent of insertion order.

This is a **potential-impact** computation only. It must not be repurposed as severity, probability, lead-time, or recovery scoring.

## Future persistence contract

3NF entities: `supply_node`, `supply_dependency_edge`, `supply_event_record`, `evidence_source_record`, `impact_assessment_record`. Semantic keys replace generic standalone persistence-object names. Persistence command handlers must preserve the Rust domain contract exactly: one item per transaction boundary, insert on absence, no-op only for byte/semantic-equivalent immutable facts, and conflict on same identity with changed immutable content. Database concurrency control, temporal validity, migrations, recovery, and durable evidence history remain unimplemented and must be proven independently; the in-memory upsert API is not durability evidence. Read models may be separated after measured contention/query evidence.

## External boundaries

ERP/WMS/TMS/EPCIS integrations enter through anti-corruption adapters. Source payloads remain source evidence; normalized domain facts retain stable source-record references. Production must never consume synthetic demo events.

## Verification

PR heads must pass exact-head Rust format, compile, Clippy `-D warnings`, tests, doctests, and warning-free rustdoc. Stable CI enforces 100% line, function, and region coverage; a separately pinned nightly lane enforces a nonzero 100% branch denominator. ContextualWisdomLab central required workflows and independent review remain mandatory. Persistence concurrency tests and realistic load tests become gates when those surfaces exist.
