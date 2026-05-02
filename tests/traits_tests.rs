/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
use chrono::{DateTime, Utc};
use qubit_mixin::{
    Auditable, Creatable, Deletable, Identifiable, Modifiable, WithCode, WithComment, WithEmail,
    WithName,
};

struct TestEntity {
    id: Option<i64>,
    code: String,
    name: String,
    comment: String,
    email: String,
    create_time: DateTime<Utc>,
    modify_time: Option<DateTime<Utc>>,
    delete_time: Option<DateTime<Utc>>,
}

impl Identifiable for TestEntity {
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: Option<i64>) {
        self.id = id;
    }
}

impl WithCode for TestEntity {
    fn code(&self) -> &str {
        &self.code
    }

    fn set_code(&mut self, code: &str) {
        self.code = code.to_string();
    }
}

impl WithName for TestEntity {
    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl WithComment for TestEntity {
    fn comment(&self) -> &str {
        &self.comment
    }

    fn set_comment(&mut self, comment: &str) {
        self.comment = comment.to_string();
    }
}

impl WithEmail for TestEntity {
    fn email(&self) -> &str {
        &self.email
    }

    fn set_email(&mut self, email: &str) {
        self.email = email.to_string();
    }
}

impl Creatable for TestEntity {
    fn create_time(&self) -> DateTime<Utc> {
        self.create_time
    }

    fn set_create_time(&mut self, time: DateTime<Utc>) {
        self.create_time = time;
    }
}

impl Modifiable for TestEntity {
    fn modify_time(&self) -> Option<DateTime<Utc>> {
        self.modify_time
    }

    fn set_modify_time(&mut self, time: Option<DateTime<Utc>>) {
        self.modify_time = time;
    }
}

impl Deletable for TestEntity {
    fn delete_time(&self) -> Option<DateTime<Utc>> {
        self.delete_time
    }

    fn set_delete_time(&mut self, time: Option<DateTime<Utc>>) {
        self.delete_time = time;
    }
}

impl Auditable for TestEntity {}

#[test]
fn test_identifiable() {
    let mut entity = create_test_entity();

    assert_eq!(entity.id(), Some(1));
    entity.set_id(Some(2));
    assert_eq!(entity.id(), Some(2));

    entity.set_id(None);
    assert_eq!(entity.id(), None);
}

#[test]
fn test_with_code() {
    let mut entity = create_test_entity();

    assert_eq!(entity.code(), "TEST001");
    entity.set_code("NEW_CODE");
    assert_eq!(entity.code(), "NEW_CODE");
}

#[test]
fn test_with_name() {
    let mut entity = create_test_entity();

    assert_eq!(entity.name(), "Test Entity");
    entity.set_name("Updated Name");
    assert_eq!(entity.name(), "Updated Name");
}

#[test]
fn test_with_comment() {
    let mut entity = create_test_entity();

    assert_eq!(entity.comment(), "Test comment");
    entity.set_comment("Updated comment");
    assert_eq!(entity.comment(), "Updated comment");
}

#[test]
fn test_with_email() {
    let mut entity = create_test_entity();

    assert_eq!(entity.email(), "test@example.com");
    entity.set_email("new@example.com");
    assert_eq!(entity.email(), "new@example.com");
}

#[test]
fn test_creatable() {
    let mut entity = create_test_entity();
    let initial_time = entity.create_time();

    // Add a small delay to ensure different timestamps
    std::thread::sleep(std::time::Duration::from_millis(10));
    let new_time = Utc::now();
    entity.set_create_time(new_time);
    assert_eq!(entity.create_time(), new_time);
    assert_ne!(entity.create_time(), initial_time);
}

#[test]
fn test_modifiable() {
    let mut entity = create_test_entity();

    assert_eq!(entity.modify_time(), None);

    let now = Utc::now();
    entity.set_modify_time(Some(now));
    assert_eq!(entity.modify_time(), Some(now));

    entity.set_modify_time(None);
    assert_eq!(entity.modify_time(), None);
}

#[test]
fn test_deletable() {
    let mut entity = create_test_entity();

    assert_eq!(entity.delete_time(), None);
    assert!(!entity.is_deleted());

    let now = Utc::now();
    entity.set_delete_time(Some(now));
    assert_eq!(entity.delete_time(), Some(now));
    assert!(entity.is_deleted());

    entity.set_delete_time(None);
    assert_eq!(entity.delete_time(), None);
    assert!(!entity.is_deleted());
}

fn create_test_entity() -> TestEntity {
    TestEntity {
        id: Some(1),
        code: "TEST001".to_string(),
        name: "Test Entity".to_string(),
        comment: "Test comment".to_string(),
        email: "test@example.com".to_string(),
        create_time: Utc::now(),
        modify_time: None,
        delete_time: None,
    }
}
