use qubit_mixin::Desensitizable;

struct Secret {
    token: String,
    visible: String,
}

impl Desensitizable for Secret {
    fn desensitize(&mut self) {
        self.token.clear();
    }
}

#[test]
fn test_desensitizable_removes_sensitive_field_without_touching_visible_data() {
    let mut secret = Secret {
        token: "secret".to_owned(),
        visible: "name".to_owned(),
    };
    secret.desensitize();
    assert_eq!("", secret.token);
    assert_eq!("name", secret.visible);
}
