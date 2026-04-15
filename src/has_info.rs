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

//! Traits for basic information retrieval functionality
//!
//! # Author
//!
//! Haixing Hu

use crate::{HasSpecificInfo, Identifiable, Info, WithCode, WithName};

/// A trait indicating that an entity class has basic information
///
/// This trait combines the `Identifiable`, `WithCode`, `WithName`, and
/// `HasSpecificInfo<Info>` traits.
///
/// # Author
///
/// Haixing Hu
pub trait HasInfo: Identifiable + WithCode + WithName + HasSpecificInfo<Info> {
    // This trait serves only as a marker trait, combining multiple basic
    // traits
}
