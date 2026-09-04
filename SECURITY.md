# Security

## Current posture

The repository currently contains an in-memory Rust domain core only. It does not accept network traffic, credentials, customer records, or production data.

Evidence locators remain available through the explicit `source_locator()` accessor because drill-through is part of the product job, but they are treated as potentially sensitive operational data. `EvidenceReference` therefore uses a custom `Debug` implementation that retains the semantic source-record key while replacing the locator with `<redacted>`. This prevents accidental credential, signed-URL, internal-route, or customer-data disclosure through routine debug logging without destructively masking the value required by authorized product flows. Regression coverage asserts that a token-bearing locator never appears in debug output.

## Required before production data

- authenticated and authorized tenant/workspace boundary;
- encryption in transit and at rest with explicit key ownership;
- append-only audit evidence for event admission and impact publication;
- SSRF-safe external source adapters and bounded parsing;
- secret-free logs and structured PII handling without destructive masking where masking would break the operational task;
- logging/telemetry review proving that explicit evidence-locator access is never emitted by default and is authorized where drill-through is required;
- threat model, dependency review, SBOM/provenance, incident runbook, backup/restore evidence, and SOC 2/CSAP control mapping.

Security findings should be reported through the organization security process rather than committed with live secrets or real customer/person data.
