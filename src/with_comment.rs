/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for comment property functionality
//!

/// A trait indicating that an entity class has a comment property
///
/// This trait provides access and setting functionality for the comment
/// information of domain objects.
///
/// # Example
///
/// ```rust
/// use qubit_mixin::WithComment;
///
/// struct Task {
///     title: String,
///     comment: String,
/// }
///
/// impl WithComment for Task {
///     fn comment(&self) -> &str {
///         &self.comment
///     }
///
///     fn set_comment(&mut self, comment: &str) {
///         self.comment = comment.to_string();
///     }
/// }
///
/// let mut task = Task {
///     title: "Review PR".to_string(),
///     comment: "Check the code quality".to_string(),
/// };
/// assert_eq!(task.comment(), "Check the code quality");
///
/// task.set_comment("Focus on error handling");
/// assert_eq!(task.comment(), "Focus on error handling");
/// ```
///
pub trait WithComment {
    /// Gets the comment of the current object
    ///
    /// # Returns
    ///
    /// The comment of the current object
    fn comment(&self) -> &str;

    /// Sets the comment of the current object
    ///
    /// # Parameters
    ///
    /// * `comment` - The new comment to be set
    fn set_comment(&mut self, comment: &str);
}
