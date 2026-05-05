use qubit_mixin::WithIndex;

struct Row {
    index: i32,
}

impl WithIndex for Row {
    fn index(&self) -> i32 {
        self.index
    }
    fn set_index(&mut self, index: i32) {
        self.index = index;
    }
}

#[test]
fn test_with_index_gets_and_sets_index() {
    let mut row = Row { index: 1 };
    row.set_index(8);
    assert_eq!(8, row.index());
}
