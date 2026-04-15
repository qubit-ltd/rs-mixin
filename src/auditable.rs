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

//! Traits for audit information recording functionality
//!
//! # Author
//!
//! Haixing Hu

use crate::{Creatable, Deletable, Modifiable};

/// A trait indicating that an entity class has auditable properties
///
/// This trait combines the creation time, modification time, and deletion
/// time recording functionalities for complete audit tracking.
///
/// # Features
///
/// - Record creation time
/// - Record last modification time
/// - Record mark deletion time
///
/// # Example
///
/// ```rust
/// use chrono::{DateTime, Utc};
/// use qubit_mixin::{Auditable, Creatable, Modifiable, Deletable};
///
/// struct Order {
///     id: i64,
///     create_time: DateTime<Utc>,
///     modify_time: Option<DateTime<Utc>>,
///     delete_time: Option<DateTime<Utc>>,
/// }
///
/// impl Creatable for Order {
///     fn create_time(&self) -> DateTime<Utc> {
///         self.create_time
///     }
///
///     fn set_create_time(&mut self, time: DateTime<Utc>) {
///         self.create_time = time;
///     }
/// }
///
/// impl Modifiable for Order {
///     fn modify_time(&self) -> Option<DateTime<Utc>> {
///         self.modify_time
///     }
///
///     fn set_modify_time(&mut self, time: Option<DateTime<Utc>>) {
///         self.modify_time = time;
///     }
/// }
///
/// impl Deletable for Order {
///     fn delete_time(&self) -> Option<DateTime<Utc>> {
///         self.delete_time
///     }
///
///     fn set_delete_time(&mut self, time: Option<DateTime<Utc>>) {
///         self.delete_time = time;
///     }
/// }
///
/// impl Auditable for Order {}
///
/// let order = Order {
///     id: 1,
///     create_time: Utc::now(),
///     modify_time: None,
///     delete_time: None,
/// };
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait Auditable: Creatable + Modifiable + Deletable {
    // This trait serves only as a marker trait, combining Creatable,
    // Modifiable, and Deletable
}
