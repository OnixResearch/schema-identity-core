let Contracts = import "contracts.ncl" in
let Expected = "faa587362daee612565efb5632f9209997946737aefd657d9f801f316dce9c88" in
let Actual = "7fee3267086dcfc142287e94285f478c34862eb2fe994c6f06207c4dbce7d905" in

{
  schema = "schema-identity-conformance.v1",
  cases = [
    {
      id = "exact-schema",
      result = { outcome = "decision", expected_schema = Expected, actual_schema = Expected, class = "exact", reason = "exact-schema" },
    },
    {
      id = "nominal-mismatch",
      result = { outcome = "decision", expected_schema = Expected, actual_schema = Actual, class = "migration-required", reason = "nominal-mismatch" },
    },
    {
      id = "directional-alias",
      result = { outcome = "decision", expected_schema = Expected, actual_schema = Actual, class = "admitted-alias", reason = "exact-directional-alias" },
    },
    {
      id = "reverse-alias-rejected",
      result = { outcome = "decision", expected_schema = Actual, actual_schema = Expected, class = "migration-required", reason = "alias-mismatch" },
    },
    {
      id = "migration-available",
      result = { outcome = "decision", expected_schema = Expected, actual_schema = Actual, class = "migration-available", reason = "exact-migration-reference" },
    },
    {
      id = "stale-migration-rejected",
      result = { outcome = "decision", expected_schema = Expected, actual_schema = Actual, class = "migration-required", reason = "migration-mismatch" },
    },
    {
      id = "policy-denial-wins",
      result = { outcome = "decision", expected_schema = Expected, actual_schema = Expected, class = "denied", reason = "policy-denied" },
    },
    {
      id = "duplicate-member-rejected",
      result = { outcome = "rejection", diagnostic = "duplicate member id field:value" },
    },
  ],
} | Contracts.Fixture
