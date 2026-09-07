# API Contract — Supply Chain Control Plane

## 1. Contract baseline

- Specification: OpenAPI 3.2.0
- Error representation: RFC 9457 `application/problem+json`
- Local identifiers: UUIDv7 under RFC 9562
- Authentication: OIDC/OAuth access tokens or service credentials verified through the `keyverse` identity product or a verified customer IdP
- Command replay control: required `Idempotency-Key`
- Mutable-resource concurrency: `ETag` and `If-Match`
- Collection pagination: opaque keyset cursor
- Heavy work: asynchronous job resources
- Content negotiation: JSON by default; standards-specific media types on adapter endpoints

This document fixes endpoint semantics and security behavior. It does not claim that the endpoints are implemented.

## 2. Common request context

Authenticated requests resolve the following server-side context:

```json
{
  "tenant_workspace_id": "019f...",
  "actor_subject_reference": "urn:cwl:identity:subject:...",
  "service_principal_reference": null,
  "purpose_code": "supply_disruption_response",
  "correlation_id": "019f...",
  "decision_reference": null,
  "locale_code": "ko-KR"
}
```

Clients cannot override verified tenant or actor identity with arbitrary headers. Purpose and decision context are accepted only when the caller is authorized to assert them.

## 3. Common headers

| Header | Use |
| --- | --- |
| `Authorization` | Verified bearer or service credential |
| `Idempotency-Key` | Required for state-changing POST commands |
| `If-Match` | Required for optimistic updates where documented |
| `X-Correlation-Id` | Optional caller correlation; server supplies one if absent |
| `Accept-Language` | Human-readable message locale only |
| `Traceparent` | W3C trace propagation without domain identity authority |

## 4. Problem details

Every API problem includes stable machine semantics:

```json
{
  "type": "https://contextualwisdomlab.github.io/supply-chain-control-plane/problems/replay-conflict",
  "title": "Replay conflict",
  "status": 409,
  "detail": "The idempotency key was already used with different immutable content.",
  "instance": "urn:cwl:supply-chain:problem:019f...",
  "problem_code": "replay_conflict",
  "correlation_id": "019f..."
}
```

`detail` never contains stack traces, SQL, filesystem paths, credentials, evidence locators, or unrestricted source payloads.

Core problem codes:

```text
invalid_request
unauthenticated
access_denied
resource_not_found
version_conflict
replay_conflict
identity_conflict
unsupported_schema
unsupported_unit
unknown_source_authority
invalid_truth_transition
evidence_required
evidence_unavailable
knowledge_cutoff_violation
model_not_approved
insufficient_data
out_of_distribution
assessment_abstained
scenario_infeasible
solver_terminated
approval_required
decision_expired
writeback_not_authorized
adapter_unavailable
external_conflict
rate_limited
resource_limit_exceeded
```

## 5. Workspace and configuration APIs

```text
GET    /v1/workspaces/{workspace_id}
GET    /v1/workspaces/{workspace_id}/capabilities
GET    /v1/workspaces/{workspace_id}/source-authorities
POST   /v1/workspaces/{workspace_id}/source-authorities
GET    /v1/workspaces/{workspace_id}/model-releases
POST   /v1/workspaces/{workspace_id}/model-activations
GET    /v1/workspaces/{workspace_id}/adapters
POST   /v1/workspaces/{workspace_id}/adapters
```

Configuration mutations require explicit administrator scope, `Idempotency-Key`, and audit receipt. Model activation additionally requires an approved model release and compatible scope.

## 6. Network Registry APIs

```text
POST   /v1/network/supply-nodes:upsert
POST   /v1/network/supply-dependencies:upsert
GET    /v1/network/supply-nodes/{supply_node_id}
GET    /v1/network/supply-dependencies/{supply_dependency_id}
GET    /v1/network/supply-nodes/{supply_node_id}/relationships
POST   /v1/network/identity-resolution-cases
POST   /v1/network/identity-resolution-cases/{case_id}/decisions
POST   /v1/network/snapshots
GET    /v1/network/snapshots/{network_snapshot_id}
```

### Upsert semantics

Request:

```json
{
  "semantic_key": "erp_a:item:MAT-001",
  "node_kind_code": "product_item",
  "source_authority_code": "erp_a",
  "truth_status_code": "observed",
  "business_validity": {
    "valid_from": "2026-09-01T00:00:00Z",
    "valid_to": null
  },
  "evidence_reference": {
    "source_record_key": "erp_a:item:MAT-001:rev-8",
    "evidence_reference": "urn:cwl:tenant:...:evidence_source_record:019f..."
  },
  "immutable_content_digest": "sha256:..."
}
```

Response:

```json
{
  "operation_outcome_code": "inserted",
  "resource_reference": "urn:cwl:tenant:...:supply_node:019f...",
  "operation_receipt": "urn:cwl:tenant:...:operation_receipt:019f..."
}
```

Allowed outcomes are `inserted` and `unchanged`. Changed immutable content under the same identity returns `409 replay_conflict`.

## 7. Evidence APIs

```text
POST   /v1/evidence/captures
POST   /v1/evidence/records
GET    /v1/evidence/records/{evidence_record_id}
POST   /v1/evidence/records/{evidence_record_id}/access-receipts
POST   /v1/evidence/records/{evidence_record_id}/disposition-requests
GET    /v1/evidence/records/{evidence_record_id}/chain-of-custody
```

The default evidence response excludes the protected locator. Authorized drill-through returns a short-lived access reference, not a durable raw locator in general API payloads.

## 8. Event APIs

```text
POST   /v1/supply-events:ingest
POST   /v1/supply-events:batch-ingest
GET    /v1/supply-events/{supply_event_id}
POST   /v1/supply-events/{supply_event_id}/corrections
GET    /v1/supply-events
POST   /v1/epcis/capture
POST   /v1/epcis/query
```

EPCIS endpoints are standards adapters. Internally normalized event identity, truth status, evidence, and temporal semantics remain explicit.

## 9. Disruption case APIs

```text
POST   /v1/disruption-cases
GET    /v1/disruption-cases/{disruption_case_id}
GET    /v1/disruption-cases
POST   /v1/disruption-cases/{disruption_case_id}/subjects
POST   /v1/disruption-cases/{disruption_case_id}/evidence
POST   /v1/disruption-cases/{disruption_case_id}/hypotheses
POST   /v1/disruption-cases/{disruption_case_id}/assignments
POST   /v1/disruption-cases/{disruption_case_id}/transitions
GET    /v1/disruption-cases/{disruption_case_id}/timeline
```

Case transition commands require the current `ETag`. Invalid transitions return `409 invalid_truth_transition` or a case-specific transition problem.

## 10. Impact assessment APIs

```text
POST   /v1/impact-assessments
GET    /v1/impact-assessments/{impact_assessment_run_id}
POST   /v1/impact-assessments/{impact_assessment_run_id}/cancellations
GET    /v1/impact-assessments/{impact_assessment_run_id}/paths
GET    /v1/impact-assessments/{impact_assessment_run_id}/estimates
GET    /v1/impact-assessments/{impact_assessment_run_id}/artifacts
```

Create request:

```json
{
  "disruption_case_reference": "urn:cwl:tenant:...:disruption_case:019f...",
  "assessment_type_code": "potential_impact",
  "knowledge_cutoff": "2026-09-02T06:00:00Z",
  "network_snapshot_reference": "urn:cwl:tenant:...:network_snapshot:019f...",
  "position_snapshot_reference": null,
  "model_release_reference": "urn:cwl:global:supply_chain:model_release:potential-impact-1.0.0",
  "maximum_depth": 12,
  "included_relation_type_codes": ["depends_on", "contains", "feeds"]
}
```

Response: `202 Accepted` with a durable job reference. Synchronous completion may be used only for explicitly bounded potential-impact queries that satisfy the configured resource budget.

Every estimate response includes:

- target measure and unit;
- horizon;
- truth status;
- model/solver version;
- input snapshot references;
- uncertainty or abstention;
- provenance references;
- run termination state.

## 11. Recovery scenario APIs

```text
POST   /v1/recovery-scenarios
GET    /v1/recovery-scenarios/{recovery_scenario_id}
POST   /v1/recovery-scenarios/{recovery_scenario_id}/revisions
POST   /v1/recovery-scenarios/{recovery_scenario_id}/runs
GET    /v1/scenario-runs/{scenario_run_id}
GET    /v1/scenario-runs/{scenario_run_id}/results
GET    /v1/scenario-runs/{scenario_run_id}/binding-constraints
POST   /v1/scenario-runs/{scenario_run_id}/cancellations
```

Scenario results expose feasible alternatives and Pareto trade-offs. The API does not return a universal winner unless an approved decision policy defines one and its version is recorded.

## 12. Decision APIs

```text
POST   /v1/decisions
GET    /v1/decisions/{decision_record_id}
POST   /v1/decisions/{decision_record_id}/options
POST   /v1/decisions/{decision_record_id}/submissions
POST   /v1/decisions/{decision_record_id}/approvals
POST   /v1/decisions/{decision_record_id}/rejections
POST   /v1/decisions/{decision_record_id}/returns
POST   /v1/decisions/{decision_record_id}/supersessions
GET    /v1/decisions/{decision_record_id}/basis
```

Approval responses include the policy version, segregation-of-duties result, scope, expiry, and audit receipt.

## 13. Action and writeback APIs

```text
POST   /v1/action-commitments
GET    /v1/action-commitments/{action_commitment_id}
POST   /v1/action-commitments/{action_commitment_id}/authorizations
POST   /v1/action-commitments/{action_commitment_id}/dispatches
POST   /v1/action-commitments/{action_commitment_id}/cancellations
GET    /v1/action-commitments/{action_commitment_id}/writeback-receipts
POST   /v1/action-commitments/{action_commitment_id}/outcomes
POST   /v1/action-commitments/{action_commitment_id}/closure-reviews
```

A dispatch request identifies an approved action and adapter capability; it does not contain raw credentials. Dispatch returns an attempt receipt even when the external system rejects or times out.

## 14. Read model and search APIs

```text
GET    /v1/command-center
GET    /v1/action-queue
GET    /v1/network/impact-view
POST   /v1/search/evidence
POST   /v1/search/supply-context
GET    /v1/reports/disruption-summary
GET    /v1/reports/scenario-comparison
GET    /v1/reports/outcome-evaluation
```

Read APIs expose projection freshness, source cutoffs, and partial-data status. Search rank is accompanied by source and channel contribution where relevant.

## 15. Export APIs

```text
POST   /v1/exports
GET    /v1/exports/{export_job_id}
GET    /v1/exports/{export_job_id}/manifest
POST   /v1/exports/{export_job_id}/access-receipts
```

Supported planned formats include JSON, CSV, Parquet, GraphML, JSON-LD/PROV, EPCIS, SVG, and accessible PDF where semantically applicable. Export authorization is purpose-specific and produces an immutable manifest containing selection scope, hashes, schema versions, and redaction decisions.

## 16. Job resource contract

```json
{
  "job_reference": "urn:cwl:tenant:...:assessment_job:019f...",
  "job_state_code": "queued",
  "submitted_at": "2026-09-02T06:01:00Z",
  "status_endpoint": "/v1/impact-assessments/019f...",
  "cancel_endpoint": "/v1/impact-assessments/019f.../cancellations"
}
```

States are `requested`, `validating`, `queued`, `running`, `completed`, `failed`, `cancelled`, `abstained`, `infeasible_input`, and `superseded`. A failed job preserves a stable problem code and safe diagnostic artifact reference.

## 17. Compatibility policy

- Additive fields remain optional until all supported clients understand them.
- Enum expansion follows an unknown-value contract; clients must not map unknown values to an existing value.
- Removed or behavior-changing fields require a new API version.
- Deprecation responses include successor operation, first deprecated release, and removal release.
- Provider-specific payloads remain behind adapter namespaces or artifact references.
- Contract fixtures are tested by producer and consumer at exact release SHAs.
