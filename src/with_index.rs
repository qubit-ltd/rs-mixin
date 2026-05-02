/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for index property functionality
//!

/// A trait indicating that an entity class has an index property
///
pub trait WithIndex {
    /// Gets the index of the current object
    ///
    /// # Returns
    ///
    /// The index of the current object
    fn index(&self) -> i32;

    /// Sets the index of the current object
    ///
    /// # Parameters
    ///
    /// * `index` - The new index to be set
    fn set_index(&mut self, index: i32);
}
