////////////////////////////////////////////////////////////////////////////////
//
//  Copyright (C) 2025 Haixing Hu
//
//  This program is free software: you cantml redistribute it and/or modify
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

//! Traits for validation functionality
//!
//! # Author
//!
//! Haixing Hu

/// A trait indicating that an entity class can be validated
///
/// This trait is used for validating the data of objects.
///
/// # Author
///
/// Haixing Hu
pub trait Validatable {
    /// Validation error type
    type Error;

    /// Validates whether the data of this entity is valid
    ///
    /// # Returns
    ///
    /// `Ok(())` if the data is valid; otherwise returns `Err` containing
    /// the validation error
    fn validate(&self) -> Result<(), Self::Error>;
}
