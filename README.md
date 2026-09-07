# Supply Chain Control Plane

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/ContextualWisdomLab/supply-chain-control-plane)

**Evidence-first supply-chain decision and resilience control plane.**

Supply Chain Control Plane connects customer-owned ERP, PLM, MES, QMS, WMS, TMS, SRM, carrier, partner, and traceability facts so teams can move from an observed disruption to an auditable decision and verified action. It does not replace those transactional systems or present an inferred relationship as operational truth.

## Product promise

```text
observe a disruption
→ validate identity, time, authority, and evidence
→ trace deterministic downstream exposure
→ quantify commitment impact with validated models
→ compare feasible recovery scenarios
→ approve an action
→ execute through idempotent writeback
→ verify the outcome and learn
```

Every material result is designed to retain its exact input snapshot, business-valid and recorded times, truth status, source evidence, dependency or constraint path, model/solver version, uncertainty or feasibility state, approval, writeback receipt, and observed outcome.

## Current implementation

The current foundation is a Rust 1.98.0 domain library. It implements only the evidence-linked deterministic exposure part of the product:

- typed supply nodes;
- evidence-backed directed dependencies;
- evidence-backed disruption events;
- strict duplicate rejection;
- replay-safe node, dependency, and event upserts with `Inserted`, `Unchanged`, and immutable-conflict outcomes;
- cycle-safe downstream reachability;
- deterministic shortest dependency paths, including equal-length alternatives;
- originating event evidence and edge-aligned dependency evidence on every impact record; and
- default diagnostic redaction of evidence locators while retaining explicit authorized access.

This behavior is a **potential-impact** calculation. It is not probability, severity, shortage, lead time, causality, recovery time, or optimization advice.

The repository does not yet contain production persistence, authenticated network APIs, real source adapters, quantitative recovery models, buyer UI, operational deployment, signed release artifacts, or customer deployment evidence.

## Product boundary

Supply Chain Control Plane owns the cross-system decision context:

```text
Network Registry
Evidence Registry
Event Registry
Disruption Management
Impact Assessment
Recovery Scenario
Decision Control
Action Control
Outcome Learning
Model Governance
```

External authorities retain their own facts:

| Fact | Authority retained outside this repository |
| --- | --- |
| Legal supplier, customer, and material master | ERP or MDM |
| Bill of materials and routing | PLM, ERP, or MES |
| Purchase and sales orders | ERP |
| Physical inventory and warehouse tasks | WMS or ERP |
| Production execution | MES |
| Shipment execution and proof of delivery | TMS or carrier |
| Quality inspection and disposition | QMS |
| Credentials and identity federation | `keyverse` or a verified customer IdP |
| Internal people, organization, position, and assignment | `Orgmetra` where adopted |
| Formal risk acceptance and control evidence | `governance-risk-compliance` |
| Application, capability, technology, and transformation truth | `enterprise-architecture-core` |
| Ontology publication and semantic release | `ConceptWeave` |
| Data catalog, glossary, lineage, and governed consumption | `semantic-data-portal` |
| CWL product usage, pricing, and commercial entitlement | `metering-billing-platform` |
| Accounting journals and financial statements | Customer accounting authority or `accounting-information-platform` where adopted |

Integrations use released, versioned anti-corruption contracts. Cross-system SQL, copied domain truth, mutable sibling branch heads, and production filesystem references to other repositories are prohibited.

## First commercial wedge

The first complete buyer workflow is **Supply Disruption Impact and Recovery**:

1. Admit a trusted event from EPCIS, ERP, WMS, TMS, a partner, or authorized human evidence.
2. Resolve the affected object against a cross-system supply-network identity.
3. Reproduce what was knowable at a declared knowledge cutoff.
4. Trace materials, sites, capacity, inventory, shipments, orders, and commitments downstream.
5. Quantify shortages and service exposure through validated models.
6. Compare feasible responses under supply, capacity, lane, contract, quality, cost, and policy constraints.
7. Bind a human approval to an immutable scenario result.
8. Execute through an idempotent adapter and reconcile external state.
9. Compare predicted and realized outcomes.

## Standards and interoperability direction

- ASCM SCOR Digital Standard supplies a process crosswalk, not the internal aggregate hierarchy.
- GS1 EPCIS 2.0.1 and CBV 2.0.0 are the primary visibility-event interoperability targets.
- UN/CEFACT Buy-Ship-Pay models inform international trade mappings.
- W3C PROV-O informs provenance projections.
- OpenAPI 3.2.0, AsyncAPI 3.0.0, CloudEvents 1.0.2, RFC 9457, and RFC 9562 define planned external contracts.
- ISO 28000, ISO 20400, ISO 22301, NIST SP 800-161, and WCAG 2.2 inform security, procurement, resilience, software-supply-chain, and accessibility controls without creating certification claims.

See [`docs/doctoring/research-traceability.md`](docs/doctoring/research-traceability.md) for the current research and standards evidence boundary.

## Evaluate the current foundation

No package or release artifact is published yet. Evaluate the implemented Rust slice from an exact source checkout:

```bash
git clone https://github.com/ContextualWisdomLab/supply-chain-control-plane.git
cd supply-chain-control-plane
cargo test --all-targets --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo fmt --all -- --check
```

These commands validate the bounded domain library; they do not start a network service or prove the planned buyer workflow.

The current Product CI also enforces 100% line, function, and region coverage on stable Rust and a nonzero 100% branch denominator on the pinned nightly lane. Future owned production code retains 100% statement and branch coverage and complete public API documentation.

Quantitative production logic is Rust-first. CPU `f64` is the reference. Accelerated backends require parity evidence. A solver or model reports uncertainty, infeasibility, limit termination, and abstention rather than fabricating certainty.

## Documentation authority

| Question | Authoritative document |
| --- | --- |
| What product is this and why will a buyer adopt it? | [`docs/PRODUCT_VISION.md`](docs/PRODUCT_VISION.md) |
| Who uses it and what must it do? | [`docs/PRD.md`](docs/PRD.md) |
| How must it be built and verified? | [`docs/TRD.md`](docs/TRD.md) |
| Which bounded contexts and authorities exist? | [`ARCHITECTURE.md`](ARCHITECTURE.md) |
| What do the domain terms, aggregates, and state machines mean? | [`docs/DOMAIN_MODEL.md`](docs/DOMAIN_MODEL.md) |
| Which UML views make behavior and deployment reviewable? | [`docs/UML.md`](docs/UML.md) |
| How does it relate to CWL and customer systems? | [`docs/CONTEXT_MAP.md`](docs/CONTEXT_MAP.md) |
| What is the normalized persistence model? | [`docs/DATA_MODEL.md`](docs/DATA_MODEL.md) |
| What is the reviewable logical ERD? | [`docs/ERD.md`](docs/ERD.md) |
| What are the HTTP contracts? | [`docs/API_CONTRACT.md`](docs/API_CONTRACT.md) |
| What are the event contracts? | [`docs/EVENT_CONTRACT.md`](docs/EVENT_CONTRACT.md) |
| What are the security boundaries and threats? | [`docs/SECURITY_AND_THREAT_MODEL.md`](docs/SECURITY_AND_THREAT_MODEL.md) |
| How is the product operated and recovered? | [`docs/OPERABILITY.md`](docs/OPERABILITY.md) |
| Which managed, private, disconnected, and evaluation deployments are supported? | [`docs/DEPLOYMENT_PROFILES.md`](docs/DEPLOYMENT_PROFILES.md) |
| What evidence must tests provide? | [`docs/TEST_STRATEGY.md`](docs/TEST_STRATEGY.md) |
| What will the buyer workflows and UI states be? | [`docs/UX.md`](docs/UX.md) |
| Which research or standard supports which decision? | [`docs/doctoring/research-traceability.md`](docs/doctoring/research-traceability.md) |
| What is implemented and what remains open? | [`docs/product-technical-gap-baseline.md`](docs/product-technical-gap-baseline.md) |

The documentation landing page is [`docs/index.md`](docs/index.md).

## Current maturity

The repository is an early pre-release foundation with an expanded planning baseline. Planning completeness does not imply implementation completeness. Release, deployment, certification, customer benefit, performance, and accuracy claims require the evidence named in the roadmap, test strategy, gap baseline, and release plan.

## Contributing

Read [`AGENTS.md`](AGENTS.md), [`CLAUDE.md`](CLAUDE.md), the PRD/TRD, architecture, domain model, applicable ADRs, and the gap baseline before changing behavior. Use test-first development, preserve owner boundaries, update documentation and evidence on the same revision, and do not close a valid product delta without verified succession.

Use [GitHub Issues](https://github.com/ContextualWisdomLab/supply-chain-control-plane/issues) for reproducible product and documentation defects. Report vulnerabilities through [`SECURITY.md`](SECURITY.md), not a public issue.

## License

Supply Chain Control Plane is licensed under the [Apache License 2.0](LICENSE). That source grant does not grant rights to customer data, third-party standards, external SDKs, model artifacts, or future imported assets; each retains its own provenance and license boundary.
