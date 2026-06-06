// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use chrono::{
    DateTime,
    NaiveDate,
    Utc,
};
use std::time::Duration;

use qubit_mixin::{
    Auditable,
    Creatable,
    DataWithMaxAge,
    Deletable,
    Desensitizable,
    HasInfo,
    HasInfoWithEntity,
    HasSpecificInfo,
    Identifiable,
    Modifiable,
    Predefinable,
    Validatable,
    WithBirthday,
    WithCode,
    WithEntity,
    WithIndex,
    WithKey,
    WithName,
    WithPassword,
    WithSecurityKey,
    WithStatus,
    WithUdid,
    WithUsername,
    WithUuid,
    WithVisibility,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    On,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Vis {
    Public,
    Private,
}

struct KitchenSink {
    id: Option<i64>,
    status: Status,
    visibility: Vis,
    index: i32,
    key: String,
    password: String,
    security_key: String,
    udid: String,
    username: String,
    uuid: String,
    birthday: Option<NaiveDate>,
    predefined: bool,
    max_age: Duration,
    sensitive: String,
    create_time: DateTime<Utc>,
    modify_time: Option<DateTime<Utc>>,
    delete_time: Option<DateTime<Utc>>,
    core: qubit_mixin::Info,
    entity: Option<String>,
}

impl Identifiable for KitchenSink {
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: Option<i64>) {
        self.id = id;
    }
}

impl WithStatus<Status> for KitchenSink {
    fn status(&self) -> Status {
        self.status
    }

    fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}

impl WithVisibility<Vis> for KitchenSink {
    fn visibility(&self) -> Vis {
        self.visibility
    }

    fn set_visibility(&mut self, visibility: Vis) {
        self.visibility = visibility;
    }
}

impl WithIndex for KitchenSink {
    fn index(&self) -> i32 {
        self.index
    }

    fn set_index(&mut self, index: i32) {
        self.index = index;
    }
}

impl WithKey for KitchenSink {
    fn key(&self) -> &str {
        &self.key
    }

    fn set_key(&mut self, key: &str) {
        self.key = key.to_string();
    }
}

impl WithPassword for KitchenSink {
    fn password(&self) -> &str {
        &self.password
    }

    fn set_password(&mut self, password: &str) {
        self.password = password.to_string();
    }
}

impl WithSecurityKey for KitchenSink {
    fn security_key(&self) -> &str {
        &self.security_key
    }

    fn set_security_key(&mut self, key: &str) {
        self.security_key = key.to_string();
    }
}

impl WithUdid for KitchenSink {
    fn udid(&self) -> &str {
        &self.udid
    }

    fn set_udid(&mut self, udid: &str) {
        self.udid = udid.to_string();
    }
}

impl WithUsername for KitchenSink {
    fn username(&self) -> &str {
        &self.username
    }

    fn set_username(&mut self, username: &str) {
        self.username = username.to_string();
    }
}

impl WithUuid for KitchenSink {
    fn uuid(&self) -> &str {
        &self.uuid
    }

    fn set_uuid(&mut self, uuid: &str) {
        self.uuid = uuid.to_string();
    }
}

impl WithBirthday for KitchenSink {
    fn birthday(&self) -> Option<NaiveDate> {
        self.birthday
    }

    fn set_birthday(&mut self, birthday: Option<NaiveDate>) {
        self.birthday = birthday;
    }
}

impl Predefinable for KitchenSink {
    fn is_predefined(&self) -> bool {
        self.predefined
    }

    fn set_predefined(&mut self, predefined: bool) {
        self.predefined = predefined;
    }
}

impl DataWithMaxAge for KitchenSink {
    fn max_age(&self) -> Duration {
        self.max_age
    }

    fn set_max_age(&mut self, age: Duration) {
        self.max_age = age;
    }
}

impl Creatable for KitchenSink {
    fn create_time(&self) -> DateTime<Utc> {
        self.create_time
    }

    fn set_create_time(&mut self, time: DateTime<Utc>) {
        self.create_time = time;
    }
}

impl Modifiable for KitchenSink {
    fn modify_time(&self) -> Option<DateTime<Utc>> {
        self.modify_time
    }

    fn set_modify_time(&mut self, time: Option<DateTime<Utc>>) {
        self.modify_time = time;
    }
}

impl Deletable for KitchenSink {
    fn delete_time(&self) -> Option<DateTime<Utc>> {
        self.delete_time
    }

    fn set_delete_time(&mut self, time: Option<DateTime<Utc>>) {
        self.delete_time = time;
    }
}

impl Auditable for KitchenSink {}

impl Desensitizable for KitchenSink {
    fn desensitize(&mut self) {
        self.sensitive.clear();
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ValidationError(&'static str);

impl Validatable for KitchenSink {
    type Error = ValidationError;

    fn validate(&self) -> Result<(), Self::Error> {
        if self.username.is_empty() {
            return Err(ValidationError("username"));
        }
        Ok(())
    }
}

impl qubit_mixin::WithCode for KitchenSink {
    fn code(&self) -> &str {
        self.core.code()
    }

    fn set_code(&mut self, code: &str) {
        self.core.set_code(code);
    }
}

impl qubit_mixin::WithName for KitchenSink {
    fn name(&self) -> &str {
        self.core.name()
    }

    fn set_name(&mut self, name: &str) {
        self.core.set_name(name);
    }
}

impl HasSpecificInfo<qubit_mixin::Info> for KitchenSink {
    fn info(&self) -> qubit_mixin::Info {
        self.core.clone()
    }

    fn set_info(&mut self, info: qubit_mixin::Info) {
        self.core = info;
    }
}

impl WithEntity for KitchenSink {
    fn entity(&self) -> Option<&str> {
        self.entity.as_deref()
    }

    fn set_entity(&mut self, entity: Option<&str>) {
        self.entity = entity.map(str::to_owned);
    }
}

impl HasSpecificInfo<qubit_mixin::InfoWithEntity> for KitchenSink {
    fn info(&self) -> qubit_mixin::InfoWithEntity {
        qubit_mixin::InfoWithEntity::new(
            self.core.id(),
            self.core.code().to_string(),
            self.core.name().to_string(),
            self.core.delete_time(),
            self.entity.clone(),
        )
    }

    fn set_info(&mut self, info: qubit_mixin::InfoWithEntity) {
        self.core.set_id(info.id());
        self.core.set_code(info.code());
        self.core.set_name(info.name());
        self.core.set_delete_time(info.delete_time());
        self.entity = info.entity().map(str::to_owned);
    }
}

impl HasInfo for KitchenSink {}

impl HasInfoWithEntity for KitchenSink {}

fn sample_sink(entity: Option<&str>) -> KitchenSink {
    KitchenSink {
        id: Some(1),
        status: Status::On,
        visibility: Vis::Public,
        index: 5,
        key: "k".to_string(),
        password: "p".to_string(),
        security_key: "sk".to_string(),
        udid: "udid".to_string(),
        username: "alice".to_string(),
        uuid: "uuid".to_string(),
        birthday: Some(NaiveDate::from_ymd_opt(2000, 1, 2).unwrap()),
        predefined: false,
        max_age: Duration::from_secs(60),
        sensitive: "secret".to_string(),
        create_time: Utc::now(),
        modify_time: None,
        delete_time: None,
        core: qubit_mixin::Info::of_id(42),
        entity: entity.map(str::to_owned),
    }
}

#[test]
fn test_kitchen_sink_traits() {
    let mut row = sample_sink(Some("ORGANIZATION"));

    assert_eq!(row.status(), Status::On);
    row.set_status(Status::Off);
    assert_eq!(row.status(), Status::Off);

    assert_eq!(row.visibility(), Vis::Public);
    row.set_visibility(Vis::Private);
    assert_eq!(row.visibility(), Vis::Private);

    assert_eq!(row.index(), 5);
    row.set_index(9);
    assert_eq!(row.index(), 9);

    assert_eq!(row.key(), "k");
    row.set_key("key2");
    assert_eq!(row.key(), "key2");

    assert_eq!(row.password(), "p");
    row.set_password("pw2");
    assert_eq!(row.password(), "pw2");

    assert_eq!(row.security_key(), "sk");
    row.set_security_key("sk2");
    assert_eq!(row.security_key(), "sk2");

    assert_eq!(row.udid(), "udid");
    row.set_udid("u2");
    assert_eq!(row.udid(), "u2");

    assert_eq!(row.username(), "alice");
    row.set_username("bob");
    assert_eq!(row.username(), "bob");

    assert_eq!(row.uuid(), "uuid");
    row.set_uuid("u-u-i-d");
    assert_eq!(row.uuid(), "u-u-i-d");

    let bd = NaiveDate::from_ymd_opt(1999, 3, 4).unwrap();
    row.set_birthday(Some(bd));
    assert_eq!(row.birthday(), Some(bd));
    row.set_birthday(None);
    assert_eq!(row.birthday(), None);

    assert!(!row.is_predefined());
    row.set_predefined(true);
    assert!(row.is_predefined());

    assert_eq!(row.max_age(), Duration::from_secs(60));
    row.set_max_age(Duration::from_secs(120));
    assert_eq!(row.max_age(), Duration::from_secs(120));

    row.desensitize();
    assert!(row.sensitive.is_empty());

    assert!(row.validate().is_ok());

    let mut bad = sample_sink(None);
    bad.set_username("");
    assert_eq!(bad.validate(), Err(ValidationError("username")));

    let full: qubit_mixin::Info = HasSpecificInfo::info(&row);
    assert_eq!(full.id(), Some(42));
    let full_we: qubit_mixin::InfoWithEntity = HasSpecificInfo::info(&row);
    assert_eq!(full_we.entity(), Some("ORGANIZATION"));
}
