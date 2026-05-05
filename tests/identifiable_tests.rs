use qubit_mixin::Identifiable;

struct Row {
    id: Option<i64>,
}

impl Identifiable for Row {
    fn id(&self) -> Option<i64> {
        self.id
    }
    fn set_id(&mut self, id: Option<i64>) {
        self.id = id;
    }
}

#[test]
fn test_identifiable_gets_sets_and_clears_id() {
    let mut row = Row { id: Some(1) };
    assert_eq!(row.id(), Some(1));
    row.set_id(Some(2));
    assert_eq!(row.id(), Some(2));
    row.set_id(None);
    assert_eq!(row.id(), None);
}
