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

use chrono::{
    Datelike,
    NaiveDate,
};

/// Default legal adult age used by [`WithBirthday::is_adult_on`]
pub const DEFAULT_ADULT_AGE: i32 = 18;

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

    /// Calculates the full age in years on a reference date
    ///
    /// # Parameters
    ///
    /// * `date` - The reference date used for calculating the age
    ///
    /// # Returns
    ///
    /// `Some(years)` if the birthday is set; otherwise `None`.
    fn age_on(&self, date: NaiveDate) -> Option<i32> {
        self.birthday().map(|birthday| {
            let mut years = date.year() - birthday.year();
            if (date.month(), date.day()) < (birthday.month(), birthday.day()) {
                years -= 1;
            }
            years
        })
    }

    /// Reports whether this object is adult on a reference date
    ///
    /// # Parameters
    ///
    /// * `date` - The reference date used for calculating the age
    ///
    /// # Returns
    ///
    /// `Some(true)` if the calculated age is at least
    /// [`DEFAULT_ADULT_AGE`], `Some(false)` if it is lower, or `None` if the
    /// birthday is not set.
    #[inline(always)]
    fn is_adult_on(&self, date: NaiveDate) -> Option<bool> {
        self.is_adult_on_with_threshold(date, DEFAULT_ADULT_AGE)
    }

    /// Reports whether this object reaches a custom adult-age threshold
    ///
    /// # Parameters
    ///
    /// * `date` - The reference date used for calculating the age
    /// * `adult_age` - The inclusive full-year threshold
    ///
    /// # Returns
    ///
    /// `Some(true)` if the calculated age is at least `adult_age`,
    /// `Some(false)` if it is lower, or `None` if the birthday is not set.
    #[inline(always)]
    fn is_adult_on_with_threshold(&self, date: NaiveDate, adult_age: i32) -> Option<bool> {
        self.age_on(date).map(|years| years >= adult_age)
    }
}
