# Preimplementation licensing gate evidence

All commands ran from /home/brittonr/git/OnixResearch/schema-identity-core
using /tmp/kiln-agpl-cairn-bfa-target/debug/cairn, built by the parent offline
with --locked from unmodified exact Cairn
bfa88df5b2ac5f05e4a977bb8d4b8248d4511449. Parent-provided source NAR:
sha256-nQ/JSSXUQ5u1NvTQeIaBAO2koGlPPaLa/ol2NdUgha8=.
Parent-provided executable BLAKE3:
4b6b5110e762277ca8d1bd4643754407b3cbd91ad84fbe2dfec812950acbe8af.

The complete unchanged owning policy was explicitly selected with
--root . --policy cairn-policy/generated/cairn-policy.json. Policy hash:
3e86638087e65018870bdfefb44da4959d6d58829e43b3cb110e7f40c889222b. No policy was weakened or replaced.

| Command | Observed result | Receipt hash |
|---|---|---|
| cairn gate proposal owned-source-agpl | PASS, valid true, no issues | 7da7fd09320c4fcb0e9269031c84abb2563ebe19a4bf05ec00719eb0105cca6c |
| cairn gate design owned-source-agpl | PASS, valid true, no issues | f9c4f93c7e8116702a4fb6d98ffd52c0ad974bd7255e18e4e792198c97122b87 |
| cairn gate tasks owned-source-agpl | PASS, valid true, no issues | 29202d1db0170baed36370cdd70b8c679fd3b508323a4bd67ee850fce88733c6 |

All gates ran before product metadata changes. The workspace now selects
AGPL-3.0-or-later and both existing Cargo members inherit it. Root and
package-local LICENSE, LICENSE-MIT, and NOTICE carry the full current grant,
original historical MIT text, attribution, and third-party boundaries. Source
identities, immutable dependency objects, extraction facts, and prior grants
remain intact. No dependency, pin, algorithm, API, registry, or publication
changed. Owned policy/fixture licensing is stated in the current-source grant;
no executable license projection or fixture expectations required regeneration.

These gates establish structural preimplementation admission only, not legal
correctness, publication, or release readiness. Final coordinated licensing
and packaging verification remains parent-owned and pending. No builds, tests,
lint, formatters, or broad validation ran during worker implementation. The
change remains open with verification unchecked.
