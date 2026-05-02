/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for emptiness checking functionality
//!

/// A trait indicating that an object has an `is_empty()` method
///
/// This trait is used to determine whether an object is empty in the
/// business logic sense.
///
pub trait Emptyful {
    /// Determines whether this object is empty in the business logic sense
    ///
    /// # Returns
    ///
    /// `true` if this object is empty; otherwise `false`
    fn is_empty(&self) -> bool;
}
