/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use chrono::Utc;
use qubit_mixin::{
    Deletable,
    Emptyful,
    Identifiable,
    Info,
    Normalizable,
    WithCode,
    WithName,
};

struct CatalogRow {
    id: Option<i64>,
    code: String,
    name: String,
    delete_time: Option<chrono::DateTime<Utc>>,
}

impl Identifiable for CatalogRow {
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: Option<i64>) {
        self.id = id;
    }
}

impl WithCode for CatalogRow {
    fn code(&self) -> &str {
        &self.code
    }

    fn set_code(&mut self, code: &str) {
        self.code = code.to_owned();
    }
}

impl WithName for CatalogRow {
    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
}

impl Deletable for CatalogRow {
    fn delete_time(&self) -> Option<chrono::DateTime<Utc>> {
        self.delete_time
    }

    fn set_delete_time(&mut self, time: Option<chrono::DateTime<Utc>>) {
        self.delete_time = time;
    }
}

#[test]
fn test_info_new() {
    let info = Info::new(Some(1), "CODE001".to_string(), "Test".to_string(), None);
    assert_eq!(info.id(), Some(1));
    assert_eq!(info.code(), "CODE001");
    assert_eq!(info.name(), "Test");
    assert_eq!(info.delete_time(), None);
    assert!(!info.is_deleted());
}

#[test]
fn test_info_of_id() {
    let info = Info::of_id(42);
    assert_eq!(info.id(), Some(42));
    assert_eq!(info.code(), "");
    assert_eq!(info.name(), "");
}

#[test]
fn test_info_of_code() {
    let info = Info::of_code("TEST".to_string());
    assert_eq!(info.id(), None);
    assert_eq!(info.code(), "TEST");
    assert_eq!(info.name(), "");
}

#[test]
fn test_info_of_name() {
    let info = Info::of_name("Sample".to_string());
    assert_eq!(info.id(), None);
    assert_eq!(info.code(), "");
    assert_eq!(info.name(), "Sample");
}

#[test]
fn test_info_default() {
    let info = Info::default();
    assert_eq!(info.id(), None);
    assert_eq!(info.code(), "");
    assert_eq!(info.name(), "");
    assert_eq!(info.delete_time(), None);
}

#[test]
fn test_info_setters() {
    let mut info = Info::default();

    info.set_id(Some(100));
    assert_eq!(info.id(), Some(100));

    info.set_code("NEW_CODE");
    assert_eq!(info.code(), "NEW_CODE");

    info.set_name("New Name");
    assert_eq!(info.name(), "New Name");

    let now = Utc::now();
    info.set_delete_time(Some(now));
    assert_eq!(info.delete_time(), Some(now));
    assert!(info.is_deleted());
}

#[test]
fn test_info_is_empty() {
    let info = Info::default();
    assert!(info.is_empty());

    let info2 = Info::of_id(1);
    assert!(!info2.is_empty());

    let info3 = Info::of_code("CODE".to_string());
    assert!(!info3.is_empty());
}

#[test]
fn test_info_normalize() {
    let mut info = Info::new(Some(1), "  CODE  ".to_string(), "  Name  ".to_string(), None);

    info.normalize();
    assert_eq!(info.code(), "CODE");
    assert_eq!(info.name(), "Name");
}

#[test]
fn test_info_clone() {
    let info1 = Info::new(Some(1), "CODE".to_string(), "Name".to_string(), None);

    let info2 = info1.clone();
    assert_eq!(info1, info2);
}

#[test]
fn test_info_is_complete_requires_all_business_fields() {
    let deleted_at = Utc::now();
    let complete = Info::new(Some(1), "CODE".to_owned(), "Name".to_owned(), Some(deleted_at));
    assert!(complete.is_complete());

    let missing_id = Info::new(None, "CODE".to_owned(), "Name".to_owned(), Some(deleted_at));
    let missing_code = Info::new(Some(1), "".to_owned(), "Name".to_owned(), Some(deleted_at));
    let missing_name = Info::new(Some(1), "CODE".to_owned(), "".to_owned(), Some(deleted_at));
    let missing_delete_time = Info::new(Some(1), "CODE".to_owned(), "Name".to_owned(), None);

    assert!(!missing_id.is_complete());
    assert!(!missing_code.is_complete());
    assert!(!missing_name.is_complete());
    assert!(!missing_delete_time.is_complete());
}

#[test]
fn test_info_sorts_by_id_code_name_and_delete_time() {
    let earlier = Utc::now();
    let later = earlier + chrono::Duration::seconds(1);
    let mut rows = [
        Info::new(Some(2), "A".to_owned(), "A".to_owned(), Some(earlier)),
        Info::new(Some(1), "B".to_owned(), "A".to_owned(), Some(earlier)),
        Info::new(Some(1), "A".to_owned(), "B".to_owned(), Some(later)),
        Info::new(Some(1), "A".to_owned(), "A".to_owned(), Some(earlier)),
    ];

    rows.sort();

    assert_eq!("A", rows[0].code());
    assert_eq!("A", rows[0].name());
    assert_eq!(Some(earlier), rows[0].delete_time());
    assert_eq!("B", rows[1].name());
    assert_eq!("B", rows[2].code());
    assert_eq!(Some(2), rows[3].id());
}

#[test]
fn test_info_copies_to_and_from_trait_backed_rows() {
    let deleted_at = Utc::now();
    let mut row = CatalogRow {
        id: Some(7),
        code: "OLD".to_owned(),
        name: "Old Name".to_owned(),
        delete_time: Some(deleted_at),
    };

    let basic_snapshot = Info::from_basic(&row);
    assert_eq!(Some(7), basic_snapshot.id());
    assert_eq!("OLD", basic_snapshot.code());
    assert_eq!("Old Name", basic_snapshot.name());
    assert_eq!(None, basic_snapshot.delete_time());

    let snapshot = Info::from_deletable(&row);
    assert_eq!(Some(7), snapshot.id());
    assert_eq!("OLD", snapshot.code());
    assert_eq!("Old Name", snapshot.name());
    assert_eq!(Some(deleted_at), snapshot.delete_time());

    let replacement = Info::new(Some(8), "NEW".to_owned(), "New Name".to_owned(), None);
    replacement.apply_to_deletable(&mut row);

    assert_eq!(Some(8), row.id());
    assert_eq!("NEW", row.code());
    assert_eq!("New Name", row.name());
    assert_eq!(None, row.delete_time());
}
