use chrono::{
    TimeZone,
    Utc,
};
use qubit_mixin::Modifiable;

struct Row {
    modified: Option<chrono::DateTime<Utc>>,
}

impl Modifiable for Row {
    fn modify_time(&self) -> Option<chrono::DateTime<Utc>> {
        self.modified
    }
    fn set_modify_time(&mut self, time: Option<chrono::DateTime<Utc>>) {
        self.modified = time;
    }
}

#[test]
fn test_modifiable_gets_sets_and_clears_modify_time() {
    let modified_at = Utc.timestamp_opt(7, 0).unwrap();
    let mut row = Row { modified: None };
    row.set_modify_time(Some(modified_at));
    assert_eq!(row.modify_time(), Some(modified_at));
    row.set_modify_time(None);
    assert_eq!(row.modify_time(), None);
}
