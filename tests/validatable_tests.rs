use qubit_mixin::Validatable;

#[derive(Debug, PartialEq, Eq)]
struct Error(&'static str);

struct Row {
    name: String,
}

impl Validatable for Row {
    type Error = Error;

    fn validate(&self) -> Result<(), Self::Error> {
        if self.name.is_empty() {
            Err(Error("name"))
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_validatable_returns_domain_error() {
    assert_eq!(
        Ok(()),
        Row {
            name: "Alice".to_owned()
        }
        .validate()
    );
    assert_eq!(
        Err(Error("name")),
        Row {
            name: String::new()
        }
        .validate()
    );
}
