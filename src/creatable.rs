/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for creation time recording functionality
//!

use chrono::{
    DateTime,
    Utc,
};

/// A trait indicating that an entity class records the creation time
///
/// This trait provides access and setting functionality for the creation
/// time of domain objects, used to record the timestamp when an object
/// was first created.
///
/// # Example
///
/// ```rust
/// use chrono::{DateTime, Utc};
/// use qubit_mixin::Creatable;
///
/// struct Article {
///     title: String,
///     create_time: DateTime<Utc>,
/// }
///
/// impl Creatable for Article {
///     fn create_time(&self) -> DateTime<Utc> {
///         self.create_time
///     }
///
///     fn set_create_time(&mut self, time: DateTime<Utc>) {
///         self.create_time = time;
///     }
/// }
///
/// let now = Utc::now();
/// let mut article = Article {
///     title: "Hello".to_string(),
///     create_time: now,
///  };
/// assert_eq!(article.create_time(), now);
/// ```
///
pub trait Creatable {
    /// Gets the creation time of the current object
    ///
    /// # Returns
    ///
    /// The creation time of the current object
    fn create_time(&self) -> DateTime<Utc>;

    /// Sets the creation time of the current object
    ///
    /// # Parameters
    ///
    /// * `time` - The new creation time to be set
    fn set_create_time(&mut self, time: DateTime<Utc>);
}
