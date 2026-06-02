use qubit_mixin::WithEntity;

struct Row {
    entity: Option<String>,
}

impl WithEntity for Row {
    fn entity(&self) -> Option<&str> {
        self.entity.as_deref()
    }

    fn set_entity(&mut self, entity: Option<&str>) {
        self.entity = entity.map(str::to_owned);
    }
}

#[test]
fn test_with_entity_gets_sets_and_clears_entity_name() {
    let mut row = Row { entity: None };
    row.set_entity(Some("ORGANIZATION"));
    assert_eq!(Some("ORGANIZATION"), row.entity());

    row.set_entity(None);
    assert_eq!(None, row.entity());
}
