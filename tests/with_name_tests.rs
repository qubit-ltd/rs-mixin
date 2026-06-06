use qubit_mixin::WithName;

struct Row {
    name: String,
}

impl WithName for Row {
    fn name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }
}

#[test]
fn test_with_name_gets_and_sets_name() {
    let mut row = Row {
        name: "old".to_owned(),
    };
    row.set_name("new");
    assert_eq!(row.name(), "new");
}
