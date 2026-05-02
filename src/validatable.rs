/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for validation functionality
//!

/// A trait indicating that an entity class can be validated
///
/// This trait is used for validating the data of objects.
///
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
