use qubit_mixin::WithVisibility;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Visibility {
    Public,
    Private,
}

struct Row {
    visibility: Visibility,
}

impl WithVisibility<Visibility> for Row {
    fn visibility(&self) -> Visibility {
        self.visibility
    }
    fn set_visibility(&mut self, visibility: Visibility) {
        self.visibility = visibility;
    }
}

#[test]
fn test_with_visibility_gets_and_sets_copy_value() {
    let mut row = Row {
        visibility: Visibility::Public,
    };
    row.set_visibility(Visibility::Private);
    assert_eq!(Visibility::Private, row.visibility());
}
