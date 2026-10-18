## Why

Aspen contains useful schema identity and compatibility mechanics, but they are embedded in Molten modules and product-specific Preserves records.

OnixResearch needs a product-neutral schema identity core for storage, protocols, effects, configuration, and runtime boundaries. The core must separate stable lineage from exact schema shape and stable member identity.

## What Changes

- Extract a `schema-identity-core` crate from the reviewed Aspen schema identity implementation.
- Define stable lineage, exact schema, field, variant, and operation-member identity types.
- Support explicit structural, nominal, and branded-structural identity modes.
- Define directional aliases, migration references, and closed compatibility decisions.
- Use `content-identity-core` only for deterministic domain-separated framing.
- Keep schema policy, registry access, persistence, and migration execution in consumers.
- Publish positive, negative, order-invariance, and migration fixtures.
- Prove product neutrality with Aspen and one independent consumer pilot.

## Impact

- **Source**: Aspen schema identity and compatibility modules
- **New component**: product-neutral `schema-identity-core` with immutable Git publication
- **Consumers**: Aspen, Kamacite, Valence, configuration systems, and later Ambient adapters
- **Compatibility**: names remain metadata unless a consumer explicitly includes them in a nominal key
