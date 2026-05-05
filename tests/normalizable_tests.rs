use qubit_mixin::Normalizable;

struct Row {
    name: String,
}

impl Normalizable for Row {
    fn normalize(&mut self) {
        self.name = self.name.trim().to_owned();
    }
}

#[test]
fn test_normalizable_updates_internal_representation() {
    let mut row = Row {
        name: "  Alice  ".to_owned(),
    };
    row.normalize();
    assert_eq!("Alice", row.name);
}
