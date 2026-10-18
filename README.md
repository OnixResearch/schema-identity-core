# schema-identity-core

`schema-identity-core` owns product-neutral schema lineage, exact schema identity, stable member identity, directional aliases, migration references, and closed compatibility decisions.

Consumers retain schema registries, descriptor decoding, migration execution, business meaning, authorization, persistence, clocks, currentness, and runtime policy.

## Architecture

The `schema-identity-core` crate is a pure `no_std + alloc` functional core. It admits bounded acyclic graph descriptors and uses pinned `content-identity-core` framing.

The `schema-identity-conformance` crate publishes deterministic positive and negative vectors for consumer pilots.

See:

- [`docs/architecture.md`](docs/architecture.md)
- [`docs/source-inventory.md`](docs/source-inventory.md)
- [`docs/conformance.md`](docs/conformance.md)
- [`SOURCE.md`](SOURCE.md)

## Verification

```sh
nix develop -c cargo test --workspace --all-targets --all-features
nix develop -c cargo clippy --workspace --all-targets --all-features -- -D warnings
nix develop -c pre-commit run octet-deny-all --all-files
nix flake check -L
```

Compatibility and digest equality do not grant authority or prove semantic equivalence, migration success, persistence, provenance, or release eligibility.

## License

Current and new owned core and conformance packages, code, contracts, schemas, fixtures, templates, documentation, and policy are licensed under **AGPL-3.0-or-later**. See `LICENSE` for the complete license and `NOTICE` for attribution and historical/third-party boundaries. Both Cargo members inherit this workspace license.

`LICENSE-MIT` preserves the earlier extraction grant; it does not offer MIT terms for new owned contributions. Prior grants remain effective for content they already cover, including byte-identical retained content. The source identities and immutable dependency objects in `SOURCE.md` retain their original terms. This source preparation does not publish a revision or change consumer pins.

## References

- [Aspen](https://github.com/OnixResearch/aspen) — reviewed schema identity and compatibility source.
- [content-identity-core](https://seed.radicle.garden/zA14jhTdwST4PxPBdsryc4zS7Kpq.git) — canonical BLAKE3 framing dependency.
- [Kamacite](https://github.com/OnixResearch/kamacite) — schema interchange reference.
- [Valence](https://github.com/OnixResearch/valence) — independent consumer-pilot candidate.
