# Architecture

## Domain-driven boundary

**Core subdomain — Disruption Impact.** Converts an observed disruption plus explicit supply dependencies into evidence-linked potential downstream impact.

**Supporting subdomains — Network Registry and Evidence Registry.** Normalize customer-owned supplier/facility/item/inventory/shipment/order identities and retain immutable source pointers for both observed events and dependency facts.

**Generic subdomains — Identity, audit, transport, observability.** Future services should reuse ContextualWisdomLab platform capabilities rather than duplicate them.

### Bounded contexts and context map

`Network Registry` publishes supply-node identity and evidence-backed dependency facts to `Disruption Impact`. `Evidence Registry` supplies immutable observation references. Future `Scenario Planning` consumes impact results but must not write inferred facts back as observed evidence. External ERP/WMS/TMS/EPCIS adapters sit behind anti-corruption layers so vendor semantics cannot leak into the core model.

### Ubiquitous language

- **Supply node:** a supplier, facility, item, inventory position, shipment, or order in the dependency graph.
- **Supply dependency:** directed fact that a downstream node depends on an upstream node, backed by source evidence.
- **Supply event:** observed disruption-relevant fact tied to source evidence.
- **Potential impact:** graph-reachable downstream node; not a severity/probability claim.
- **Evidence reference:** stable source-record key plus drill-through locator.

### Aggregate and invariants

`SupplyGraph` is the current in-memory aggregate. It owns node identity, dependency uniqueness, dependency evidence, event uniqueness, event evidence, and cycle-safe reachability. Transaction boundaries must remain minimal when persistence arrives: one node/edge/event item-level UPSERT per command, with immutable evidence and explicit idempotency keys.

## Persistence direction

No production database exists yet. The intended 3NF relational contract separates `supply_node`, `supply_dependency_edge`, `supply_event_record`, `evidence_source_record`, and `impact_assessment_record`; keys use semantic names such as `supply_node_key`, never a standalone persistence object named `id`. Hot event ingestion and read-heavy impact queries should use separate command/query paths if evidence shows lock or partition contention.
