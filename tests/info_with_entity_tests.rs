// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use chrono::Utc;
use qubit_mixin::{
    Deletable,
    Emptyful,
    Identifiable,
    Info,
    InfoWithEntity,
    Normalizable,
    WithCode,
    WithEntity,
    WithName,
};

#[test]
fn test_info_with_entity_new_and_accessors() {
    let now = Utc::now();
    let entity = "ORGANIZATION".to_owned();
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
    assert_eq!(row.entity(), Some(entity.as_str()));

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

    row.set_entity(Some("USER"));
    assert_eq!(row.entity(), Some("USER"));
}

#[test]
fn test_info_with_entity_default() {
    let row = InfoWithEntity::default();
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
        Some("ORG".to_owned()),
    );
    let row2 = row.clone();
    assert_eq!(row, row2);
}

#[test]
fn test_info_with_entity_converts_from_and_into_info() {
    let now = Utc::now();
    let info = Info::new(Some(1), "C".to_owned(), "N".to_owned(), Some(now));
    let row =
        InfoWithEntity::from_info(info.clone(), Some("CATEGORY".to_owned()));

    assert_eq!(info, row.info);
    assert_eq!(Some("CATEGORY"), row.entity());

    let extracted = row.into_info();
    assert_eq!(info, extracted);
}

#[test]
fn test_info_with_entity_uses_standard_from_conversions() {
    let now = Utc::now();
    let info = Info::new(Some(1), "C".to_owned(), "N".to_owned(), Some(now));

    let row: InfoWithEntity = info.clone().into();
    assert_eq!(info, row.info);
    assert_eq!(None, row.entity());

    let row =
        InfoWithEntity::from_info(info.clone(), Some("CATEGORY".to_owned()));
    let extracted: Info = row.into();
    assert_eq!(info, extracted);
}

#[test]
fn test_info_with_entity_is_complete_requires_info_and_entity() {
    let now = Utc::now();
    let complete = InfoWithEntity::new(
        Some(1),
        "C".to_owned(),
        "N".to_owned(),
        Some(now),
        Some("ORG".to_owned()),
    );
    assert!(complete.is_complete());

    let missing_entity = InfoWithEntity::new(
        Some(1),
        "C".to_owned(),
        "N".to_owned(),
        Some(now),
        None,
    );
    let missing_info = InfoWithEntity::new(
        None,
        "C".to_owned(),
        "N".to_owned(),
        Some(now),
        Some("ORG".to_owned()),
    );

    assert!(!missing_entity.is_complete());
    assert!(!missing_info.is_complete());
}

#[test]
fn test_info_with_entity_empty_and_normalize_delegate_to_embedded_info() {
    let mut row = InfoWithEntity::default();
    assert!(row.is_empty());

    row.set_entity(Some("ORG"));
    assert!(!row.is_empty());

    row.info = Info::new(Some(1), "  C  ".to_owned(), "  N  ".to_owned(), None);
    row.normalize();
    assert_eq!("C", row.code());
    assert_eq!("N", row.name());
}

#[test]
fn test_info_with_entity_sorts_by_info_then_entity() {
    let now = Utc::now();
    let mut rows = [
        InfoWithEntity::new(
            Some(2),
            "A".to_owned(),
            "A".to_owned(),
            Some(now),
            Some("A".to_owned()),
        ),
        InfoWithEntity::new(
            Some(1),
            "A".to_owned(),
            "A".to_owned(),
            Some(now),
            Some("B".to_owned()),
        ),
        InfoWithEntity::new(
            Some(1),
            "A".to_owned(),
            "A".to_owned(),
            Some(now),
            Some("A".to_owned()),
        ),
    ];

    rows.sort();

    assert_eq!(Some(1), rows[0].id());
    assert_eq!(Some("A"), rows[0].entity());
    assert_eq!(Some("B"), rows[1].entity());
    assert_eq!(Some(2), rows[2].id());
}
