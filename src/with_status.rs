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

//! Traits for status property functionality
//!

use crate::Identifiable;

/// A trait indicating that an entity class has a status property
///
/// This trait provides access and setting functionality for the status of
/// domain objects, where the status is represented using a generic
/// parameter, allowing different enum types to represent the status.
///
/// # Type Parameters
///
/// * `S` - The type of the status, usually an enum type
///
/// # Example
///
/// ```rust
/// use qubit_mixin::{WithStatus, Identifiable};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// enum OrderStatus {
///     Pending,
///     Paid,
///     Shipped,
///     Completed,
///     Cancelled,
/// }
///
/// struct Order {
///     id: Option<i64>,
///     status: OrderStatus,
/// }
///
/// impl Identifiable for Order {
///     fn id(&self) -> Option<i64> {
///         self.id
///     }
///
///     fn set_id(&mut self, id: Option<i64>) {
///         self.id = id;
///     }
/// }
///
/// impl WithStatus<OrderStatus> for Order {
///     fn status(&self) -> OrderStatus {
///         self.status
///     }
///
///     fn set_status(&mut self, status: OrderStatus) {
///         self.status = status;
///     }
/// }
///
/// let mut order = Order {
///     id: Some(1),
///     status: OrderStatus::Pending,
/// };
/// assert_eq!(order.status(), OrderStatus::Pending);
///
/// order.set_status(OrderStatus::Paid);
/// assert_eq!(order.status(), OrderStatus::Paid);
/// ```
///
pub trait WithStatus<S>: Identifiable
where
    S: Copy,
{
    /// Gets the status of the current object
    ///
    /// # Returns
    ///
    /// The status of the current object
    fn status(&self) -> S;

    /// Sets the status of the current object
    ///
    /// # Parameters
    ///
    /// * `status` - The new status to be set
    fn set_status(&mut self, status: S);
}
