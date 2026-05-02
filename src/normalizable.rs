/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for normalization functionality
//!

/// A trait indicating that an entity class can be normalized
///
/// This trait is used for normalizing objects, such as trimming whitespace
/// from strings, unifying case, etc.
///
pub trait Normalizable {
    /// Normalizes this entity
    fn normalize(&mut self);
}
