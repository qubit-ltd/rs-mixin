use chrono::{
    TimeZone,
    Utc,
};
use qubit_mixin::Deletable;

struct Row {
    deleted: Option<chrono::DateTime<Utc>>,
}

impl Deletable for Row {
    fn delete_time(&self) -> Option<chrono::DateTime<Utc>> {
        self.deleted
    }
    fn set_delete_time(&mut self, time: Option<chrono::DateTime<Utc>>) {
        self.deleted = time;
    }
}

#[test]
fn test_deletable_reports_soft_delete_state() {
    let deleted_at = Utc.timestamp_opt(42, 0).unwrap();
    let mut row = Row { deleted: None };
    assert!(!row.is_deleted());
    row.set_delete_time(Some(deleted_at));
    assert_eq!(row.delete_time(), Some(deleted_at));
    assert!(row.is_deleted());
}
