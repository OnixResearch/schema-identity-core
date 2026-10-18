# Schema Identity Core Specification

## Purpose

Defines the `schema-identity-core` capability.

## Requirements

### Requirement: Stable lineage identity
r[onix.schema_identity.lineage] The core MUST represent each durable type evolution line with a typed `TypeLineageId` that is separate from schema shape and display names.

#### Scenario: Rename preserves lineage
r[onix.schema_identity.lineage.rename]
- GIVEN two exact schemas with the same admitted nominal lineage key and different display names
- WHEN the core derives their lineage identities
- THEN the lineage identities are equal

#### Scenario: Different owners do not share lineage
r[onix.schema_identity.lineage.owner]
- GIVEN equal local lineage labels under different nominal owner domains
- WHEN the core derives their lineage identities
- THEN the lineage identities are different

### Requirement: Exact schema identity
r[onix.schema_identity.exact_schema] The core MUST derive `SchemaId` from a versioned canonical projection of the complete exact schema descriptor.

#### Scenario: Equal exact descriptors agree
r[onix.schema_identity.exact_schema.equal]
- GIVEN two complete descriptors with equal normalized identity-bearing facts
- WHEN the core derives their exact schema identities
- THEN the identities are equal

#### Scenario: One changed fact changes identity
r[onix.schema_identity.exact_schema.changed]
- GIVEN two descriptors that differ in one field type, variant payload, openness rule, member identity, or nested schema identity
- WHEN the core derives their exact schema identities
- THEN the identities are different

### Requirement: Stable member identities
r[onix.schema_identity.member_ids] Records, variants, and operation sets MUST use typed stable member identities that are separate from names and ordinal positions.

#### Scenario: Source reordering preserves identity
r[onix.schema_identity.member_ids.order]
- GIVEN equal members with equal IDs in different declaration orders
- WHEN the core normalizes the descriptor
- THEN the exact schema identities are equal

#### Scenario: Display rename preserves member identity
r[onix.schema_identity.member_ids.rename]
- GIVEN one member ID with two display names under a policy that excludes names from exact identity
- WHEN the core derives the member identity
- THEN the member identity is unchanged

#### Scenario: Duplicate ID fails closed
r[onix.schema_identity.member_ids.duplicate]
- GIVEN two members in one scope with the same stable ID
- WHEN the core admits the descriptor
- THEN descriptor admission fails without a schema identity

### Requirement: Explicit identity modes
r[onix.schema_identity.modes] Every schema descriptor MUST select one supported structural, nominal, or branded-structural identity mode.

#### Scenario: Structural equality is admitted
r[onix.schema_identity.modes.structural]
- GIVEN two structural descriptors with equal normalized shape
- WHEN compatibility is classified
- THEN the decision is structural match

#### Scenario: Nominal mismatch does not fall back
r[onix.schema_identity.modes.nominal]
- GIVEN two nominal descriptors with equal shape and different lineage identities
- WHEN compatibility is classified
- THEN the decision is not structural match

#### Scenario: Brand mismatch fails
r[onix.schema_identity.modes.brand]
- GIVEN two branded-structural descriptors with equal shape and different brand identities
- WHEN compatibility is classified
- THEN the decision requires another explicit compatibility mechanism

### Requirement: Directional aliases
r[onix.schema_identity.directional_alias] An alias MUST bind exact source schema, target schema, direction, scope, and policy identity.

#### Scenario: Exact forward alias passes
r[onix.schema_identity.directional_alias.forward]
- GIVEN an admitted alias from the actual schema to the expected schema for the requested scope
- WHEN compatibility is classified
- THEN the decision is admitted alias

#### Scenario: Reverse alias fails
r[onix.schema_identity.directional_alias.reverse]
- GIVEN only an alias from schema A to schema B
- WHEN compatibility requests schema B to schema A
- THEN the alias does not satisfy the request

#### Scenario: Crossed scope fails
r[onix.schema_identity.directional_alias.scope]
- GIVEN an alias admitted for storage only
- WHEN compatibility requests effect-handler substitution
- THEN the alias does not satisfy the request

### Requirement: Explicit migration references
r[onix.schema_identity.migration_reference] A migration reference MUST bind source schema, target schema, direction, migration artifact, and policy profile.

#### Scenario: Exact migration is available
r[onix.schema_identity.migration_reference.exact]
- GIVEN a migration reference that matches the requested source, target, direction, and profile
- WHEN compatibility is classified
- THEN the decision records migration available

#### Scenario: Crossed migration fails
r[onix.schema_identity.migration_reference.crossed]
- GIVEN a migration reference for different source or target schema identities
- WHEN compatibility is classified
- THEN the decision remains migration required or denied

#### Scenario: Reference does not execute migration
r[onix.schema_identity.migration_reference.no_execution]
- GIVEN an exact migration reference
- WHEN the pure core classifies compatibility
- THEN it performs no migration, storage, process, file, or network effect

### Requirement: Closed compatibility decisions
r[onix.schema_identity.compatibility_decision] The core MUST return a closed compatibility decision with exact input identities and a stable reason class.

#### Scenario: Policy denial wins
r[onix.schema_identity.compatibility_decision.policy]
- GIVEN otherwise compatible schemas and an explicit consumer policy denial
- WHEN compatibility is classified
- THEN the decision is denied

#### Scenario: Names do not imply compatibility
r[onix.schema_identity.compatibility_decision.names]
- GIVEN two schemas with equal display names and incompatible exact identities
- WHEN compatibility is classified
- THEN equal names do not change the denial or migration-required decision

### Requirement: Pure schema identity core
r[onix.schema_identity.functional_core] Identity and compatibility decisions MUST use explicit in-memory values without I/O, clocks, environment access, or mutable global state.

#### Scenario: Equal inputs give equal decisions
r[onix.schema_identity.functional_core.equal]
- GIVEN equal descriptors, aliases, migration references, and policy observations
- WHEN two consumers call the core
- THEN they receive equal identities and compatibility decisions

### Requirement: Content identity boundary
r[onix.schema_identity.content_identity_boundary] `content-identity-core` MUST own framing only and MUST NOT own schema modes, member semantics, aliases, migrations, or compatibility policy.

#### Scenario: Schema policy remains local
r[onix.schema_identity.content_identity_boundary.local]
- GIVEN one schema identity projection
- WHEN the core frames it with `content-identity-core`
- THEN all schema facts were selected by `schema-identity-core` before framing

### Requirement: Portable compatibility fixtures
r[onix.schema_identity.compatibility_fixtures] The repository MUST publish exact positive and negative vectors for identities, aliases, migrations, and compatibility decisions.

#### Scenario: Positive vectors agree
r[onix.schema_identity.compatibility_fixtures.positive]
- GIVEN a published admitted compatibility vector
- WHEN an independent consumer runs the core
- THEN every identity and decision equals the vector

#### Scenario: Negative vectors agree
r[onix.schema_identity.compatibility_fixtures.negative]
- GIVEN a duplicate, crossed, wrong-direction, or policy-denied vector
- WHEN an independent consumer runs the core
- THEN the stable rejection or decision equals the vector

### Requirement: Independent consumer pilots
r[onix.schema_identity.consumer_pilots] Aspen and at least one independent consumer MUST pass compatibility pilots before stable extraction publication.

#### Scenario: Two consumers agree
r[onix.schema_identity.consumer_pilots.agree]
- GIVEN Aspen and one Kamacite or Valence pilot
- WHEN both run the shared fixture cohort
- THEN both produce equal identity and compatibility results

#### Scenario: Divergence blocks stability
r[onix.schema_identity.consumer_pilots.divergent]
- GIVEN one pilot that changes an identity or compatibility class
- WHEN release readiness is evaluated
- THEN the extraction remains non-stable

### Requirement: Immutable extraction source
r[onix.schema_identity.source_revision] Cargo and Nix consumers MUST select the same immutable reviewed `schema-identity-core` revision.

#### Scenario: Source revisions agree
r[onix.schema_identity.source_revision.agree]
- GIVEN Cargo and Nix dependency declarations
- WHEN source validation runs
- THEN both declarations use the same full revision

### Requirement: Reviewed extraction inventory
r[onix.schema_identity.source_inventory] The extraction MUST record exact Aspen sources, copied fixtures, licenses, semantic changes, and retained product authority.

#### Scenario: Inventory maps every imported mechanism
r[onix.schema_identity.source_inventory.complete]
- GIVEN the extracted source tree
- WHEN provenance validation runs
- THEN every imported mechanism maps to one reviewed Aspen source artifact
