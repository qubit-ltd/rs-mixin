/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for entity-associated basic information retrieval functionality
//!

use crate::{
    HasInfo,
    HasSpecificInfo,
    InfoWithEntity,
    WithEntity,
};

/// A trait indicating that an entity class has basic information with
/// entity association
///
/// # Type Parameters
///
/// * `E` - The type of the associated entity
///
pub trait HasInfoWithEntity<E>:
    HasInfo + WithEntity<E> + HasSpecificInfo<InfoWithEntity<E>>
where
    E: Clone,
{
    // This trait serves only as a marker trait, combining HasInfo,
    // WithEntity, and HasSpecificInfo
}
