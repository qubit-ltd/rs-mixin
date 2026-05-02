/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for key property functionality
//!

/// A trait indicating that an entity class has a key property
///
pub trait WithKey {
    /// Gets the key of the current object
    ///
    /// # Returns
    ///
    /// The key of the current object
    fn key(&self) -> &str;

    /// Sets the key of the current object
    ///
    /// # Parameters
    ///
    /// * `key` - The new key to be set
    fn set_key(&mut self, key: &str);
}
