/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Basic information structure with an entity discriminator
//!

use chrono::{
    DateTime,
    Utc,
};
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    Deletable,
    Emptyful,
    Identifiable,
    Info,
    Normalizable,
    WithCode,
    WithEntity,
    WithName,
};

/// Represents the basic information of a deletable object with an entity
/// discriminator
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct InfoWithEntity {
    /// Basic information
    #[serde(flatten)]
    pub info: Info,

    /// Associated entity name
    pub entity: Option<String>,
}

impl InfoWithEntity {
    /// Creates a new `InfoWithEntity` object
    ///
    /// # Parameters
    ///
    /// * `id` - Unique identifier
    /// * `code` - Code
    /// * `name` - Name
    /// * `delete_time` - Mark deletion time
    /// * `entity` - Associated entity name
    ///
    /// # Returns
    ///
    /// The newly created `InfoWithEntity` object
    pub fn new(
        id: Option<i64>,
        code: String,
        name: String,
        delete_time: Option<DateTime<Utc>>,
        entity: Option<String>,
    ) -> Self {
        Self {
            info: Info::new(id, code, name, delete_time),
            entity,
        }
    }

    /// Creates a new value from an existing [`Info`] and optional entity name
    ///
    /// # Parameters
    ///
    /// * `info` - The embedded basic information
    /// * `entity` - The entity discriminator, or `None` if not associated
    ///
    /// # Returns
    ///
    /// A new `InfoWithEntity` value preserving `info` unchanged.
    pub fn from_info(info: Info, entity: Option<String>) -> Self {
        Self { info, entity }
    }

    /// Consumes this value and returns the embedded [`Info`]
    ///
    /// # Returns
    ///
    /// The embedded basic information.
    pub fn into_info(self) -> Info {
        self.info
    }

    /// Reports whether all basic information and the entity are populated
    ///
    /// # Returns
    ///
    /// `true` when [`Info::is_complete`] is `true` and the entity is
    /// present.
    pub fn is_complete(&self) -> bool {
        self.info.is_complete() && self.entity.is_some()
    }
}

impl Identifiable for InfoWithEntity {
    fn id(&self) -> Option<i64> {
        self.info.id()
    }

    fn set_id(&mut self, id: Option<i64>) {
        self.info.set_id(id);
    }
}

impl WithCode for InfoWithEntity {
    fn code(&self) -> &str {
        self.info.code()
    }

    fn set_code(&mut self, code: &str) {
        self.info.set_code(code);
    }
}

impl WithName for InfoWithEntity {
    fn name(&self) -> &str {
        self.info.name()
    }

    fn set_name(&mut self, name: &str) {
        self.info.set_name(name);
    }
}

impl Deletable for InfoWithEntity {
    fn delete_time(&self) -> Option<DateTime<Utc>> {
        self.info.delete_time()
    }

    fn set_delete_time(&mut self, time: Option<DateTime<Utc>>) {
        self.info.set_delete_time(time);
    }
}

impl WithEntity for InfoWithEntity {
    fn entity(&self) -> Option<&str> {
        self.entity.as_deref()
    }

    fn set_entity(&mut self, entity: Option<&str>) {
        self.entity = entity.map(str::to_owned);
    }
}

impl Emptyful for InfoWithEntity {
    fn is_empty(&self) -> bool {
        self.info.is_empty() && self.entity.is_none()
    }
}

impl Normalizable for InfoWithEntity {
    fn normalize(&mut self) {
        self.info.normalize();
    }
}

impl From<InfoWithEntity> for Info {
    fn from(value: InfoWithEntity) -> Self {
        value.info
    }
}

impl From<Info> for InfoWithEntity {
    fn from(info: Info) -> Self {
        Self { info, entity: None }
    }
}
