/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for password property functionality
//!

/// A trait indicating that an entity class has a password property
///
pub trait WithPassword {
    /// Gets the password of the current object
    ///
    /// # Returns
    ///
    /// The password of the current object
    fn password(&self) -> &str;

    /// Sets the password of the current object
    ///
    /// # Parameters
    ///
    /// * `password` - The new password to be set
    fn set_password(&mut self, password: &str);
}
