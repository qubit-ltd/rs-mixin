/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for security key property functionality
//!

/// A trait indicating that an entity class has a security key property
///
pub trait WithSecurityKey {
    /// Gets the security key of the current object
    ///
    /// # Returns
    ///
    /// The security key of the current object
    fn security_key(&self) -> &str;

    /// Sets the security key of the current object
    ///
    /// # Parameters
    ///
    /// * `key` - The new security key to be set
    fn set_security_key(&mut self, key: &str);
}
