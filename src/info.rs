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

//! Basic information structure
//!
//! # Author
//!
//! Haixing Hu

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{Deletable, Emptyful, Identifiable, Normalizable, WithCode, WithName};

/// Represents the basic information of a deletable object
///
/// This structure records:
/// - Unique identifier (ID)
/// - Code (usually globally unique)
/// - Name
/// - Mark deletion time
///
/// # Example
///
/// ```rust
/// use qubit_mixin::Info;
///
/// let info = Info::new(
///     Some(1),
///     "CODE001".to_string(),
///     "Test".to_string(),
///     None,
/// );
/// assert_eq!(info.id, Some(1));
/// assert_eq!(info.code, "CODE001");
/// ```
///
/// # Author
///
/// Haixing Hu
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Info {
    /// Unique identifier
    pub id: Option<i64>,

    /// Code, usually globally unique
    pub code: String,

    /// Name
    pub name: String,

    /// Mark deletion time
    pub delete_time: Option<DateTime<Utc>>,
}

impl Info {
    /// Creates a new `Info` object
    ///
    /// # Parameters
    ///
    /// * `id` - Unique identifier, `None` indicates that no ID has been
    ///   assigned yet
    /// * `code` - Code
    /// * `name` - Name
    /// * `delete_time` - Mark deletion time, `None` indicates not deleted
    ///
    /// # Returns
    ///
    /// The newly created `Info` object
    pub fn new(
        id: Option<i64>,
        code: String,
        name: String,
        delete_time: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            code,
            name,
            delete_time,
        }
    }

    /// Creates an `Info` object by ID
    ///
    /// # Parameters
    ///
    /// * `id` - Object ID
    ///
    /// # Returns
    ///
    /// An `Info` object with the specified ID, other fields are default
    /// values
    pub fn of_id(id: i64) -> Self {
        Self {
            id: Some(id),
            code: String::new(),
            name: String::new(),
            delete_time: None,
        }
    }

    /// Creates an `Info` object by code
    ///
    /// # Parameters
    ///
    /// * `code` - Object code
    ///
    /// # Returns
    ///
    /// An `Info` object with the specified code, other fields are default
    /// values
    pub fn of_code(code: String) -> Self {
        Self {
            id: None,
            code,
            name: String::new(),
            delete_time: None,
        }
    }

    /// Creates an `Info` object by name
    ///
    /// # Parameters
    ///
    /// * `name` - Object name
    ///
    /// # Returns
    ///
    /// An `Info` object with the specified name, other fields are default
    /// values
    pub fn of_name(name: String) -> Self {
        Self {
            id: None,
            code: String::new(),
            name,
            delete_time: None,
        }
    }
}

impl Identifiable for Info {
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: Option<i64>) {
        self.id = id;
    }
}

impl WithCode for Info {
    fn code(&self) -> &str {
        &self.code
    }

    fn set_code(&mut self, code: &str) {
        self.code = code.to_string();
    }
}

impl WithName for Info {
    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl Deletable for Info {
    fn delete_time(&self) -> Option<DateTime<Utc>> {
        self.delete_time
    }

    fn set_delete_time(&mut self, time: Option<DateTime<Utc>>) {
        self.delete_time = time;
    }
}

impl Emptyful for Info {
    fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.code.is_empty()
            && self.name.is_empty()
            && self.delete_time.is_none()
    }
}

impl Normalizable for Info {
    fn normalize(&mut self) {
        self.code = self.code.trim().to_string();
        self.name = self.name.trim().to_string();
    }
}
