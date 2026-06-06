use qubit_mixin::WithCode;

struct Row {
    code: String,
}

impl WithCode for Row {
    fn code(&self) -> &str {
        &self.code
    }
    fn set_code(&mut self, code: &str) {
        self.code = code.to_owned();
    }
}

#[test]
fn test_with_code_gets_and_sets_code() {
    let mut row = Row {
        code: "A".to_owned(),
    };
    row.set_code("B");
    assert_eq!(row.code(), "B");
}
