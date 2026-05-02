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

//! # Domain Object Mixin Traits
//!
//! This library provides a series of mixin traits for adding common
//! properties and behaviors to domain objects. These traits follow the
//! composition-over-inheritance design principle, allowing domain objects
//! to acquire desired functionality by implementing different trait
//! combinations.
//!
//! # Main Features
//!
//! - **Timestamp Traits**: Manage creation, modification, and deletion
//!   times
//! - **Identifier Traits**: Provide identification information such as ID,
//!   code, and name
//! - **State Management Traits**: Manage status, visibility, and other
//!   states
//! - **User Information Traits**: Provide user-related information such as
//!   username, email, and password
//! - **Entity Association Traits**: Manage entity associations
//! - **Validation and Normalization**: Provide data validation and
//!   normalization functionality
//!

pub mod auditable;
pub mod creatable;
pub mod data_with_max_age;
pub mod deletable;
pub mod desensitizable;
pub mod emptyful;
pub mod has_info;
pub mod has_info_with_entity;
pub mod has_specific_info;
pub mod identifiable;
pub mod info;
pub mod info_with_entity;
pub mod modifiable;
pub mod normalizable;
pub mod predefinable;
pub mod validatable;
pub mod with_birthday;
pub mod with_code;
pub mod with_comment;
pub mod with_email;
pub mod with_entity;
pub mod with_index;
pub mod with_key;
pub mod with_name;
pub mod with_password;
pub mod with_security_key;
pub mod with_status;
pub mod with_udid;
pub mod with_username;
pub mod with_uuid;
pub mod with_visibility;

// Re-export main traits
pub use auditable::Auditable;
pub use creatable::Creatable;
pub use data_with_max_age::DataWithMaxAge;
pub use deletable::Deletable;
pub use desensitizable::Desensitizable;
pub use emptyful::Emptyful;
pub use has_info::HasInfo;
pub use has_info_with_entity::HasInfoWithEntity;
pub use has_specific_info::HasSpecificInfo;
pub use identifiable::Identifiable;
pub use info::Info;
pub use info_with_entity::InfoWithEntity;
pub use modifiable::Modifiable;
pub use normalizable::Normalizable;
pub use predefinable::Predefinable;
pub use validatable::Validatable;
pub use with_birthday::WithBirthday;
pub use with_code::WithCode;
pub use with_comment::WithComment;
pub use with_email::WithEmail;
pub use with_entity::WithEntity;
pub use with_index::WithIndex;
pub use with_key::WithKey;
pub use with_name::WithName;
pub use with_password::WithPassword;
pub use with_security_key::WithSecurityKey;
pub use with_status::WithStatus;
pub use with_udid::WithUdid;
pub use with_username::WithUsername;
pub use with_uuid::WithUuid;
pub use with_visibility::WithVisibility;
