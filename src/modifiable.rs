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

//! Traits for last modification time recording functionality
//!

use chrono::{DateTime, Utc};

/// A trait indicating that an entity class records the last modification
/// time
///
/// This trait provides access and setting functionality for the last
/// modification time of domain objects, used to record the timestamp when
/// an object was most recently modified.
///
/// # Example
///
/// ```rust
/// use chrono::{DateTime, Utc};
/// use qubit_mixin::Modifiable;
///
/// struct Document {
///     content: String,
///     modify_time: Option<DateTime<Utc>>,
/// }
///
/// impl Modifiable for Document {
///     fn modify_time(&self) -> Option<DateTime<Utc>> {
///         self.modify_time
///     }
///
///     fn set_modify_time(&mut self, time: Option<DateTime<Utc>>) {
///         self.modify_time = time;
///     }
/// }
///
/// let mut doc = Document {
///     content: "Initial".to_string(),
///     modify_time: None,
/// };
/// assert_eq!(doc.modify_time(), None);
///
/// let now = Utc::now();
/// doc.set_modify_time(Some(now));
/// assert_eq!(doc.modify_time(), Some(now));
/// ```
///
pub trait Modifiable {
    /// Gets the last modification time of the current object
    ///
    /// # Returns
    ///
    /// The last modification time of the current object, or `None` if the
    /// object has not been modified
    fn modify_time(&self) -> Option<DateTime<Utc>>;

    /// Sets the last modification time of the current object
    ///
    /// # Parameters
    ///
    /// * `time` - The new last modification time to be set, `None`
    ///   indicates that the object has not been modified
    fn set_modify_time(&mut self, time: Option<DateTime<Utc>>);
}
