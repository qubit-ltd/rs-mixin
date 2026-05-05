use qubit_mixin::Predefinable;

struct Row {
    predefined: bool,
}

impl Predefinable for Row {
    fn is_predefined(&self) -> bool {
        self.predefined
    }
    fn set_predefined(&mut self, predefined: bool) {
        self.predefined = predefined;
    }
}

#[test]
fn test_predefinable_gets_and_sets_flag() {
    let mut row = Row { predefined: false };
    row.set_predefined(true);
    assert!(row.is_predefined());
}
