#[allow(
    non_trait_imports,
    reason = "schema framing errors remain explicit at identity constructors; owner: schema-identity-core"
)]
use crate::SchemaIdentityError;
#[allow(
    non_trait_imports,
    reason = "bounded owned text is the checked identifier representation; owner: schema-identity-core"
)]
use alloc::string::String;

// r[impl onix.schema_identity.lineage]
const LINEAGE_DOMAIN: &str = "schema-lineage.v1";
const BRAND_DOMAIN: &str = "schema-brand.v1";

/// Maximum UTF-8 byte length for a checked text value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextLimit(u16);

impl TextLimit {
    /// Creates a non-zero text limit.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaIdentityError::ZeroLimit`] when `maximum_bytes` is zero.
    pub const fn new(maximum_bytes: u16) -> Result<Self, SchemaIdentityError> {
        if maximum_bytes == 0 {
            return Err(SchemaIdentityError::ZeroLimit {
                field: "text limit",
            });
        }
        Ok(Self(maximum_bytes))
    }

    /// Returns the maximum UTF-8 byte length.
    #[must_use]
    pub const fn maximum_bytes(self) -> u16 {
        self.0
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct CheckedText(String);

impl CheckedText {
    fn new(
        value: impl Into<String>,
        limit: TextLimit,
        field: &'static str,
    ) -> Result<Self, SchemaIdentityError> {
        let value = value.into();
        if value.is_empty() {
            return Err(SchemaIdentityError::EmptyText { field });
        }
        let actual_bytes =
            u32::try_from(value.len()).map_err(|_| SchemaIdentityError::TextTooLong {
                field,
                actual_bytes: u32::MAX,
                maximum_bytes: limit.maximum_bytes(),
            })?;
        if actual_bytes > u32::from(limit.maximum_bytes()) {
            return Err(SchemaIdentityError::TextTooLong {
                field,
                actual_bytes,
                maximum_bytes: limit.maximum_bytes(),
            });
        }
        Ok(Self(value))
    }

    const fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

macro_rules! text_id {
    ($(#[$metadata:meta])* $name:ident, $field:literal) => {
        $(#[$metadata])*
        #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
        pub struct $name(CheckedText);

        impl $name {
            /// Creates a checked identifier.
            ///
            /// # Errors
            ///
            /// Returns an error when the text is empty or exceeds `limit`.
            pub fn new(value: impl Into<String>, limit: TextLimit) -> Result<Self, SchemaIdentityError> {
                CheckedText::new(value, limit, $field).map(Self)
            }

            /// Returns the checked identifier text.
            #[must_use]
            pub const fn as_str(&self) -> &str {
                self.0.as_str()
            }
        }
    };
}

text_id!(/// Stable identity for a member within one schema scope.
    MemberId, "member id");
text_id!(/// Product-owned scope for directional alias use.
    AliasScopeId, "alias scope id");
text_id!(/// Identity of the consumer policy profile used for a decision.
    PolicyProfileId, "policy profile id");
text_id!(/// Identity of a migration artifact. The core never executes it.
    MigrationArtifactId, "migration artifact id");
text_id!(/// Non-identity display metadata.
    DisplayName, "display name");

/// Caller-owned nominal owner and local key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NominalScope {
    owner: CheckedText,
    local_key: CheckedText,
}

impl NominalScope {
    /// Creates a checked nominal scope.
    ///
    /// # Errors
    ///
    /// Returns an error when the owner or local key is empty or exceeds `limit`.
    pub fn new(
        owner: impl Into<String>,
        local_key: impl Into<String>,
        limit: TextLimit,
    ) -> Result<Self, SchemaIdentityError> {
        Ok(Self {
            owner: CheckedText::new(owner, limit, "nominal owner")?,
            local_key: CheckedText::new(local_key, limit, "nominal local key")?,
        })
    }

    /// Derives one durable lineage identity.
    ///
    /// # Errors
    ///
    /// Returns an error when the pinned framing dependency rejects the domain.
    pub fn lineage_id(&self) -> Result<TypeLineageId, SchemaIdentityError> {
        derive_nominal(LINEAGE_DOMAIN, self).map(TypeLineageId)
    }

    /// Derives one brand identity.
    ///
    /// # Errors
    ///
    /// Returns an error when the pinned framing dependency rejects the domain.
    pub fn brand_id(&self) -> Result<BrandId, SchemaIdentityError> {
        derive_nominal(BRAND_DOMAIN, self).map(BrandId)
    }
}

fn derive_nominal(
    domain: &'static str,
    scope: &NominalScope,
) -> Result<[u8; 32], SchemaIdentityError> {
    crate::framing::nominal_digest(crate::framing::NominalDigestInput {
        domain,
        owner: scope.owner.as_str(),
        local_key: scope.local_key.as_str(),
    })
}

macro_rules! digest_id {
    ($(#[$metadata:meta])* $name:ident) => {
        $(#[$metadata])*
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub struct $name(pub(crate) [u8; 32]);

        impl $name {
            /// Returns the canonical lowercase hexadecimal identity.
            #[must_use]
            pub fn to_hex(self) -> String {
                crate::framing::digest_hex(self.0)
            }

            /// Returns the raw BLAKE3 digest bytes.
            #[must_use]
            pub const fn as_bytes(&self) -> &[u8; 32] {
                &self.0
            }
        }
    };
}

digest_id!(/// Identity for one durable schema evolution line.
    TypeLineageId);
digest_id!(/// Identity for one branded-structural domain.
    BrandId);
digest_id!(/// Identity for one exact schema descriptor.
    SchemaId);
digest_id!(/// Identity for one normalized structural shape.
    StructuralId);
digest_id!(/// Identity for one exact directional migration reference.
    MigrationReferenceId);

/// Descriptor-local node identifier. Node numbers do not enter schema identity.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeId(u32);

impl NodeId {
    /// Creates a descriptor-local node identifier.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the descriptor-local numeric value.
    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }
}
