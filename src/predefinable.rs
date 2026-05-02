/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for predefinition identification functionality
//!

/// A trait indicating that an entity class has a predefinition
/// identification property
///
/// This trait is used to identify whether an object is a system-predefined
/// object.
///
pub trait Predefinable {
    /// Determines whether this object is a predefined object
    ///
    /// # Returns
    ///
    /// `true` if this object is predefined; otherwise `false`
    fn is_predefined(&self) -> bool;

    /// Sets whether this object is a predefined object
    ///
    /// # Parameters
    ///
    /// * `predefined` - `true` indicates that this object is predefined,
    ///   `false` indicates that it is not
    fn set_predefined(&mut self, predefined: bool);
}
