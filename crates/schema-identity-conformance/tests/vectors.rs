#![allow(
    unknown_lints,
    reason = "Octet Dylint names load only during the pinned cargo-octet gate; owner: schema-identity-core"
)]
#![allow(
    clippy::useless_attribute,
    reason = "Octet allow attributes become active only under the pinned Dylint driver; owner: schema-identity-core"
)]

//! Positive and negative portable vector checks.

#[allow(
    non_trait_imports,
    reason = "the integration test names the single conformance entry point; owner: schema-identity-core"
)]
use schema_identity_conformance::{CHECKED_VECTORS_JSON, verify_fixture_json};

#[test]
fn checked_vectors_match_the_pure_core() -> Result<(), schema_identity_conformance::FixtureError> {
    verify_fixture_json(CHECKED_VECTORS_JSON)
}

#[test]
fn malformed_vector_input_fails_closed() {
    let error = verify_fixture_json("{not-json}").err();
    assert!(error.is_some());
}
