/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for code property functionality
//!

/// A trait indicating that an entity class has a code property
///
/// This trait provides access and setting functionality for the code of
/// domain objects, where the code is usually a globally unique identifier.
///
/// # Example
///
/// ```rust
/// use qubit_mixin::WithCode;
///
/// struct Product {
///     code: String,
///     name: String,
/// }
///
/// impl WithCode for Product {
///     fn code(&self) -> &str {
///         &self.code
///     }
///
///     fn set_code(&mut self, code: &str) {
///         self.code = code.to_string();
///     }
/// }
///
/// let mut product = Product {
///     code: "PROD001".to_string(),
///     name: "Widget".to_string(),
/// };
/// assert_eq!(product.code(), "PROD001");
///
/// product.set_code("PROD002");
/// assert_eq!(product.code(), "PROD002");
/// ```
///
pub trait WithCode {
    /// Gets the code of the current object
    ///
    /// # Returns
    ///
    /// The code of the current object
    fn code(&self) -> &str;

    /// Sets the code of the current object
    ///
    /// # Parameters
    ///
    /// * `code` - The new code to be set
    fn set_code(&mut self, code: &str);
}
