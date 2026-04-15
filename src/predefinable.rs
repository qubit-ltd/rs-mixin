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

//! Traits for predefinition identification functionality
//!
//! # Author
//!
//! Haixing Hu

/// A trait indicating that an entity class has a predefinition
/// identification property
///
/// This trait is used to identify whether an object is a system-predefined
/// object.
///
/// # Author
///
/// Haixing Hu
pub trait Predefinable {
    /// Determines whether this object is a predefined object
    ///
    /// # Returns
    ///
    /// `true` if this object is predefined; otherwise `false`
    fn is_predefined(&self) -> bool;

    /// Sets whether this object is a predefined object
    ///
    /// # Parameters
    ///
    /// * `predefined` - `true` indicates that this object is predefined,
    ///   `false` indicates that it is not
    fn set_predefined(&mut self, predefined: bool);
}
