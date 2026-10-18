# Aspen extraction inventory

Reviewed Aspen source object: `f722df98343336d95044309639e83283214239eb`.

| Aspen artifact | Imported mechanism | Product dependency removed or retained in Aspen |
|---|---|---|
| `src/schema/identity.rs` | Module ownership boundary | Split-part build layout stays in Aspen |
| `src/schema/parts/identity/p000/body.rs` | Explicit modes, normalized shape identity, alias and compatibility inputs | Preserves values, Molten errors, schema labels, policy refs, evidence refs, and receipt DTOs stay in Aspen |
| `src/schema/parts/identity/p001/body.rs` | Directional alias, structural and branded matching, migration availability, denial precedence | Artifact registry search and storage/protocol/effect helper policy stay in Aspen |
| `src/schema/parts/identity/p002/body.rs` | Fail-closed parsing and duplicate checks | Preserves record parsing stays in Aspen |
| `src/schema/parts/identity/p003/body.rs` | Order, unique-mode, brand, alias, migration, denial, and determinism vectors | Molten test support and Hegel harness stay in Aspen |
| `cairn/specs/local-schema-identity/spec.md` | Accepted behavior baseline | Product CLI, ledger, storage, protocol, effect, and policy-contract requirements stay in Aspen |

## Deliberate semantic changes

- `unique` becomes explicit `Nominal` mode with a typed `TypeLineageId`.
- Lineage identity and exact schema identity are separate.
- Records, variants, and operation sets require stable member IDs.
- A bounded acyclic graph replaces Preserves shape values.
- Local node numbers and declaration order do not enter identity.
- Aliases bind policy and caller-owned scope exactly.
- Migration references bind artifact and policy but cannot execute.
- Compatibility returns closed typed classes and reasons.
- Every resource limit is explicit.

## Retained consumer authority

Aspen retains product schema decoding, artifact registries, typed storage, migration recipes and execution, policy and evidence refs, capabilities, receipts, CLI behavior, and release claims.

The extraction uses the owner authorization for OnixResearch-controlled source. Third-party Preserves code was not copied. The pinned `content-identity-core` dependency retains its published license and source identity.
