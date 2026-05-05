use qubit_mixin::WithEmail;

struct Row {
    email: String,
}

impl WithEmail for Row {
    fn email(&self) -> &str {
        &self.email
    }
    fn set_email(&mut self, email: &str) {
        self.email = email.to_owned();
    }
}

#[test]
fn test_with_email_gets_and_sets_email() {
    let mut row = Row {
        email: "a@example.com".to_owned(),
    };
    row.set_email("b@example.com");
    assert_eq!(row.email(), "b@example.com");
}
