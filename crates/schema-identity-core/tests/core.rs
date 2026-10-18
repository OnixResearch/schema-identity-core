#![allow(
    unknown_lints,
    reason = "Octet Dylint names load only during the pinned cargo-octet gate; owner: schema-identity-core"
)]
#![allow(
    clippy::useless_attribute,
    reason = "Octet allow attributes become active only under the pinned Dylint driver; owner: schema-identity-core"
)]

//! Positive and negative tests for schema identity and compatibility invariants.

#[allow(
    non_trait_imports,
    reason = "the integration test names every checked schema domain type explicitly; owner: schema-identity-core"
)]
use schema_identity_core::{
    AdmissionLimits, AliasBinding, AliasObservation, AliasScopeId, CompatibilityClass,
    CompatibilityContext, CompatibilityInput, CompatibilityMechanisms, DisplayName, IdentityMode,
    MemberId, MemberLimit, MigrationArtifactId, MigrationObservation, MigrationReference, NodeId,
    NodeLimit, NominalScope, Openness, PolicyObservation, PolicyProfileId, ReasonClass,
    RecordMember, ScalarKind, SchemaDescriptor, SchemaIdentityError, SchemaNode, SchemaNodeKind,
    TextLimit, admit_schema, classify_compatibility,
};

// r[verify onix.schema_identity.lineage.rename]
// r[verify onix.schema_identity.lineage.owner]
// r[verify onix.schema_identity.exact_schema.equal]
// r[verify onix.schema_identity.exact_schema.changed]
// r[verify onix.schema_identity.member_ids.order]
// r[verify onix.schema_identity.member_ids.rename]
// r[verify onix.schema_identity.member_ids.duplicate]
// r[verify onix.schema_identity.modes.structural]
// r[verify onix.schema_identity.modes.nominal]
// r[verify onix.schema_identity.modes.brand]
// r[verify onix.schema_identity.directional_alias.forward]
// r[verify onix.schema_identity.directional_alias.reverse]
// r[verify onix.schema_identity.directional_alias.scope]
// r[verify onix.schema_identity.migration_reference.exact]
// r[verify onix.schema_identity.migration_reference.crossed]
// r[verify onix.schema_identity.migration_reference.no_execution]
// r[verify onix.schema_identity.compatibility_decision.policy]
// r[verify onix.schema_identity.compatibility_decision.names]
// r[verify onix.schema_identity.functional_core.equal]
// r[verify onix.schema_identity.content_identity_boundary.local]
// r[verify onix.schema_identity.compatibility_fixtures.positive]
// r[verify onix.schema_identity.compatibility_fixtures.negative]
// r[verify onix.schema_identity.consumer_pilots.agree]
// r[verify onix.schema_identity.consumer_pilots.divergent]
// r[verify onix.schema_identity.source_revision.agree]
// r[verify onix.schema_identity.source_inventory.complete]
const TEXT_LIMIT_BYTES: u16 = 64;
const MAXIMUM_NODES: u32 = 32;
const MAXIMUM_MEMBERS: u32 = 64;
const ROOT_ONE: NodeId = NodeId::new(1);
const FIRST_VALUE: NodeId = NodeId::new(2);
const SECOND_VALUE: NodeId = NodeId::new(3);
const ROOT_TWO: NodeId = NodeId::new(9);
const FIRST_VALUE_TWO: NodeId = NodeId::new(8);
const SECOND_VALUE_TWO: NodeId = NodeId::new(7);
const CYCLIC_ROOT: NodeId = NodeId::new(11);
const CYCLIC_CHILD: NodeId = NodeId::new(12);
const UNREACHABLE_NODE: NodeId = NodeId::new(13);
const ALIAS_GENERATION: u64 = 4;
const PROFILE_NODE_COUNT: u32 = 3;

fn text_limit() -> Result<TextLimit, SchemaIdentityError> {
    TextLimit::new(TEXT_LIMIT_BYTES)
}

fn limits() -> Result<AdmissionLimits, SchemaIdentityError> {
    Ok(AdmissionLimits::new(
        NodeLimit::new(MAXIMUM_NODES)?,
        MemberLimit::new(MAXIMUM_MEMBERS)?,
    ))
}

#[allow(
    ambiguous_params,
    reason = "the test-only member factory keeps identity and display metadata adjacent; owner: schema-identity-core"
)]
fn member(
    id: &str,
    name: &str,
    value_node: NodeId,
    is_required: bool,
) -> Result<RecordMember, SchemaIdentityError> {
    Ok(RecordMember::new(
        MemberId::new(id, text_limit()?)?,
        value_node,
        is_required,
        Some(DisplayName::new(name, text_limit()?)?),
    ))
}

#[allow(
    ambiguous_params,
    reason = "the test-only structural fixture factory uses explicit named node constants at each call; owner: schema-identity-core"
)]
fn structural_profile(
    root: NodeId,
    text_node: NodeId,
    count_node: NodeId,
    has_reversed_members: bool,
    is_renamed: bool,
) -> Result<SchemaDescriptor, SchemaIdentityError> {
    debug_assert_ne!(root, text_node);
    debug_assert_ne!(root, count_node);
    let display_name = if is_renamed {
        "renamed-profile"
    } else {
        "profile"
    };
    let first_name = if is_renamed { "display-title" } else { "title" };
    let first = member("field:title", first_name, text_node, true)?;
    let second = member("field:count", "count", count_node, false)?;
    let members = if has_reversed_members {
        vec![second, first]
    } else {
        vec![first, second]
    };
    Ok(SchemaDescriptor::new(
        IdentityMode::Structural,
        Openness::Closed,
        root,
        vec![
            SchemaNode::new(root, SchemaNodeKind::Record { members }),
            SchemaNode::new(text_node, SchemaNodeKind::Scalar(ScalarKind::Text)),
            SchemaNode::new(count_node, SchemaNodeKind::Scalar(ScalarKind::Unsigned64)),
        ],
        Some(DisplayName::new(display_name, text_limit()?)?),
    ))
}

#[allow(
    ambiguous_params,
    reason = "the test-only nominal fixture factory keeps owner and local key explicit; owner: schema-identity-core"
)]
fn nominal_profile(
    owner: &str,
    key: &str,
    has_changed_type: bool,
) -> Result<SchemaDescriptor, SchemaIdentityError> {
    debug_assert!(!owner.is_empty());
    debug_assert!(!key.is_empty());
    let lineage = NominalScope::new(owner, key, text_limit()?)?.lineage_id()?;
    let value_kind = if has_changed_type {
        ScalarKind::Bytes
    } else {
        ScalarKind::Text
    };
    Ok(SchemaDescriptor::new(
        IdentityMode::Nominal { lineage },
        Openness::Closed,
        ROOT_ONE,
        vec![
            SchemaNode::new(
                ROOT_ONE,
                SchemaNodeKind::Record {
                    members: vec![member("field:value", "value", FIRST_VALUE, true)?],
                },
            ),
            SchemaNode::new(FIRST_VALUE, SchemaNodeKind::Scalar(value_kind)),
        ],
        None,
    ))
}

fn scope() -> Result<AliasScopeId, SchemaIdentityError> {
    AliasScopeId::new("storage", text_limit()?)
}

fn other_scope() -> Result<AliasScopeId, SchemaIdentityError> {
    AliasScopeId::new("effect", text_limit()?)
}

fn policy() -> Result<PolicyProfileId, SchemaIdentityError> {
    PolicyProfileId::new("schema-policy.v1", text_limit()?)
}

fn other_policy() -> Result<PolicyProfileId, SchemaIdentityError> {
    PolicyProfileId::new("other-policy.v1", text_limit()?)
}

#[allow(
    too_many_parameters,
    reason = "the test-only compatibility factory mirrors every explicit boundary fact; owner: schema-identity-core"
)]
const fn compatibility_input<'a>(
    expected: &'a schema_identity_core::AdmittedSchema,
    actual: &'a schema_identity_core::AdmittedSchema,
    request_scope: &'a AliasScopeId,
    request_policy: &'a PolicyProfileId,
    policy_observation: PolicyObservation,
    alias: Option<&'a AliasObservation>,
    migration: Option<&'a MigrationObservation>,
) -> CompatibilityInput<'a> {
    CompatibilityInput::new(
        expected,
        actual,
        CompatibilityContext::new(request_scope, request_policy, policy_observation),
        CompatibilityMechanisms::new(alias, migration),
    )
}

#[test]
fn lineage_uses_owner_and_local_key_not_display_names() -> Result<(), SchemaIdentityError> {
    let first = NominalScope::new("owner:a", "profile", text_limit()?)?.lineage_id()?;
    let renamed = NominalScope::new("owner:a", "profile", text_limit()?)?.lineage_id()?;
    let other_owner = NominalScope::new("owner:b", "profile", text_limit()?)?.lineage_id()?;

    assert_eq!(first, renamed);
    assert_ne!(first, other_owner);
    Ok(())
}

#[test]
fn order_node_numbers_and_display_names_do_not_change_exact_identity()
-> Result<(), SchemaIdentityError> {
    let first = admit_schema(
        structural_profile(ROOT_ONE, FIRST_VALUE, SECOND_VALUE, false, false)?,
        limits()?,
    )?;
    let reordered = admit_schema(
        structural_profile(ROOT_TWO, FIRST_VALUE_TWO, SECOND_VALUE_TWO, true, true)?,
        limits()?,
    )?;

    assert_eq!(first.schema_id(), reordered.schema_id());
    assert_eq!(first.structural_id(), reordered.structural_id());
    Ok(())
}

#[test]
fn one_identity_bearing_fact_changes_exact_identity() -> Result<(), SchemaIdentityError> {
    let first = admit_schema(nominal_profile("owner:a", "profile", false)?, limits()?)?;
    let changed_type = admit_schema(nominal_profile("owner:a", "profile", true)?, limits()?)?;
    let changed_owner = admit_schema(nominal_profile("owner:b", "profile", false)?, limits()?)?;

    assert_ne!(first.schema_id(), changed_type.schema_id());
    assert_ne!(first.schema_id(), changed_owner.schema_id());
    Ok(())
}

#[test]
fn duplicate_member_identity_fails_without_schema_identity() -> Result<(), SchemaIdentityError> {
    let duplicate = member("field:value", "value-copy", FIRST_VALUE, false)?;
    let descriptor = SchemaDescriptor::new(
        IdentityMode::Structural,
        Openness::Closed,
        ROOT_ONE,
        vec![
            SchemaNode::new(
                ROOT_ONE,
                SchemaNodeKind::Record {
                    members: vec![
                        member("field:value", "value", FIRST_VALUE, true)?,
                        duplicate,
                    ],
                },
            ),
            SchemaNode::new(FIRST_VALUE, SchemaNodeKind::Scalar(ScalarKind::Text)),
        ],
        None,
    );

    let error = admit_schema(descriptor, limits()?).err();
    assert!(error.is_some());
    assert_eq!(
        error,
        Some(SchemaIdentityError::DuplicateMemberId {
            member_id: "field:value".into(),
        })
    );
    Ok(())
}

#[test]
fn malformed_missing_unreachable_and_cyclic_graphs_fail_closed() -> Result<(), SchemaIdentityError>
{
    let missing = SchemaDescriptor::new(
        IdentityMode::Structural,
        Openness::Closed,
        ROOT_ONE,
        vec![SchemaNode::new(
            ROOT_ONE,
            SchemaNodeKind::Optional { value: FIRST_VALUE },
        )],
        None,
    );
    assert_eq!(
        admit_schema(missing, limits()?).err(),
        Some(SchemaIdentityError::MissingNode {
            node_id: FIRST_VALUE,
        })
    );

    let unreachable = SchemaDescriptor::new(
        IdentityMode::Structural,
        Openness::Closed,
        ROOT_ONE,
        vec![
            SchemaNode::new(ROOT_ONE, SchemaNodeKind::Scalar(ScalarKind::Text)),
            SchemaNode::new(UNREACHABLE_NODE, SchemaNodeKind::Scalar(ScalarKind::Bytes)),
        ],
        None,
    );
    assert_eq!(
        admit_schema(unreachable, limits()?).err(),
        Some(SchemaIdentityError::UnreachableNode {
            node_id: UNREACHABLE_NODE,
        })
    );

    let cyclic = SchemaDescriptor::new(
        IdentityMode::Structural,
        Openness::Closed,
        CYCLIC_ROOT,
        vec![
            SchemaNode::new(
                CYCLIC_ROOT,
                SchemaNodeKind::Optional {
                    value: CYCLIC_CHILD,
                },
            ),
            SchemaNode::new(
                CYCLIC_CHILD,
                SchemaNodeKind::Sequence {
                    element: CYCLIC_ROOT,
                },
            ),
        ],
        None,
    );
    assert_eq!(
        admit_schema(cyclic, limits()?).err(),
        Some(SchemaIdentityError::CyclicDescriptor)
    );
    Ok(())
}

#[test]
fn explicit_modes_never_fall_back_to_structural() -> Result<(), SchemaIdentityError> {
    let structural_left = admit_schema(
        structural_profile(ROOT_ONE, FIRST_VALUE, SECOND_VALUE, false, false)?,
        limits()?,
    )?;
    let structural_right = admit_schema(
        structural_profile(ROOT_TWO, FIRST_VALUE_TWO, SECOND_VALUE_TWO, true, true)?,
        limits()?,
    )?;
    let nominal_left = admit_schema(nominal_profile("owner:a", "profile", false)?, limits()?)?;
    let nominal_right = admit_schema(nominal_profile("owner:b", "profile", false)?, limits()?)?;
    let request_scope = scope()?;
    let request_policy = policy()?;

    let structural = classify_compatibility(&compatibility_input(
        &structural_left,
        &structural_right,
        &request_scope,
        &request_policy,
        PolicyObservation::Allow,
        None,
        None,
    ))?;
    assert_eq!(structural.class(), CompatibilityClass::Exact);

    let nominal = classify_compatibility(&compatibility_input(
        &nominal_left,
        &nominal_right,
        &request_scope,
        &request_policy,
        PolicyObservation::Allow,
        None,
        None,
    ))?;
    assert_eq!(nominal.class(), CompatibilityClass::MigrationRequired);
    assert_eq!(nominal.reason(), ReasonClass::NominalMismatch);
    Ok(())
}

#[test]
fn alias_requires_exact_direction_scope_policy_and_currentness() -> Result<(), SchemaIdentityError>
{
    let expected = admit_schema(nominal_profile("owner:a", "expected", false)?, limits()?)?;
    let actual = admit_schema(nominal_profile("owner:a", "actual", false)?, limits()?)?;
    let request_scope = scope()?;
    let crossed_scope = other_scope()?;
    let request_policy = policy()?;
    let binding = AliasBinding::new(
        actual.schema_id(),
        expected.schema_id(),
        request_scope.clone(),
        request_policy.clone(),
        Some(ALIAS_GENERATION),
    );
    let active = AliasObservation::new(binding.clone(), true);
    let inactive = AliasObservation::new(binding, false);

    let admitted = classify_compatibility(&compatibility_input(
        &expected,
        &actual,
        &request_scope,
        &request_policy,
        PolicyObservation::Allow,
        Some(&active),
        None,
    ))?;
    assert_eq!(admitted.class(), CompatibilityClass::AdmittedAlias);

    let wrong_scope = classify_compatibility(&compatibility_input(
        &expected,
        &actual,
        &crossed_scope,
        &request_policy,
        PolicyObservation::Allow,
        Some(&active),
        None,
    ))?;
    assert_eq!(wrong_scope.reason(), ReasonClass::AliasMismatch);

    let stale = classify_compatibility(&compatibility_input(
        &expected,
        &actual,
        &request_scope,
        &request_policy,
        PolicyObservation::Allow,
        Some(&inactive),
        None,
    ))?;
    assert_eq!(stale.class(), CompatibilityClass::MigrationRequired);

    let reverse = classify_compatibility(&compatibility_input(
        &actual,
        &expected,
        &request_scope,
        &request_policy,
        PolicyObservation::Allow,
        Some(&active),
        None,
    ))?;
    assert_eq!(reverse.reason(), ReasonClass::AliasMismatch);
    Ok(())
}

#[test]
fn migration_reference_binds_source_target_artifact_and_policy() -> Result<(), SchemaIdentityError>
{
    let expected = admit_schema(nominal_profile("owner:a", "expected", false)?, limits()?)?;
    let actual = admit_schema(nominal_profile("owner:a", "actual", false)?, limits()?)?;
    let request_scope = scope()?;
    let request_policy = policy()?;
    let wrong_policy = other_policy()?;
    let migration_reference = MigrationReference::new(
        actual.schema_id(),
        expected.schema_id(),
        MigrationArtifactId::new("blake3:migration", text_limit()?)?,
        request_policy.clone(),
    );
    let migration = MigrationObservation::new(migration_reference.clone(), true);
    let stale_migration = MigrationObservation::new(migration_reference, false);

    let available = classify_compatibility(&compatibility_input(
        &expected,
        &actual,
        &request_scope,
        &request_policy,
        PolicyObservation::Allow,
        None,
        Some(&migration),
    ))?;
    assert_eq!(available.class(), CompatibilityClass::MigrationAvailable);
    assert_eq!(available.reason(), ReasonClass::ExactMigrationReference);
    assert_eq!(
        available.migration_reference(),
        Some(migration.reference().reference_id()?)
    );

    let crossed = classify_compatibility(&compatibility_input(
        &expected,
        &actual,
        &request_scope,
        &wrong_policy,
        PolicyObservation::Allow,
        None,
        Some(&migration),
    ))?;
    assert_eq!(crossed.class(), CompatibilityClass::MigrationRequired);
    assert_eq!(crossed.reason(), ReasonClass::MigrationMismatch);

    let stale = classify_compatibility(&compatibility_input(
        &expected,
        &actual,
        &request_scope,
        &request_policy,
        PolicyObservation::Allow,
        None,
        Some(&stale_migration),
    ))?;
    assert_eq!(stale.class(), CompatibilityClass::MigrationRequired);
    assert_eq!(stale.reason(), ReasonClass::MigrationMismatch);
    Ok(())
}

#[test]
fn explicit_policy_denial_wins_over_every_compatibility_mechanism()
-> Result<(), SchemaIdentityError> {
    let schema = admit_schema(nominal_profile("owner:a", "profile", false)?, limits()?)?;
    let request_scope = scope()?;
    let request_policy = policy()?;
    let report = classify_compatibility(&compatibility_input(
        &schema,
        &schema,
        &request_scope,
        &request_policy,
        PolicyObservation::Deny,
        None,
        None,
    ))?;

    assert_eq!(report.class(), CompatibilityClass::Denied);
    assert_eq!(report.reason(), ReasonClass::PolicyDenied);
    assert_eq!(report.expected_schema(), schema.schema_id());
    assert_eq!(report.actual_schema(), schema.schema_id());
    Ok(())
}

#[test]
fn explicit_resource_limits_reject_zero_and_excess() -> Result<(), SchemaIdentityError> {
    assert_eq!(
        NodeLimit::new(0),
        Err(SchemaIdentityError::ZeroLimit {
            field: "maximum nodes",
        })
    );
    let one_node_limit =
        AdmissionLimits::new(NodeLimit::new(1)?, MemberLimit::new(MAXIMUM_MEMBERS)?);
    let descriptor = structural_profile(ROOT_ONE, FIRST_VALUE, SECOND_VALUE, false, false)?;
    assert_eq!(
        admit_schema(descriptor, one_node_limit).err(),
        Some(SchemaIdentityError::NodeLimitExceeded {
            actual: PROFILE_NODE_COUNT,
            maximum: 1,
        })
    );
    Ok(())
}
