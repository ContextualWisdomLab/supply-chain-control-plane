# Security and Threat Model — Supply Chain Control Plane

## 1. Security objective

The product handles sensitive supplier, inventory, production, logistics, customer-commitment, pricing, operational, and decision evidence. It can also dispatch high-impact commands to operating systems. Security therefore protects confidentiality, integrity, availability, tenant isolation, decision authorization, evidence provenance, and operational safety.

This document is a design baseline, not a certification claim.

## 2. Protected assets

- tenant and workspace identity;
- supplier and customer relationships;
- item, BOM, capacity, inventory, production, order, and shipment context;
- disruptions, vulnerabilities, shortages, and recovery plans;
- evidence artifacts and protected locators;
- model, solver, prompt, and configuration artifacts;
- approvals, actions, writeback commands, and external receipts;
- audit, retention, legal-hold, and export records;
- service credentials, signing keys, and encryption keys;
- release artifacts, SBOM, and provenance.

## 3. Data classification

| Class | Examples | Default handling |
| --- | --- | --- |
| `public` | Published standards and public product documentation | Public access after review |
| `internal` | Non-sensitive configuration and operational metadata | Authenticated tenant access |
| `confidential_supply_chain` | Network mappings, suppliers, lanes, capacity, scenarios | Need-to-know, encrypted, purpose limited |
| `restricted_operational` | Live inventory, orders, production, shortages, customer commitments | Strong authorization, field-level filtering, detailed audit |
| `restricted_personal` | Named contacts, comments, communication metadata | Data minimization and privacy controls |
| `restricted_security` | Vulnerability, incident, architecture, and control evidence | Security role and purpose checks |
| `secret` | Tokens, keys, credentials, signing material | External secret manager; never domain payload or log |

## 4. Trust boundaries

```text
untrusted source systems and documents
        │
        ▼
source adapter / quarantine / validation boundary
        │
        ▼
trusted command API
        │
        ▼
PostgreSQL + object evidence + event transport
        │
        ▼
analysis and scenario workers
        │
        ▼
decision approval boundary
        │
        ▼
writeback adapter and external operating system
```

Separate boundaries exist between tenants, human users, service principals, models, browser clients, external CWL services, object storage, event transport, and release infrastructure.

## 5. Threats and required controls

### Identity spoofing

Threats:

- forged bearer tokens;
- confused tenant claims;
- service principal impersonation;
- replayed delegation;
- email-address or display-name identity joins.

Controls:

- exact OIDC issuer, audience, signature, token type, expiry, nonce/delegation validation;
- `keyverse` or verified customer-IdP subject references rather than email authority;
- tenant membership resolved server-side;
- short-lived scoped service credentials;
- explicit delegation chain and revocation;
- identity mapping review and audit.

### Cross-tenant access

Threats:

- guessed identifiers;
- missing tenant predicate;
- projection or object-storage prefix leakage;
- background job executed under the wrong tenant.

Controls:

- tenant key on all owned records;
- PostgreSQL RLS or equivalent database enforcement;
- tenant-scoped object paths and encryption context;
- tenant-bound job and event identity;
- cross-tenant adversarial tests returning non-disclosing `404`;
- no tenant identity accepted from an unverified client field.

### Evidence tampering and source spoofing

Threats:

- forged source event;
- modified artifact;
- changed source locator;
- late evidence presented as contemporaneous;
- inference presented as authoritative.

Controls:

- cryptographic digest and capture receipt;
- source authority registry;
- event, availability, and recorded times;
- immutable evidence metadata and chain of custody;
- truth-status transition policy;
- correction and supersession rather than overwrite;
- signed source events where available;
- historical knowledge-cutoff enforcement.

### Prompt injection and model manipulation

Threats:

- source document instructs the model to ignore policy;
- retrieved content changes tool permissions;
- poisoned evidence causes fabricated facts;
- model output is treated as authorization or numeric truth.

Controls:

- source content remains untrusted data;
- system, policy, evidence, and secrets are separated;
- schema-constrained output with evidence references;
- no arbitrary tool discovery from source content;
- model has no writeback credential or approval capability;
- independent output validation and human review;
- model output truth status is `inferred` or `proposed`;
- prompt, model, provider, schema, and evidence hashes are recorded.

### Malicious documents and parser exploitation

Threats:

- active HTML, scripts, macros, external fetches, malformed MIME, decompression bombs, path traversal, polyglot files, parser resource exhaustion.

Controls:

- no active-content execution in trusted services;
- magic-byte and MIME validation;
- byte, nesting, expansion, table, row, and time limits;
- rootless isolated analysis through the quarantine runtime where available;
- deny-by-default egress;
- immutable input artifact and parser version receipt;
- fail-closed parsing and explicit partial-result state.

### SSRF and outbound exfiltration

Threats:

- connector follows attacker URL;
- DNS rebinding;
- redirect to internal network;
- model or document causes secret exfiltration.

Controls:

- approved host, scheme, port, method, redirect, DNS, and address policy;
- private/link-local/metadata address denial unless separately authorized;
- EgressWeave or equivalent released policy boundary;
- request and response size limits;
- no secrets exposed to model or source content;
- outbound request receipt and redacted audit.

### Event replay and ordering attacks

Threats:

- duplicate event causes duplicate fact or action;
- same event ID carries changed payload;
- old event arrives after a correction;
- broker order is mistaken for business order.

Controls:

- outbox/inbox and payload digest;
- source-event and domain idempotency keys;
- changed-payload conflict quarantine;
- version, valid time, recorded time, correction relation, and source sequence handling;
- no global ordering assumption.

### Graph and solver resource exhaustion

Threats:

- cyclic or adversarial dependency graph;
- unbounded depth or fan-out;
- oversized scenario;
- solver monopolizes CPU/GPU/memory;
- malicious objective or numeric overflow.

Controls:

- explicit node, edge, depth, fan-out, path, time, and memory budgets;
- cycle-safe algorithms;
- asynchronous jobs with cancellation;
- checked numeric conversion and finite-value validation;
- per-tenant quotas and concurrency;
- isolated worker pools and resource limits;
- termination and partial-result semantics.

### Unauthorized or duplicate writeback

Threats:

- analytical worker writes directly to ERP;
- expired decision executes;
- retry produces duplicate change;
- approval and execution performed by the same prohibited actor;
- adapter maps to the wrong tenant or object.

Controls:

- models and analysis workers have no write credentials;
- Decision Control authorizes Action Control through an immutable receipt;
- segregation-of-duties policy;
- decision expiry and version checks;
- adapter capability allowlist;
- idempotency key and external receipt reconciliation;
- preview/dry-run for supported high-risk commands;
- kill switch per adapter, tenant, command type, and model release.

### Supply-chain software compromise

Threats:

- malicious dependency, action, container, model artifact, or generated client;
- mutable tag substitution;
- compromised build or release.

Controls:

- exact dependency and action pins;
- lockfiles and dependency review;
- SBOM, vulnerability scan, provenance, signing, and verification;
- reproducible or independently rebuilt release evidence where feasible;
- protected branches, independent approval, and no force push;
- model and adapter artifacts pinned by digest;
- release promotion rather than rebuilding per environment.

## 6. Authorization model

Authorization evaluates:

```text
tenant
+ actor or service principal
+ role
+ purpose
+ resource type and identity
+ field classification
+ requested action
+ decision or approval context
+ time and delegation validity
```

Representative roles:

```text
supply_risk_analyst
operations_planner
procurement_leader
supply_operator
decision_approver
data_steward
model_reviewer
security_reviewer
tenant_administrator
external_auditor
```

Role alone is insufficient for restricted evidence, export, model activation, risk acceptance, or writeback.

## 7. Segregation of duties

The following duties are separate by default:

- evidence producer and evidence approver;
- model developer and model activator;
- scenario proposer and high-risk decision approver;
- decision approver and writeback credential administrator;
- action executor and closure verifier;
- export requester and restricted export approver.

Solo-maintainer development governance does not imply a production tenant may waive business segregation. Explicit low-risk policy exceptions are time-bound and audited.

## 8. Encryption and secrets

- TLS is required in transit.
- Restricted PostgreSQL, object storage, queue, backup, and artifact data is encrypted at rest.
- Tenant-aware envelope encryption is used for protected evidence locators and highly restricted artifacts.
- Keys reside in a secret/KMS boundary and are rotated with versioned rewrap receipts.
- Raw PATs, API keys, passwords, cookies, and private keys never enter domain tables, events, prompts, or logs.
- Backup restore includes key-availability verification.

## 9. Audit and privacy

Audit records capture who, what, tenant, purpose, decision, resource, fields, result, time, correlation, and policy version. Audit logs avoid prompt bodies, evidence bodies, secrets, and unrestricted PII.

PII is not destructively masked in authoritative evidence when legitimate work requires it. Instead, purpose-specific views expose only necessary fields, while authorized access to full evidence is separately audited.

## 10. Security verification

Required tests include:

- token and tenant confusion;
- guessed cross-tenant identifiers;
- RLS bypass attempts;
- hostile evidence locator and debug/log leakage;
- prompt injection and tool escalation;
- SSRF, redirects, DNS rebinding, metadata endpoints;
- malformed and oversized EPCIS/MIME/JSON/CSV/document inputs;
- graph cycles, path explosion, and solver exhaustion;
- duplicate and reordered events;
- duplicate writeback under timeout;
- stale/expired approval;
- model artifact substitution;
- SBOM/provenance/signature verification;
- backup restore and key rotation.

## 11. Security incident posture

The product supports:

- immediate adapter/model/tenant kill switch;
- credential and signing-key rotation;
- writeback suspension with read-only investigation;
- evidence and audit preservation under legal hold;
- affected version and source identification through provenance;
- correction and compensating action without history deletion;
- post-incident control evidence publication to `governance-risk-compliance`.
