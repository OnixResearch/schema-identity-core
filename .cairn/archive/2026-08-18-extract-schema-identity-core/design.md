## Context

Aspen supports structural, unique, branded-structural, alias, migration, and policy-denial decisions. Its current normalized record shapes use product Preserves values and field names.

The extraction must add stable member identities and separate lineage identity from exact shape identity. It must not turn `content-identity-core` into a schema policy engine.

## Decisions

### Decision: Separate lineage and exact identity

**Choice:** `TypeLineageId` names one durable evolution line. `SchemaId` identifies one exact canonical schema version within or outside a lineage.

**Rationale:** Compatible evolution must not make two different runtime layouts share one exact identity.

### Decision: Give every member a stable identity

**Choice:** Records, variants, and operation sets use stable member IDs that are separate from display names and ordinal positions.

**Rationale:** Renames and source reordering must not silently change meaning, while ID reuse must fail closed.

### Decision: Keep identity modes explicit

**Choice:** The core supports `Structural`, `Nominal`, and `BrandedStructural` modes. Each mode has a distinct canonical projection and decision rule.

**Rationale:** Implicit structural fallback makes unique types compatible without owner consent.

### Decision: Keep aliases directional

**Choice:** An alias binds exact source, target, scope, policy identity, and optional expiry or generation observation. Reverse use requires another alias.

**Rationale:** Compatibility is not symmetric by default.

### Decision: Migration references are evidence, not execution

**Choice:** A migration reference binds source schema, target schema, direction, migration artifact, and policy profile. The core only classifies its presence and exact binding.

**Rationale:** Migration execution, purity, atomicity, and success remain consumer responsibilities.

### Decision: Compatibility returns a closed decision

**Choice:** The core returns exact, structural, branded, admitted-alias, migration-available, migration-required, or denied decisions with stable reason classes.

**Rationale:** Callers must not infer compatibility from optional fields or equal names.

### Decision: Content identity owns framing only

**Choice:** `content-identity-core` frames canonical schema projections. `schema-identity-core` owns which schema facts enter each projection.

**Rationale:** Shared hashing mechanics must not absorb schema meaning.

### Decision: Pure core and injected registries

**Choice:** The core accepts already-loaded descriptors, aliases, migrations, and policy observations. Shells own files, registries, clocks, and network access.

**Rationale:** Compatibility decisions need direct deterministic tests.

## Risks / Trade-offs

- Introducing stable member IDs requires explicit migration from name or ordinal identities.
- A canonical projection defect can create a long-lived identity error.
- Structural compatibility proves equal normalized shape only, not equal business meaning.
- A compatibility decision does not authorize storage, execution, or effect handling.
