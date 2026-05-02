/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
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
