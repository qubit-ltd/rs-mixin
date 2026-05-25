use qubit_mixin::HasSpecificInfo;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Details {
    code: String,
}

struct Row {
    details: Details,
}

impl HasSpecificInfo<Details> for Row {
    fn info(&self) -> Details {
        self.details.clone()
    }
    fn set_info(&mut self, info: Details) {
        self.details = info;
    }
}

#[test]
fn test_has_specific_info_gets_and_replaces_typed_info() {
    let mut row = Row {
        details: Details { code: "A".to_owned() },
    };
    row.set_info(Details { code: "B".to_owned() });
    assert_eq!(Details { code: "B".to_owned() }, row.info());
}
