# Architecture portfolio

## Success contract

Goal: extract Aspen schema identity mechanics into a reusable, bounded, pure Rust core.

Completion evidence:

- typed lineage, exact schema, stable member, alias, migration, and decision APIs;
- deterministic positive and negative vectors;
- Aspen and one independent consumer pilot at the same immutable source object;
- strict Octet, Clippy, tests, `no_std` Wasm, Nix, source, and Cairn gates;
- synchronized accepted requirements and an archived change.

False completion includes copied Molten Preserves DTOs, name or ordinal identity, implicit structural fallback, migration execution, sibling paths, floating revisions, one consumer, warning-only Octet output, or unchecked task boxes.

Audit risks include canonical projection collisions, node-order dependence, duplicate member IDs, graph cycles, stale aliases, crossed migration bindings, policy bypass, hidden I/O, and claims that digest or compatibility grants authority.

Budget: three mechanism families, two correlated review rounds, 24 targeted source reads, one adversarial audit, and deterministic repository validators. Allowed outcomes are validated, blocked, exhausted, or user-decision-required.

## Approach registry

| Family | Mechanism | State | Evidence or blocker |
|---|---|---|---|
| typed-extraction | Product-neutral graph descriptors select schema facts; pinned `content-identity-core` frames them | active | Aspen source and accepted requirements map to a pure bounded core |
| preserves-copy | Copy Aspen Preserves DTOs and normalizer | falsified | Retains Molten records, registry helpers, names, and product receipt policy; lacks stable member IDs |
| interchange-wrapper | Reuse Kamacite source/value identity as schema identity | falsified | Kamacite owns interchange identity, not lineage, schema modes, aliases, or migration classification |

The selected family reuses framing only. Schema semantics remain in this repository. Consumer registries, policy loading, migration execution, storage, and authorization remain outside it.

## Adversarial audit

The selected family was tested against node renumbering, member reordering, display renames, owner-domain crossing, changed field types, duplicate member IDs, missing nodes, unreachable nodes, graph cycles, nominal fallback, brand mismatch, reverse aliases, crossed scopes, inactive aliases, crossed migration policy, zero limits, exceeded limits, and policy denial over exact identity.

The first digest pass exposed an order-sensitive dependency bug: parent nodes tried to hash before child digests existed. The implementation now checks child readiness and rejects unresolved cycles. The deterministic positive and negative cohort passes after this correction.
