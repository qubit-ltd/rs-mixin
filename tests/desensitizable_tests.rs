use qubit_mixin::Desensitizable;

#[derive(Clone)]
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

#[test]
fn test_desensitizable_creates_desensitized_clone_without_mutating_source() {
    let secret = Secret {
        token: "secret".to_owned(),
        visible: "name".to_owned(),
    };
    let clone = secret.desensitized();
    assert_eq!("secret", secret.token);
    assert_eq!("name", secret.visible);
    assert_eq!("", clone.token);
    assert_eq!("name", clone.visible);
}
