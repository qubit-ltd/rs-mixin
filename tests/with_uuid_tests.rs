use qubit_mixin::WithUuid;

struct Row {
    uuid: String,
}

impl WithUuid for Row {
    fn uuid(&self) -> &str {
        &self.uuid
    }
    fn set_uuid(&mut self, uuid: &str) {
        self.uuid = uuid.to_owned();
    }
}

#[test]
fn test_with_uuid_gets_and_sets_uuid() {
    let mut row = Row {
        uuid: "uuid-1".to_owned(),
    };
    row.set_uuid("uuid-2");
    assert_eq!("uuid-2", row.uuid());
}
