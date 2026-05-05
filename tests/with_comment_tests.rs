use qubit_mixin::WithComment;

struct Row {
    comment: String,
}

impl WithComment for Row {
    fn comment(&self) -> &str {
        &self.comment
    }
    fn set_comment(&mut self, comment: &str) {
        self.comment = comment.to_owned();
    }
}

#[test]
fn test_with_comment_gets_and_sets_comment() {
    let mut row = Row {
        comment: "draft".to_owned(),
    };
    row.set_comment("published");
    assert_eq!(row.comment(), "published");
}
