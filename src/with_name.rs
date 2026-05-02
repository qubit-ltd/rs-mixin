/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for name property functionality
//!

/// A trait indicating that an entity class has a name property
///
/// This trait provides access and setting functionality for the name of
/// domain objects.
///
/// # Example
///
/// ```rust
/// use qubit_mixin::WithName;
///
/// struct Category {
///     name: String,
///     description: String,
/// }
///
/// impl WithName for Category {
///     fn name(&self) -> &str {
///         &self.name
///     }
///
///     fn set_name(&mut self, name: &str) {
///         self.name = name.to_string();
///     }
/// }
///
/// let mut category = Category {
///     name: "Electronics".to_string(),
///     description: "Electronic devices".to_string(),
/// };
/// assert_eq!(category.name(), "Electronics");
///
/// category.set_name("Computers");
/// assert_eq!(category.name(), "Computers");
/// ```
///
pub trait WithName {
    /// Gets the name of the current object
    ///
    /// # Returns
    ///
    /// The name of the current object
    fn name(&self) -> &str;

    /// Sets the name of the current object
    ///
    /// # Parameters
    ///
    /// * `name` - The new name to be set
    fn set_name(&mut self, name: &str);
}
