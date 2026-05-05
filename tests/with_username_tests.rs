use qubit_mixin::WithUsername;

struct Account {
    username: String,
}

impl WithUsername for Account {
    fn username(&self) -> &str {
        &self.username
    }
    fn set_username(&mut self, username: &str) {
        self.username = username.to_owned();
    }
}

#[test]
fn test_with_username_gets_and_sets_username() {
    let mut account = Account {
        username: "alice".to_owned(),
    };
    account.set_username("bob");
    assert_eq!("bob", account.username());
}
