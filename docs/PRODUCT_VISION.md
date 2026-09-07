# Product Vision — Supply Chain Control Plane

## Product category

Supply Chain Control Plane is an evidence-first supply-chain decision and resilience control plane. It sits above customer-owned ERP, PLM, MES, QMS, WMS, TMS, SRM, planning, carrier, partner, and traceability systems. It connects their facts without replacing their transactional authority.

The product exists to close the gap between knowing that something changed and deciding what to do next with defensible evidence.

> Observe a disruption, trace its downstream exposure, compare feasible responses, approve an action, execute through controlled writeback, and verify the result without losing provenance or uncertainty.

## Buyer problem

Supply-chain teams usually have enough raw data to know that an event occurred but not enough connected context to answer all of the following at decision speed:

- Which materials, sites, production steps, shipments, orders, customers, and commitments depend on the affected object?
- Which facts are authoritative, observed, inferred, proposed, corrected, or stale?
- What was known at the time of the decision rather than learned afterward?
- Which response options are feasible under inventory, capacity, lead-time, contract, quality, logistics, and policy constraints?
- Who approved the response, what was written back to operating systems, and did the action work?

Existing systems often answer only one part of the chain. ERP owns transactions, planning systems optimize within configured models, visibility platforms collect events, risk products monitor external signals, and collaboration tools hold human context. The control plane connects those domains while preserving their ownership boundaries.

## Primary product promise

For every reported exposure, scenario, recommendation, decision, and action, the product preserves:

1. the exact input snapshot;
2. the business-valid and system-recorded times;
3. the source authority and truth status;
4. the dependency or constraint path;
5. the model, solver, rules, and version used;
6. uncertainty, feasibility, and abstention state;
7. approval and writeback receipts; and
8. observed outcomes and post-action review.

## First commercial wedge

The first commercial workflow is **Supply Disruption Impact and Recovery**:

```text
observed disruption
→ evidence and identity validation
→ deterministic downstream exposure
→ shortage and commitment impact estimation
→ constrained recovery alternatives
→ human approval
→ idempotent ERP/TMS/WMS writeback
→ outcome verification
```

The current Rust `SupplyGraph` implements only the deterministic evidence-linked exposure part of this workflow. It is a foundation, not a claim that the full workflow is shipped.

## Target buyers

- Chief supply chain officers and supply-chain transformation leaders
- Supply-risk, resilience, procurement, and supplier-management teams
- Sales and operations planning and integrated business planning teams
- Manufacturing, logistics, fulfillment, and customer-service operations
- Enterprise architecture, data governance, GRC, and internal audit teams supporting supply-chain decisions
- Regulated or traceability-intensive industries where evidence and chain of custody matter

## Product users

| User | Decision supported |
| --- | --- |
| Supply-risk analyst | Establish what changed, which evidence supports it, and what may be exposed |
| Operations planner | Quantify shortage, capacity, service, and timing consequences |
| Procurement leader | Compare alternate supply, expediting, contract, and supplier-response options |
| Plant or logistics operator | Execute approved changes and report operational outcomes |
| Customer-commitment owner | Decide which commitments require protection, reallocation, or renegotiation |
| Executive | Review material exposure, trade-offs, action ownership, and recovery progress |
| Data steward | Resolve identity, lineage, quality, and source-authority conflicts |
| Security or compliance reviewer | Verify access, evidence, approvals, segregation of duties, and audit history |

## Product scope

### Core responsibilities

- Cross-system supply-network identity and relationship mapping
- Bitemporal event and evidence registry
- Disruption case management
- Deterministic and quantitative impact assessment
- Recovery scenario modeling and constrained optimization
- Decision, approval, action, writeback, and outcome control
- Model, solver, input-snapshot, and validation provenance
- Buyer-facing workflow, audit, and interoperable exports

### External authorities retained

- Legal supplier, customer, and material masters remain in ERP or MDM.
- Bill of materials and routing remain in PLM, ERP, or MES.
- Purchase and sales orders remain in ERP.
- Physical inventory and warehouse operations remain in WMS or ERP.
- Production execution remains in MES.
- Shipment execution and proof of delivery remain in TMS or carrier systems.
- Quality inspection and disposition remain in QMS.
- Identity credentials remain in the `keyverse` identity product or a verified customer identity provider.
- Internal people, organization, position, assignment, and responsibility remain in `Orgmetra` where adopted.
- Formal enterprise risk acceptance and control evidence remain in `governance-risk-compliance`.
- Application, capability, technology, initiative, and transformation truth remains in `enterprise-architecture-core`.
- Reusable ontology publication remains in `ConceptWeave`; catalog, glossary, lineage, and governed consumption remain in `semantic-data-portal`.
- CWL product usage, pricing, contract, and commercial entitlement remain in `metering-billing-platform`.
- Accounting journal, ledger, posting, and financial statements remain in the customer accounting authority or `accounting-information-platform` where adopted.

## Differentiation

The product is not differentiated by another generic control-tower dashboard. Its defensible properties are:

- **Evidence before score:** every relation and event is admitted with provenance and truth status.
- **Time-safe analysis:** the analysis can reproduce what was knowable at a specified knowledge cutoff.
- **Federated authority:** source systems retain their domain truth; the control plane owns cross-system decision context.
- **Validated mathematics:** production quantitative logic is Rust-first, reproducible, uncertainty-aware, and tested against known truth where possible.
- **Action closure:** exposure analysis ends in an approved action, writeback receipt, and outcome review rather than a static alert.
- **Interoperability:** SCOR DS supplies process vocabulary, GS1 EPCIS supplies visibility-event interoperability, and UN/CEFACT supplies international trade semantics without becoming the internal aggregate model.
- **CWL composition:** identity, orchestration, ontology, temporal analysis, document evidence, security, and architecture context are consumed through released owner contracts rather than copied.

## Explicit non-goals

- Replacing ERP, MES, WMS, TMS, PLM, QMS, SRM, or accounting systems
- Treating graph reachability as probability, severity, shortage, or causality
- Letting an LLM approve a decision, invent a constraint, or write directly to an operating system
- Creating a universal risk score from arbitrary heuristic weights
- Copying another CWL product's database, domain truth, source code, or mutable branch head
- Claiming certification, conformity, production readiness, or customer value without the required evidence

## Success measures

Product value is measured through decision and outcome evidence rather than feature count:

- time from trusted disruption observation to validated exposure;
- time from exposure to approved feasible response;
- commitment value protected or loss avoided, with attribution limits stated;
- false positive and false negative rates for impact and shortage predictions;
- scenario feasibility and realized-versus-predicted outcome error;
- writeback success, duplicate-action rate, and recovery success;
- provenance completeness and reproducibility;
- planner adoption, repeated use, and decision-cycle completion;
- integration deployment time and source-data reconciliation quality.

## Product maturity statement

The repository currently contains an early, pre-release Rust domain foundation and planning baseline. Durable storage, authenticated service interfaces, real source adapters, quantitative recovery models, user interfaces, operability evidence, and signed releases remain implementation work. No document in this repository converts a planned capability into a shipped claim.


Commercial packaging, pilot progression, procurement evidence, and sales-claim admission are defined in `docs/COMMERCIALIZATION.md`. Deployment and shared-responsibility variants are defined in `docs/DEPLOYMENT_PROFILES.md`.
