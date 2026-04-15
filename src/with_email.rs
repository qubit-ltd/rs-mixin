////////////////////////////////////////////////////////////////////////////////
//
//  Copyright (C) 2025 Haixing Hu
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program. If not, see <https://www.gnu.org/licenses/>.
//
////////////////////////////////////////////////////////////////////////////////

//! Traits for email address property functionality
//!
//! # Author
//!
//! Haixing Hu

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
/// # Author
///
/// Haixing Hu
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
