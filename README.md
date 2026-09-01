# Supply Chain Control Plane

Supply Chain Control Plane turns scattered supplier, facility, item, inventory, shipment, order, and production evidence into a time-aware dependency view that helps an operations team answer: **what can this disruption affect next, which dependency path connects it, and what evidence supports the disruption and every dependency step?**

The first commercial vertical is intentionally narrow: register evidence-backed supply-network dependencies, record an evidence-backed disruption event, and return deterministic downstream reachability with the shortest admitted dependency path, originating event evidence, and evidence for each edge. It does **not** invent probability, severity, recovery time, or optimization weights. Those claims require separately validated models and data.

## Current slice

The Rust domain core provides:

- supply nodes with explicit kinds;
- evidence-backed directed dependency edges (`downstream` depends on `upstream`);
- immutable evidence references on observed events;
- duplicate/self/unknown-reference guards;
- deterministic, cycle-safe downstream impact reachability;
- path-level explanation with originating disruption evidence and edge-by-edge dependency evidence.

```bash
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```

This repository is pre-release. There is no production persistence, authenticated API, deployment image, customer-facing web workflow, approved publication license, or released artifact yet. See `docs/product-technical-gap-baseline.md` for the commercialization sequence.
