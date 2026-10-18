let Contracts = import "../contracts.ncl" in
{ schema = "schema-identity-conformance.v1", cases = [{ id = "missing-result" }] } | Contracts.Fixture
