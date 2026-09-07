# Operability — Supply Chain Control Plane

## 1. Operating model

The product is operated as a modular application with separately scalable workers for import, assessment, simulation, optimization, export, and adapter writeback. A deployment may use Docker Compose, Podman, Colima, or Kubernetes-compatible orchestration while preserving the same health, configuration, storage, queue, and security contracts. Managed, customer-private, restricted/disconnected, and developer-evaluation responsibility profiles are defined in `docs/DEPLOYMENT_PROFILES.md`.

## 2. Service signals

| Signal | Meaning |
| --- | --- |
| `/healthz` | Process is alive; no dependency claim |
| `/startupz` | Required configuration and local initialization completed |
| `/readyz` | Instance can accept its declared traffic class |
| `/drainz` | Instance no longer accepts new work and reports outstanding work |

Read API, command API, worker, projection, and writeback readiness are independently observable. A worker without an approved model or adapter cannot report ready for that workload.

## 3. Workload classes

```text
interactive_read
interactive_command
source_ingestion
projection_update
impact_assessment
scenario_simulation
scenario_optimization
writeback_dispatch
report_export
maintenance_recovery
```

Each class has separate concurrency, timeout, CPU, memory, database, queue, and retry policy. There is no universal model or job timeout; provider and model operations respect per-model administrative configuration and explicit user cancellation.

## 4. Service objectives

Initial release-candidate targets are engineering acceptance targets, not current claims.

| SLI | Target |
| --- | --- |
| Every buyer-facing page route and page-data request p95 | ≤ 20 ms at the published reference workload |
| Common read API p95 | ≤ 20 ms at the published reference workload |
| Interactive command acceptance p95 | ≤ 50 ms, excluding asynchronous execution |
| Event admission receipt p95 | ≤ 100 ms under declared ingestion load |
| Job state freshness | ≤ 2 seconds after durable transition |
| Writeback duplicate external effects | 0 in retry, timeout, and replay tests |
| Provenance completeness | 100% for decision-relevant results |
| Monthly service availability target | 99.9% for GA control/read plane |
| Recovery point target | ≤ 5 minutes for durable metadata; zero acknowledged-event loss through outbox/inbox |
| Recovery time target | ≤ 60 minutes for a tested single-region restore profile |

Heavy graph, simulation, and optimization workloads return a durable job reference through the interactive request budget. Their completion SLOs are workload-specific and published with the supported data-size profile.

## 5. Capacity reference profiles

### Foundation profile

- 100,000 supply objects;
- 1,000,000 admitted relations;
- 10,000,000 normalized event records;
- 100 concurrent interactive users;
- 100 event admissions per second sustained;
- depth-6 potential-impact assessment under the declared worker profile.

### Enterprise profile

- 1,000,000 supply objects;
- 10,000,000 admitted relations;
- 1,000,000,000 event records through partitioned storage and archival;
- 1,000 concurrent interactive users;
- 1,000 event admissions per second sustained;
- separately scaled assessment and projection workers.

These profiles become supported only after k6, database, graph, queue, object-storage, and recovery evidence is published. Sample reduction, unreported exclusions, and unrealistic warm-cache-only tests are prohibited.

## 6. Queue and job behavior

Each durable job stores:

- tenant and workload class;
- immutable request digest;
- input snapshot and model release;
- state transition history;
- retry classification and attempt count;
- checkpoint reference when the algorithm supports safe checkpointing;
- cancellation request and observed cancellation state;
- terminal result or safe diagnostic artifact;
- resource usage and provenance.

Retry classes:

```text
transient_transport
transient_capacity
provider_rate_limit
permanent_schema
permanent_policy
permanent_identity
security_quarantine
operator_intervention
```

Security, identity, schema, and policy failures do not retry automatically.

## 7. Database operation

- Bounded connection pools per workload class
- Connection and statement timeouts based on operation type
- Lock-wait and transaction-duration metrics
- Query plans captured for high-latency regressions without sensitive literals
- Tenant-and-time partitioning for event/audit tables when measurements justify it
- UPSERT per semantic item, never full-table replacement
- Migration advisory lock and single migration owner
- DDL-free normal startup
- Clean install, N-1 upgrade, rollback/forward-fix, and schema compatibility tests
- `close_connection` and pool shutdown verified during drain and test cleanup

PostgreSQL memory, shared buffers, WAL, checkpoints, autovacuum, and temporary space are tuned from actual hardware and workload evidence. Test project names are overridden only for isolation.

## 8. Object storage operation

- Content-addressed artifact identity
- Tenant-scoped encryption context and prefix
- Immutable or object-lock policy for regulated evidence where configured
- Digest verification on write and read
- Lifecycle and retention policy receipts
- Multipart upload abort and orphan cleanup
- Backup/replication and restore test
- No public bucket or stable raw evidence locator

## 9. Event transport operation

- Outbox publication lag metric
- Inbox processing lag and failure metric
- Payload digest and schema version
- Consumer offset/checkpoint backup
- Replay tooling with tenant and time bounds
- Dead-letter inspection and redrive with approval
- At-least-once delivery and idempotent effect contract

## 10. Observability

OpenTelemetry instrumentation covers:

- HTTP request and response outcome;
- database pool wait, query class, transaction duration, and conflict;
- event outbox/inbox lag;
- job queue depth, wait, runtime, cancellation, and retry;
- adapter calls, status, rate limiting, and reconciliation;
- model and solver release, workload size, termination, and resource use;
- projection freshness;
- writeback success, partial failure, and duplicate-prevention result;
- backup, restore, migration, and release verification.

Forbidden telemetry:

- raw evidence;
- prompts and responses;
- credentials and cookies;
- protected evidence locators;
- unrestricted PII;
- unbounded tenant or object identifiers as metric labels.

Logs are structured and include correlation and safe opaque references. High-cardinality identities belong in traces or audited records, not metrics.

## 11. Resource governance

Budgets exist for:

```text
cpu_time
resident_memory
gpu_memory
process_count
open_file_count
disk_bytes
temporary_bytes
network_bytes
database_connections
graph_nodes_visited
graph_edges_visited
maximum_path_count
maximum_payload_bytes
job_runtime
```

When a budget is exceeded, the system returns an explicit resource-limit state, preserves diagnostics, and avoids partial authoritative mutation.

## 12. Failure modes

| Failure | Expected posture |
| --- | --- |
| PostgreSQL unavailable | readiness false; no unacknowledged authoritative command success |
| Object storage unavailable | metadata-only operations continue only where evidence is not required |
| Event broker unavailable | outbox retains events; bounded backpressure applied |
| Projection stale | UI shows freshness and can fall back to authoritative query where safe |
| Model worker unavailable | assessment remains queued or fails with workload-specific state |
| Adapter unavailable | no alternate-provider duplicate writeback; retry or operator decision |
| External system timeout | retain ambiguous attempt, reconcile before retry when required |
| Identity provider unavailable | existing short-lived sessions follow configured policy; new authentication fails closed |
| Encryption key unavailable | restricted evidence access fails closed; recovery incident raised |

## 13. Backup and disaster recovery

Backup scope:

- PostgreSQL base backup and WAL/PITR;
- object evidence and model artifacts;
- event schemas, offsets, and replay receipts;
- encryption-key versions and recovery procedure;
- release, migration, configuration, and policy manifests.

Restore exercise verifies:

1. clean infrastructure deployment;
2. database restoration to a declared point;
3. object digest and access recovery;
4. event replay without duplicate domain effects;
5. model and adapter artifact verification;
6. tenant isolation and audit continuity;
7. read, assessment, decision, and controlled writeback smoke tests.

A backup is not considered valid until a restore rehearsal succeeds.

## 14. Release and rollback

- Build once; promote the same signed artifact by digest.
- Database compatibility is verified before promotion.
- Feature and model activation are independently reversible.
- Writeback capabilities can be disabled without disabling read-only incident response.
- Rollback does not silently downgrade an incompatible database or event schema.
- Forward-fix is preferred where a rollback would violate historical compatibility.
- Every deployment produces a release receipt containing artifact digest, schema version, model set, configuration digest, and verification results.

## 15. Runbooks

Required runbooks:

```text
database_unavailable
object_storage_unavailable
event_backlog
projection_stale
adapter_timeout
ambiguous_writeback
model_regression
cross_tenant_incident
evidence_key_unavailable
backup_restore
schema_migration_failure
release_rollback
security_kill_switch
```

Each runbook identifies detection, customer impact, immediate containment, evidence preservation, decision owner, recovery steps, validation, and follow-up action.
