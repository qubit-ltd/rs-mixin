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

//! Traits for emptiness checking functionality
//!

/// A trait indicating that an object has an `is_empty()` method
///
/// This trait is used to determine whether an object is empty in the
/// business logic sense.
///
pub trait Emptyful {
    /// Determines whether this object is empty in the business logic sense
    ///
    /// # Returns
    ///
    /// `true` if this object is empty; otherwise `false`
    fn is_empty(&self) -> bool;
}
