use chrono::{
    DateTime,
    Utc,
};
use qubit_mixin::{
    Auditable,
    Creatable,
    Deletable,
    Modifiable,
};

struct Row {
    created: DateTime<Utc>,
    modified: Option<DateTime<Utc>>,
    deleted: Option<DateTime<Utc>>,
}

impl Creatable for Row {
    fn create_time(&self) -> DateTime<Utc> {
        self.created
    }
    fn set_create_time(&mut self, time: DateTime<Utc>) {
        self.created = time;
    }
}
impl Modifiable for Row {
    fn modify_time(&self) -> Option<DateTime<Utc>> {
        self.modified
    }
    fn set_modify_time(&mut self, time: Option<DateTime<Utc>>) {
        self.modified = time;
    }
}
impl Deletable for Row {
    fn delete_time(&self) -> Option<DateTime<Utc>> {
        self.deleted
    }
    fn set_delete_time(&mut self, time: Option<DateTime<Utc>>) {
        self.deleted = time;
    }
}
impl Auditable for Row {}

fn touch_auditable<T: Auditable>(row: &mut T, time: DateTime<Utc>) {
    row.set_create_time(time);
    row.set_modify_time(Some(time));
    row.set_delete_time(Some(time));
}

#[test]
fn test_auditable_combines_create_modify_and_delete_traits() {
    let now = Utc::now();
    let mut row = Row {
        created: now,
        modified: None,
        deleted: None,
    };
    touch_auditable(&mut row, now);
    assert_eq!(row.create_time(), now);
    assert_eq!(row.modify_time(), Some(now));
    assert_eq!(row.delete_time(), Some(now));
    assert!(row.is_deleted());
}
