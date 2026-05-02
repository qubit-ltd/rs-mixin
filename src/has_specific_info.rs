/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for specific information retrieval functionality
//!

/// A trait indicating that an entity class has specific information
///
/// # Type Parameters
///
/// * `T` - The type of the information
///
pub trait HasSpecificInfo<T> {
    /// Gets the specific information of the current object
    ///
    /// # Returns
    ///
    /// The specific information of the current object
    fn info(&self) -> T;

    /// Sets the specific information of the current object
    ///
    /// # Parameters
    ///
    /// * `info` - The specific information to be set
    fn set_info(&mut self, info: T);
}
