// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Onix Research
// Earlier source grants remain effective; see NOTICE.

#![no_std]
#![forbid(unsafe_code)]
#![allow(
    unknown_lints,
    reason = "Octet Dylint names load only during the pinned cargo-octet gate; owner: schema-identity-core"
)]
#![allow(
    clippy::useless_attribute,
    reason = "Octet allow attributes become active only under the pinned Dylint driver; owner: schema-identity-core"
)]

//! Product-neutral schema identity and compatibility decisions.
//!
//! The crate is a deterministic functional core. Callers retain registries,
//! policy loading, clocks, migration execution, persistence, and authority.

extern crate alloc;

mod compatibility;
mod failure;
mod framing;
mod identity;
mod model;
mod validation;
pub use compatibility::AliasBinding;
pub use compatibility::AliasObservation;
pub use compatibility::CompatibilityClass;
pub use compatibility::CompatibilityContext;
pub use compatibility::CompatibilityInput;
pub use compatibility::CompatibilityMechanisms;
pub use compatibility::CompatibilityReport;
pub use compatibility::MigrationObservation;
pub use compatibility::MigrationReference;
pub use compatibility::PolicyObservation;
pub use compatibility::ReasonClass;
pub use compatibility::classify_compatibility;
pub use failure::SchemaIdentityError;
pub use identity::AliasScopeId;
pub use identity::BrandId;
pub use identity::DisplayName;
pub use identity::MemberId;
pub use identity::MigrationArtifactId;
pub use identity::MigrationReferenceId;
pub use identity::NodeId;
pub use identity::NominalScope;
pub use identity::PolicyProfileId;
pub use identity::SchemaId;
pub use identity::StructuralId;
pub use identity::TextLimit;
pub use identity::TypeLineageId;
pub use model::IdentityMode;
pub use model::Openness;
pub use model::OperationMember;
pub use model::RecordMember;
pub use model::ScalarKind;
pub use model::SchemaDescriptor;
pub use model::SchemaNode;
pub use model::SchemaNodeKind;
pub use model::VariantMember;
pub use validation::AdmissionLimits;
pub use validation::AdmittedSchema;
pub use validation::MemberLimit;
pub use validation::NodeLimit;
pub use validation::admit_schema;
