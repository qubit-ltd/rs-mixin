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

use chrono::{DateTime, NaiveDate, Utc};
use std::time::Duration;

use qubit_mixin::{
    Auditable, Creatable, DataWithMaxAge, Deletable, Desensitizable, HasInfo, HasInfoWithEntity,
    HasSpecificInfo, Identifiable, Modifiable, Predefinable, Validatable, WithBirthday, WithCode,
    WithEntity, WithIndex, WithKey, WithName, WithPassword, WithSecurityKey, WithStatus, WithUdid,
    WithUsername, WithUuid, WithVisibility,
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

struct KitchenSink<E: Clone> {
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
    entity: Option<E>,
}

impl<E: Clone> Identifiable for KitchenSink<E> {
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: Option<i64>) {
        self.id = id;
    }
}

impl<E: Clone> WithStatus<Status> for KitchenSink<E> {
    fn status(&self) -> Status {
        self.status
    }

    fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}

impl<E: Clone> WithVisibility<Vis> for KitchenSink<E> {
    fn visibility(&self) -> Vis {
        self.visibility
    }

    fn set_visibility(&mut self, visibility: Vis) {
        self.visibility = visibility;
    }
}

impl<E: Clone> WithIndex for KitchenSink<E> {
    fn index(&self) -> i32 {
        self.index
    }

    fn set_index(&mut self, index: i32) {
        self.index = index;
    }
}

impl<E: Clone> WithKey for KitchenSink<E> {
    fn key(&self) -> &str {
        &self.key
    }

    fn set_key(&mut self, key: &str) {
        self.key = key.to_string();
    }
}

impl<E: Clone> WithPassword for KitchenSink<E> {
    fn password(&self) -> &str {
        &self.password
    }

    fn set_password(&mut self, password: &str) {
        self.password = password.to_string();
    }
}

impl<E: Clone> WithSecurityKey for KitchenSink<E> {
    fn security_key(&self) -> &str {
        &self.security_key
    }

    fn set_security_key(&mut self, key: &str) {
        self.security_key = key.to_string();
    }
}

impl<E: Clone> WithUdid for KitchenSink<E> {
    fn udid(&self) -> &str {
        &self.udid
    }

    fn set_udid(&mut self, udid: &str) {
        self.udid = udid.to_string();
    }
}

impl<E: Clone> WithUsername for KitchenSink<E> {
    fn username(&self) -> &str {
        &self.username
    }

    fn set_username(&mut self, username: &str) {
        self.username = username.to_string();
    }
}

impl<E: Clone> WithUuid for KitchenSink<E> {
    fn uuid(&self) -> &str {
        &self.uuid
    }

    fn set_uuid(&mut self, uuid: &str) {
        self.uuid = uuid.to_string();
    }
}

impl<E: Clone> WithBirthday for KitchenSink<E> {
    fn birthday(&self) -> Option<NaiveDate> {
        self.birthday
    }

    fn set_birthday(&mut self, birthday: Option<NaiveDate>) {
        self.birthday = birthday;
    }
}

impl<E: Clone> Predefinable for KitchenSink<E> {
    fn is_predefined(&self) -> bool {
        self.predefined
    }

    fn set_predefined(&mut self, predefined: bool) {
        self.predefined = predefined;
    }
}

impl<E: Clone> DataWithMaxAge for KitchenSink<E> {
    fn max_age(&self) -> Duration {
        self.max_age
    }

    fn set_max_age(&mut self, age: Duration) {
        self.max_age = age;
    }
}

impl<E: Clone> Creatable for KitchenSink<E> {
    fn create_time(&self) -> DateTime<Utc> {
        self.create_time
    }

    fn set_create_time(&mut self, time: DateTime<Utc>) {
        self.create_time = time;
    }
}

impl<E: Clone> Modifiable for KitchenSink<E> {
    fn modify_time(&self) -> Option<DateTime<Utc>> {
        self.modify_time
    }

    fn set_modify_time(&mut self, time: Option<DateTime<Utc>>) {
        self.modify_time = time;
    }
}

impl<E: Clone> Deletable for KitchenSink<E> {
    fn delete_time(&self) -> Option<DateTime<Utc>> {
        self.delete_time
    }

    fn set_delete_time(&mut self, time: Option<DateTime<Utc>>) {
        self.delete_time = time;
    }
}

impl<E: Clone> Auditable for KitchenSink<E> {}

impl<E: Clone> Desensitizable for KitchenSink<E> {
    fn desensitize(&mut self) {
        self.sensitive.clear();
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ValidationError(&'static str);

impl<E: Clone> Validatable for KitchenSink<E> {
    type Error = ValidationError;

    fn validate(&self) -> Result<(), Self::Error> {
        if self.username.is_empty() {
            return Err(ValidationError("username"));
        }
        Ok(())
    }
}

impl<E: Clone> qubit_mixin::WithCode for KitchenSink<E> {
    fn code(&self) -> &str {
        self.core.code()
    }

    fn set_code(&mut self, code: &str) {
        self.core.set_code(code);
    }
}

impl<E: Clone> qubit_mixin::WithName for KitchenSink<E> {
    fn name(&self) -> &str {
        self.core.name()
    }

    fn set_name(&mut self, name: &str) {
        self.core.set_name(name);
    }
}

impl<E: Clone> HasSpecificInfo<qubit_mixin::Info> for KitchenSink<E> {
    fn info(&self) -> qubit_mixin::Info {
        self.core.clone()
    }

    fn set_info(&mut self, info: qubit_mixin::Info) {
        self.core = info;
    }
}

impl<E: Clone> WithEntity<E> for KitchenSink<E> {
    fn entity(&self) -> Option<&E> {
        self.entity.as_ref()
    }

    fn set_entity(&mut self, entity: Option<E>) {
        self.entity = entity;
    }
}

impl<E: Clone> HasSpecificInfo<qubit_mixin::InfoWithEntity<E>> for KitchenSink<E> {
    fn info(&self) -> qubit_mixin::InfoWithEntity<E> {
        qubit_mixin::InfoWithEntity::new(
            self.core.id(),
            self.core.code().to_string(),
            self.core.name().to_string(),
            self.core.delete_time(),
            self.entity.clone(),
        )
    }

    fn set_info(&mut self, info: qubit_mixin::InfoWithEntity<E>) {
        self.core.set_id(info.id());
        self.core.set_code(info.code());
        self.core.set_name(info.name());
        self.core.set_delete_time(info.delete_time());
        self.entity = info.entity().cloned();
    }
}

impl<E: Clone> HasInfo for KitchenSink<E> {}

impl<E: Clone> HasInfoWithEntity<E> for KitchenSink<E> {}

fn sample_sink<E: Clone>(entity: Option<E>) -> KitchenSink<E> {
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
        entity,
    }
}

#[test]
fn test_kitchen_sink_traits() {
    let mut row = sample_sink(Some(100i32));

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

    let mut bad = sample_sink::<i32>(None);
    bad.set_username("");
    assert_eq!(bad.validate(), Err(ValidationError("username")));

    let full: qubit_mixin::Info = HasSpecificInfo::info(&row);
    assert_eq!(full.id(), Some(42));
    let full_we: qubit_mixin::InfoWithEntity<i32> = HasSpecificInfo::info(&row);
    assert_eq!(full_we.entity(), Some(&100));
}
