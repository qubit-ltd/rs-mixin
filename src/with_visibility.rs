/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for visibility property functionality
//!

/// A trait indicating that an entity class has a visibility property
///
/// # Type Parameters
///
/// * `V` - The type of the visibility, usually an enum type
///
pub trait WithVisibility<V>
where
    V: Copy,
{
    /// Gets the visibility of the current object
    ///
    /// # Returns
    ///
    /// The visibility of the current object
    fn visibility(&self) -> V;

    /// Sets the visibility of the current object
    ///
    /// # Parameters
    ///
    /// * `visibility` - The new visibility to be set
    fn set_visibility(&mut self, visibility: V);
}
