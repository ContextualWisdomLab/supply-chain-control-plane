# AGENTS.md — Supply Chain Control Plane

Read `README.md`, `docs/PRD.md`, `docs/TRD.md`, `ARCHITECTURE.md`, the ADRs, and `docs/product-technical-gap-baseline.md` before changing behavior.

## Product invariant

Every impact claim must be traceable to explicit network dependencies and an observed evidence reference. Do not add heuristic risk/severity/recovery weights. Quantitative models require cited evidence, a named model contract, validation data, and a Rust implementation boundary.

## Engineering invariant

Behavior changes are test-first. Preserve semantic persistence names (`supply_node`, `supply_dependency_edge`, `supply_event_record`, `evidence_source_record`, `impact_assessment_record`) rather than generic one-word persistence objects. Never force-push shared writer branches. Re-fetch the branch/PR head before each write.

Organization-wide review, security, and governance rules from `ContextualWisdomLab/.github` remain authoritative.
