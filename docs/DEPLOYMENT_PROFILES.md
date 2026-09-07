# Deployment Profiles — Supply Chain Control Plane

## 1. Purpose

The product supports different customer security, sovereignty, integration, and operating models without changing domain semantics. A deployment profile fixes responsibility, topology, network policy, storage, key ownership, update flow, evidence retention, and recovery obligations. A profile is supported only after its exact installation, upgrade, load, security, backup, and restore evidence is published.

## 2. Common invariant

Every profile preserves:

- the same versioned API, event, truth-status, temporal, evidence, and idempotency contracts;
- tenant isolation or a formally documented single-tenant boundary;
- PostgreSQL and object-storage durability;
- controlled egress and hostile-artifact isolation;
- signed artifact verification and immutable release manifests;
- customer-visible health, readiness, audit, backup, and recovery evidence;
- no production dependency on mutable repository branches or sibling paths.

## 3. Profile A — Managed isolated tenant

Use when the buyer permits vendor operation but requires logical and infrastructure isolation.

```text
customer identity federation
→ dedicated edge and application namespace/account
→ isolated database and object-storage tenancy
→ managed workers and adapters
→ customer source systems through private connectivity
```

Responsibilities:

| Responsibility | Product operator | Customer |
| --- | --- | --- |
| Application and worker operation | Accountable | Consulted |
| Identity source and user lifecycle | Integrates and validates | Accountable |
| Source-system authority and data quality | Consulted | Accountable |
| Encryption-key policy | Operates under contract | Approves or supplies key boundary |
| Backup and restore execution | Accountable | Reviews evidence |
| Adapter credentials | Stores through approved secret boundary | Issues and revokes |
| Action approval | Enforces policy | Accountable |

## 4. Profile B — Customer-managed private deployment

Use for private cloud, data-residency, regulated, or deep integration requirements.

- Signed OCI and deployment bundles are promoted by digest.
- Customer owns infrastructure account, networking, database, object storage, secrets, and operational access.
- Product release manifests declare supported Kubernetes-compatible and Compose/Podman profiles.
- Remote vendor access is not assumed; support uses customer-approved evidence bundles.
- Telemetry export is opt-in, policy filtered, and never contains raw evidence, credentials, protected locators, or unrestricted PII.

The shared-responsibility matrix is part of the contract and release package.

## 5. Profile C — Restricted or disconnected deployment

Use when continuous internet access is prohibited.

- Installation uses offline-verifiable signed bundles, checksums, SBOM, provenance, migrations, clients, models, and documentation.
- Provider-dependent LLM features are disabled unless a customer-approved internal `contextual-orchestrator` endpoint is available.
- External signal ingestion occurs through approved transfer or internal adapters.
- Vulnerability and license databases have signed offline update packages.
- Time synchronization, certificate rotation, key recovery, and release transfer have documented procedures.
- No feature silently falls back to an external provider.

Disconnected operation is not claimed until an isolated installation, upgrade, rollback/forward-fix, backup, restore, and revocation rehearsal passes.

## 6. Profile D — Developer evaluation

This profile is for local evaluation only.

- Compose, Podman, or Colima may run application, PostgreSQL, object storage, event transport, and test identity services.
- Synthetic data is permitted only for unit/simulation tests and product walkthrough scaffolding.
- It does not permit real customer PII, unrestricted source credentials, production writeback, or certification claims.
- Local success is not transferable to managed, private, or disconnected production profiles.

## 7. Connectivity patterns

Supported patterns are declared per adapter:

```text
private_link_or_vpn
customer_outbound_push
control_plane_pull_through_egress_policy
customer_managed_gateway
file_or_event_drop
manual_evidence_upload
```

Each pattern defines source authentication, network allowlist, replay identity, schema version, payload limits, observability, key rotation, failure ownership, and revocation.

## 8. Data residency and retention

A deployment manifest identifies:

```text
data_region
metadata_region
evidence_region
backup_region
telemetry_region
encryption_key_authority
retention_profile
legal_hold_capability
support_evidence_export_policy
```

Cross-region replication, support export, and disaster recovery never occur by implication. They require explicit customer policy and auditable configuration.

## 9. Upgrade and support boundary

For every supported profile:

- installation and N-1 upgrade are rehearsed on the exact release candidate;
- schema-ahead and schema-behind states fail closed;
- the same signed artifact is promoted rather than rebuilt;
- database and object-storage backup responsibilities are explicit;
- model, adapter, and writeback activation can be rolled back independently;
- support bundles are purpose limited and redact secrets and protected evidence;
- end-of-support and security-fix windows are published.
