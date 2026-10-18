# Conformance vectors

`fixtures/vectors.ncl` is the typed source for the portable cohort. Nickel 1.17.0 exports `fixtures/generated/vectors.json`. The byte-equal `crates/schema-identity-conformance/src/vectors.json` copy makes the resource self-contained after Cargo vendoring. The conformance crate recomputes the same report from ordinary in-memory values.

The cohort includes:

- exact nominal identity;
- nominal mismatch without structural fallback;
- exact directional alias admission;
- reverse alias rejection;
- exact migration availability;
- policy denial precedence;
- duplicate stable member rejection.

Core tests also cover member order, local node-number changes, display renames, owner-separated lineages, changed types, missing nodes, unreachable nodes, cycles, stale aliases, crossed scopes, crossed policy profiles, zero limits, and exceeded limits.

A matching vector proves deterministic agreement for the supplied fixture only. It does not prove business equivalence, migration correctness, registry freshness, authorization, persistence, or release eligibility.
