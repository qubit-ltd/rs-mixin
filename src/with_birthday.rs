/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for birthday property functionality
//!

use chrono::NaiveDate;

/// A trait indicating that an entity class has a birthday property
///
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
