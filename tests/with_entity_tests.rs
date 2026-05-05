use qubit_mixin::WithEntity;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Payload {
    value: i32,
}

struct Row {
    entity: Option<Payload>,
}

impl WithEntity<Payload> for Row {
    fn entity(&self) -> Option<&Payload> {
        self.entity.as_ref()
    }
    fn set_entity(&mut self, entity: Option<Payload>) {
        self.entity = entity;
    }
}

#[test]
fn test_with_entity_gets_sets_and_clears_entity() {
    let mut row = Row { entity: None };
    row.set_entity(Some(Payload { value: 5 }));
    assert_eq!(Some(&Payload { value: 5 }), row.entity());
    row.set_entity(None);
    assert_eq!(None, row.entity());
}
