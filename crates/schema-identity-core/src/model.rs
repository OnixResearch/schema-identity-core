#[allow(
    non_trait_imports,
    reason = "nominal and member identities are explicit descriptor facts; owner: schema-identity-core"
)]
use crate::{BrandId, DisplayName, MemberId, NodeId, TypeLineageId};
#[allow(
    non_trait_imports,
    reason = "owned bounded descriptor containers are the public data model; owner: schema-identity-core"
)]
use alloc::{boxed::Box, vec::Vec};

/// Explicit schema identity mode. No mode falls back to another mode.
#[doc = "`schema_identity_sealed`"]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityMode {
    /// Compatibility can use equal normalized structure.
    Structural,
    /// Compatibility requires an exact schema, alias, or migration unless lineage and shape produce the same exact identity.
    Nominal {
        /// Durable evolution line selected by the caller.
        lineage: TypeLineageId,
    },
    /// Compatibility can use equal normalized structure only under the same brand.
    BrandedStructural {
        /// Caller-owned brand identity.
        brand: BrandId,
    },
}

/// Whether a schema shape admits undeclared members.
#[doc = "`schema_identity_sealed`"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Openness {
    /// Only declared members are admitted.
    Closed,
    /// Additional members are admitted by consumer semantics.
    Open,
}

/// Closed scalar classes used by the product-neutral projection.
#[doc = "`schema_identity_sealed`"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScalarKind {
    /// Boolean value.
    Boolean,
    /// Signed 64-bit integer.
    Signed64,
    /// Unsigned 64-bit integer.
    Unsigned64,
    /// UTF-8 text.
    Text,
    /// Byte string.
    Bytes,
    /// Product-neutral symbolic atom.
    Symbol,
}

/// One record member with a stable identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordMember {
    member_id: MemberId,
    value_node: NodeId,
    is_required: bool,
    display_name: Option<DisplayName>,
}

impl RecordMember {
    /// Creates a record member.
    #[must_use]
    pub const fn new(
        member_id: MemberId,
        value_node: NodeId,
        is_required: bool,
        display_name: Option<DisplayName>,
    ) -> Self {
        Self {
            member_id,
            value_node,
            is_required,
            display_name,
        }
    }

    /// Returns the stable member identity.
    #[must_use]
    pub const fn member_id(&self) -> &MemberId {
        &self.member_id
    }

    /// Returns the referenced value node.
    #[must_use]
    pub const fn value_node(&self) -> NodeId {
        self.value_node
    }

    /// Returns whether the member is required.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.is_required
    }

    /// Returns non-identity display metadata.
    #[must_use]
    pub const fn display_name(&self) -> Option<&DisplayName> {
        self.display_name.as_ref()
    }
}

/// One variant member with a stable identity and optional payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VariantMember {
    member_id: MemberId,
    payload_node: Option<NodeId>,
    display_name: Option<DisplayName>,
}

impl VariantMember {
    /// Creates a variant member.
    #[must_use]
    pub const fn new(
        member_id: MemberId,
        payload_node: Option<NodeId>,
        display_name: Option<DisplayName>,
    ) -> Self {
        Self {
            member_id,
            payload_node,
            display_name,
        }
    }

    /// Returns the stable member identity.
    #[must_use]
    pub const fn member_id(&self) -> &MemberId {
        &self.member_id
    }

    /// Returns the optional payload node.
    #[must_use]
    pub const fn payload_node(&self) -> Option<NodeId> {
        self.payload_node
    }

    /// Returns non-identity display metadata.
    #[must_use]
    pub const fn display_name(&self) -> Option<&DisplayName> {
        self.display_name.as_ref()
    }
}

/// One operation member with stable request and response bindings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationMember {
    member_id: MemberId,
    request_node: NodeId,
    response_node: NodeId,
    display_name: Option<DisplayName>,
}

impl OperationMember {
    /// Creates an operation member.
    #[must_use]
    pub const fn new(
        member_id: MemberId,
        request_node: NodeId,
        response_node: NodeId,
        display_name: Option<DisplayName>,
    ) -> Self {
        Self {
            member_id,
            request_node,
            response_node,
            display_name,
        }
    }

    /// Returns the stable member identity.
    #[must_use]
    pub const fn member_id(&self) -> &MemberId {
        &self.member_id
    }

    /// Returns the request node.
    #[must_use]
    pub const fn request_node(&self) -> NodeId {
        self.request_node
    }

    /// Returns the response node.
    #[must_use]
    pub const fn response_node(&self) -> NodeId {
        self.response_node
    }

    /// Returns non-identity display metadata.
    #[must_use]
    pub const fn display_name(&self) -> Option<&DisplayName> {
        self.display_name.as_ref()
    }
}

/// One node in a bounded acyclic schema graph.
#[doc = "`schema_identity_sealed`"]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SchemaNodeKind {
    /// Scalar leaf.
    Scalar(ScalarKind),
    /// Optional value.
    Optional {
        /// Referenced value node.
        value: NodeId,
    },
    /// Homogeneous sequence.
    Sequence {
        /// Referenced element node.
        element: NodeId,
    },
    /// Key-value map.
    Map {
        /// Referenced key node.
        key: NodeId,
        /// Referenced value node.
        value: NodeId,
    },
    /// Record members.
    Record {
        /// Stable record members.
        members: Vec<RecordMember>,
    },
    /// Tagged variant members.
    Variant {
        /// Stable variant members.
        members: Vec<VariantMember>,
    },
    /// Named operation signatures.
    OperationSet {
        /// Stable operation members.
        members: Vec<OperationMember>,
    },
    /// Reference to an already admitted exact schema.
    SchemaReference {
        /// Referenced exact schema identity.
        schema_id: crate::SchemaId,
    },
}

/// Descriptor node with an identity-neutral local node number.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaNode {
    node_id: NodeId,
    kind: SchemaNodeKind,
}

impl SchemaNode {
    /// Creates a schema node.
    #[must_use]
    pub const fn new(node_id: NodeId, kind: SchemaNodeKind) -> Self {
        Self { node_id, kind }
    }

    /// Returns the descriptor-local node identity.
    #[must_use]
    pub const fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Returns the node kind.
    #[must_use]
    pub const fn kind(&self) -> &SchemaNodeKind {
        &self.kind
    }
}

/// Complete schema descriptor supplied to the pure admission core.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaDescriptor {
    mode: IdentityMode,
    openness: Openness,
    root_node: NodeId,
    nodes: Box<[SchemaNode]>,
    display_name: Option<DisplayName>,
}

impl SchemaDescriptor {
    /// Creates a schema descriptor. Admission validates graph invariants.
    #[must_use]
    pub fn new(
        mode: IdentityMode,
        openness: Openness,
        root_node: NodeId,
        nodes: Vec<SchemaNode>,
        display_name: Option<DisplayName>,
    ) -> Self {
        Self {
            mode,
            openness,
            root_node,
            nodes: nodes.into_boxed_slice(),
            display_name,
        }
    }

    /// Returns the explicit identity mode.
    #[must_use]
    pub const fn mode(&self) -> &IdentityMode {
        &self.mode
    }

    /// Returns the schema openness rule.
    #[must_use]
    pub const fn openness(&self) -> Openness {
        self.openness
    }

    /// Returns the root node.
    #[must_use]
    pub const fn root_node(&self) -> NodeId {
        self.root_node
    }

    /// Returns all descriptor nodes.
    #[must_use]
    pub const fn nodes(&self) -> &[SchemaNode] {
        &self.nodes
    }

    /// Returns non-identity display metadata.
    #[must_use]
    pub const fn display_name(&self) -> Option<&DisplayName> {
        self.display_name.as_ref()
    }
}
