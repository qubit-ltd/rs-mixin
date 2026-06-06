// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Traits for desensitization functionality

/// A trait indicating that an entity class can be desensitized
///
/// This trait is used for desensitizing sensitive information.
pub trait Desensitizable {
    /// Desensitizes the sensitive information of this entity
    fn desensitize(&mut self);

    /// Returns a desensitized clone of this entity
    ///
    /// # Returns
    ///
    /// A cloned value after applying [`desensitize`](Self::desensitize) to
    /// the clone. The original value is not modified.
    fn desensitized(&self) -> Self
    where
        Self: Clone + Sized,
    {
        let mut result = self.clone();
        result.desensitize();
        result
    }
}
