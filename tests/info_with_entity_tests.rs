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

use chrono::Utc;
use qubit_mixin::{Deletable, Identifiable, InfoWithEntity, WithCode, WithEntity, WithName};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Payload {
    value: i32,
}

#[test]
fn test_info_with_entity_new_and_accessors() {
    let now = Utc::now();
    let entity = Payload { value: 7 };
    let mut row = InfoWithEntity::new(
        Some(10),
        "C1".to_string(),
        "N1".to_string(),
        Some(now),
        Some(entity.clone()),
    );

    assert_eq!(row.id(), Some(10));
    assert_eq!(row.code(), "C1");
    assert_eq!(row.name(), "N1");
    assert_eq!(row.delete_time(), Some(now));
    assert!(row.is_deleted());
    assert_eq!(row.entity(), Some(&entity));

    row.set_id(Some(99));
    assert_eq!(row.id(), Some(99));

    row.set_code("C2");
    assert_eq!(row.code(), "C2");

    row.set_name("N2");
    assert_eq!(row.name(), "N2");

    row.set_delete_time(None);
    assert_eq!(row.delete_time(), None);
    assert!(!row.is_deleted());

    row.set_entity(None);
    assert_eq!(row.entity(), None);

    row.set_entity(Some(Payload { value: 3 }));
    assert_eq!(row.entity(), Some(&Payload { value: 3 }));
}

#[test]
fn test_info_with_entity_default() {
    let row: InfoWithEntity<Payload> = InfoWithEntity::default();
    assert_eq!(row.id(), None);
    assert_eq!(row.code(), "");
    assert_eq!(row.name(), "");
    assert_eq!(row.delete_time(), None);
    assert!(!row.is_deleted());
    assert_eq!(row.entity(), None);
}

#[test]
fn test_info_with_entity_clone_partial_eq() {
    let row = InfoWithEntity::new(
        None,
        "x".to_string(),
        "y".to_string(),
        None,
        Some(Payload { value: 1 }),
    );
    let row2 = row.clone();
    assert_eq!(row, row2);
}
