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

//! Traits for associated entity property functionality
//!
//! # Author
//!
//! Haixing Hu

/// A trait indicating that an entity class has an associated entity
/// property
///
/// # Type Parameters
///
/// * `E` - The type of the associated entity
///
/// # Author
///
/// Haixing Hu
pub trait WithEntity<E>
where
    E: Clone,
{
    /// Gets the entity associated with the current object
    ///
    /// # Returns
    ///
    /// The entity associated with the current object, or `None` if no
    /// entity is associated
    fn entity(&self) -> Option<&E>;

    /// Sets the entity associated with the current object
    ///
    /// # Parameters
    ///
    /// * `entity` - The associated entity to be set, `None` indicates
    ///   clearing the association
    fn set_entity(&mut self, entity: Option<E>);
}
