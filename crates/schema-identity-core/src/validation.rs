#[allow(
    non_trait_imports,
    reason = "checked schema domain types keep admission control flow readable; owner: schema-identity-core"
)]
use crate::{
    IdentityMode, NodeId, SchemaDescriptor, SchemaId, SchemaIdentityError, SchemaNode,
    SchemaNodeKind, StructuralId, framing,
};
#[allow(
    non_trait_imports,
    reason = "bounded collections and domain types stay local to descriptor admission; owner: schema-identity-core"
)]
use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec,
    vec::Vec,
};

/// Non-zero maximum descriptor node count.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NodeLimit(u32);

impl NodeLimit {
    /// Creates a non-zero node limit.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaIdentityError::ZeroLimit`] when `maximum` is zero.
    pub const fn new(maximum: u32) -> Result<Self, SchemaIdentityError> {
        if maximum == 0 {
            return Err(SchemaIdentityError::ZeroLimit {
                field: "maximum nodes",
            });
        }
        Ok(Self(maximum))
    }

    /// Returns the admitted node count.
    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }
}

/// Non-zero maximum total member count.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemberLimit(u32);

impl MemberLimit {
    /// Creates a non-zero member limit.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaIdentityError::ZeroLimit`] when `maximum` is zero.
    pub const fn new(maximum: u32) -> Result<Self, SchemaIdentityError> {
        if maximum == 0 {
            return Err(SchemaIdentityError::ZeroLimit {
                field: "maximum members",
            });
        }
        Ok(Self(maximum))
    }

    /// Returns the admitted total member count.
    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }
}

/// Explicit descriptor resource limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdmissionLimits {
    maximum_nodes: NodeLimit,
    maximum_members: MemberLimit,
}

impl AdmissionLimits {
    /// Creates descriptor limits from typed non-zero values.
    #[must_use]
    pub const fn new(maximum_nodes: NodeLimit, maximum_members: MemberLimit) -> Self {
        Self {
            maximum_nodes,
            maximum_members,
        }
    }

    /// Returns the maximum descriptor node count.
    #[must_use]
    pub const fn maximum_nodes(self) -> u32 {
        self.maximum_nodes.value()
    }

    /// Returns the maximum total member count.
    #[must_use]
    pub const fn maximum_members(self) -> u32 {
        self.maximum_members.value()
    }
}

// r[impl onix.schema_identity.exact_schema]
// r[impl onix.schema_identity.member_ids]
// r[impl onix.schema_identity.member_ids.order]
// r[impl onix.schema_identity.modes]
/// Fully checked descriptor and its exact and structural identities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedSchema {
    descriptor: SchemaDescriptor,
    schema_id: SchemaId,
    structural_id: StructuralId,
}

impl AdmittedSchema {
    /// Returns the admitted descriptor.
    #[must_use]
    pub const fn descriptor(&self) -> &SchemaDescriptor {
        &self.descriptor
    }

    /// Returns the exact schema identity.
    #[must_use]
    pub const fn schema_id(&self) -> SchemaId {
        self.schema_id
    }

    /// Returns the normalized structural identity.
    #[must_use]
    pub const fn structural_id(&self) -> StructuralId {
        self.structural_id
    }

    /// Returns the admitted explicit mode.
    #[must_use]
    pub const fn mode(&self) -> &IdentityMode {
        self.descriptor.mode()
    }
}

/// Admits one complete descriptor and derives deterministic identities.
///
/// # Errors
///
/// Returns an error for a malformed, cyclic, unreachable, duplicate, oversized,
/// or unframeable descriptor.
pub fn admit_schema(
    descriptor: SchemaDescriptor,
    limits: AdmissionLimits,
) -> Result<AdmittedSchema, SchemaIdentityError> {
    validate_count(descriptor.nodes(), limits)?;
    let nodes = index_nodes(descriptor.nodes())?;
    debug_assert_eq!(nodes.len(), descriptor.nodes().len());
    validate_root(descriptor.root_node(), &nodes)?;
    validate_nodes(descriptor.nodes(), &nodes, limits)?;
    validate_reachability(descriptor.root_node(), &nodes)?;
    debug_assert!(nodes.contains_key(&descriptor.root_node()));
    let digests = derive_node_digests(descriptor.nodes())?;
    debug_assert_eq!(digests.len(), nodes.len());
    let root_digest =
        digests
            .get(&descriptor.root_node())
            .ok_or(SchemaIdentityError::MissingNode {
                node_id: descriptor.root_node(),
            })?;
    let exact = framing::exact_digest(descriptor.mode(), descriptor.openness(), root_digest)?;
    Ok(AdmittedSchema {
        descriptor,
        schema_id: SchemaId(exact),
        structural_id: StructuralId(*root_digest),
    })
}

fn validate_count(
    nodes: &[SchemaNode],
    limits: AdmissionLimits,
) -> Result<(), SchemaIdentityError> {
    if nodes.is_empty() {
        return Err(SchemaIdentityError::EmptyDescriptor);
    }
    let actual =
        u32::try_from(nodes.len()).map_err(|_| SchemaIdentityError::NodeLimitExceeded {
            actual: u32::MAX,
            maximum: limits.maximum_nodes(),
        })?;
    if actual > limits.maximum_nodes() {
        return Err(SchemaIdentityError::NodeLimitExceeded {
            actual,
            maximum: limits.maximum_nodes(),
        });
    }
    Ok(())
}

fn index_nodes(nodes: &[SchemaNode]) -> Result<BTreeMap<NodeId, &SchemaNode>, SchemaIdentityError> {
    let mut indexed = BTreeMap::new();
    for node in nodes {
        if indexed.insert(node.node_id(), node).is_some() {
            return Err(SchemaIdentityError::DuplicateNodeId {
                node_id: node.node_id(),
            });
        }
    }
    Ok(indexed)
}

fn validate_root(
    root: NodeId,
    nodes: &BTreeMap<NodeId, &SchemaNode>,
) -> Result<(), SchemaIdentityError> {
    if nodes.contains_key(&root) {
        Ok(())
    } else {
        Err(SchemaIdentityError::MissingNode { node_id: root })
    }
}

fn validate_nodes(
    nodes: &[SchemaNode],
    index: &BTreeMap<NodeId, &SchemaNode>,
    limits: AdmissionLimits,
) -> Result<(), SchemaIdentityError> {
    debug_assert_eq!(nodes.len(), index.len());
    debug_assert!(limits.maximum_members() > 0);
    let mut member_count = 0_u32;
    for node in nodes {
        let members = member_ids(node.kind());
        member_count = member_count
            .checked_add(u32::try_from(members.len()).map_err(|_| {
                SchemaIdentityError::MemberLimitExceeded {
                    actual: u32::MAX,
                    maximum: limits.maximum_members(),
                }
            })?)
            .ok_or(SchemaIdentityError::MemberLimitExceeded {
                actual: u32::MAX,
                maximum: limits.maximum_members(),
            })?;
        if member_count > limits.maximum_members() {
            return Err(SchemaIdentityError::MemberLimitExceeded {
                actual: member_count,
                maximum: limits.maximum_members(),
            });
        }
        validate_unique_members(&members)?;
        for child in child_ids(node.kind()) {
            if !index.contains_key(&child) {
                return Err(SchemaIdentityError::MissingNode { node_id: child });
            }
        }
    }
    Ok(())
}

fn member_ids(kind: &SchemaNodeKind) -> Vec<&crate::MemberId> {
    match kind {
        SchemaNodeKind::Record { members } => {
            members.iter().map(crate::RecordMember::member_id).collect()
        }
        SchemaNodeKind::Variant { members } => members
            .iter()
            .map(crate::VariantMember::member_id)
            .collect(),
        SchemaNodeKind::OperationSet { members } => members
            .iter()
            .map(crate::OperationMember::member_id)
            .collect(),
        SchemaNodeKind::Scalar(_)
        | SchemaNodeKind::Optional { .. }
        | SchemaNodeKind::Sequence { .. }
        | SchemaNodeKind::Map { .. }
        | SchemaNodeKind::SchemaReference { .. } => Vec::new(),
    }
}

fn validate_unique_members(members: &[&crate::MemberId]) -> Result<(), SchemaIdentityError> {
    let mut sorted = members.to_vec();
    sorted.sort_unstable();
    for pair in sorted.windows(2) {
        if pair[0] == pair[1] {
            return Err(SchemaIdentityError::DuplicateMemberId {
                member_id: pair[0].as_str().into(),
            });
        }
    }
    Ok(())
}

fn child_ids(kind: &SchemaNodeKind) -> Vec<NodeId> {
    match kind {
        SchemaNodeKind::Scalar(_) | SchemaNodeKind::SchemaReference { .. } => Vec::new(),
        SchemaNodeKind::Optional { value } => vec![*value],
        SchemaNodeKind::Sequence { element } => vec![*element],
        SchemaNodeKind::Map { key, value } => vec![*key, *value],
        SchemaNodeKind::Record { members } => members
            .iter()
            .map(crate::RecordMember::value_node)
            .collect(),
        SchemaNodeKind::Variant { members } => members
            .iter()
            .filter_map(crate::VariantMember::payload_node)
            .collect(),
        SchemaNodeKind::OperationSet { members } => members
            .iter()
            .flat_map(|member| [member.request_node(), member.response_node()])
            .collect(),
    }
}

fn validate_reachability(
    root: NodeId,
    nodes: &BTreeMap<NodeId, &SchemaNode>,
) -> Result<(), SchemaIdentityError> {
    debug_assert!(!nodes.is_empty());
    debug_assert!(nodes.contains_key(&root));
    let mut pending = Vec::with_capacity(nodes.len());
    let mut scheduled = BTreeSet::new();
    let mut visited = BTreeSet::new();
    pending.push(root);
    let is_root_new = scheduled.insert(root);
    debug_assert!(is_root_new);
    for position in 0..nodes.len() {
        let Some(node_id) = pending.get(position).copied() else {
            break;
        };
        if !visited.insert(node_id) {
            continue;
        }
        let node = nodes
            .get(&node_id)
            .ok_or(SchemaIdentityError::MissingNode { node_id })?;
        for child in child_ids(node.kind()) {
            if !scheduled.contains(&child) {
                if scheduled.len() >= nodes.len() {
                    return Err(SchemaIdentityError::CyclicDescriptor);
                }
                let is_child_new = scheduled.insert(child);
                debug_assert!(is_child_new);
                pending.push(child);
            }
        }
    }
    debug_assert!(pending.len() <= nodes.len());
    debug_assert!(visited.len() <= nodes.len());
    for node_id in nodes.keys() {
        if !visited.contains(node_id) {
            return Err(SchemaIdentityError::UnreachableNode { node_id: *node_id });
        }
    }
    Ok(())
}

fn derive_node_digests(
    nodes: &[SchemaNode],
) -> Result<BTreeMap<NodeId, [u8; 32]>, SchemaIdentityError> {
    debug_assert!(!nodes.is_empty());
    let mut digests = BTreeMap::new();
    for _round in 0..nodes.len() {
        let before = digests.len();
        debug_assert!(before <= nodes.len());
        for node in nodes {
            if digests.contains_key(&node.node_id()) {
                continue;
            }
            if let Some(digest) = framing::node_digest(node.kind(), &digests)? {
                if digests.len() >= nodes.len() {
                    return Err(SchemaIdentityError::CyclicDescriptor);
                }
                digests.insert(node.node_id(), digest);
                debug_assert!(digests.len() <= nodes.len());
            }
        }
        if digests.len() == nodes.len() {
            return Ok(digests);
        }
        if digests.len() == before {
            return Err(SchemaIdentityError::CyclicDescriptor);
        }
    }
    Err(SchemaIdentityError::CyclicDescriptor)
}
