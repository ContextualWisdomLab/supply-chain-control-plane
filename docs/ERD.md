# Logical ERD — Supply Chain Control Plane

## 1. ERD authority

This document is the reviewable logical relationship view of `docs/DATA_MODEL.md`. The field lists, constraints, temporal semantics, and migration rules in `docs/DATA_MODEL.md` remain authoritative. Generated physical ERDs must be compared against migrations and may not silently redefine the model.

## 2. Workspace, evidence, network, and events

```mermaid
erDiagram
    tenant_workspace ||--o{ workspace_role_assignment : authorizes
    tenant_workspace ||--o{ supply_party : contains
    tenant_workspace ||--o{ supply_site : contains
    tenant_workspace ||--o{ product_item : contains
    tenant_workspace ||--o{ evidence_source_record : owns
    tenant_workspace ||--o{ source_capture_record : records
    tenant_workspace ||--o{ supply_event_record : records

    supply_party ||--o{ party_role_assignment : receives
    supply_site ||--o{ item_site_assignment : hosts
    product_item ||--o{ item_site_assignment : assigned_to
    product_item ||--o{ material_relation : parent
    product_item ||--o{ material_relation : component

    evidence_source_record ||--o{ supply_dependency_edge : supports
    evidence_source_record ||--o{ material_relation : supports
    evidence_source_record ||--o{ supply_lane : supports
    evidence_source_record ||--o{ supply_event_record : supports
    source_capture_record ||--o{ evidence_source_record : captured

    supply_event_record ||--o{ event_subject_binding : concerns
    supply_event_record ||--o{ event_relation : source_event
    supply_event_record ||--o{ event_relation : target_event
    supply_event_record ||--o{ event_correction_record : corrected

    tenant_workspace {
      uuid tenant_workspace_id PK
      text tenant_reference
      text workspace_name
      text workspace_status_code
      timestamptz created_at
      timestamptz retired_at
    }
    evidence_source_record {
      uuid evidence_source_record_id PK
      uuid tenant_workspace_id FK
      text source_system_code
      text source_record_key
      text content_digest
      timestamptz available_at
      timestamptz recorded_at
    }
    supply_dependency_edge {
      uuid supply_dependency_id PK
      uuid tenant_workspace_id FK
      text upstream_object_reference
      text downstream_object_reference
      text dependency_type_code
      text truth_status_code
      timestamptz valid_from
      timestamptz valid_to
      text immutable_content_digest
    }
    supply_event_record {
      uuid supply_event_id PK
      uuid tenant_workspace_id FK
      text event_type_code
      text event_identity_key
      timestamptz event_started_at
      timestamptz event_ended_at
      timestamptz available_at
      timestamptz recorded_at
      text truth_status_code
    }
```

## 3. Cases, assessments, scenarios, decisions, and actions

```mermaid
erDiagram
    tenant_workspace ||--o{ disruption_case : owns
    disruption_case ||--o{ case_subject_binding : concerns
    disruption_case ||--o{ case_evidence_binding : uses
    disruption_case ||--o{ case_hypothesis : contains
    disruption_case ||--o{ case_status_transition : transitions
    disruption_case ||--o{ impact_assessment_run : evaluates

    network_snapshot ||--o{ network_snapshot_member : contains
    position_snapshot ||--o{ position_snapshot_member : contains
    model_release ||--o{ impact_assessment_run : executes
    impact_assessment_run ||--o{ assessment_input_snapshot : freezes
    impact_assessment_run ||--o{ impact_path : produces
    impact_path ||--o{ impact_path_step : contains
    impact_assessment_run ||--o{ impact_estimate : produces
    impact_estimate ||--o{ impact_uncertainty : qualifies

    impact_assessment_run ||--o{ recovery_scenario : bases
    recovery_scenario ||--o{ scenario_action : proposes
    recovery_scenario ||--o{ scenario_constraint : constrains
    recovery_scenario ||--o{ scenario_run : executes
    scenario_run ||--o{ scenario_result : produces

    recovery_scenario ||--o{ decision_option : offers
    decision_record ||--o{ decision_option : evaluates
    decision_record ||--o{ approval_record : requires
    decision_record ||--o{ action_commitment : authorizes
    action_commitment ||--o{ writeback_attempt : dispatches
    writeback_attempt ||--o{ writeback_receipt : receives
    action_commitment ||--o{ outcome_observation : observes
    outcome_observation ||--o{ outcome_evaluation : evaluates

    disruption_case {
      uuid disruption_case_id PK
      uuid tenant_workspace_id FK
      text case_state_code
      text case_priority_code
      timestamptz knowledge_cutoff
      bigint current_version_number
    }
    impact_assessment_run {
      uuid impact_assessment_run_id PK
      uuid disruption_case_id FK
      uuid network_snapshot_id FK
      uuid position_snapshot_id FK
      uuid model_release_id FK
      text run_state_code
      text termination_code
      text configuration_digest
    }
    recovery_scenario {
      uuid recovery_scenario_id PK
      uuid disruption_case_id FK
      uuid impact_assessment_run_id FK
      text scenario_state_code
      bigint scenario_version_number
    }
    decision_record {
      uuid decision_record_id PK
      uuid disruption_case_id FK
      text decision_state_code
      timestamptz decided_at
      timestamptz expires_at
    }
    action_commitment {
      uuid action_commitment_id PK
      uuid decision_record_id FK
      text action_type_code
      text action_state_code
      text target_system_code
      text idempotency_key
    }
```

## 4. Integration, jobs, projection, and governance

```mermaid
erDiagram
    tenant_workspace ||--o{ adapter_registration : configures
    adapter_registration ||--o{ adapter_capability : declares
    adapter_registration ||--o{ source_capture_record : captures
    adapter_registration ||--o{ writeback_attempt : dispatches

    outbox_event ||--o{ event_delivery_attempt : publishes
    inbox_event ||--o{ projection_receipt : processes
    durable_job ||--o{ job_state_transition : transitions
    durable_job ||--o{ job_attempt : attempts
    durable_job ||--o{ job_artifact : produces

    model_definition ||--o{ model_release : versions
    model_release ||--o{ model_validation_run : validates
    model_validation_run ||--o{ validation_metric : measures
    model_release ||--o{ model_activation_record : activates

    audit_event_record ||--o{ audit_evidence_binding : supports
    retention_policy ||--o{ evidence_source_record : governs
    legal_hold_record ||--o{ legal_hold_binding : protects

    adapter_registration {
      uuid adapter_registration_id PK
      uuid tenant_workspace_id FK
      text adapter_code
      text adapter_version
      text source_system_kind
      text registration_status_code
    }
    durable_job {
      uuid durable_job_id PK
      uuid tenant_workspace_id FK
      text workload_class_code
      text request_digest
      text job_state_code
      timestamptz created_at
      timestamptz completed_at
    }
    model_release {
      uuid model_release_id PK
      uuid model_definition_id FK
      text release_version
      text artifact_digest
      text model_status_code
      timestamptz valid_from
      timestamptz retired_at
    }
```

## 5. Physical-model rules

- Foreign keys include tenant consistency, not only object identity.
- Valid-time intervals cannot overlap where one active semantic fact is allowed.
- Immutable event/evidence/decision/action history is append-only.
- External IDs reside in external-reference or object-mapping tables.
- Decision-critical amounts, quantities, units, statuses, and authorization fields are relational columns rather than opaque JSONB.
- Event and audit partitions are introduced only with measured volume and query evidence; the logical identity remains stable.
- Read projections may denormalize but have no command authority.
- Each migration must reproduce this ERD from a clean database and from the supported N-1 schema.
