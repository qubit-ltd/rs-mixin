// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Traits for entity-discriminated basic information retrieval functionality

use crate::{
    HasInfo,
    HasSpecificInfo,
    InfoWithEntity,
    WithEntity,
};

/// A trait indicating that an entity class has basic information with an
/// entity discriminator
pub trait HasInfoWithEntity:
    HasInfo + WithEntity + HasSpecificInfo<InfoWithEntity>
{
    // This trait serves only as a marker trait, combining HasInfo,
    // WithEntity, and HasSpecificInfo
}
