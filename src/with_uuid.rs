/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for universally unique identifier property functionality
//!

/// A trait indicating that an entity class has a universally unique
/// identifier (UUID) property
///
pub trait WithUuid {
    /// Gets the universally unique identifier of the current object
    ///
    /// # Returns
    ///
    /// The universally unique identifier of the current object
    fn uuid(&self) -> &str;

    /// Sets the universally unique identifier of the current object
    ///
    /// # Parameters
    ///
    /// * `uuid` - The new universally unique identifier to be set
    fn set_uuid(&mut self, uuid: &str);
}
