////////////////////////////////////////////////////////////////////////////////
//
//  Copyright (C) 2025 Haixing Hu
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program. If not, see <https://www.gnu.org/licenses/>.
//
////////////////////////////////////////////////////////////////////////////////

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
