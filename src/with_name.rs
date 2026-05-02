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
