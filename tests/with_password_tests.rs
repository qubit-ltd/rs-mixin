use qubit_mixin::WithPassword;

struct Account {
    password: String,
}

impl WithPassword for Account {
    fn password(&self) -> &str {
        &self.password
    }
    fn set_password(&mut self, password: &str) {
        self.password = password.to_owned();
    }
}

#[test]
fn test_with_password_gets_and_sets_password() {
    let mut account = Account {
        password: "old".to_owned(),
    };
    account.set_password("new");
    assert_eq!("new", account.password());
}
