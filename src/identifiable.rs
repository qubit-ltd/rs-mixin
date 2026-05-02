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
