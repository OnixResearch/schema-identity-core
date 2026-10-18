# Source authority

The repository owner authorized this extraction under the MIT license.

That extraction authorization remains a historical fact. Current and new owned material is offered under AGPL-3.0-or-later; see `LICENSE` and `NOTICE`. `LICENSE-MIT` preserves prior grants, which remain effective for the content they cover, including byte-identical retained content; it does not offer MIT terms for new contributions. Third-party material and exact pinned dependencies retain their own terms. The publication identities below are historical retained objects, not a newly published AGPL revision.

The reviewed Aspen source object is `f722df98343336d95044309639e83283214239eb` on its canonical `molten` publication line.

Imported mechanism inventory:

- `src/schema/identity.rs` — module composition;
- `src/schema/parts/identity/p000/body.rs` — modes, identity inputs, shape normalization, aliases, and compatibility records;
- `src/schema/parts/identity/p001/body.rs` — compatibility classification and bounded registry boundary;
- `src/schema/parts/identity/p002/body.rs` — parsing and validation helpers;
- `src/schema/parts/identity/p003/body.rs` — positive, negative, direction, mode, and determinism tests;
- `cairn/specs/local-schema-identity/spec.md` — accepted product requirements;
- `cairn/archive/2026-06-12-local-schema-identity/` — implementation design and lifecycle evidence.

The extraction replaces Molten Preserves values and product artifact refs with typed graph descriptors, stable member IDs, caller-owned nominal scope, and closed product-neutral decisions. It retains no Aspen registry, storage, CLI, receipt, policy, capability, or migration-execution authority.

Identity framing uses `content-identity-core` from canonical repository `rad:zA14jhTdwST4PxPBdsryc4zS7Kpq` at reviewed object `7f55597b5dc879b7601856e8d7fd0dbacaa2a498`.

The canonical repository for this component is `rad:z6gGpUJtzdVBCCtZTzh4cV1skv4H`. Its public acquisition adapter is `https://seed.radicle.garden/z6gGpUJtzdVBCCtZTzh4cV1skv4H.git`.

The reviewed consumer source object is `2562c8aa38a034061f9af9f3e17280494a5b8de2`.

Cargo consumers pin that object:

```toml
schema-identity-core = { git = "https://seed.radicle.garden/z6gGpUJtzdVBCCtZTzh4cV1skv4H.git", rev = "2562c8aa38a034061f9af9f3e17280494a5b8de2" }
```

Nix consumers pin the same object:

```nix
schema-identity.url = "git+https://seed.radicle.garden/z6gGpUJtzdVBCCtZTzh4cV1skv4H.git?rev=2562c8aa38a034061f9af9f3e17280494a5b8de2";
```

`profiles/source-boundary.ncl` rejects floating branches, shortened or changed revisions, wrong RIDs, GitHub substitutions, sibling paths, and Cargo/Nix drift.

Publication proves object availability from checked adapters only. It does not prove source correctness, semantic compatibility, consumer authority, migration success, remote honesty, or release eligibility.
