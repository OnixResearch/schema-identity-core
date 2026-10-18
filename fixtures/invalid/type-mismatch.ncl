let Contracts = import "../contracts.ncl" in
{ schema = "schema-identity-conformance.v1", cases = [{ id = 7, result = { outcome = "decision" } }] } | Contracts.Fixture
