# Security

## Current posture

The repository currently contains an in-memory Rust domain core only. It does not accept network traffic, credentials, customer records, or production data.

## Required before production data

- authenticated and authorized tenant/workspace boundary;
- encryption in transit and at rest with explicit key ownership;
- append-only audit evidence for event admission and impact publication;
- SSRF-safe external source adapters and bounded parsing;
- secret-free logs and structured PII handling without destructive masking where masking would break the operational task;
- threat model, dependency review, SBOM/provenance, incident runbook, backup/restore evidence, and SOC 2/CSAP control mapping.

Security findings should be reported through the organization security process rather than committed with live secrets or real customer/person data.
