use qubit_mixin::{
    Identifiable,
    WithStatus,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Status {
    Draft,
    Active,
}

struct Row {
    id: Option<i64>,
    status: Status,
}

impl Identifiable for Row {
    fn id(&self) -> Option<i64> {
        self.id
    }
    fn set_id(&mut self, id: Option<i64>) {
        self.id = id;
    }
}

impl WithStatus<Status> for Row {
    fn status(&self) -> Status {
        self.status
    }
    fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}

#[test]
fn test_with_status_keeps_identifier_and_status_accessors() {
    let mut row = Row {
        id: Some(1),
        status: Status::Draft,
    };
    row.set_id(Some(2));
    row.set_status(Status::Active);
    assert_eq!(Some(2), row.id());
    assert_eq!(Status::Active, row.status());
}
