# Product Requirements — Supply Chain Control Plane

## 1. Purpose

Supply Chain Control Plane is a Supply Chain Decision and Resilience Control Plane over customer-owned ERP, PLM, MES, QMS, WMS, TMS, SRM, carrier, partner, and traceability systems. Supply Chain Control Plane connects supply-network facts, events, evidence, scenarios, decisions, actions, and outcomes across customer-owned systems. Its first end-to-end product objective is to help a supply-chain team respond to a disruption without confusing an observed fact, an inferred impact, a proposed response, and an executed operational change.

This document defines the intended full product. The repository's implemented Rust slice is identified separately in Section 15.

## 2. Product job

Given a customer-owned supply-network workspace and a newly observed disruption, the product shall:

1. establish the source, identity, timing, and truth status of the observation;
2. identify potentially affected downstream objects with auditable dependency paths;
3. estimate shortages, timing, service, cost, and constraint effects only through validated models;
4. generate or accept feasible recovery scenarios;
5. present trade-offs and uncertainty for human decision;
6. record approval, dispatch, and idempotent writeback;
7. observe outcomes and compare them with the decision basis; and
8. retain enough evidence to reproduce what was known and why the action was taken.

## 3. Product boundary

### 3.1 The product owns

- cross-system supply object references and identity resolutions;
- admitted network relationships with provenance and validity;
- normalized supply events and corrections;
- disruption cases, hypotheses, evidence, and status history;
- impact-assessment inputs, paths, estimates, uncertainty, and results;
- recovery scenarios, constraints, alternatives, runs, and trade-offs;
- decision records, approvals, action commitments, writeback intents, and receipts;
- outcome observations, model evaluation, and post-action review;
- its tenant, authorization, audit, retention, operability, and release evidence.

### 3.2 The product does not own

- supplier/customer/item/BOM/routing master truth;
- order, inventory, shipment, production, quality, or accounting transaction truth;
- identity credentials;
- another CWL product's domain truth;
- unreviewed LLM assertions as facts;
- regulatory certification or assurance conclusions.

## 4. Personas and critical tasks

### 4.1 Supply-risk analyst

- admit an observation without losing its source and timing;
- separate confirmed facts from hypotheses;
- trace downstream exposure;
- request identity or evidence resolution;
- open and maintain a disruption case.

### 4.2 Operations planner

- review inventory, supply, demand, capacity, lane, and order constraints;
- run scenarios with a fixed input snapshot;
- compare service, cost, lateness, and feasibility;
- understand why a scenario is infeasible or why the system abstained.

### 4.3 Procurement and supplier-management leader

- identify alternate source, expedite, allocation, contract, and supplier-response options;
- review supplier commitments and evidence;
- distinguish operational performance from questionnaire or assessment estimates.

### 4.4 Approver and executive

- review material consequences, alternatives, evidence gaps, and residual risk;
- approve, reject, return, or expire a decision;
- see who owns each action and whether the response is working.

### 4.5 Operator

- receive a concrete approved action rather than an analytical alert;
- dispatch or acknowledge writeback;
- report partial success, rejection, and recovery evidence.

### 4.6 Data steward and administrator

- map external identifiers and resolve conflicts;
- control source authority, data quality, retention, access, integration, and model use;
- verify exact contract and adapter versions.

## 5. End-to-end user journeys

### 5.1 Disruption to exposure

```text
source event or analyst observation
→ schema and evidence validation
→ identity resolution
→ truth-status admission
→ network snapshot selection
→ deterministic reachability
→ evidence-linked potential impacts
```

Acceptance conditions:

- every admitted event and dependency has a stable evidence reference;
- exact replay is a no-op and changed immutable content is a conflict;
- the result is deterministic for the same input snapshot;
- the output is labeled potential impact, not probability or severity;
- cycles terminate and do not re-report the source node;
- unavailable or unauthorized evidence is reported without leaking its locator.

### 5.2 Exposure to quantified impact

```text
potential impacts
→ inventory and supply positions
→ demand and commitment positions
→ capacity, timing, quality, lane, and policy constraints
→ validated quantitative model
→ shortage, lateness, service, and uncertainty estimates
```

Acceptance conditions:

- the model declares its target quantity, assumptions, training or calibration evidence, and limitations;
- results carry model version, input snapshot hash, solver status, and uncertainty;
- infeasible, insufficient-data, out-of-distribution, and timeout states are explicit;
- no rule-of-thumb weight is silently substituted for a missing model;
- corrections do not rewrite prior assessment history.

### 5.3 Recovery scenario comparison

```text
impact assessment
→ candidate actions
→ scenario constraints
→ deterministic or stochastic solver run
→ Pareto-feasible alternatives
→ evidence and trade-off comparison
```

Required alternatives include, where data and authority allow:

- alternate supply;
- expedite or route change;
- inventory reallocation;
- production resequencing;
- substitution or specification change;
- customer-priority allocation;
- demand shaping or commitment renegotiation;
- deliberate no-action with documented residual consequences.

Acceptance conditions:

- scenario inputs are immutable after a run starts;
- all hard constraints are distinguished from preferences;
- infeasible scenarios explain the binding constraints;
- results do not claim optimality unless the solver proves it under the recorded model;
- equal or near-equal options expose trade-offs rather than forcing a fabricated winner.

### 5.4 Decision to controlled execution

```text
scenario or analyst proposal
→ decision record
→ segregation-of-duties review
→ approval
→ action commitment
→ writeback intent
→ external-system receipt
```

Acceptance conditions:

- a model or LLM cannot approve its own proposal;
- an action cannot be dispatched without valid authorization and an approved decision, except for explicitly configured read-only or reversible low-risk actions;
- every command uses an idempotency key;
- external system identifiers are isolated in adapter mappings;
- timeout and retry do not create duplicate operational changes;
- a partial or rejected writeback remains visible and actionable.

### 5.5 Outcome and learning

```text
executed action
→ observed operational result
→ predicted-versus-realized comparison
→ model and workflow evaluation
→ post-action review
→ corrected policy, model, or network fact proposal
```

Acceptance conditions:

- outcome observations remain distinct from the decision's original evidence;
- hindsight information never alters what was recorded as knowable at decision time;
- model performance is segmented by relevant site, item, supplier, lane, horizon, and event class when sample size permits;
- improvements require a reviewed new version rather than silent production behavior change.

## 6. Functional requirements

### FR-001 Workspace and tenancy

The system shall isolate every command, query, job, object-storage artifact, event, audit record, and model artifact by tenant workspace. Cross-tenant identifiers shall not disclose object existence.

### FR-002 Identity resolution

The system shall map source-system identifiers to stable supply object references, preserve source authority and validity, and support reviewed merge, split, supersession, and conflict resolution.

### FR-003 Network facts

The system shall admit typed nodes and directed dependency or flow relations with evidence, validity, truth status, and source authority. It shall reject blank identity, unknown references, direct self-dependency where the relation forbids it, and same-key changed-content overwrite.

### FR-004 Evidence registry

The system shall preserve source-record identity, capture receipt, cryptographic digest, content classification, retention state, and authorized drill-through locator. Routine logs and errors shall not expose sensitive locators.

### FR-005 Event registry

The system shall ingest normalized events, preserve source payload references, support correction and supersession without destructive update, and distinguish occurrence, availability, and recording times.

### FR-006 Disruption cases

The system shall group evidence, events, subjects, hypotheses, impact runs, decisions, and actions into a case with an append-only status transition history.

### FR-007 Potential-impact assessment

The system shall provide deterministic evidence-linked graph exposure with bounded traversal and reproducible path selection.

### FR-008 Quantified-impact assessment

The system shall run only registered and approved quantitative models, retain model and input provenance, and expose uncertainty, abstention, data sufficiency, and solver termination state.

### FR-009 Scenario planning

The system shall support versioned scenarios, hard and soft constraints, candidate actions, multi-objective trade-offs, reproducible solver runs, and infeasibility explanation.

### FR-010 Decision control

The system shall record options, decision basis, approvers, separation-of-duty checks, decision expiry, residual risk, and selected action commitments.

### FR-011 Writeback control

The system shall issue versioned adapter commands only after policy and authorization checks, preserve idempotency and correlation, and record authoritative external receipts.

### FR-012 Outcome verification

The system shall link outcome evidence to actions and calculate prediction and execution error without rewriting prior decisions.

### FR-013 Search and evidence retrieval

The system shall support exact, semantic, graph, and temporal retrieval through policy-filtered read models while preserving channel contribution and source provenance.

### FR-014 Collaboration

The system shall support comments, evidence requests, assignments, supplier-response capture, decision documents, and notification intents without making communication systems the domain record.

### FR-015 Interoperability

The system shall provide versioned APIs and events, EPCIS 2.0.1 capture/query adapter support where applicable, SCOR DS process crosswalks, and UN/CEFACT mappings for international trade semantics.

### FR-016 Administration

Administrators shall manage workspaces, roles, purposes, adapters, source authorities, retention, model approvals, solver policies, event schemas, and release compatibility.

### FR-017 Audit and export

Every material read, decision, export, approval, model run, and writeback shall have an audit receipt. Exports shall preserve provenance and authorization scope.

### FR-018 Internationalization

User-facing product resources shall support Korean, English, Japanese, Simplified Chinese, Vietnamese, Spanish, German, and French through versioned database-managed resources. The browser shall not receive the entire catalog.

## 7. Truth and claim requirements

Every persisted or emitted claim shall declare one of the following statuses:

| Status | Meaning |
| --- | --- |
| `authoritative` | Approved by the domain authority that owns the fact |
| `observed` | Deterministically captured from a source but not owned by this product |
| `inferred` | Produced by a statistical, graph, or analytical process |
| `proposed` | Awaiting human or owner-system review |
| `superseded` | Previously valid but replaced by a newer fact or decision |
| `rejected` | Reviewed and not accepted |

An `inferred` or `proposed` claim shall never become `authoritative` merely because a model confidence value is high.

## 8. Temporal requirements

The product shall distinguish:

- business-valid interval;
- event occurrence time or interval;
- source assertion/publication time;
- observation/capture time;
- availability time;
- system recording time;
- supersession time;
- analysis knowledge cutoff.

For any historical analysis:

```text
available_at <= knowledge_cutoff
```

shall be enforced. A later document describing an earlier event cannot be used in an earlier decision replay.

## 9. Quantitative decision requirements

- Production arithmetic is Rust-first.
- CPU `f64` is the reference implementation unless an ADR defines another numerical contract.
- GPU and multithreaded implementations require parity tests.
- Probability sampling shall state the target population, inclusion mechanism, failure denominator, and error target.
- Multilevel, multiple-membership, and temporal structure shall be preserved when material; individual-level aggregation shall not substitute for a required hierarchical model.
- Synthetic data may establish known-truth unit and simulation tests but shall not serve as production interoperability or buyer-accuracy evidence.
- Model outputs shall include calibration, error, coverage, and abstention evidence appropriate to the model class.

## 10. Security and privacy requirements

- Authentication uses the `keyverse` identity product or a verified customer OIDC authority.
- Authorization evaluates tenant, actor, role, purpose, object, field, and action.
- Database-level tenant enforcement is required for network services.
- Evidence and partner documents are untrusted input.
- Connectors use deny-by-default egress, bounded payloads, MIME validation, and no active-content execution.
- LLM inputs are minimized and policy filtered; source content cannot alter system instructions or permissions.
- Secrets and raw credentials are never stored in domain events or logs.
- Writeback, export, approval, and high-risk configuration changes require immutable audit receipts.

## 11. User-experience requirements

- The primary interface organizes work around a disruption case and next action, not around internal service names.
- Each quantitative value provides its unit, time basis, scope, input snapshot, model status, and accessible exact-value representation.
- Graph views provide a keyboard-operable table and path list.
- Loading, empty, partial, stale, conflict, access-denied, error, and recovery states are designed explicitly.
- Users can distinguish fact, inference, proposal, decision, action, and outcome by text and semantics, not color alone.
- Mobile and intermediate-width layouts must not clip translated labels, tables, dialogs, or graph controls.

## 12. Non-functional requirements

### Performance

- Every buyer-facing page route and page-data response, and every common read API, targets p95 at or below 20 ms under the declared reference workload. Heavy computation remains asynchronous and is not hidden inside the page request.
- Heavy traversal, simulation, optimization, import, and export shall be asynchronous; command acceptance and job identity return through a bounded request path.
- Load tests shall not hide latency with reduced samples, excluded measurements, or unrealistic cache warm-up.

### Reliability

- At-least-once delivery is assumed; domain effects remain exactly-once through idempotency and conflict semantics.
- Transactional outbox and inbox receipts are required for state-changing events.
- Recovery exercises cover database restore, object evidence restore, queue replay, model artifact recovery, and adapter reconciliation.

### Compatibility

- HTTP APIs use OpenAPI 3.2.0.
- Message APIs use AsyncAPI 3.0.0 and CloudEvents 1.0.2.
- Errors use RFC 9457 problem details.
- External and database identifiers use RFC 9562 UUIDv7 where locally generated.
- Breaking API, event, persistence, and model changes require versioned migration and deprecation plans.

### Quality

- Production statement, branch, function, and relevant region coverage are 100% for owned code.
- Public modules, types, traits, functions, methods, endpoints, schemas, and error contracts are documented.
- Tests include unit, property, fuzz, integration, contract, security, recovery, performance, accessibility, and realistic buyer-journey evidence.

## 13. Product success criteria

A release candidate shall demonstrate:

- provenance completeness for all exposure paths and decision outputs;
- zero duplicate operational changes under retry and timeout tests;
- zero cross-tenant access in adversarial tests;
- reproducible results for identical input snapshots, model versions, and seeds;
- quantified prediction and scenario error against declared validation data;
- an end-to-end disruption-to-outcome journey using non-synthetic interoperability fixtures and sanitized realistic operations data;
- accessible operation in all supported product languages;
- signed, versioned artifacts with SBOM, provenance, migration, backup, restore, and rollback evidence.

## 14. Commercial outcome sequence

```text
M0 master planning baseline
→ M1 evidence-first graph foundation
→ M2 durable bitemporal persistence and tenant security
→ M3 source integration and traceability
→ M4 disruption case and impact service
→ M5 quantified impact and recovery scenarios
→ M6 decision, action, and writeback closure
→ M7 buyer UX and partner collaboration
→ M8 operability, release, and deployment evidence
→ M9 advanced supplier and network intelligence
```

Detailed entry and exit criteria are in `docs/ROADMAP.md`.

## 15. Current implemented slice

The current writer branch implements:

1. semantically identified supply nodes;
2. evidence-backed directed dependencies;
3. evidence-backed disruption events;
4. replay-safe node, dependency, and event upserts;
5. deterministic cycle-safe downstream reachability;
6. deterministic shortest dependency paths;
7. event evidence and edge-aligned dependency evidence on every impact record;
8. evidence-locator redaction in routine Rust `Debug`; and
9. exact Rust quality and coverage gates.

It does not implement persistence, network APIs, authentication, adapters, quantitative impact, scenario optimization, writeback, operational deployment, customer UI, or a released artifact. Its current result is a deterministic evidence-linked potential-impact record only.
