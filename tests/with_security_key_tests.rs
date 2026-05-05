use qubit_mixin::WithSecurityKey;

struct Account {
    security_key: String,
}

impl WithSecurityKey for Account {
    fn security_key(&self) -> &str {
        &self.security_key
    }
    fn set_security_key(&mut self, key: &str) {
        self.security_key = key.to_owned();
    }
}

#[test]
fn test_with_security_key_gets_and_sets_key() {
    let mut account = Account {
        security_key: "old".to_owned(),
    };
    account.set_security_key("new");
    assert_eq!("new", account.security_key());
}
