/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for device unique identifier property functionality
//!

/// A trait indicating that an entity class has a device unique identifier
/// (UDID) property
///
pub trait WithUdid {
    /// Gets the device unique identifier of the current object
    ///
    /// # Returns
    ///
    /// The device unique identifier of the current object
    fn udid(&self) -> &str;

    /// Sets the device unique identifier of the current object
    ///
    /// # Parameters
    ///
    /// * `udid` - The new device unique identifier to be set
    fn set_udid(&mut self, udid: &str);
}
