# ADR 0006 — Federated CWL Integration Through Released Owner Contracts

- Status: Proposed
- Date: 2026-09-02
- Decision owners: Supply Chain Control Plane and CWL architecture owners

## Context

ContextualWisdomLab contains specialized owners for identity, enterprise architecture, graph contracts, ontology publication and consumption, orchestration, temporal analysis, psychometrics, ingestion, evidence processing, retrieval, collaboration, security, runtime isolation, commercial billing, and accounting. Copying their source, schemas, prompts, or database truth into this repository would create conflicting authorities and mutable cross-repository dependencies.

Several owner products remain pre-release. The control plane must benefit from them without making an unprotected sibling branch a production prerequisite or reimplementing their missing functionality permanently in the consumer.

## Decision

- Domain truth remains with its canonical owner.
- This repository consumes only released, versioned API, event, schema, package, model-artifact, or OCI contracts.
- Integration CI pins exact release versions, digests, or immutable release SHAs; production never consumes sibling branch heads or local cross-repository paths.
- Missing reusable capability is developed in the owner repository with a RED contract test, implementation, documentation, release evidence, and consumer validation.
- Temporary bridges require an owner issue, explicit expiry, deletion condition, and production prohibition.
- Cross-service SQL and copied domain kernels are prohibited.
- Projections retain authority, truth status, provenance, source version, temporal validity, and processing receipt.

The intended owners include:

- `context-graph-contracts` for shared identifiers, temporal/provenance structures, and interoperability contracts;
- `keyverse` for identity and credential lifecycle;
- `Orgmetra` for people, organization, position, assignment, and internal responsibility references;
- `enterprise-architecture-core` for enterprise context, initiatives, and transformations;
- `ConceptWeave` for ontology publication and `semantic-data-portal` for ontology/data catalog, governance, and consumption;
- `contextual-orchestrator` for LLM routing and execution, with `pg-llm-batch` as a released batch/token capability rather than an alternative provider path;
- `TEPP` and `fast-mlsirm` for validated temporal and psychometric analysis;
- `mhtml-etl-gateway`, `mightyETL`, `newsdom-api`, `OriginWeave`, `ThreadWeave`, `LineageWeave`, `RankWeave`, `EmbedRelay`, `inkspan`, `DiagramWeave`, and `naruon` for their published specialist contracts;
- `governance-risk-compliance`, `wardnet`, `EgressWeave`, `pingora-gateway`, `quarantine-sandbox-runtime`, `appguardrail`, `noema`, and `ContextualWisdomLab/.github` for the released controls they actually own;
- `metering-billing-platform` for CWL product usage and commercial entitlement truth; and
- `accounting-information-platform` for journal, ledger, posting, and financial-control truth where that product is adopted.

Repository names above preserve the live organization casing. Their inclusion expresses an intended authority boundary, not a claim that every contract is currently released or suitable for synchronous production use.

## Alternatives considered

### Copy owner source into this repository

Rejected. It creates divergent implementations and ambiguous ownership.

### Read owner databases directly

Rejected. It bypasses invariants, authorization, temporal semantics, and versioned contracts.

### Exclude an immature owner and build a permanent local substitute

Rejected. A reusable capability must be completed in its canonical owner. A temporary bridge is permissible only under the expiry and deletion rules above.

### Make every owner a synchronous runtime dependency immediately

Rejected. Early milestones use stable local ports and released contracts. Synchronous coupling is introduced only when required by product behavior and supported by availability, latency, failure, security, and recovery evidence.

## Consequences

- Some product work requires coordinated owner and consumer PR stacks.
- Release engineering and integration CI are part of the architecture rather than operational afterthoughts.
- Optional integrations can mature independently without blocking the evidence-first core.
- Supply Chain Control Plane remains understandable as a supply-chain product instead of becoming a copy of the CWL ecosystem.
