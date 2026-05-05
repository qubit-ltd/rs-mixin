use chrono::NaiveDate;
use qubit_mixin::WithBirthday;

struct Person {
    birthday: Option<NaiveDate>,
}

impl WithBirthday for Person {
    fn birthday(&self) -> Option<NaiveDate> {
        self.birthday
    }
    fn set_birthday(&mut self, birthday: Option<NaiveDate>) {
        self.birthday = birthday;
    }
}

#[test]
fn test_with_birthday_gets_sets_and_clears_birthday() {
    let date = NaiveDate::from_ymd_opt(2000, 1, 2).unwrap();
    let mut person = Person { birthday: None };
    person.set_birthday(Some(date));
    assert_eq!(Some(date), person.birthday());
    person.set_birthday(None);
    assert_eq!(None, person.birthday());
}
