# ADR 0003 — Bitemporal Evidence, Multi-Clock Events, and Explicit Truth Status

- Status: Proposed
- Date: 2026-09-02
- Decision owners: Evidence Registry, Event Registry, and Impact Assessment owners

## Context

A supply-chain fact can describe an earlier business event, be asserted later, arrive even later, and be corrected afterward. Reconstructing decisions from a single `created_at` timestamp creates future-information leakage and makes audit results irreproducible.

The product also combines facts from authoritative systems, deterministic observations, statistical or LLM inference, and human proposals. Treating all edges and attributes as equivalent makes an inferred relationship indistinguishable from an approved operational fact.

## Decision

Material records carry the relevant clocks:

```text
valid_from / valid_to
occurred_at / occurred_until
asserted_at
available_at
recorded_at
superseded_at
knowledge_cutoff
```

An analysis may consume a record only when the record was available at its declared cutoff:

```text
available_at <= knowledge_cutoff
```

Every material fact or relation carries exactly one truth status:

```text
authoritative
observed
inferred
proposed
superseded
rejected
```

Status transitions are explicit and evidence-backed. Inference is never silently promoted to authoritative truth. Corrections and supersession append new records or close validity intervals; they do not rewrite historical decisions.

## Alternatives considered

### Keep only event time

Rejected. A retrospective document could leak future knowledge into an earlier analysis.

### Keep only system insertion time

Rejected. The system could not answer when the business fact was valid or occurred.

### Store confidence without truth status

Rejected. Confidence is not authority. A high-confidence inference remains an inference.

### Put all temporal attributes into JSON

Rejected. Core validity, availability, and system-time invariants require normalized constraints and indexes.

## Consequences

- Historical replay can reconstruct what was knowable at a decision cutoff.
- Persistence and APIs must distinguish absent, unknown, invalid, late, and superseded facts.
- Model validation must use rolling-origin or equivalent leakage-safe splits.
- UI labels must distinguish fact, observation, inference, proposal, decision, and outcome without relying on color alone.
- Storage volume increases because corrections preserve history.
