# ADR 0005 — Rust-First Quantitative Decision Core with CPU Reference and Verified Acceleration

- Status: Proposed
- Date: 2026-09-02
- Decision owners: Impact Assessment, Recovery Scenario, and Model Governance owners

## Context

Shortage estimation, inventory allocation, network flow, simulation, and scenario comparison directly influence operational decisions. These computations require predictable performance, numerical traceability, explicit termination states, and reproducibility. Convenience implementations, arbitrary weights, or LLM-generated calculations would not satisfy that boundary.

## Decision

Production quantitative and scientific arithmetic is implemented in Rust.

Each activated model or solver has:

- a declared estimand or objective;
- input, unit, constraint, missingness, and temporal contracts;
- a CPU `f64` reference implementation;
- deterministic seed and input-snapshot manifests where randomness exists;
- out-of-sample or known-truth validation;
- uncertainty, feasibility, and abstention states;
- versioned model artifacts and activation approval; and
- CPU/multithreading/GPU parity evidence for accelerated backends.

A solver that stops early reports its termination reason, best-known result, and bound or gap when available. An infeasible or unidentified problem does not return a fabricated recommendation.

Python may be used only for a validated research dependency that has no adequate Rust alternative. Its boundary, removal condition, and absence from production hot paths must be recorded in an ADR.

## Alternatives considered

### Implement analytics in Python by default

Rejected. It would conflict with the product's performance, reproducibility, and runtime boundary, and would make GIL or native-extension behavior part of critical operations without necessity.

### Use an LLM to estimate impacts and scenarios

Rejected. LLMs may extract or explain evidence, but they are not the numerical authority.

### Use a single heuristic risk score

Rejected. Different buyer decisions require explicit quantities, objectives, constraints, and uncertainty rather than an opaque weighted sum.

### Use GPU output without a CPU oracle

Rejected. Acceleration must not redefine mathematical behavior.

## Consequences

- Model development has a higher initial evidence burden.
- Scientific validation is a release gate, not an optional report.
- GPU support can be added without sacrificing a stable numerical reference.
- Model artifacts and decisions remain reproducible after code and price changes.
