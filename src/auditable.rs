/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for audit information recording functionality
//!

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
pub trait Auditable: Creatable + Modifiable + Deletable {
    // This trait serves only as a marker trait, combining Creatable,
    // Modifiable, and Deletable
}
