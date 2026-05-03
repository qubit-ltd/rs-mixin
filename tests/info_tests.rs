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
    let mut info = Info::new(
        Some(1),
        "  CODE  ".to_string(),
        "  Name  ".to_string(),
        None,
    );

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
