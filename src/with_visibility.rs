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

//! Traits for visibility property functionality
//!
//! # Author
//!
//! Haixing Hu

/// A trait indicating that an entity class has a visibility property
///
/// # Type Parameters
///
/// * `V` - The type of the visibility, usually an enum type
///
/// # Author
///
/// Haixing Hu
pub trait WithVisibility<V>
where
    V: Copy,
{
    /// Gets the visibility of the current object
    ///
    /// # Returns
    ///
    /// The visibility of the current object
    fn visibility(&self) -> V;

    /// Sets the visibility of the current object
    ///
    /// # Parameters
    ///
    /// * `visibility` - The new visibility to be set
    fn set_visibility(&mut self, visibility: V);
}
