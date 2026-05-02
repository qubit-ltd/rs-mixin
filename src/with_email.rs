/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for email address property functionality
//!

/// A trait indicating that an entity class has an email address property
///
/// This trait provides access and setting functionality for the email
/// address of domain objects.
///
/// # Example
///
/// ```rust
/// use qubit_mixin::WithEmail;
///
/// struct Contact {
///     name: String,
///     email: String,
/// }
///
/// impl WithEmail for Contact {
///     fn email(&self) -> &str {
///         &self.email
///     }
///
///     fn set_email(&mut self, email: &str) {
///         self.email = email.to_string();
///     }
/// }
///
/// let mut contact = Contact {
///     name: "Alice".to_string(),
///     email: "alice@example.com".to_string(),
/// };
/// assert_eq!(contact.email(), "alice@example.com");
///
/// contact.set_email("alice.new@example.com");
/// assert_eq!(contact.email(), "alice.new@example.com");
/// ```
///
pub trait WithEmail {
    /// Gets the email address of the current object
    ///
    /// # Returns
    ///
    /// The email address of the current object
    fn email(&self) -> &str;

    /// Sets the email address of the current object
    ///
    /// # Parameters
    ///
    /// * `email` - The new email address to be set
    fn set_email(&mut self, email: &str);
}
