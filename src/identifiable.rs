/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for unique identifier functionality
//!

/// A trait indicating that an entity class has an ID property
///
/// This trait provides access and setting functionality for the unique
/// identifier (ID) of domain objects.
///
/// # Example
///
/// ```rust
/// use qubit_mixin::Identifiable;
///
/// struct User {
///     id: Option<i64>,
///     name: String,
/// }
///
/// impl Identifiable for User {
///     fn id(&self) -> Option<i64> {
///         self.id
///     }
///
///     fn set_id(&mut self, id: Option<i64>) {
///         self.id = id;
///     }
/// }
///
/// let mut user = User {
///     id: None,
///     name: "Alice".to_string(),
/// };
/// user.set_id(Some(1));
/// assert_eq!(user.id(), Some(1));
/// ```
///
pub trait Identifiable {
    /// Gets the unique identifier of the current object
    ///
    /// # Returns
    ///
    /// The unique identifier of the current object, or `None` if it has
    /// not been set yet
    fn id(&self) -> Option<i64>;

    /// Sets the unique identifier of the current object
    ///
    /// # Parameters
    ///
    /// * `id` - The new unique identifier to be set, `None` indicates
    ///   clearing the identifier
    fn set_id(&mut self, id: Option<i64>);
}
