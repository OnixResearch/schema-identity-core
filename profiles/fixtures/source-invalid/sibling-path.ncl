let Contracts = import "../../source-contracts.ncl" in
let source = import "../../source-boundary.ncl" in
(source & { cargo.adapter = "../schema-identity-core" }) | Contracts.Source
