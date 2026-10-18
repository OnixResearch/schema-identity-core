#[allow(
    non_trait_imports,
    reason = "closed compatibility domain types stay explicit at the pure decision boundary; owner: schema-identity-core"
)]
use crate::{
    AdmittedSchema, AliasScopeId, IdentityMode, MigrationArtifactId, MigrationReferenceId,
    PolicyProfileId, SchemaId, SchemaIdentityError, framing,
};

// r[impl onix.schema_identity.directional_alias]
/// Exact directional alias binding. The core does not load or persist aliases.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AliasBinding {
    source_schema: SchemaId,
    target_schema: SchemaId,
    scope: AliasScopeId,
    policy_profile: PolicyProfileId,
    generation: Option<u64>,
}

impl AliasBinding {
    /// Creates one exact directional alias binding.
    #[must_use]
    pub const fn new(
        source_schema: SchemaId,
        target_schema: SchemaId,
        scope: AliasScopeId,
        policy_profile: PolicyProfileId,
        generation: Option<u64>,
    ) -> Self {
        Self {
            source_schema,
            target_schema,
            scope,
            policy_profile,
            generation,
        }
    }

    /// Returns the source schema.
    #[must_use]
    pub const fn source_schema(&self) -> SchemaId {
        self.source_schema
    }

    /// Returns the target schema.
    #[must_use]
    pub const fn target_schema(&self) -> SchemaId {
        self.target_schema
    }

    /// Returns the product-owned alias scope.
    #[must_use]
    pub const fn scope(&self) -> &AliasScopeId {
        &self.scope
    }

    /// Returns the policy profile bound to the alias.
    #[must_use]
    pub const fn policy_profile(&self) -> &PolicyProfileId {
        &self.policy_profile
    }

    /// Returns the optional caller-observed generation.
    #[must_use]
    pub const fn generation(&self) -> Option<u64> {
        self.generation
    }
}

/// Caller-supplied currentness observation for one alias.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AliasObservation {
    binding: AliasBinding,
    is_active: bool,
}

impl AliasObservation {
    /// Creates an explicit alias currentness observation.
    #[must_use]
    pub const fn new(binding: AliasBinding, is_active: bool) -> Self {
        Self { binding, is_active }
    }

    /// Returns the observed binding.
    #[must_use]
    pub const fn binding(&self) -> &AliasBinding {
        &self.binding
    }

    /// Returns whether the caller observed the binding as active.
    #[must_use]
    pub const fn is_active(&self) -> bool {
        self.is_active
    }
}

// r[impl onix.schema_identity.migration_reference]
/// Exact migration reference. It is evidence and never an execution request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationReference {
    source_schema: SchemaId,
    target_schema: SchemaId,
    artifact_id: MigrationArtifactId,
    policy_profile: PolicyProfileId,
}

impl MigrationReference {
    /// Creates one exact directional migration reference.
    #[must_use]
    pub const fn new(
        source_schema: SchemaId,
        target_schema: SchemaId,
        artifact_id: MigrationArtifactId,
        policy_profile: PolicyProfileId,
    ) -> Self {
        Self {
            source_schema,
            target_schema,
            artifact_id,
            policy_profile,
        }
    }

    /// Returns the source schema.
    #[must_use]
    pub const fn source_schema(&self) -> SchemaId {
        self.source_schema
    }

    /// Returns the target schema.
    #[must_use]
    pub const fn target_schema(&self) -> SchemaId {
        self.target_schema
    }

    /// Returns the caller-owned migration artifact identity.
    #[must_use]
    pub const fn artifact_id(&self) -> &MigrationArtifactId {
        &self.artifact_id
    }

    /// Returns the bound policy profile.
    #[must_use]
    pub const fn policy_profile(&self) -> &PolicyProfileId {
        &self.policy_profile
    }

    /// Derives an identity over every exact migration binding.
    ///
    /// # Errors
    ///
    /// Returns an error when the pinned framing dependency rejects the domain or input.
    pub fn reference_id(&self) -> Result<MigrationReferenceId, SchemaIdentityError> {
        framing::migration_digest(self).map(MigrationReferenceId)
    }
}

/// Caller-supplied currentness observation for one migration reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MigrationObservation {
    reference: MigrationReference,
    is_current: bool,
}

impl MigrationObservation {
    /// Creates an explicit migration currentness observation.
    #[must_use]
    pub const fn new(reference: MigrationReference, is_current: bool) -> Self {
        Self {
            reference,
            is_current,
        }
    }

    /// Returns the exact migration reference.
    #[must_use]
    pub const fn reference(&self) -> &MigrationReference {
        &self.reference
    }

    /// Returns whether the caller observed the migration as current.
    #[must_use]
    pub const fn is_current(&self) -> bool {
        self.is_current
    }
}

/// Explicit consumer policy observation supplied to the pure core.
#[doc = "`schema_identity_sealed`"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicyObservation {
    /// Compatibility mechanisms may be evaluated.
    Allow,
    /// Policy denies compatibility before any mechanism can admit it.
    Deny,
}

// r[impl onix.schema_identity.compatibility_decision]
/// Closed compatibility result classes.
#[doc = "`schema_identity_sealed`"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompatibilityClass {
    /// Exact schema identities match.
    Exact,
    /// Both schemas explicitly use structural mode and their structures match.
    Structural,
    /// Both schemas use the same brand and their structures match.
    Branded,
    /// One active exact directional alias admits the request.
    AdmittedAlias,
    /// One exact migration reference is available.
    MigrationAvailable,
    /// No exact compatibility mechanism admits the request.
    MigrationRequired,
    /// Explicit consumer policy denied the request.
    Denied,
}

/// Stable reason classes for compatibility results.
#[doc = "`schema_identity_sealed`"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReasonClass {
    /// Exact schema identities are equal.
    ExactSchema,
    /// Explicit structural modes have equal normalized structures.
    EqualStructuralShape,
    /// Explicit branded modes have equal brand and structure.
    EqualBrandAndShape,
    /// An active alias matched source, target, scope, and policy.
    ExactDirectionalAlias,
    /// An exact migration reference matched source, target, and policy.
    ExactMigrationReference,
    /// Explicit consumer policy denied compatibility.
    PolicyDenied,
    /// Nominal modes did not have one exact identity.
    NominalMismatch,
    /// Branded modes did not have equal brand and structure.
    BrandMismatch,
    /// A supplied alias did not match every required fact.
    AliasMismatch,
    /// A supplied migration reference did not match every required fact.
    MigrationMismatch,
    /// No explicit mechanism was supplied.
    NoExplicitMechanism,
}

/// Caller-owned scope and policy facts for one decision.
#[derive(Clone, Copy, Debug)]
pub struct CompatibilityContext<'a> {
    scope: &'a AliasScopeId,
    policy_profile: &'a PolicyProfileId,
    policy: PolicyObservation,
}

impl<'a> CompatibilityContext<'a> {
    /// Creates explicit compatibility context.
    #[must_use]
    pub const fn new(
        scope: &'a AliasScopeId,
        policy_profile: &'a PolicyProfileId,
        policy: PolicyObservation,
    ) -> Self {
        Self {
            scope,
            policy_profile,
            policy,
        }
    }
}

/// Optional exact mechanisms supplied to one decision.
#[derive(Clone, Copy, Debug)]
pub struct CompatibilityMechanisms<'a> {
    alias: Option<&'a AliasObservation>,
    migration: Option<&'a MigrationObservation>,
}

impl<'a> CompatibilityMechanisms<'a> {
    /// Creates explicit optional compatibility mechanisms.
    #[must_use]
    pub const fn new(
        alias: Option<&'a AliasObservation>,
        migration: Option<&'a MigrationObservation>,
    ) -> Self {
        Self { alias, migration }
    }
}

/// Complete explicit inputs to one compatibility decision.
#[derive(Clone, Copy, Debug)]
pub struct CompatibilityInput<'a> {
    expected: &'a AdmittedSchema,
    actual: &'a AdmittedSchema,
    context: CompatibilityContext<'a>,
    mechanisms: CompatibilityMechanisms<'a>,
}

impl<'a> CompatibilityInput<'a> {
    /// Creates complete compatibility inputs.
    #[must_use]
    pub const fn new(
        expected: &'a AdmittedSchema,
        actual: &'a AdmittedSchema,
        context: CompatibilityContext<'a>,
        mechanisms: CompatibilityMechanisms<'a>,
    ) -> Self {
        Self {
            expected,
            actual,
            context,
            mechanisms,
        }
    }

    /// Returns the expected schema.
    #[must_use]
    pub const fn expected(&self) -> &AdmittedSchema {
        self.expected
    }

    /// Returns the actual schema.
    #[must_use]
    pub const fn actual(&self) -> &AdmittedSchema {
        self.actual
    }

    /// Returns the requested alias scope.
    #[must_use]
    pub const fn scope(&self) -> &AliasScopeId {
        self.context.scope
    }

    /// Returns the selected policy profile.
    #[must_use]
    pub const fn policy_profile(&self) -> &PolicyProfileId {
        self.context.policy_profile
    }
}

/// Compatibility decision bound to exact input identities.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompatibilityReport {
    class: CompatibilityClass,
    reason: ReasonClass,
    expected_schema: SchemaId,
    actual_schema: SchemaId,
    migration_reference: Option<MigrationReferenceId>,
}

impl CompatibilityReport {
    /// Returns the closed decision class.
    #[must_use]
    pub const fn class(self) -> CompatibilityClass {
        self.class
    }

    /// Returns the stable reason class.
    #[must_use]
    pub const fn reason(self) -> ReasonClass {
        self.reason
    }

    /// Returns the expected exact schema identity.
    #[must_use]
    pub const fn expected_schema(self) -> SchemaId {
        self.expected_schema
    }

    /// Returns the actual exact schema identity.
    #[must_use]
    pub const fn actual_schema(self) -> SchemaId {
        self.actual_schema
    }

    /// Returns the exact migration reference identity when available.
    #[must_use]
    pub const fn migration_reference(self) -> Option<MigrationReferenceId> {
        self.migration_reference
    }
}

// r[impl onix.schema_identity.functional_core]
/// Classifies compatibility without I/O, clocks, registry access, or migration execution.
///
/// # Errors
///
/// Returns an error only when identity framing for an exact migration reference fails.
pub fn classify_compatibility(
    input: &CompatibilityInput<'_>,
) -> Result<CompatibilityReport, SchemaIdentityError> {
    debug_assert!(!input.scope().as_str().is_empty());
    debug_assert!(!input.policy_profile().as_str().is_empty());
    if input.context.policy == PolicyObservation::Deny {
        return Ok(report(
            input,
            CompatibilityClass::Denied,
            ReasonClass::PolicyDenied,
            None,
        ));
    }
    if input.expected.schema_id() == input.actual.schema_id() {
        return Ok(report(
            input,
            CompatibilityClass::Exact,
            ReasonClass::ExactSchema,
            None,
        ));
    }
    if structural_match(input) {
        return Ok(report(
            input,
            CompatibilityClass::Structural,
            ReasonClass::EqualStructuralShape,
            None,
        ));
    }
    if branded_match(input) {
        return Ok(report(
            input,
            CompatibilityClass::Branded,
            ReasonClass::EqualBrandAndShape,
            None,
        ));
    }
    if let Some(alias) = input.mechanisms.alias
        && alias.is_active()
        && framing::alias_matches(alias.binding(), input)
    {
        return Ok(report(
            input,
            CompatibilityClass::AdmittedAlias,
            ReasonClass::ExactDirectionalAlias,
            None,
        ));
    }
    if let Some(migration) = input.mechanisms.migration
        && migration.is_current()
        && migration_matches(migration.reference(), input)
    {
        let reference_id = migration.reference().reference_id()?;
        return Ok(report(
            input,
            CompatibilityClass::MigrationAvailable,
            ReasonClass::ExactMigrationReference,
            Some(reference_id),
        ));
    }
    Ok(report(
        input,
        CompatibilityClass::MigrationRequired,
        mismatch_reason(input),
        None,
    ))
}

#[allow(
    missing_const_fn,
    reason = "trait-based identity equality is not const on the pinned Rust cohort; owner: schema-identity-core"
)]
fn structural_match(input: &CompatibilityInput<'_>) -> bool {
    matches!(input.expected.mode(), IdentityMode::Structural)
        && matches!(input.actual.mode(), IdentityMode::Structural)
        && input.expected.structural_id() == input.actual.structural_id()
}

#[allow(
    missing_const_fn,
    reason = "trait-based brand and identity equality is not const on the pinned Rust cohort; owner: schema-identity-core"
)]
fn branded_match(input: &CompatibilityInput<'_>) -> bool {
    let IdentityMode::BrandedStructural {
        brand: expected_brand,
    } = input.expected.mode()
    else {
        return false;
    };
    let IdentityMode::BrandedStructural {
        brand: actual_brand,
    } = input.actual.mode()
    else {
        return false;
    };
    expected_brand == actual_brand && input.expected.structural_id() == input.actual.structural_id()
}

#[allow(
    missing_const_fn,
    reason = "trait-based migration binding equality is not const on the pinned Rust cohort; owner: schema-identity-core"
)]
fn migration_matches(reference: &MigrationReference, input: &CompatibilityInput<'_>) -> bool {
    reference.source_schema() == input.actual.schema_id()
        && reference.target_schema() == input.expected.schema_id()
        && reference.policy_profile() == input.policy_profile()
}

#[allow(
    missing_const_fn,
    reason = "trait-based schema identity assertions are not const on the pinned Rust cohort; owner: schema-identity-core"
)]
fn mismatch_reason(input: &CompatibilityInput<'_>) -> ReasonClass {
    debug_assert_ne!(input.expected.schema_id(), input.actual.schema_id());
    debug_assert!(input.context.policy == PolicyObservation::Allow);
    if input.mechanisms.migration.is_some() {
        return ReasonClass::MigrationMismatch;
    }
    if input.mechanisms.alias.is_some() {
        return ReasonClass::AliasMismatch;
    }
    if matches!(input.expected.mode(), IdentityMode::Nominal { .. })
        || matches!(input.actual.mode(), IdentityMode::Nominal { .. })
    {
        return ReasonClass::NominalMismatch;
    }
    if matches!(
        input.expected.mode(),
        IdentityMode::BrandedStructural { .. }
    ) || matches!(input.actual.mode(), IdentityMode::BrandedStructural { .. })
    {
        return ReasonClass::BrandMismatch;
    }
    ReasonClass::NoExplicitMechanism
}

const fn report(
    input: &CompatibilityInput<'_>,
    class: CompatibilityClass,
    reason: ReasonClass,
    migration_reference: Option<MigrationReferenceId>,
) -> CompatibilityReport {
    CompatibilityReport {
        class,
        reason,
        expected_schema: input.expected.schema_id(),
        actual_schema: input.actual.schema_id(),
        migration_reference,
    }
}
