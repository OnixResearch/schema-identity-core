let ResultContract = {
  outcome | String,
  expected_schema | String | optional,
  actual_schema | String | optional,
  class | String | optional,
  reason | String | optional,
  diagnostic | String | optional,
} in

let CaseContract = {
  id | String,
  result | ResultContract,
} in

let FixtureContract = {
  schema | String,
  cases | Array CaseContract,
} in

{
  Result = ResultContract,
  Case = CaseContract,
  Fixture = FixtureContract,
}
