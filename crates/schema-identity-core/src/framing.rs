#[allow(
    non_trait_imports,
    reason = "schema facts are selected locally before framing; owner: schema-identity-core"
)]
use crate::{
    AliasBinding, IdentityMode, MigrationReference, NodeId, Openness, OperationMember,
    RecordMember, ScalarKind, SchemaIdentityError, SchemaNodeKind, VariantMember,
};
#[allow(
    non_trait_imports,
    reason = "bounded maps and local sorting vectors implement canonical projection; owner: schema-identity-core"
)]
use alloc::{collections::BTreeMap, vec::Vec};
#[allow(
    non_trait_imports,
    reason = "the pinned pure framing component is the reviewed identity mechanism; owner: schema-identity-core"
)]
use content_identity_core::{Domain, IdentityHasher};

// r[impl onix.schema_identity.content_identity_boundary]
const STRUCTURAL_DOMAIN: &str = "schema-structural.v1";
const EXACT_DOMAIN: &str = "schema-exact.v1";
const MIGRATION_DOMAIN: &str = "schema-migration-reference.v1";

struct SchemaHasher(IdentityHasher);

impl SchemaHasher {
    #[allow(
        concrete_construction_in_core,
        reason = "the pinned content identity component is the repository-owned deterministic framing mechanism; owner: schema-identity-core"
    )]
    fn new(domain: Domain) -> Self {
        Self(IdentityHasher::new(domain))
    }

    #[allow(
        mutating_input_in_pure,
        reason = "local hasher mutation cannot escape deterministic schema projection; owner: schema-identity-core"
    )]
    fn update_tagged_str(&mut self, tag: &[u8], value: &str) -> Result<(), SchemaIdentityError> {
        self.0.update_tagged_str(tag, value)?;
        Ok(())
    }

    #[allow(
        mutating_input_in_pure,
        reason = "local hasher mutation cannot escape deterministic schema projection; owner: schema-identity-core"
    )]
    fn update_tagged_bytes(&mut self, tag: &[u8], value: &[u8]) -> Result<(), SchemaIdentityError> {
        self.0.update_tagged_bytes(tag, value)?;
        Ok(())
    }

    #[allow(
        mutating_input_in_pure,
        reason = "local hasher mutation cannot escape deterministic schema projection; owner: schema-identity-core"
    )]
    fn update_tagged_bool(&mut self, tag: &[u8], value: bool) {
        self.0.update_tagged_bool(tag, value);
    }

    #[allow(
        mutating_input_in_pure,
        reason = "local hasher mutation cannot escape deterministic schema projection; owner: schema-identity-core"
    )]
    fn update_len_le(&mut self, value: usize) -> Result<(), SchemaIdentityError> {
        self.0.update_len_le(value)?;
        Ok(())
    }

    fn finish(self) -> [u8; 32] {
        *self.0.finish().as_bytes()
    }
}

#[derive(Clone, Copy)]
pub(crate) struct NominalDigestInput<'a> {
    pub(crate) domain: &'static str,
    pub(crate) owner: &'a str,
    pub(crate) local_key: &'a str,
}

pub(crate) fn nominal_digest(
    input: NominalDigestInput<'_>,
) -> Result<[u8; 32], SchemaIdentityError> {
    let mut hasher = SchemaHasher::new(Domain::new(input.domain)?);
    hasher.update_tagged_str(b"owner", input.owner)?;
    hasher.update_tagged_str(b"local-key", input.local_key)?;
    Ok(hasher.finish())
}

#[allow(
    concrete_construction_in_core,
    reason = "Digest formatting uses the pinned pure content identity value; owner: schema-identity-core"
)]
pub(crate) fn digest_hex(digest: [u8; 32]) -> alloc::string::String {
    content_identity_core::Digest::from_bytes(digest).to_hex()
}

pub(crate) fn node_digest(
    kind: &SchemaNodeKind,
    digests: &BTreeMap<NodeId, [u8; 32]>,
) -> Result<Option<[u8; 32]>, SchemaIdentityError> {
    if !children_are_ready(kind, digests) {
        return Ok(None);
    }
    debug_assert!(children_are_ready(kind, digests));
    debug_assert!(!STRUCTURAL_DOMAIN.is_empty());
    let mut hasher = SchemaHasher::new(Domain::new(STRUCTURAL_DOMAIN)?);
    match kind {
        SchemaNodeKind::Scalar(scalar) => {
            hasher.update_tagged_str(b"kind", "scalar")?;
            hasher.update_tagged_str(b"scalar", scalar_name(*scalar))?;
        }
        SchemaNodeKind::Optional { value } => {
            let Some(value_digest) = digests.get(value) else {
                return Ok(None);
            };
            hasher.update_tagged_str(b"kind", "optional")?;
            add_digest(&mut hasher, b"value", value_digest)?;
        }
        SchemaNodeKind::Sequence { element } => {
            let Some(element_digest) = digests.get(element) else {
                return Ok(None);
            };
            hasher.update_tagged_str(b"kind", "sequence")?;
            add_digest(&mut hasher, b"element", element_digest)?;
        }
        SchemaNodeKind::Map { key, value } => {
            let Some(key_digest) = digests.get(key) else {
                return Ok(None);
            };
            let Some(value_digest) = digests.get(value) else {
                return Ok(None);
            };
            hasher.update_tagged_str(b"kind", "map")?;
            add_digest(&mut hasher, b"key", key_digest)?;
            add_digest(&mut hasher, b"value", value_digest)?;
        }
        SchemaNodeKind::Record { members } => {
            hash_record(&mut hasher, members, digests)?;
        }
        SchemaNodeKind::Variant { members } => {
            hash_variant(&mut hasher, members, digests)?;
        }
        SchemaNodeKind::OperationSet { members } => {
            hash_operations(&mut hasher, members, digests)?;
        }
        SchemaNodeKind::SchemaReference { schema_id } => {
            hasher.update_tagged_str(b"kind", "schema-reference")?;
            add_digest(&mut hasher, b"schema", schema_id.as_bytes())?;
        }
    }
    Ok(Some(hasher.finish()))
}

#[allow(
    missing_const_fn,
    reason = "BTreeMap lookup is not const on the pinned Rust cohort; owner: schema-identity-core"
)]
fn children_are_ready(kind: &SchemaNodeKind, digests: &BTreeMap<NodeId, [u8; 32]>) -> bool {
    match kind {
        SchemaNodeKind::Scalar(_) | SchemaNodeKind::SchemaReference { .. } => true,
        SchemaNodeKind::Optional { value } => digests.contains_key(value),
        SchemaNodeKind::Sequence { element } => digests.contains_key(element),
        SchemaNodeKind::Map { key, value } => {
            digests.contains_key(key) && digests.contains_key(value)
        }
        SchemaNodeKind::Record { members } => members
            .iter()
            .all(|member| digests.contains_key(&member.value_node())),
        SchemaNodeKind::Variant { members } => members.iter().all(|member| {
            member
                .payload_node()
                .is_none_or(|node| digests.contains_key(&node))
        }),
        SchemaNodeKind::OperationSet { members } => members.iter().all(|member| {
            digests.contains_key(&member.request_node())
                && digests.contains_key(&member.response_node())
        }),
    }
}

#[allow(
    mutating_input_in_pure,
    reason = "local hasher mutation cannot escape deterministic record projection; owner: schema-identity-core"
)]
fn hash_record(
    hasher: &mut SchemaHasher,
    members: &[RecordMember],
    digests: &BTreeMap<NodeId, [u8; 32]>,
) -> Result<(), SchemaIdentityError> {
    let mut sorted: Vec<&RecordMember> = members.iter().collect();
    sorted.sort_by(|left, right| left.member_id().cmp(right.member_id()));
    hasher.update_tagged_str(b"kind", "record")?;
    hasher.update_len_le(sorted.len())?;
    for member in sorted {
        let Some(value_digest) = digests.get(&member.value_node()) else {
            return Err(SchemaIdentityError::MissingNode {
                node_id: member.value_node(),
            });
        };
        hasher.update_tagged_str(b"member", member.member_id().as_str())?;
        hasher.update_tagged_bool(b"required", member.is_required());
        add_digest(hasher, b"value", value_digest)?;
    }
    Ok(())
}

#[allow(
    mutating_input_in_pure,
    reason = "local hasher mutation cannot escape deterministic variant projection; owner: schema-identity-core"
)]
fn hash_variant(
    hasher: &mut SchemaHasher,
    members: &[VariantMember],
    digests: &BTreeMap<NodeId, [u8; 32]>,
) -> Result<(), SchemaIdentityError> {
    let member_count = members.len();
    let mut sorted: Vec<&VariantMember> = members.iter().collect();
    sorted.sort_by(|left, right| left.member_id().cmp(right.member_id()));
    debug_assert_eq!(sorted.len(), member_count);
    debug_assert_eq!(members.len(), member_count);
    hasher.update_tagged_str(b"kind", "variant")?;
    hasher.update_len_le(sorted.len())?;
    for member in sorted {
        hasher.update_tagged_str(b"member", member.member_id().as_str())?;
        match member.payload_node() {
            Some(payload_node) => {
                let Some(payload_digest) = digests.get(&payload_node) else {
                    return Err(SchemaIdentityError::MissingNode {
                        node_id: payload_node,
                    });
                };
                hasher.update_tagged_bool(b"has-payload", true);
                add_digest(hasher, b"payload", payload_digest)?;
            }
            None => {
                hasher.update_tagged_bool(b"has-payload", false);
            }
        }
    }
    Ok(())
}

#[allow(
    mutating_input_in_pure,
    reason = "local hasher mutation cannot escape deterministic operation projection; owner: schema-identity-core"
)]
fn hash_operations(
    hasher: &mut SchemaHasher,
    members: &[OperationMember],
    digests: &BTreeMap<NodeId, [u8; 32]>,
) -> Result<(), SchemaIdentityError> {
    let member_count = members.len();
    let mut sorted: Vec<&OperationMember> = members.iter().collect();
    sorted.sort_by(|left, right| left.member_id().cmp(right.member_id()));
    debug_assert_eq!(sorted.len(), member_count);
    debug_assert_eq!(members.len(), member_count);
    hasher.update_tagged_str(b"kind", "operation-set")?;
    hasher.update_len_le(sorted.len())?;
    for member in sorted {
        let Some(request_digest) = digests.get(&member.request_node()) else {
            return Err(SchemaIdentityError::MissingNode {
                node_id: member.request_node(),
            });
        };
        let Some(response_digest) = digests.get(&member.response_node()) else {
            return Err(SchemaIdentityError::MissingNode {
                node_id: member.response_node(),
            });
        };
        hasher.update_tagged_str(b"member", member.member_id().as_str())?;
        add_digest(hasher, b"request", request_digest)?;
        add_digest(hasher, b"response", response_digest)?;
    }
    Ok(())
}

pub(crate) fn exact_digest(
    mode: &IdentityMode,
    openness: Openness,
    structural_digest: &[u8; 32],
) -> Result<[u8; 32], SchemaIdentityError> {
    let mut hasher = SchemaHasher::new(Domain::new(EXACT_DOMAIN)?);
    hasher.update_tagged_str(b"openness", openness_name(openness))?;
    add_digest(&mut hasher, b"structure", structural_digest)?;
    match mode {
        IdentityMode::Structural => {
            hasher.update_tagged_str(b"mode", "structural")?;
        }
        IdentityMode::Nominal { lineage } => {
            hasher.update_tagged_str(b"mode", "nominal")?;
            add_digest(&mut hasher, b"lineage", lineage.as_bytes())?;
        }
        IdentityMode::BrandedStructural { brand } => {
            hasher.update_tagged_str(b"mode", "branded-structural")?;
            add_digest(&mut hasher, b"brand", brand.as_bytes())?;
        }
    }
    Ok(hasher.finish())
}

pub(crate) fn migration_digest(
    reference: &MigrationReference,
) -> Result<[u8; 32], SchemaIdentityError> {
    let mut hasher = SchemaHasher::new(Domain::new(MIGRATION_DOMAIN)?);
    add_digest(&mut hasher, b"source", reference.source_schema().as_bytes())?;
    add_digest(&mut hasher, b"target", reference.target_schema().as_bytes())?;
    hasher.update_tagged_str(b"artifact", reference.artifact_id().as_str())?;
    hasher.update_tagged_str(b"policy", reference.policy_profile().as_str())?;
    Ok(hasher.finish())
}

pub(crate) fn alias_matches(binding: &AliasBinding, input: &crate::CompatibilityInput<'_>) -> bool {
    binding.source_schema() == input.actual().schema_id()
        && binding.target_schema() == input.expected().schema_id()
        && binding.scope() == input.scope()
        && binding.policy_profile() == input.policy_profile()
}

#[allow(
    mutating_input_in_pure,
    reason = "local hasher mutation cannot escape deterministic digest framing; owner: schema-identity-core"
)]
fn add_digest(
    hasher: &mut SchemaHasher,
    tag: &[u8],
    digest: &[u8; 32],
) -> Result<(), SchemaIdentityError> {
    hasher.update_tagged_bytes(tag, digest)?;
    Ok(())
}

const fn scalar_name(kind: ScalarKind) -> &'static str {
    match kind {
        ScalarKind::Boolean => "boolean",
        ScalarKind::Signed64 => "signed-64",
        ScalarKind::Unsigned64 => "unsigned-64",
        ScalarKind::Text => "text",
        ScalarKind::Bytes => "bytes",
        ScalarKind::Symbol => "symbol",
    }
}

const fn openness_name(openness: Openness) -> &'static str {
    match openness {
        Openness::Closed => "closed",
        Openness::Open => "open",
    }
}
