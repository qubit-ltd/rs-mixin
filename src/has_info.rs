/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for basic information retrieval functionality
//!

use crate::{HasSpecificInfo, Identifiable, Info, WithCode, WithName};

/// A trait indicating that an entity class has basic information
///
/// This trait combines the `Identifiable`, `WithCode`, `WithName`, and
/// `HasSpecificInfo<Info>` traits.
///
pub trait HasInfo: Identifiable + WithCode + WithName + HasSpecificInfo<Info> {
    // This trait serves only as a marker trait, combining multiple basic
    // traits
}
