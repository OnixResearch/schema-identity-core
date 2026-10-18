# Architecture

`schema-identity-core` is a `no_std + alloc` functional core. It owns deterministic schema admission, identity projections, directional compatibility, and stable reason classes.

## Core flow

```text
caller-owned descriptor and limits
  -> descriptor admission
  -> checked acyclic graph
  -> structural and exact schema identities
  -> caller-supplied policy, alias, and migration observations
  -> closed compatibility report
```

The descriptor uses local node numbers only for graph links. Node numbers, source order, display names, registry names, and paths do not enter identity. Stable member IDs, scalar classes, required markers, openness, nested shapes, schema references, explicit mode, lineage, and brand do enter identity.

The core sorts member identities before framing. It rejects duplicate node IDs, duplicate member IDs, missing nodes, unreachable nodes, cycles, zero limits, and exceeded limits before it returns a `SchemaId`.

## Identity ownership

`content-identity-core` supplies versioned BLAKE3 framing. This repository selects all schema facts and owns the schema domains. Equal digests do not prove authenticity, trust, business equivalence, or authorization.

`TypeLineageId` derives from an explicit caller-owned owner domain and local key. Display names are metadata. A caller can use a name in its nominal key only by making that choice explicit.

`SchemaId` identifies one complete exact identity-bearing descriptor. `StructuralId` identifies one normalized root shape. Nominal and branded modes never fall back to structural mode.

## Compatibility ownership

Compatibility receives already-loaded values:

- expected and actual admitted schemas;
- product-owned alias scope;
- policy profile and allow or deny observation;
- optional active directional alias observation;
- optional exact migration reference.

Policy denial wins. Aliases bind exact source, target, scope, policy, and optional generation. Migrations bind exact source, target, artifact, and policy. The core reports migration availability but does not load or execute migration code.

## Consumer authority

Consumers retain:

- schema registries and discovery;
- descriptor decoding and product DTOs;
- clocks, alias currentness, and policy loading;
- persistence and transactions;
- migration execution and rollback;
- authorization and capability checks;
- diagnostics, receipts, and release decisions.

The core has no filesystem, network, process, clock, environment, registry, logging, storage, or mutable global access. It defines no effect port because it performs no external effect.
