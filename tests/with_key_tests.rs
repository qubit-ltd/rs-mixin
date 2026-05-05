use qubit_mixin::WithKey;

struct Row {
    key: String,
}

impl WithKey for Row {
    fn key(&self) -> &str {
        &self.key
    }
    fn set_key(&mut self, key: &str) {
        self.key = key.to_owned();
    }
}

#[test]
fn test_with_key_gets_and_sets_key() {
    let mut row = Row {
        key: "k1".to_owned(),
    };
    row.set_key("k2");
    assert_eq!(row.key(), "k2");
}
