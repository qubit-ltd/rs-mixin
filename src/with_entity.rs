/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Traits for entity discriminator functionality
//!

/// A trait indicating that an entity class has an owning entity name
/// property
///
/// The entity value is a string discriminator, typically used to indicate
/// which domain entity a shared record belongs to.
pub trait WithEntity {
    /// Gets the owning entity name of the current object
    ///
    /// # Returns
    ///
    /// The entity name associated with the current object, or `None` if no
    /// entity is associated.
    fn entity(&self) -> Option<&str>;

    /// Sets the owning entity name of the current object
    ///
    /// # Parameters
    ///
    /// * `entity` - The entity name to be set, `None` indicates clearing
    ///   the association.
    fn set_entity(&mut self, entity: Option<&str>);
}
