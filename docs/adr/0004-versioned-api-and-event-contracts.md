# ADR 0004 — Versioned HTTP and Event Contracts with Exactly-Once Domain Effects

- Status: Proposed
- Date: 2026-09-02
- Decision owners: Application/API and Event Boundary owners

## Context

The control plane must connect many customer and CWL systems. Distributed delivery is at least once, external schemas evolve independently, and operational commands can be retried after timeouts. A generic REST surface plus unversioned event payloads would make reconciliation and safe consumer upgrades impossible.

## Decision

- HTTP contracts use OpenAPI 3.2.0.
- Message contracts use AsyncAPI 3.0.0 and CloudEvents 1.0.2.
- HTTP errors use RFC 9457 problem details.
- Locally generated public identifiers use RFC 9562 UUIDv7.
- State-changing HTTP commands require `Idempotency-Key`.
- Event delivery identity is `source + id`; domain effects also require a business command or source-event key.
- Command state and outbox event are written in the same transaction.
- Consumers retain inbox receipts with payload hash, schema version, processing result, and retry state.
- Breaking API, event, or schema changes receive a new version and an explicit coexistence/deprecation window.

## Alternatives considered

### Use transport-specific schemas only

Rejected. Kafka, NATS, HTTP, and file delivery must not redefine domain semantics.

### Assume exactly-once transport

Rejected. Broker guarantees do not remove application retries, redelivery, or producer ambiguity.

### Use provider object IDs as internal keys

Rejected. Provider migration and multi-provider operation would leak into the domain model.

### Silently accept unknown enum values

Rejected for commands. Unsupported semantics fail closed and return a stable problem type. Read projections may preserve unknown external values as unmapped evidence.

## Consequences

- Producers and consumers can verify compatibility independently.
- Duplicate delivery does not produce duplicate actions.
- Event and API schemas become release artifacts with their own lifecycle.
- Integration tests must cover duplicate, reorder, correction, late arrival, unsupported version, and partial failure.
