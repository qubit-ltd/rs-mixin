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

//! Basic information structure with entity association
//!

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{Deletable, Identifiable, Info, WithCode, WithEntity, WithName};

/// Represents the basic information of a deletable object with entity
/// association
///
/// # Type Parameters
///
/// * `E` - The type of the associated entity
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InfoWithEntity<E>
where
    E: Clone,
{
    /// Basic information
    #[serde(flatten)]
    pub info: Info,

    /// Associated entity
    pub entity: Option<E>,
}

impl<E> InfoWithEntity<E>
where
    E: Clone,
{
    /// Creates a new `InfoWithEntity` object
    ///
    /// # Parameters
    ///
    /// * `id` - Unique identifier
    /// * `code` - Code
    /// * `name` - Name
    /// * `delete_time` - Mark deletion time
    /// * `entity` - Associated entity
    ///
    /// # Returns
    ///
    /// The newly created `InfoWithEntity` object
    pub fn new(
        id: Option<i64>,
        code: String,
        name: String,
        delete_time: Option<DateTime<Utc>>,
        entity: Option<E>,
    ) -> Self {
        Self {
            info: Info::new(id, code, name, delete_time),
            entity,
        }
    }
}

impl<E> Default for InfoWithEntity<E>
where
    E: Clone,
{
    fn default() -> Self {
        Self {
            info: Info::default(),
            entity: None,
        }
    }
}

impl<E> Identifiable for InfoWithEntity<E>
where
    E: Clone,
{
    fn id(&self) -> Option<i64> {
        self.info.id()
    }

    fn set_id(&mut self, id: Option<i64>) {
        self.info.set_id(id);
    }
}

impl<E> WithCode for InfoWithEntity<E>
where
    E: Clone,
{
    fn code(&self) -> &str {
        self.info.code()
    }

    fn set_code(&mut self, code: &str) {
        self.info.set_code(code);
    }
}

impl<E> WithName for InfoWithEntity<E>
where
    E: Clone,
{
    fn name(&self) -> &str {
        self.info.name()
    }

    fn set_name(&mut self, name: &str) {
        self.info.set_name(name);
    }
}

impl<E> Deletable for InfoWithEntity<E>
where
    E: Clone,
{
    fn delete_time(&self) -> Option<DateTime<Utc>> {
        self.info.delete_time()
    }

    fn set_delete_time(&mut self, time: Option<DateTime<Utc>>) {
        self.info.set_delete_time(time);
    }
}

impl<E> WithEntity<E> for InfoWithEntity<E>
where
    E: Clone,
{
    fn entity(&self) -> Option<&E> {
        self.entity.as_ref()
    }

    fn set_entity(&mut self, entity: Option<E>) {
        self.entity = entity;
    }
}
