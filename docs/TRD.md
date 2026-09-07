# Technical Requirements — Supply Chain Control Plane

## 1. Technical objective

The system shall turn heterogeneous supply-chain facts and observations into reproducible, evidence-linked decisions without becoming the transactional system of record for ERP, PLM, MES, QMS, WMS, TMS, SRM, carrier, or accounting domains.

The architecture begins as a modular Rust application with explicit ports and bounded contexts. Network-service extraction is justified only by independently deployable trust, scaling, lifecycle, or ownership requirements.

## 2. Runtime baseline

- Rust 1.98.0 is the current pinned toolchain for owned production logic.
- Core graph, temporal, statistical, simulation, optimization, and integrity logic remains Rust-first.
- PostgreSQL 18.6 is the current reference durable relational system; subsequent supported minors require the same upgrade, recovery, and compatibility evidence and are pinned in the release manifest.
- CPU `f64` is the numerical reference for quantitative decision logic.
- PostgreSQL, object storage, and a durable event transport are external runtime dependencies once persistence is introduced.
- JavaScript is not required on the server. Browser JavaScript shall remain bounded by measured bundle, heap, main-thread, and accessibility budgets.

## 3. Logical architecture

```text
source systems and partners
        │
        ▼
anti-corruption adapters
        │
        ▼
identity + evidence + event admission
        │
        ├── network command model
        ├── event command model
        └── immutable source artifacts
        │
        ▼
disruption case and impact application services
        │
        ├── deterministic graph assessment
        ├── quantitative impact models
        └── scenario solver
        │
        ▼
decision + approval + action control
        │
        ▼
writeback adapters and outcome observations
        │
        ▼
read models, reports, model evaluation, and audit
```

## 4. Bounded modules

| Module | Responsibility | Prohibited responsibility |
| --- | --- | --- |
| `workspace_access` | Tenant, actor, purpose, role, and policy context | Credential storage or customer IdP implementation |
| `network_registry` | Cross-system object references and evidence-backed relations | Replacing source-system masters |
| `evidence_registry` | Immutable evidence metadata, hashes, classification, and drill-through control | Executing source content |
| `event_registry` | Normalized supply events, timing, corrections, and source receipts | Rewriting source transactions |
| `disruption_management` | Cases, evidence, hypotheses, state transitions, and ownership | Numerical scoring |
| `impact_assessment` | Deterministic exposure and registered quantitative impact runs | Unreviewed heuristic risk scores |
| `scenario_management` | Constraints, actions, runs, and trade-offs | Direct operational writeback |
| `decision_control` | Options, approvals, expiry, residual risk, and selected actions | Self-approval by model or proposer |
| `action_control` | Commitments, writeback intents, receipts, and outcomes | Adapters bypassing authorization |
| `model_governance` | Model artifacts, validation, approval, activation, and rollback | Training data ownership outside policy |
| `integration_hub` | Versioned adapter ports, inbox/outbox, and reconciliation | Cross-service SQL or copied owner code |
| `reporting_projection` | Read models, search projections, exports, and UI queries | Becoming a second write authority |

## 5. Domain API baseline

The existing `SupplyGraph` admits typed supply nodes, evidence-backed directed dependencies, and evidence-backed `SupplyEvent` records. Its output is a deterministic evidence-linked potential-impact record, not a quantitative impact estimate. Strict commands reject duplicate identities. Replay-oriented upserts return `Inserted`, `Unchanged`, or an explicit conflict. `downstream_impacts` performs deterministic breadth-first reachability and returns event and path-edge evidence.

The durable design shall preserve this behavior while decomposing transaction boundaries:

```rust
pub trait NetworkCommandPort {
    async fn upsert_supply_node(
        &self,
        command: UpsertSupplyNode,
    ) -> Result<UpsertReceipt, NetworkCommandError>;

    async fn upsert_supply_dependency(
        &self,
        command: UpsertSupplyDependency,
    ) -> Result<UpsertReceipt, NetworkCommandError>;
}

pub trait EventCommandPort {
    async fn record_supply_event(
        &self,
        command: RecordSupplyEvent,
    ) -> Result<EventReceipt, EventCommandError>;
}

pub trait ImpactAssessmentPort {
    async fn start_impact_assessment(
        &self,
        command: StartImpactAssessment,
    ) -> Result<AssessmentJobReceipt, ImpactAssessmentError>;
}
```

Each command includes tenant context, semantic identity, immutable content digest, evidence reference, valid time, recorded time, idempotency key, correlation identifier, and actor/purpose context.

## 6. Persistence contract

### 6.1 Relational source of record

The control plane's authoritative state uses normalized PostgreSQL tables. JSONB is limited to immutable external payload references, non-decision diagnostic metadata, or standards extensions whose semantics are independently schema-validated. Business rules, money, quantities, dimensions, statuses, validity, and authorization-relevant attributes use typed columns and relations.

### 6.2 Item-level transactions

- One node, dependency, event, evidence receipt, approval, or writeback effect is committed per minimal aggregate transaction.
- Command state and outbox event are written in the same database transaction.
- Exact replay returns the original operation receipt or a semantically equivalent no-op receipt.
- Same identity plus changed immutable content returns a conflict; it never becomes last-write-wins.
- Corrections append a new fact and reference the corrected record.

### 6.3 Temporal contract

All versioned facts preserve business validity and system history. Hard deletion is prohibited for decision, evidence, event, approval, writeback, and audit history except through a documented retention/disposition process that preserves a tombstone receipt.

### 6.4 Partitioning and contention

Partitioning is introduced from measured evidence. Event and audit tables are candidates for tenant-and-time partitioning. A global monotonically increasing hot key is prohibited. Read/write separation and projection stores require an ADR based on observed lock, query, replication, and consistency requirements.

## 7. Evidence and artifact storage

- Small metadata remains in PostgreSQL.
- Source artifacts, standards-conformant payloads, large documents, scenario artifacts, and model artifacts use content-addressed object storage.
- Object keys are opaque and tenant-scoped.
- Stored content has SHA-256 or stronger digest, media-type validation, byte length, capture receipt, encryption context, retention state, and chain-of-custody metadata.
- Source artifacts are not rendered or executed in the trusted service process.
- Large or hostile files are inspected through the quarantine runtime when available through a released contract.

## 8. API contract

- HTTP APIs conform to OpenAPI 3.2.0.
- Errors use RFC 9457 `application/problem+json` and stable problem-type identifiers.
- Commands require `Idempotency-Key`.
- Mutable resources use `ETag` and `If-Match` where optimistic concurrency is exposed.
- Collections use opaque keyset cursors and deterministic ordering.
- Heavy work returns `202 Accepted` with a durable job reference.
- All responses are policy-filtered by tenant, actor, purpose, and field permissions.
- Network paths, evidence locators, stack traces, credentials, and internal infrastructure details are not exposed in generic errors.

The endpoint inventory and problem codes are defined in `docs/API_CONTRACT.md`.

## 9. Event contract

- Events use CloudEvents 1.0.2 envelopes and AsyncAPI 3.0.0 descriptions.
- `source + id` is the delivery deduplication identity.
- Domain-level idempotency also uses the explicit business command or source-event key.
- Schemas are immutable by version and use compatibility tests.
- Events contain opaque references and minimal decision context, not secrets, full PII, or raw evidence.
- Consumers record inbox receipts with payload hash and processing result.

The event catalog is defined in `docs/EVENT_CONTRACT.md`.

## 10. External adapter contract

Each adapter declares:

```text
adapter_code
adapter_version
source_system_kind
supported_object_types
supported_event_types
supported_commands
schema_versions
maximum_payload_bytes
ordering_contract
retry_contract
idempotency_contract
truth_status_capability
writeback_risk_class
```

Adapters must:

- translate vendor semantics behind anti-corruption layers;
- retain source identifiers in mapping tables rather than core columns;
- validate content before domain admission;
- preserve source payload hashes and capture receipts;
- fail closed on ambiguous identity, unknown unit, unsupported state, or schema mismatch;
- prove retry safety and reconciliation;
- avoid mutable branch or sibling filesystem dependencies.

## 11. Standards interoperability

- SCOR DS is a process and performance crosswalk, not the internal aggregate model.
- GS1 EPCIS 2.0.1 and CBV 2.0.0 are the primary visibility-event adapter targets.
- UN/CEFACT Buy-Ship-Pay semantics provide international trade mappings.
- W3C PROV-O provides interchange vocabulary for provenance projections.
- Versioned standard URIs and local mapping versions are recorded; unversioned URLs are not used as immutable evidence.

## 12. Quantitative compute architecture

```text
validated input snapshot
→ feature and constraint preparation
→ CPU reference calculation
→ optional parallel/GPU backend
→ parity and numerical checks
→ result artifact
→ uncertainty and solver-status publication
```

Required compute crates are planned as:

```text
demand_forecast_core
lead_time_model
inventory_optimization
constrained_allocation
network_flow_solver
disruption_simulation
scenario_comparison
model_validation
```

Rules:

- exact decimal or rational representations are used where required for money, contractual quantities, or discrete units;
- floating-point comparisons declare tolerance and scale;
- randomness uses recorded deterministic seed manifests;
- solver timeout returns a termination state and best-known bound, not a fabricated optimum;
- infeasibility returns binding constraints or a reproducible diagnostic artifact;
- GPU tests execute on supported hardware or software backends rather than being silently skipped;
- model validity is evaluated outside the serving path before activation.

## 13. LLM boundary

All LLM work routes through `contextual-orchestrator` using released clients and schemas. Permitted uses include:

- extracting proposed event attributes from unstructured evidence;
- comparing source statements;
- drafting evidence requests and decision narratives;
- summarizing an already computed scenario;
- proposing mappings or labels for review.

LLMs shall not:

- create authoritative network facts without owner review;
- calculate inventory, shortage, probability, cost, allocation, or solver output;
- change permission or policy state;
- approve or dispatch an action;
- see credentials, unrestricted partner evidence, or unnecessary PII;
- execute instructions embedded in source documents.

Every LLM output includes model identifier, provider identifier, prompt/schema hash, reasoning configuration, evidence references, output validation, and review state.

## 14. CWL integration contract

The product consumes only released, versioned owner contracts. Initial integration ports are documented in `docs/CONTEXT_MAP.md`. No production build consumes a mutable sibling head, branch URL, local cross-repository path, copied adapter, or cross-service SQL query.

If an owner contract is insufficient, the fix is developed in the owner with RED contract tests, released, and then consumed by version. A temporary bridge requires an owner issue, expiry condition, removal test, and explicit non-production classification.

## 15. Security architecture

The service applies:

- verified OIDC/JWT and service identity;
- tenant and purpose-aware authorization;
- database-level tenant enforcement;
- encrypted transport and storage;
- deny-by-default outbound policy;
- bounded parser and graph resources;
- writeback segregation of duties;
- audit and evidence chain of custody;
- dependency, SBOM, provenance, and signed-release controls.

The threat analysis and controls are in `docs/SECURITY_AND_THREAT_MODEL.md`.

## 16. Operability architecture

- Liveness, startup, readiness, and drain are distinct signals.
- OpenTelemetry records request, job, database, queue, adapter, solver, and writeback behavior without sensitive payloads.
- Heavy jobs use durable state, cancellation, retry classification, checkpointing where mathematically safe, and dead-letter handling.
- Backup and restore include PostgreSQL, object storage, schema versions, model artifacts, event offsets, and encryption-key availability.
- Resource budgets cover CPU, memory, PID, disk, network, database connections, graph depth, job size, and solver time.

See `docs/OPERABILITY.md`.

## 17. Verification contract

Every exact PR head shall pass:

- formatting, compile, Clippy `-D warnings`, tests, doctests, and warning-free docs;
- production statement, function, region, and branch coverage at 100%;
- public API documentation coverage at 100%;
- schema and contract conformance;
- security and dependency checks;
- migration clean install and supported upgrade rehearsal when persistence exists;
- realistic PostgreSQL, object storage, adapter, and buyer-journey tests when those surfaces exist;
- independent review and organization governance.

The complete matrix is in `docs/TEST_STRATEGY.md`.
