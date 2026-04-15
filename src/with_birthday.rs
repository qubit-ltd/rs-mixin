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

//! Traits for birthday property functionality
//!
//! # Author
//!
//! Haixing Hu

use chrono::NaiveDate;

/// A trait indicating that an entity class has a birthday property
///
/// # Author
///
/// Haixing Hu
pub trait WithBirthday {
    /// Gets the birthday of the current object
    ///
    /// # Returns
    ///
    /// The birthday of the current object, or `None` if not set
    fn birthday(&self) -> Option<NaiveDate>;

    /// Sets the birthday of the current object
    ///
    /// # Parameters
    ///
    /// * `birthday` - The new birthday to be set, `None` indicates clearing
    ///   the birthday information
    fn set_birthday(&mut self, birthday: Option<NaiveDate>);
}
