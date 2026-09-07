# Domain Model — Supply Chain Control Plane

## 1. Ubiquitous language

| Term | Definition |
| --- | --- |
| Supply party | A legal or operating entity participating in a supply relationship; roles are assigned by context and time |
| Party role assignment | A time-valid role such as supplier, customer, carrier, manufacturer, broker, or regulator |
| Supply site | A physical or logical location relevant to sourcing, production, storage, transit, service, or demand |
| Product item | A material, component, product, package, asset class, or service item referenced across source systems |
| Supply node | A typed identity participating in a dependency or flow graph |
| Supply dependency | An evidence-backed directed fact that a downstream object depends on an upstream object |
| Material relation | A versioned BOM, substitution, co-product, packaging, or transformation relationship |
| Supply lane | A time-valid origin, destination, mode, service, and constraint relationship |
| Supply position | A time-indexed inventory, capacity, demand, supply, order, shipment, production, or quality position |
| Evidence reference | A stable source-record key plus a protected locator and content digest |
| Supply event | A normalized observation about a supply object, relation, state, transaction, or condition |
| Truth status | `authoritative`, `observed`, `inferred`, `proposed`, `superseded`, or `rejected` |
| Knowledge cutoff | The latest availability time an assessment may use |
| Disruption case | A managed investigation and response lifecycle grouping evidence, exposure, scenarios, decisions, and actions |
| Case hypothesis | A proposed explanation or impact claim that is not yet an authoritative fact |
| Potential impact | A downstream object reachable through admitted dependencies; not probability or severity |
| Quantified impact | A model-derived estimate with target quantity, uncertainty, input snapshot, and validation provenance |
| Recovery scenario | A versioned set of candidate actions and constraints evaluated against a fixed assessment basis |
| Hard constraint | A condition that cannot be violated in a feasible scenario |
| Preference | A weighted or ordered objective that may trade off against another objective |
| Decision record | An immutable account of options, basis, approval, expiry, and selected action |
| Action commitment | An approved obligation assigned to an actor or adapter with due state and completion evidence |
| Writeback intent | A policy-approved request to change an external operating system |
| Writeback receipt | The authoritative response returned by the external system or adapter reconciliation process |
| Outcome observation | Evidence about what happened after an action |
| Model release | A versioned, validated, approved model or solver contract allowed in production |
| Input snapshot | The immutable set of network, event, position, constraint, policy, and model references used by a run |
| Exact replay | Repeated admission of the same semantic identity and immutable content |
| Replay conflict | Reuse of an identity with changed immutable content |

## 2. Entity and value-object map

### Network Registry

Entities:

- `SupplyParty`
- `PartyRoleAssignment`
- `SupplySite`
- `ProductItem`
- `SupplyDependency`
- `MaterialRelation`
- `SupplyLane`
- `ExternalObjectReference`
- `IdentityResolution`

Value objects:

- `SupplyObjectKey`
- `SourceAuthority`
- `TruthStatus`
- `BusinessValidity`
- `EvidenceReference`
- `MeasurementUnit`
- `QuantityValue`
- `LocationReference`

### Event Registry

Entities:

- `SupplyEvent`
- `EventCorrection`
- `EventSubjectBinding`
- `SourceCaptureReceipt`

Value objects:

- `EventOccurrence`
- `AvailabilityTime`
- `RecordedTime`
- `EventTypeCode`
- `EventAttribute`
- `ContentDigest`

### Disruption Management

Entities:

- `DisruptionCase`
- `CaseHypothesis`
- `CaseAssignment`
- `CaseEvidenceBinding`
- `CaseStatusTransition`

Value objects:

- `CaseSeverityBand` only when selected through an approved policy, never inferred from reachability;
- `CasePriority`
- `CaseState`
- `ResponseDueTime`

### Impact Assessment

Entities:

- `ImpactAssessmentRun`
- `ImpactPath`
- `ImpactEstimate`
- `AssessmentArtifact`

Value objects:

- `KnowledgeCutoff`
- `InputSnapshotReference`
- `ModelReleaseReference`
- `SolverTermination`
- `UncertaintyInterval`
- `CalibrationReference`
- `AbstentionReason`

### Recovery Scenario

Entities:

- `RecoveryScenario`
- `ScenarioConstraint`
- `ScenarioAction`
- `ScenarioRun`
- `ScenarioResult`
- `ScenarioTradeoff`

Value objects:

- `ConstraintKind`
- `ConstraintStrength`
- `ObjectiveDefinition`
- `FeasibilityState`
- `OptimalityGap`
- `ScenarioHorizon`

### Decision and Action Control

Entities:

- `DecisionRecord`
- `DecisionOption`
- `DecisionApproval`
- `ActionCommitment`
- `WritebackIntent`
- `WritebackReceipt`
- `OutcomeObservation`
- `PostActionReview`

Value objects:

- `DecisionState`
- `ApprovalRole`
- `RiskClass`
- `IdempotencyKey`
- `CorrelationIdentifier`
- `ExternalCommandReference`
- `OutcomeState`

## 3. Aggregate boundaries

### SupplyNodeAggregate

Owns one stable supply-node identity, type, current source mappings, and reviewed identity transitions. It does not own all dependencies touching the node.

### SupplyDependencyAggregate

Owns one semantic dependency identity, endpoint references, relation type, validity, source authority, truth status, and evidence. It rejects exact identity with changed immutable content.

### EvidenceRecordAggregate

Owns one content identity, capture receipt, digest, classification, protected locator, retention state, and chain-of-custody history.

### SupplyEventAggregate

Owns one normalized event identity and correction lineage. It never destructively rewrites an admitted event.

### DisruptionCaseAggregate

Owns case state, subjects, hypotheses, assignments, decision package references, and permitted transitions. Large evidence and assessment results are referenced rather than embedded.

### ImpactAssessmentRunAggregate

Owns immutable run configuration, knowledge cutoff, input snapshot, model release, termination state, and result artifact references.

### RecoveryScenarioAggregate

Owns scenario revisions, actions, constraints, and approved run submissions. Completed runs are immutable.

### DecisionRecordAggregate

Owns options, decision basis, approvals, segregation-of-duties validation, selected option, expiry, and supersession.

### ActionCommitmentAggregate

Owns dispatch eligibility, adapter command reference, attempts, receipts, reconciliation, outcome state, and closure evidence.

### ModelReleaseAggregate

Owns model identity, version, mathematical contract, validation evidence, approved scope, activation, deactivation, and rollback.

## 4. Domain services

- `IdentityResolutionService` — proposes and validates cross-system identity mappings.
- `NetworkSnapshotService` — creates immutable network snapshots at a knowledge cutoff.
- `PotentialImpactService` — computes deterministic evidence-linked reachability.
- `QuantifiedImpactService` — executes an approved impact model against an immutable snapshot.
- `ScenarioFeasibilityService` — validates hard constraints before or during solver execution.
- `ScenarioComparisonService` — exposes Pareto trade-offs without inventing a universal score.
- `DecisionAuthorizationService` — validates role, purpose, segregation, expiry, and residual-risk conditions.
- `WritebackDispatchService` — maps an approved commitment to an adapter command and reconciles its result.
- `OutcomeEvaluationService` — compares predicted and realized outcomes under the original decision basis.

## 5. Domain events

Representative domain events:

- `SupplyNodeUpserted`
- `SupplyDependencyUpserted`
- `SupplyEventRecorded`
- `SupplyEventCorrected`
- `DisruptionCaseOpened`
- `CaseHypothesisProposed`
- `ImpactAssessmentStarted`
- `ImpactAssessmentCompleted`
- `ImpactAssessmentAbstained`
- `RecoveryScenarioSubmitted`
- `ScenarioRunCompleted`
- `DecisionApproved`
- `DecisionExpired`
- `ActionCommitted`
- `WritebackDispatched`
- `WritebackSucceeded`
- `WritebackFailed`
- `OutcomeRecorded`
- `PostActionReviewCompleted`
- `ModelReleaseActivated`
- `ModelReleaseRetired`

External event names and payload versions are defined in `docs/EVENT_CONTRACT.md`.

## 6. State machines

### Disruption case

```text
open
→ triaged
→ assessing
→ scenario_review
→ decision_pending
→ executing
→ monitoring
→ closed
```

Alternative terminal or suspended states:

```text
rejected_observation
superseded
cancelled
```

A case cannot move to `closed` while required action commitments are unresolved unless a decision explicitly accepts the residual state.

### Impact assessment

```text
requested
→ validating
→ queued
→ running
→ completed
```

Alternative outcomes:

```text
abstained
infeasible_input
failed
cancelled
superseded
```

### Recovery scenario run

```text
requested
→ validating
→ queued
→ running
→ feasible | infeasible | indeterminate
```

### Decision

```text
draft
→ review_pending
→ approved | rejected | returned
approved
→ executing | expired | superseded
executing
→ completed | partially_completed | failed
```

### Action commitment

```text
planned
→ authorized
→ dispatched
→ acknowledged
→ completed
```

Alternative outcomes:

```text
rejected
failed
partially_completed
cancelled
expired
```

## 7. Domain policies

- Evidence admission policy
- Source authority policy
- Identity conflict policy
- Knowledge-cutoff policy
- Model eligibility policy
- Scenario hard-constraint policy
- Decision segregation policy
- Writeback risk and approval policy
- Outcome closure policy
- Retention and legal-hold policy

Policies are versioned domain objects. A policy version is recorded on every decision-relevant evaluation.

## 8. Explicit modeling prohibitions

- `supplier`, `customer`, and `carrier` are not immutable party types; they are contextual, time-valid roles.
- `risk_score` is not a universal column. Each estimate has a named target and model.
- `status` without a bounded-context-specific type is prohibited.
- Generic one-word tables such as `node`, `event`, `record`, or `data` are prohibited.
- An LLM confidence value is not truth status, statistical probability, or authorization.
- Graph distance is not lead time, causal strength, impact severity, or probability.
- A scenario result is not a decision; a decision is not an executed action; an action dispatch is not an outcome.
