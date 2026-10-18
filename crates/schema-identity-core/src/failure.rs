#[allow(
    non_trait_imports,
    reason = "node identity gives exact fail-closed diagnostics; owner: schema-identity-core"
)]
use crate::NodeId;
#[allow(
    non_trait_imports,
    reason = "owned diagnostics and formatter types are confined to the error contract; owner: schema-identity-core"
)]
use alloc::string::String;
use alloc::string::ToString;
#[allow(
    non_trait_imports,
    reason = "the standard formatter is the required Display adapter; owner: schema-identity-core"
)]
use core::fmt;

/// Stable failures from descriptor admission and identity framing.
#[doc = "`schema_identity_sealed`"]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SchemaIdentityError {
    /// A bounded identifier or metadata value was empty.
    EmptyText {
        /// Rejected field class.
        field: &'static str,
    },
    /// A bounded identifier or metadata value exceeded its limit.
    TextTooLong {
        /// Rejected field class.
        field: &'static str,
        /// Observed UTF-8 byte count.
        actual_bytes: u32,
        /// Admitted UTF-8 byte count.
        maximum_bytes: u16,
    },
    /// A caller supplied a zero resource limit.
    ZeroLimit {
        /// Rejected limit class.
        field: &'static str,
    },
    /// A descriptor supplied no nodes.
    EmptyDescriptor,
    /// A descriptor exceeded its node limit.
    NodeLimitExceeded {
        /// Observed node count.
        actual: u32,
        /// Admitted node count.
        maximum: u32,
    },
    /// A descriptor exceeded its total member limit.
    MemberLimitExceeded {
        /// Observed member count.
        actual: u32,
        /// Admitted member count.
        maximum: u32,
    },
    /// Two nodes reused one node identifier.
    DuplicateNodeId {
        /// Reused node identity.
        node_id: NodeId,
    },
    /// One member scope reused a stable member identifier.
    DuplicateMemberId {
        /// Reused member identity.
        member_id: String,
    },
    /// The root or a child referred to a missing node.
    MissingNode {
        /// Missing node identity.
        node_id: NodeId,
    },
    /// A descriptor included identity-bearing nodes outside the root graph.
    UnreachableNode {
        /// Unreachable node identity.
        node_id: NodeId,
    },
    /// The descriptor graph contained a cycle.
    CyclicDescriptor,
    /// The pinned content-framing dependency rejected an input.
    Framing {
        /// Stable framing diagnostic.
        message: String,
    },
}

impl fmt::Display for SchemaIdentityError {
    #[allow(
        missing_const_fn,
        mutating_input_in_pure,
        reason = "the standard Display contract mutates its formatter and cannot be const; owner: schema-identity-core"
    )]
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyText { field } => write!(formatter, "{field} must not be empty"),
            Self::TextTooLong {
                field,
                actual_bytes,
                maximum_bytes,
            } => write!(
                formatter,
                "{field} has {actual_bytes} bytes and exceeds limit {maximum_bytes}"
            ),
            Self::ZeroLimit { field } => write!(formatter, "{field} must be greater than zero"),
            Self::EmptyDescriptor => {
                formatter.write_str("schema descriptor must contain at least one node")
            }
            Self::NodeLimitExceeded { actual, maximum } => {
                write!(
                    formatter,
                    "schema descriptor has {actual} nodes and exceeds limit {maximum}"
                )
            }
            Self::MemberLimitExceeded { actual, maximum } => {
                write!(
                    formatter,
                    "schema descriptor has {actual} members and exceeds limit {maximum}"
                )
            }
            Self::DuplicateNodeId { node_id } => {
                write!(formatter, "duplicate node id {}", node_id.value())
            }
            Self::DuplicateMemberId { member_id } => {
                write!(formatter, "duplicate member id {member_id}")
            }
            Self::MissingNode { node_id } => {
                write!(formatter, "missing node id {}", node_id.value())
            }
            Self::UnreachableNode { node_id } => {
                write!(formatter, "unreachable node id {}", node_id.value())
            }
            Self::CyclicDescriptor => {
                formatter.write_str("schema descriptor graph must be acyclic")
            }
            Self::Framing { message } => {
                write!(formatter, "content identity framing failed: {message}")
            }
        }
    }
}

impl core::error::Error for SchemaIdentityError {}

impl From<content_identity_core::IdentityError> for SchemaIdentityError {
    fn from(error: content_identity_core::IdentityError) -> Self {
        Self::Framing {
            message: error.to_string(),
        }
    }
}
