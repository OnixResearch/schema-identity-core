## Phase 1: Baseline and extraction

- [x] [serial] r[onix.schema_identity.source_inventory] Record Aspen source modules, canonical projections, fixtures, and product dependencies.
- [x] [serial] r[onix.schema_identity.compatibility_fixtures] Freeze Aspen positive and negative identity vectors before extraction.
- [x] [serial] r[onix.schema_identity.source_revision] Create and publish the product-neutral crate at one immutable reviewed revision.

## Phase 2: Identity model

- [x] [serial] r[onix.schema_identity.lineage] Implement typed lineage identities with caller-owned nominal scope.
- [x] [serial] r[onix.schema_identity.exact_schema] Implement exact schema identities over complete canonical descriptors.
- [x] [serial] r[onix.schema_identity.member_ids] Implement stable field, variant, and operation-member identities with duplicate rejection.
- [x] [parallel] r[onix.schema_identity.member_ids.order] Add order-invariance and ID-reuse negative tests.
- [x] [serial] r[onix.schema_identity.modes] Implement structural, nominal, and branded-structural modes without implicit fallback.

## Phase 3: Compatibility

- [x] [serial] r[onix.schema_identity.directional_alias] Implement exact directional alias admission.
- [x] [serial] r[onix.schema_identity.migration_reference] Implement bound migration references without migration execution.
- [x] [serial] r[onix.schema_identity.compatibility_decision] Implement closed compatibility decisions and stable reason classes.
- [x] [serial] r[onix.schema_identity.functional_core] Keep registry, policy loading, and persistence outside the pure core.

## Phase 4: Pilots and evidence

- [x] [parallel] r[onix.schema_identity.compatibility_fixtures.negative] Add malformed, collision, crossed-alias, wrong-direction, and stale-migration fixtures.
- [x] [serial] r[onix.schema_identity.consumer_pilots] Cut Aspen over to the extraction and add one independent Kamacite or Valence pilot.
- [x] [parallel] r[onix.schema_identity.content_identity_boundary] Add a repository check that rejects schema policy inside `content-identity-core` adapters.
- [x] [serial] r[onix.schema_identity.source_revision] Run repository checks, consumer pilots, Cairn validation, and all change gates.
