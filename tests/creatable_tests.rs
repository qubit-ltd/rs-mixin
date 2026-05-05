use chrono::{
    DateTime,
    TimeZone,
    Utc,
};
use qubit_mixin::Creatable;

struct Row {
    created: DateTime<Utc>,
}

impl Creatable for Row {
    fn create_time(&self) -> DateTime<Utc> {
        self.created
    }
    fn set_create_time(&mut self, time: DateTime<Utc>) {
        self.created = time;
    }
}

#[test]
fn test_creatable_gets_and_sets_creation_time() {
    let initial = Utc.timestamp_opt(1, 0).unwrap();
    let next = Utc.timestamp_opt(2, 0).unwrap();
    let mut row = Row { created: initial };
    assert_eq!(row.create_time(), initial);
    row.set_create_time(next);
    assert_eq!(row.create_time(), next);
}
