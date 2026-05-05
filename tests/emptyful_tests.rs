use qubit_mixin::Emptyful;

struct Bag {
    values: Vec<i32>,
}

impl Emptyful for Bag {
    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[test]
fn test_emptyful_reports_collection_state() {
    assert!(Bag { values: vec![] }.is_empty());
    assert!(!Bag { values: vec![1] }.is_empty());
}
