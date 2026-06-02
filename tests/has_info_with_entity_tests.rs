use qubit_mixin::{
    Deletable,
    HasInfo,
    HasInfoWithEntity,
    HasSpecificInfo,
    Identifiable,
    Info,
    InfoWithEntity,
    WithCode,
    WithEntity,
    WithName,
};

struct Row {
    info: Info,
    entity: Option<String>,
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
impl WithEntity for Row {
    fn entity(&self) -> Option<&str> {
        self.entity.as_deref()
    }

    fn set_entity(&mut self, entity: Option<&str>) {
        self.entity = entity.map(str::to_owned);
    }
}
impl HasSpecificInfo<InfoWithEntity> for Row {
    fn info(&self) -> InfoWithEntity {
        InfoWithEntity::new(
            self.info.id(),
            self.info.code().to_owned(),
            self.info.name().to_owned(),
            self.info.delete_time(),
            self.entity.clone(),
        )
    }

    fn set_info(&mut self, info: InfoWithEntity) {
        self.info.set_id(info.id());
        self.info.set_code(info.code());
        self.info.set_name(info.name());
        self.info.set_delete_time(info.delete_time());
        self.entity = info.entity().map(str::to_owned);
    }
}
impl HasInfo for Row {}
impl HasInfoWithEntity for Row {}

fn assert_has_entity<T: HasInfoWithEntity>(row: &T) {
    assert_eq!(Some("ORGANIZATION"), row.entity());
}

#[test]
fn test_has_info_with_entity_combines_basic_info_and_entity_accessors() {
    let row = Row {
        info: Info::new(Some(3), "C".to_owned(), "N".to_owned(), None),
        entity: Some("ORGANIZATION".to_owned()),
    };
    assert_has_entity(&row);
    let snapshot: InfoWithEntity = HasSpecificInfo::info(&row);
    assert_eq!(Some("ORGANIZATION"), snapshot.entity());
}
