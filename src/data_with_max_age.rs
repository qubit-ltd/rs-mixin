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

//! Traits for maximum age functionality
//!

use std::time::Duration;

/// A trait indicating that data has a maximum age
///
/// This trait is used to define the maximum validity period of data.
///
pub trait DataWithMaxAge {
    /// Gets the maximum age of this data
    ///
    /// # Returns
    ///
    /// The maximum age of this data
    fn max_age(&self) -> Duration;

    /// Sets the maximum age of this data
    ///
    /// # Parameters
    ///
    /// * `age` - The maximum age to be set
    fn set_max_age(&mut self, age: Duration);
}
