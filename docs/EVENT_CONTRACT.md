# Event Contract — Supply Chain Control Plane

## 1. Envelope

All cross-process and cross-product events use CloudEvents 1.0.2 and are described through AsyncAPI 3.0.0. HTTP, Kafka, NATS, or another transport may carry the same event semantics.

Required attributes:

```json
{
  "specversion": "1.0",
  "id": "019f...",
  "source": "urn:cwl:tenant_001:supply-chain-control-plane",
  "type": "org.contextualwisdomlab.supply_chain.supply_event.observed.v1",
  "subject": "urn:cwl:tenant_001:supply_event:019f...",
  "time": "2026-09-02T06:00:00Z",
  "datacontenttype": "application/json",
  "dataschema": "https://contextualwisdomlab.github.io/supply-chain-control-plane/schemas/supply-event-observed-v1.json",
  "tenantref": "urn:cwl:tenant_001",
  "correlationid": "019f...",
  "causationid": "019f...",
  "provenanceref": "urn:cwl:tenant_001:evidence_source_record:019f...",
  "data": {}
}
```

`source + id` is the transport deduplication key. A domain command or source event also carries its own immutable identity or idempotency key.

## 2. Event naming

```text
org.contextualwisdomlab.supply_chain.<aggregate>.<past_tense_event>.v<major>
```

Examples:

```text
org.contextualwisdomlab.supply_chain.network_fact.observed.v1
org.contextualwisdomlab.supply_chain.supply_event.corrected.v1
org.contextualwisdomlab.supply_chain.disruption_case.opened.v1
org.contextualwisdomlab.supply_chain.impact_assessment.completed.v1
org.contextualwisdomlab.supply_chain.recovery_scenario.evaluated.v1
org.contextualwisdomlab.supply_chain.decision.approved.v1
org.contextualwisdomlab.supply_chain.writeback.succeeded.v1
```

Event names describe completed facts. Commands are not published as facts.

## 3. Common data fields

Every decision-relevant event data object includes:

```json
{
  "event_version": "1.0.0",
  "truth_status_code": "observed",
  "business_validity": {
    "valid_from": "2026-09-02T05:55:00Z",
    "valid_to": null
  },
  "available_at": "2026-09-02T05:56:00Z",
  "recorded_at": "2026-09-02T05:56:01Z",
  "source_authority_code": "erp_a",
  "operation_receipt_reference": "urn:cwl:tenant_001:operation_receipt:019f..."
}
```

Events contain references rather than unrestricted evidence, credentials, personal data, prompts, or provider payloads.

## 4. Event catalog

### Network and evidence

- `network_fact.observed.v1`
- `network_fact.superseded.v1`
- `identity_resolution.proposed.v1`
- `identity_resolution.approved.v1`
- `evidence_record.captured.v1`
- `evidence_record.disposed.v1`

### Supply events

- `supply_event.observed.v1`
- `supply_event.corrected.v1`
- `supply_event.rejected.v1`

### Disruption cases

- `disruption_case.opened.v1`
- `disruption_case.triaged.v1`
- `disruption_case.state_changed.v1`
- `case_hypothesis.proposed.v1`
- `case_hypothesis.reviewed.v1`

### Impact assessment

- `impact_assessment.started.v1`
- `impact_assessment.completed.v1`
- `impact_assessment.abstained.v1`
- `impact_assessment.failed.v1`
- `impact_assessment.superseded.v1`

### Recovery scenarios

- `recovery_scenario.created.v1`
- `scenario_run.started.v1`
- `scenario_run.completed.v1`
- `scenario_run.infeasible.v1`
- `scenario_run.failed.v1`

### Decisions and actions

- `decision.submitted.v1`
- `decision.approved.v1`
- `decision.rejected.v1`
- `decision.expired.v1`
- `decision.superseded.v1`
- `action_commitment.created.v1`
- `writeback.dispatched.v1`
- `writeback.succeeded.v1`
- `writeback.failed.v1`
- `action_outcome.recorded.v1`
- `post_action_review.completed.v1`

### Model governance

- `model_release.approved.v1`
- `model_release.activated.v1`
- `model_release.retired.v1`
- `model_validation.failed.v1`

## 5. Representative event payloads

### Supply event observed

```json
{
  "event_version": "1.0.0",
  "supply_event_reference": "urn:cwl:tenant_001:supply_event:019f...",
  "event_type_code": "shipment_delayed",
  "subject_references": [
    "urn:cwl:tenant_001:shipment_reference:019f..."
  ],
  "event_occurrence": {
    "started_at": "2026-09-02T03:00:00Z",
    "ended_at": null
  },
  "available_at": "2026-09-02T03:04:00Z",
  "recorded_at": "2026-09-02T03:04:02Z",
  "truth_status_code": "observed",
  "source_authority_code": "carrier_api",
  "evidence_reference": "urn:cwl:tenant_001:evidence_source_record:019f...",
  "operation_receipt_reference": "urn:cwl:tenant_001:operation_receipt:019f..."
}
```

### Impact assessment completed

```json
{
  "event_version": "1.0.0",
  "impact_assessment_reference": "urn:cwl:tenant_001:impact_assessment_run:019f...",
  "disruption_case_reference": "urn:cwl:tenant_001:disruption_case:019f...",
  "assessment_type_code": "potential_impact",
  "knowledge_cutoff": "2026-09-02T03:05:00Z",
  "input_snapshot_reference": "urn:cwl:tenant_001:assessment_input_snapshot:019f...",
  "model_release_reference": "urn:cwl:global:model_release:potential-impact-1.0.0",
  "termination_code": "completed",
  "result_manifest_reference": "urn:cwl:tenant_001:assessment_artifact:019f...",
  "truth_status_code": "inferred",
  "recorded_at": "2026-09-02T03:05:11Z"
}
```

### Decision approved

```json
{
  "event_version": "1.0.0",
  "decision_reference": "urn:cwl:tenant_001:decision_record:019f...",
  "selected_option_reference": "urn:cwl:tenant_001:decision_option:019f...",
  "approval_policy_reference": "urn:cwl:tenant_001:resource_policy:decision-approval-3",
  "approval_receipt_references": [
    "urn:cwl:tenant_001:decision_approval:019f..."
  ],
  "effective_until": "2026-09-02T15:00:00Z",
  "truth_status_code": "authoritative",
  "recorded_at": "2026-09-02T04:00:00Z"
}
```

### Writeback succeeded

```json
{
  "event_version": "1.0.0",
  "action_commitment_reference": "urn:cwl:tenant_001:action_commitment:019f...",
  "writeback_intent_reference": "urn:cwl:tenant_001:writeback_intent:019f...",
  "adapter_registration_reference": "urn:cwl:tenant_001:adapter_registration:erp-a-4.2.0",
  "external_receipt_reference": "erp-a:change-document:9001842",
  "receipt_reference": "urn:cwl:tenant_001:writeback_receipt:019f...",
  "truth_status_code": "observed",
  "recorded_at": "2026-09-02T04:03:22Z"
}
```

## 6. Delivery and processing

- Delivery is at least once.
- The producer writes state and an outbox row in one transaction.
- The relay publishes the immutable payload and records attempts.
- The consumer validates envelope, schema, tenant, source, and payload digest before domain processing.
- The consumer writes an inbox receipt and domain effect in one transaction where possible.
- Exact replay returns the existing processing receipt.
- Same event identity with a different payload digest is quarantined as a conflict.
- Retry schedules distinguish transient, permanent, policy, schema, and security failures.
- Dead letters retain reason, payload digest, source, schema version, attempt history, and safe evidence reference.

## 7. Ordering

Global ordering is not assumed. Event contracts declare their ordering scope:

- per aggregate;
- per source record;
- per adapter partition;
- unordered.

Consumers use version numbers, effective time, recorded time, and correction relationships rather than relying on broker arrival order.

## 8. Corrections and supersession

A correction event references the corrected event and explains the correction type. The original event remains immutable. The current projection selects the valid fact according to domain rules and knowledge cutoff.

A late observation may describe an earlier occurrence. It receives its true occurrence time and later availability/recorded time; historical decision replay still excludes it before availability.

## 9. Schema compatibility

- Patch versions clarify documentation or constraints without changing valid payload meaning.
- Minor versions add optional fields or new independently handled event types.
- Major event type suffix increments for incompatible payload or semantic changes.
- Unknown enum values are preserved as unknown and never coerced into a known state.
- Producer and consumer contract fixtures run against exact release artifacts.
- Event schemas use immutable versioned URLs and artifact digests.
