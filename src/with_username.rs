/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for username property functionality
//!

/// A trait indicating that an entity class has a username property
///
/// This trait provides access and setting functionality for the username
/// of domain objects.
///
pub trait WithUsername {
    /// Gets the username of the current object
    ///
    /// # Returns
    ///
    /// The username of the current object
    fn username(&self) -> &str;

    /// Sets the username of the current object
    ///
    /// # Parameters
    ///
    /// * `username` - The new username to be set
    fn set_username(&mut self, username: &str);
}
