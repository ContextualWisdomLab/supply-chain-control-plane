# Supply Chain Control Plane

Supply Chain Control Plane is the evidence-first disruption-impact bounded context for ContextualWisdomLab supply-network operations. It turns admitted supplier, facility, item, inventory, shipment, order, dependency, and disruption evidence into deterministic downstream potential-impact paths without presenting reachability as probability, severity, or optimization advice.

## Start here

- [README](../README.md) — product responsibility, current capabilities, maturity, validation, and contribution guidance.
- [Product requirements](PRD.md) — buyer-facing disruption-impact requirements and acceptance boundaries.
- [Technical requirements](TRD.md) — implementation, evidence, determinism, security, and quality contracts.
- [Architecture](../ARCHITECTURE.md) — bounded contexts, authority boundaries, dependency direction, and future integration seams.
- [ADR 0001](adr/0001-evidence-first-disruption-impact.md) — the evidence-first disruption-impact decision.
- [Research and standards traceability](doctoring/research-traceability.md) — sources and claim/evidence traceability.
- [Product and technical gap baseline](product-technical-gap-baseline.md) — commercialization gaps and current evidence status.
- [Security](../SECURITY.md) — security and disclosure boundary.
- [Changelog](../CHANGELOG.md) — repository change history.
- [Repository releases](https://github.com/ContextualWisdomLab/supply-chain-control-plane/releases) — published releases when they exist.
- [Ask DeepWiki](https://deepwiki.com/ContextualWisdomLab/supply-chain-control-plane) — repository-oriented Q&A and navigation.

## Product boundary

The current core owns evidence-backed supply-network admission and deterministic downstream potential-impact analysis. ERP, WMS, TMS, EPCIS, procurement, inventory, order, and other operational systems remain authoritative for their source facts. Future adapters must cross explicit versioned anti-corruption boundaries rather than read another product's application tables.

A reported potential impact means only that a valid admitted dependency path exists from an observed disruption to a downstream node. Probability, severity, lead-time, recovery-time, and optimization claims require separately validated models and evidence.

## Current architecture

The implemented `SupplyGraph` aggregate keeps three responsibilities explicit: the Disruption Impact core, a supporting Network Registry for supply nodes and dependency facts, and a supporting Evidence Registry for source references. Results preserve originating event evidence, deterministic shortest paths, and evidence aligned to every dependency edge.

Persistence, authenticated service transport, source-system adapters, tenant isolation, operational deployment, customer workflow UI, validated quantitative models, and signed release/provenance evidence remain explicit follow-up gaps.

## Onboarding

Start with the README and PRD, then read the architecture and ADR before changing domain behavior. Contributors should preserve deterministic ordering, replay-safe semantics, evidence traceability, and the distinction between graph reachability and quantitative risk claims. Keep implementation, tests, product/technical documentation, research traceability, security claims, and the gap baseline aligned on the same revision.

## Publication status

This file is the reviewed source for the repository documentation landing page. GitHub Pages is not considered published until the source reaches the protected default branch, repository settings select the intended deployment mode, deployment succeeds, and the live HTTPS content is independently verified.
