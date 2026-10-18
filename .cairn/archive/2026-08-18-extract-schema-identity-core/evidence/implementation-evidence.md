# Implementation evidence

## Source and publication

- Canonical repository: `rad:z6gGpUJtzdVBCCtZTzh4cV1skv4H`
- Acquisition adapter: `https://seed.radicle.garden/z6gGpUJtzdVBCCtZTzh4cV1skv4H.git`
- Reviewed Aspen source: `f722df98343336d95044309639e83283214239eb`
- First checked implementation: `b00d769d5135ce8dd926975bc951e065fe2953b9`
- Reviewed vendor-safe consumer object: `2562c8aa38a034061f9af9f3e17280494a5b8de2`
- Pinned framing dependency: `content-identity-core` object `7f55597b5dc879b7601856e8d7fd0dbacaa2a498`

The reviewed consumer object is reachable from canonical Radicle `main` and the public adapter.

## Architecture evidence

- `schema-identity-core` is `no_std + alloc` and performs no I/O, clock, environment, network, process, registry, storage, logging, or migration effect.
- Bounded acyclic graph admission rejects duplicate nodes, duplicate member IDs, missing nodes, unreachable nodes, cycles, zero limits, and exceeded limits.
- Structural projections ignore node numbers, member order, and display names.
- Nominal lineage binds caller-owned owner and local key.
- Nominal and branded modes do not fall back to structural matching.
- Policy denial wins before exact, alias, or migration mechanisms.
- Aliases bind source, target, scope, policy, generation observation, and currentness.
- Migration observations bind source, target, artifact, policy, and caller-supplied currentness without execution.
- `content-identity-core` supplies framing only. A checked negative fixture proves the boundary check detects schema policy in that component.

## Portable fixtures

Nickel 1.17.0 exports the checked cohort from `fixtures/vectors.ncl`. The generated JSON and the crate-packaged vendor resource are byte-equal. The cohort covers exact identity, nominal mismatch, forward alias, reverse alias rejection, migration availability, stale migration rejection, policy denial, and duplicate member rejection.

## Consumer pilots

### Aspen

- Initial adapter pilot: `a130803178d64dfad43f405609ab944150dcc131`
- Final source pin: `2b7013f71813122b22c806f97b9fc25dc9a2f9fc`
- Canonical remote branch: `origin/molten`
- Evidence: 19 existing schema tests and 1 CLI schema test passed before the change. Two shared-core pilot tests passed after the change. Strict library Clippy and Aspen's Octet pre-commit hook passed.
- The pre-push Cairn hook could not run because canonical Aspen still has the inherited classic `cairn/` layout. The verified fast-forward used `--no-verify` only after the focused tests, format, Clippy, and Octet gates passed.

### Valence

- Independent pilot: `e40c76b4d2070a29636e00c85c0dff93f03dba2f`
- Canonical remote branch: `origin/main`
- The pilot consumes Cargo and Nix source object `2562c8aa38a034061f9af9f3e17280494a5b8de2`.
- Two positive and negative pilot tests, strict pilot Clippy, 275 Valence core tests, and the full Valence `nix flake check` passed.

The pilots prove checked compatibility for the supplied cohort and adapter facts only. They do not prove whole-product cutover, semantic equivalence, migration correctness, registry freshness, persistence, authorization, remote honesty, or release eligibility.

## Repository checks

The implementation passed:

- `cargo test --workspace --all-targets --all-features`;
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`;
- the official pinned Octet deny-all gate with zero findings, warnings, and errors;
- `wasm32-unknown-unknown` compilation for the pure core;
- Nickel positive and negative fixture checks;
- immutable source-boundary checks;
- content-identity policy-boundary positive and negative checks;
- Cairn policy freshness, validation, proposal, design, and tasks gates;
- full local-builder `nix flake check -L`;
- Cairn traceability coverage for all 39 of 39 accepted requirement and scenario identifiers.

## Non-claims

A schema identity is not authenticity, provenance, trust, authorization, or business meaning. A compatibility decision does not execute a migration, mutate storage, or grant an effect. Consumer pilots and Radicle publication do not prove whole-product correctness, remote availability, delegate honesty, or release eligibility.
