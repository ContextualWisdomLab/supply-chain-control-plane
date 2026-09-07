# Data Model — Supply Chain Control Plane

The reviewable logical relationship view is `docs/ERD.md`. This document remains authoritative for fields, constraints, temporal semantics, indexing, and migration behavior.

## 1. Data modeling principles

- PostgreSQL relational tables are normalized to at least third normal form.
- Every database object uses two or more words in `snake_case`.
- Generic one-word tables such as `node`, `event`, `data`, `record`, `item`, or `mapping` are prohibited.
- External provider identifiers are isolated in reference tables and never used as internal primary keys.
- Mutable business state and immutable historical facts are separated.
- Business-valid time and system-recorded time are stored explicitly.
- JSONB is not used for authorization, amount, quantity, status, validity, or other decision-critical fields.
- Every tenant-owned table contains a tenant key and participates in database-level tenant enforcement.
- Append-only evidence, event, decision, approval, writeback, outcome, and audit history is not hard-deleted outside a governed disposition workflow.

## 2. Identifier contract

Locally created identities use UUIDv7 under RFC 9562. Cross-product references use the released `context-graph-contracts` canonical reference format. Source-system references are stored separately.

Representative keys:

```text
tenant_workspace_id
supply_party_id
supply_site_id
product_item_id
supply_dependency_id
supply_event_id
disruption_case_id
impact_assessment_run_id
recovery_scenario_id
decision_record_id
action_commitment_id
```

## 3. Workspace and authorization

```text
tenant_workspace
workspace_role_assignment
purpose_grant
service_principal_binding
resource_policy
field_access_rule
retention_policy
legal_hold_record
audit_event_record
```

### `tenant_workspace`

- `tenant_workspace_id`
- `tenant_reference`
- `workspace_name`
- `workspace_status_code`
- `created_at`
- `retired_at`

### `workspace_role_assignment`

- `workspace_role_assignment_id`
- `tenant_workspace_id`
- `identity_subject_reference`
- `role_code`
- `valid_from`
- `valid_to`
- `recorded_at`
- `recorded_by`

## 4. Network Registry

```text
supply_party
party_external_reference
party_role_assignment
supply_site
site_external_reference
product_item
item_external_reference
item_site_assignment
supply_dependency_edge
material_relation
supply_lane
network_snapshot
network_snapshot_member
identity_resolution_case
identity_resolution_decision
```

### `supply_party`

- `supply_party_id`
- `tenant_workspace_id`
- `party_display_name`
- `party_lifecycle_code`
- `created_at`

The display name is not a universal legal name. Legal and source-specific names remain source references or versioned attributes.

### `party_role_assignment`

- `party_role_assignment_id`
- `tenant_workspace_id`
- `supply_party_id`
- `role_context_reference`
- `party_role_code`
- `valid_from`
- `valid_to`
- `truth_status_code`
- `evidence_source_record_id`
- `recorded_at`
- `superseded_at`

### `supply_dependency_edge`

- `supply_dependency_id`
- `tenant_workspace_id`
- `upstream_object_reference`
- `downstream_object_reference`
- `dependency_type_code`
- `valid_from`
- `valid_to`
- `truth_status_code`
- `source_authority_code`
- `evidence_source_record_id`
- `immutable_content_digest`
- `recorded_at`
- `superseded_at`

A semantic uniqueness constraint covers tenant, upstream, downstream, dependency type, and the selected identity scope. Exact replay compares immutable content digest and evidence identity.

### `material_relation`

- `material_relation_id`
- `tenant_workspace_id`
- `parent_product_item_id`
- `component_product_item_id`
- `relation_type_code`
- `component_quantity_value`
- `component_unit_code`
- `scrap_factor_value`
- `valid_from`
- `valid_to`
- `source_authority_code`
- `truth_status_code`
- `evidence_source_record_id`

### `supply_lane`

- `supply_lane_id`
- `tenant_workspace_id`
- `origin_site_id`
- `destination_site_id`
- `transport_mode_code`
- `service_level_code`
- `valid_from`
- `valid_to`
- `source_authority_code`
- `evidence_source_record_id`

## 5. Evidence Registry

```text
evidence_source_record
evidence_artifact_record
source_capture_record
evidence_access_receipt
evidence_chain_event
evidence_disposition_record
```

### `evidence_source_record`

- `evidence_source_record_id`
- `tenant_workspace_id`
- `source_system_code`
- `source_record_key`
- `source_locator_ciphertext`
- `content_digest`
- `media_type_code`
- `classification_code`
- `capture_record_id`
- `available_at`
- `recorded_at`
- `retention_policy_id`
- `legal_hold_status_code`

The protected locator is never emitted by default serialization or routine telemetry.

### `source_capture_record`

- `source_capture_record_id`
- `tenant_workspace_id`
- `adapter_registration_id`
- `adapter_version`
- `capture_started_at`
- `capture_completed_at`
- `source_snapshot_reference`
- `request_digest`
- `response_digest`
- `capture_status_code`

## 6. Event Registry

```text
supply_event_record
event_subject_binding
event_attribute_value
event_relation
event_correction_record
projection_receipt
```

### `supply_event_record`

- `supply_event_id`
- `tenant_workspace_id`
- `event_type_code`
- `event_identity_key`
- `event_started_at`
- `event_ended_at`
- `asserted_at`
- `available_at`
- `recorded_at`
- `truth_status_code`
- `source_authority_code`
- `evidence_source_record_id`
- `immutable_content_digest`
- `corrected_event_id`
- `superseded_at`

### `event_subject_binding`

- `event_subject_binding_id`
- `tenant_workspace_id`
- `supply_event_id`
- `subject_object_reference`
- `subject_role_code`

### `event_attribute_value`

Typed tables or a tagged relational structure shall represent approved event extensions. Unit, scale, precision, and semantic definition are mandatory for quantitative attributes.

## 7. Supply positions

```text
inventory_position
capacity_position
demand_position
supply_position
order_reference
shipment_reference
production_reference
quality_reference
position_snapshot
position_snapshot_member
```

Positions are time-indexed observations or source-authoritative references. They are not silently merged into one generic fact table.

### `inventory_position`

- `inventory_position_id`
- `tenant_workspace_id`
- `product_item_id`
- `supply_site_id`
- `inventory_status_code`
- `quantity_value`
- `quantity_unit_code`
- `position_time`
- `available_at`
- `source_authority_code`
- `evidence_source_record_id`

### `capacity_position`

- `capacity_position_id`
- `tenant_workspace_id`
- `capacity_resource_reference`
- `capacity_bucket_start`
- `capacity_bucket_end`
- `capacity_value`
- `capacity_unit_code`
- `availability_code`
- `source_authority_code`
- `evidence_source_record_id`

## 8. Disruption Management

```text
disruption_case
case_subject_binding
case_evidence_binding
case_hypothesis
case_assignment
case_status_transition
case_evidence_request
```

### `disruption_case`

- `disruption_case_id`
- `tenant_workspace_id`
- `case_title`
- `case_state_code`
- `case_priority_code`
- `opened_at`
- `knowledge_cutoff`
- `current_owner_reference`
- `current_version_number`
- `closed_at`

### `case_hypothesis`

- `case_hypothesis_id`
- `tenant_workspace_id`
- `disruption_case_id`
- `hypothesis_type_code`
- `hypothesis_statement`
- `truth_status_code`
- `confidence_value`
- `model_artifact_reference`
- `proposed_by_reference`
- `proposed_at`
- `reviewed_at`
- `review_result_code`

Confidence and truth status remain separate.

## 9. Impact Assessment

```text
impact_assessment_run
assessment_input_snapshot
impact_path
impact_path_step
impact_estimate
impact_uncertainty
assessment_artifact
assessment_status_transition
```

### `impact_assessment_run`

- `impact_assessment_run_id`
- `tenant_workspace_id`
- `disruption_case_id`
- `assessment_type_code`
- `knowledge_cutoff`
- `network_snapshot_id`
- `position_snapshot_id`
- `model_release_id`
- `configuration_digest`
- `random_seed_manifest`
- `run_state_code`
- `started_at`
- `completed_at`
- `termination_code`

### `impact_path_step`

- `impact_path_step_id`
- `tenant_workspace_id`
- `impact_path_id`
- `step_sequence_number`
- `upstream_object_reference`
- `downstream_object_reference`
- `supply_dependency_id`
- `evidence_source_record_id`

### `impact_estimate`

- `impact_estimate_id`
- `tenant_workspace_id`
- `impact_assessment_run_id`
- `affected_object_reference`
- `target_measure_code`
- `estimate_value`
- `estimate_unit_code`
- `horizon_start`
- `horizon_end`
- `estimate_status_code`
- `abstention_reason_code`

### `impact_uncertainty`

- `impact_uncertainty_id`
- `tenant_workspace_id`
- `impact_estimate_id`
- `uncertainty_type_code`
- `lower_bound_value`
- `upper_bound_value`
- `coverage_level_value`
- `calibration_reference`

## 10. Recovery Scenario

```text
recovery_scenario
scenario_revision
scenario_constraint
scenario_action
scenario_run
scenario_result
scenario_tradeoff
scenario_binding_constraint
```

### `scenario_constraint`

- `scenario_constraint_id`
- `tenant_workspace_id`
- `scenario_revision_id`
- `constraint_type_code`
- `constraint_strength_code`
- `subject_object_reference`
- `measure_code`
- `operator_code`
- `constraint_value`
- `constraint_unit_code`
- `valid_from`
- `valid_to`
- `evidence_source_record_id`

### `scenario_run`

- `scenario_run_id`
- `tenant_workspace_id`
- `scenario_revision_id`
- `impact_assessment_run_id`
- `model_release_id`
- `solver_release_id`
- `configuration_digest`
- `random_seed_manifest`
- `run_state_code`
- `feasibility_code`
- `optimality_gap_value`
- `started_at`
- `completed_at`
- `termination_code`

## 11. Decision and Action Control

```text
decision_record
decision_option
decision_evidence_binding
decision_approval
decision_status_transition
action_commitment
writeback_intent
writeback_attempt
writeback_receipt
outcome_observation
post_action_review
```

### `decision_approval`

- `decision_approval_id`
- `tenant_workspace_id`
- `decision_record_id`
- `approver_subject_reference`
- `approval_role_code`
- `approval_result_code`
- `approval_comment`
- `policy_version_reference`
- `approved_at`
- `revoked_at`

### `writeback_intent`

- `writeback_intent_id`
- `tenant_workspace_id`
- `action_commitment_id`
- `adapter_registration_id`
- `external_object_mapping_id`
- `command_type_code`
- `command_schema_version`
- `command_payload_digest`
- `idempotency_key`
- `authorization_decision_id`
- `created_at`

### `writeback_receipt`

- `writeback_receipt_id`
- `tenant_workspace_id`
- `writeback_attempt_id`
- `external_receipt_reference`
- `external_state_code`
- `response_payload_digest`
- `receipt_status_code`
- `received_at`
- `reconciled_at`

## 12. Model Governance

```text
model_definition
model_release
model_validation_run
model_validation_metric
model_scope_assignment
model_activation_record
solver_definition
solver_release
compute_profile
```

### `model_release`

- `model_release_id`
- `tenant_workspace_id` when tenant-specific, otherwise a controlled global authority scope;
- `model_definition_id`
- `semantic_version`
- `artifact_digest`
- `source_commit_sha`
- `training_data_reference`
- `validation_protocol_reference`
- `approval_status_code`
- `approved_scope_digest`
- `activated_at`
- `retired_at`

## 13. Integration reliability

```text
adapter_registration
adapter_capability
external_object_mapping
idempotency_record
outbox_event
inbox_event
processing_checkpoint
dead_letter_record
reconciliation_run
reconciliation_exception
```

### `idempotency_record`

- `idempotency_record_id`
- `tenant_workspace_id`
- `operation_scope_code`
- `idempotency_key`
- `request_digest`
- `operation_status_code`
- `response_reference`
- `created_at`
- `expires_at`

A repeated key with a different request digest is a conflict.

## 14. Indexing and query design

Required indexes are derived from measured queries, but the initial design includes:

- tenant plus semantic identity uniqueness;
- tenant plus valid-time overlap prevention where supported;
- source system plus source record key;
- event type plus occurrence and availability time;
- case state plus owner and priority;
- assessment/scenario job state plus creation time;
- decision/action state plus due time;
- inbox/outbox status plus next attempt time;
- evidence digest and classification;
- external object mapping by adapter and external identity.

Graph projections may use Apache AGE or another graph store only as a read model. PostgreSQL normalized records remain the command authority unless an ADR demonstrates a different owner.

## 15. Migration and compatibility

- Clean install and supported N-1 upgrade are tested on every release candidate.
- Expand, backfill, validate, and contract phases are separated.
- Runtime startup refuses unsupported schema-ahead or schema-behind states.
- Destructive migration requires verified backup, restore, rollback/forward-fix plan, and owner approval.
- Database compatibility is included in the release manifest.
