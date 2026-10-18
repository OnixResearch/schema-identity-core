let Contracts = import "source-contracts.ncl" in
let Adapter = "https://seed.radicle.garden/z6gGpUJtzdVBCCtZTzh4cV1skv4H.git" in
let Revision = "2562c8aa38a034061f9af9f3e17280494a5b8de2" in
{
  schema = "onix.schema-identity.source-boundary.v1",
  repository_id = "rad:z6gGpUJtzdVBCCtZTzh4cV1skv4H",
  acquisition_adapter = Adapter,
  reviewed_revision = Revision,
  cargo = { adapter = Adapter, revision = Revision },
  nix = { adapter = Adapter, revision = Revision },
  non_claims = [
    "publication does not prove remote availability",
    "publication does not prove delegate honesty",
    "publication does not prove consumer compatibility",
    "publication does not prove implementation correctness or release eligibility",
  ],
} | Contracts.Source
