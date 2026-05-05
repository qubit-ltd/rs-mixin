use qubit_mixin::{
    HasInfo,
    HasSpecificInfo,
    Identifiable,
    Info,
    WithCode,
    WithName,
};

struct Row {
    info: Info,
}

impl Identifiable for Row {
    fn id(&self) -> Option<i64> {
        self.info.id()
    }
    fn set_id(&mut self, id: Option<i64>) {
        self.info.set_id(id);
    }
}
impl WithCode for Row {
    fn code(&self) -> &str {
        self.info.code()
    }
    fn set_code(&mut self, code: &str) {
        self.info.set_code(code);
    }
}
impl WithName for Row {
    fn name(&self) -> &str {
        self.info.name()
    }
    fn set_name(&mut self, name: &str) {
        self.info.set_name(name);
    }
}
impl HasSpecificInfo<Info> for Row {
    fn info(&self) -> Info {
        self.info.clone()
    }
    fn set_info(&mut self, info: Info) {
        self.info = info;
    }
}
impl HasInfo for Row {}

fn assert_has_info<T: HasInfo>(row: &T) {
    assert_eq!(Some(7), row.id());
    assert_eq!("C", row.code());
    assert_eq!("Name", row.name());
}

#[test]
fn test_has_info_combines_id_code_name_and_info_accessors() {
    let row = Row {
        info: Info::new(Some(7), "C".to_owned(), "Name".to_owned(), None),
    };
    assert_has_info(&row);
    assert_eq!(Some(7), row.info().id());
}
