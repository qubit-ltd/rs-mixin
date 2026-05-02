/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for associated entity property functionality
//!

/// A trait indicating that an entity class has an associated entity
/// property
///
/// # Type Parameters
///
/// * `E` - The type of the associated entity
///
pub trait WithEntity<E>
where
    E: Clone,
{
    /// Gets the entity associated with the current object
    ///
    /// # Returns
    ///
    /// The entity associated with the current object, or `None` if no
    /// entity is associated
    fn entity(&self) -> Option<&E>;

    /// Sets the entity associated with the current object
    ///
    /// # Parameters
    ///
    /// * `entity` - The associated entity to be set, `None` indicates
    ///   clearing the association
    fn set_entity(&mut self, entity: Option<E>);
}
