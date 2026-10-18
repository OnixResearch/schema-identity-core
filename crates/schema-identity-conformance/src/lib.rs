// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Onix Research
// Earlier source grants remain effective; see NOTICE.

#![forbid(unsafe_code)]
#![allow(
    unknown_lints,
    reason = "Octet Dylint names load only during the pinned cargo-octet gate; owner: schema-identity-core"
)]
#![allow(
    clippy::useless_attribute,
    reason = "Octet allow attributes become active only under the pinned Dylint driver; owner: schema-identity-core"
)]

//! Portable positive and negative schema identity vectors.
//!
//! Consumers can compare this deterministic report with the checked Nickel
//! projection without loading registries, policy, storage, or migration code.

#[allow(
    non_trait_imports,
    reason = "the conformance adapter maps one explicit schema identity cohort; owner: schema-identity-core"
)]
use schema_identity_core::{
    AdmissionLimits, AliasBinding, AliasObservation, AliasScopeId, CompatibilityClass,
    CompatibilityContext, CompatibilityInput, CompatibilityMechanisms, IdentityMode, MemberId,
    MemberLimit, MigrationArtifactId, MigrationObservation, MigrationReference, NodeId, NodeLimit,
    NominalScope, Openness, PolicyObservation, PolicyProfileId, ReasonClass, RecordMember,
    ScalarKind, SchemaDescriptor, SchemaIdentityError, SchemaNode, SchemaNodeKind, TextLimit,
    admit_schema, classify_compatibility,
};
#[allow(
    non_trait_imports,
    reason = "Serde derives define the portable fixture DTO boundary; owner: schema-identity-core"
)]
use serde::{Deserialize, Serialize};
#[allow(
    non_trait_imports,
    reason = "the standard formatter is the required fixture error adapter; owner: schema-identity-core"
)]
use std::fmt;

// r[impl onix.schema_identity.compatibility_fixtures]
// r[impl onix.schema_identity.compatibility_fixtures.negative]
// r[impl onix.schema_identity.consumer_pilots]
// r[impl onix.schema_identity.source_inventory]
// r[impl onix.schema_identity.source_revision]
/// Fixture schema identity.
pub const FIXTURE_SCHEMA: &str = "schema-identity-conformance.v1";

/// Checked Nickel-exported vector bytes carried by the published package.
pub const CHECKED_VECTORS_JSON: &str = include_str!("vectors.json");

const TEXT_LIMIT_BYTES: u16 = 64;
const MAXIMUM_NODES: u32 = 32;
const MAXIMUM_MEMBERS: u32 = 64;
const ROOT_NODE: NodeId = NodeId::new(1);
const VALUE_NODE: NodeId = NodeId::new(2);
const ALIAS_GENERATION: u64 = 7;
const DIGEST_HEX_LENGTH: usize = 64;

/// One portable fixture report.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FixtureReport {
    /// Fixture schema identity.
    pub schema: String,
    /// Ordered positive and negative cases.
    pub cases: Vec<VectorCase>,
}

/// One named portable vector.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VectorCase {
    /// Stable case identity.
    pub id: String,
    /// Deterministic result.
    pub result: VectorResult,
}

/// Closed portable vector result.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "kebab-case")]
pub enum VectorResult {
    /// A compatibility decision bound to exact schema identities.
    Decision {
        /// Expected exact schema identity.
        expected_schema: String,
        /// Actual exact schema identity.
        actual_schema: String,
        /// Closed compatibility class.
        class: String,
        /// Stable reason class.
        reason: String,
    },
    /// Descriptor admission rejected malformed input.
    Rejection {
        /// Stable diagnostic string.
        diagnostic: String,
    },
}

/// Conformance fixture failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureError(String);

impl fmt::Display for FixtureError {
    #[allow(
        missing_const_fn,
        mutating_input_in_pure,
        reason = "the standard Display contract mutates its formatter and cannot be const; owner: schema-identity-core"
    )]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for FixtureError {}

impl From<SchemaIdentityError> for FixtureError {
    fn from(error: SchemaIdentityError) -> Self {
        Self(error.to_string())
    }
}

impl From<serde_json::Error> for FixtureError {
    fn from(error: serde_json::Error) -> Self {
        Self(error.to_string())
    }
}

/// Evaluates the shared positive and negative vector cohort.
///
/// # Errors
///
/// Returns an error if a fixture cannot be constructed or evaluated.
pub fn evaluate_fixture() -> Result<FixtureReport, FixtureError> {
    let context = fixture_context()?;
    let mut cases = identity_cases(&context)?;
    cases.extend(mechanism_cases(&context)?);
    cases.push(duplicate_member_case()?);
    Ok(FixtureReport {
        schema: FIXTURE_SCHEMA.to_owned(),
        cases,
    })
}

struct FixtureContext {
    expected: schema_identity_core::AdmittedSchema,
    actual: schema_identity_core::AdmittedSchema,
    scope: AliasScopeId,
    policy: PolicyProfileId,
    alias: AliasObservation,
    migration: MigrationObservation,
    stale_migration: MigrationObservation,
}

fn fixture_context() -> Result<FixtureContext, FixtureError> {
    let expected = admit_schema(nominal_descriptor("expected")?, limits()?)?;
    let actual = admit_schema(nominal_descriptor("actual")?, limits()?)?;
    let scope = scope()?;
    let policy = policy()?;
    let alias = AliasObservation::new(
        AliasBinding::new(
            actual.schema_id(),
            expected.schema_id(),
            scope.clone(),
            policy.clone(),
            Some(ALIAS_GENERATION),
        ),
        true,
    );
    debug_assert_ne!(expected.schema_id(), actual.schema_id());
    debug_assert_eq!(scope.as_str(), "storage");
    let migration_reference = MigrationReference::new(
        actual.schema_id(),
        expected.schema_id(),
        MigrationArtifactId::new("blake3:migration", text_limit()?)?,
        policy.clone(),
    );
    let migration = MigrationObservation::new(migration_reference.clone(), true);
    let stale_migration = MigrationObservation::new(migration_reference, false);
    Ok(FixtureContext {
        expected,
        actual,
        scope,
        policy,
        alias,
        migration,
        stale_migration,
    })
}

fn identity_cases(context: &FixtureContext) -> Result<Vec<VectorCase>, FixtureError> {
    Ok(vec![
        decision_case(
            "exact-schema",
            compatibility_input(DecisionRequest {
                expected: &context.expected,
                actual: &context.expected,
                scope: &context.scope,
                policy: &context.policy,
                observation: PolicyObservation::Allow,
                alias: None,
                migration: None,
            }),
        )?,
        decision_case(
            "nominal-mismatch",
            compatibility_input(DecisionRequest {
                expected: &context.expected,
                actual: &context.actual,
                scope: &context.scope,
                policy: &context.policy,
                observation: PolicyObservation::Allow,
                alias: None,
                migration: None,
            }),
        )?,
        decision_case(
            "directional-alias",
            compatibility_input(DecisionRequest {
                expected: &context.expected,
                actual: &context.actual,
                scope: &context.scope,
                policy: &context.policy,
                observation: PolicyObservation::Allow,
                alias: Some(&context.alias),
                migration: None,
            }),
        )?,
    ])
}

fn mechanism_cases(context: &FixtureContext) -> Result<Vec<VectorCase>, FixtureError> {
    Ok(vec![
        decision_case(
            "reverse-alias-rejected",
            compatibility_input(DecisionRequest {
                expected: &context.actual,
                actual: &context.expected,
                scope: &context.scope,
                policy: &context.policy,
                observation: PolicyObservation::Allow,
                alias: Some(&context.alias),
                migration: None,
            }),
        )?,
        decision_case(
            "migration-available",
            compatibility_input(DecisionRequest {
                expected: &context.expected,
                actual: &context.actual,
                scope: &context.scope,
                policy: &context.policy,
                observation: PolicyObservation::Allow,
                alias: None,
                migration: Some(&context.migration),
            }),
        )?,
        decision_case(
            "stale-migration-rejected",
            compatibility_input(DecisionRequest {
                expected: &context.expected,
                actual: &context.actual,
                scope: &context.scope,
                policy: &context.policy,
                observation: PolicyObservation::Allow,
                alias: None,
                migration: Some(&context.stale_migration),
            }),
        )?,
        decision_case(
            "policy-denial-wins",
            compatibility_input(DecisionRequest {
                expected: &context.expected,
                actual: &context.expected,
                scope: &context.scope,
                policy: &context.policy,
                observation: PolicyObservation::Deny,
                alias: Some(&context.alias),
                migration: Some(&context.migration),
            }),
        )?,
    ])
}

/// Compares the deterministic fixture report with checked JSON bytes.
///
/// # Errors
///
/// Returns an error for malformed JSON, fixture evaluation failure, or drift.
pub fn verify_fixture_json(expected_json: &str) -> Result<(), FixtureError> {
    let expected: FixtureReport = serde_json::from_str(expected_json)?;
    let actual = evaluate_fixture()?;
    if actual == expected {
        Ok(())
    } else {
        Err(FixtureError("schema identity fixture drift".to_owned()))
    }
}

#[derive(Clone, Copy)]
struct DecisionRequest<'a> {
    expected: &'a schema_identity_core::AdmittedSchema,
    actual: &'a schema_identity_core::AdmittedSchema,
    scope: &'a AliasScopeId,
    policy: &'a PolicyProfileId,
    observation: PolicyObservation,
    alias: Option<&'a AliasObservation>,
    migration: Option<&'a MigrationObservation>,
}

const fn compatibility_input(request: DecisionRequest<'_>) -> CompatibilityInput<'_> {
    CompatibilityInput::new(
        request.expected,
        request.actual,
        CompatibilityContext::new(request.scope, request.policy, request.observation),
        CompatibilityMechanisms::new(request.alias, request.migration),
    )
}

fn decision_case(id: &str, input: CompatibilityInput<'_>) -> Result<VectorCase, FixtureError> {
    let report = classify_compatibility(&input)?;
    Ok(VectorCase {
        id: id.to_owned(),
        result: VectorResult::Decision {
            expected_schema: report.expected_schema().to_hex(),
            actual_schema: report.actual_schema().to_hex(),
            class: class_name(report.class()).to_owned(),
            reason: reason_name(report.reason()).to_owned(),
        },
    })
}

fn duplicate_member_case() -> Result<VectorCase, FixtureError> {
    let duplicate_id = MemberId::new("field:value", text_limit()?)?;
    let duplicate_copy = duplicate_id.clone();
    debug_assert_eq!(duplicate_id.as_str(), "field:value");
    debug_assert_eq!(duplicate_id, duplicate_copy);
    let descriptor = SchemaDescriptor::new(
        IdentityMode::Structural,
        Openness::Closed,
        ROOT_NODE,
        vec![
            SchemaNode::new(
                ROOT_NODE,
                SchemaNodeKind::Record {
                    members: vec![
                        RecordMember::new(duplicate_copy, VALUE_NODE, true, None),
                        RecordMember::new(duplicate_id, VALUE_NODE, false, None),
                    ],
                },
            ),
            SchemaNode::new(VALUE_NODE, SchemaNodeKind::Scalar(ScalarKind::Text)),
        ],
        None,
    );
    let diagnostic = match admit_schema(descriptor, limits()?) {
        Ok(_) => {
            return Err(FixtureError(
                "duplicate member fixture was admitted".to_owned(),
            ));
        }
        Err(error) => error.to_string(),
    };
    Ok(VectorCase {
        id: "duplicate-member-rejected".to_owned(),
        result: VectorResult::Rejection { diagnostic },
    })
}

fn nominal_descriptor(local_key: &str) -> Result<SchemaDescriptor, FixtureError> {
    debug_assert!(!local_key.is_empty());
    let lineage = NominalScope::new("fixture-owner", local_key, text_limit()?)?.lineage_id()?;
    debug_assert_eq!(lineage.to_hex().len(), DIGEST_HEX_LENGTH);
    Ok(SchemaDescriptor::new(
        IdentityMode::Nominal { lineage },
        Openness::Closed,
        ROOT_NODE,
        vec![
            SchemaNode::new(
                ROOT_NODE,
                SchemaNodeKind::Record {
                    members: vec![RecordMember::new(
                        MemberId::new("field:value", text_limit()?)?,
                        VALUE_NODE,
                        true,
                        None,
                    )],
                },
            ),
            SchemaNode::new(VALUE_NODE, SchemaNodeKind::Scalar(ScalarKind::Text)),
        ],
        None,
    ))
}

fn text_limit() -> Result<TextLimit, FixtureError> {
    TextLimit::new(TEXT_LIMIT_BYTES).map_err(Into::into)
}

fn limits() -> Result<AdmissionLimits, FixtureError> {
    Ok(AdmissionLimits::new(
        NodeLimit::new(MAXIMUM_NODES)?,
        MemberLimit::new(MAXIMUM_MEMBERS)?,
    ))
}

fn scope() -> Result<AliasScopeId, FixtureError> {
    AliasScopeId::new("storage", text_limit()?).map_err(Into::into)
}

fn policy() -> Result<PolicyProfileId, FixtureError> {
    PolicyProfileId::new("fixture-policy.v1", text_limit()?).map_err(Into::into)
}

const fn class_name(class: CompatibilityClass) -> &'static str {
    match class {
        CompatibilityClass::Exact => "exact",
        CompatibilityClass::Structural => "structural",
        CompatibilityClass::Branded => "branded",
        CompatibilityClass::AdmittedAlias => "admitted-alias",
        CompatibilityClass::MigrationAvailable => "migration-available",
        CompatibilityClass::MigrationRequired => "migration-required",
        CompatibilityClass::Denied => "denied",
    }
}

const fn reason_name(reason: ReasonClass) -> &'static str {
    match reason {
        ReasonClass::ExactSchema => "exact-schema",
        ReasonClass::EqualStructuralShape => "equal-structural-shape",
        ReasonClass::EqualBrandAndShape => "equal-brand-and-shape",
        ReasonClass::ExactDirectionalAlias => "exact-directional-alias",
        ReasonClass::ExactMigrationReference => "exact-migration-reference",
        ReasonClass::PolicyDenied => "policy-denied",
        ReasonClass::NominalMismatch => "nominal-mismatch",
        ReasonClass::BrandMismatch => "brand-mismatch",
        ReasonClass::AliasMismatch => "alias-mismatch",
        ReasonClass::MigrationMismatch => "migration-mismatch",
        ReasonClass::NoExplicitMechanism => "no-explicit-mechanism",
    }
}
