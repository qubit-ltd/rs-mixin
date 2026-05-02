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

//! Traits for mark deletion time recording functionality
//!

use chrono::{DateTime, Utc};

/// A trait indicating that an entity class records the mark deletion time
///
/// This trait provides soft delete functionality for domain objects by
/// recording the deletion time instead of physically deleting the data.
///
/// # Example
///
/// ```rust
/// use chrono::{DateTime, Utc};
/// use qubit_mixin::Deletable;
///
/// struct Post {
///     title: String,
///     delete_time: Option<DateTime<Utc>>,
/// }
///
/// impl Deletable for Post {
///     fn delete_time(&self) -> Option<DateTime<Utc>> {
///         self.delete_time
///     }
///
///     fn set_delete_time(&mut self, time: Option<DateTime<Utc>>) {
///         self.delete_time = time;
///     }
/// }
///
/// let mut post = Post {
///     title: "Hello".to_string(),
///     delete_time: None,
/// };
/// assert!(!post.is_deleted());
///
/// post.set_delete_time(Some(Utc::now()));
/// assert!(post.is_deleted());
/// ```
///
pub trait Deletable {
    /// Gets the mark deletion time of the current object
    ///
    /// # Returns
    ///
    /// The mark deletion time of the current object, or `None` if the
    /// object has not been marked as deleted
    fn delete_time(&self) -> Option<DateTime<Utc>>;

    /// Sets the mark deletion time of the current object
    ///
    /// # Parameters
    ///
    /// * `time` - The new mark deletion time to be set, `None` indicates
    ///   that the object has not been marked as deleted
    fn set_delete_time(&mut self, time: Option<DateTime<Utc>>);

    /// Determines whether this object has been marked as deleted
    ///
    /// # Returns
    ///
    /// `true` if this object has been marked as deleted; otherwise `false`
    fn is_deleted(&self) -> bool {
        self.delete_time().is_some()
    }
}
