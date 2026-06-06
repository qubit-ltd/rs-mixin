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

fn age_through_trait_object(
    person: &dyn WithBirthday,
    date: NaiveDate,
) -> Option<i32> {
    person.age_on(date)
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

#[test]
fn test_with_birthday_calculates_age_on_reference_date() {
    let birthday = NaiveDate::from_ymd_opt(2008, 6, 3).expect("valid birthday");
    let person = Person {
        birthday: Some(birthday),
    };
    let before_birthday =
        NaiveDate::from_ymd_opt(2026, 6, 2).expect("valid reference date");
    let on_birthday =
        NaiveDate::from_ymd_opt(2026, 6, 3).expect("valid reference date");

    assert_eq!(Some(17), person.age_on(before_birthday));
    assert_eq!(Some(18), person.age_on(on_birthday));
}

#[test]
fn test_with_birthday_reports_adult_state_with_default_and_custom_thresholds() {
    let birthday = NaiveDate::from_ymd_opt(2008, 6, 3).expect("valid birthday");
    let person = Person {
        birthday: Some(birthday),
    };
    let reference =
        NaiveDate::from_ymd_opt(2026, 6, 3).expect("valid reference date");

    assert_eq!(Some(true), person.is_adult_on(reference));
    assert_eq!(
        Some(false),
        person.is_adult_on_with_threshold(reference, 21)
    );

    let person_without_birthday = Person { birthday: None };
    assert_eq!(None, person_without_birthday.age_on(reference));
    assert_eq!(None, person_without_birthday.is_adult_on(reference));
}

#[test]
fn test_with_birthday_age_helpers_work_through_trait_objects() {
    let birthday = NaiveDate::from_ymd_opt(2008, 6, 3).expect("valid birthday");
    let person = Person {
        birthday: Some(birthday),
    };
    let reference =
        NaiveDate::from_ymd_opt(2026, 6, 3).expect("valid reference date");

    assert_eq!(Some(18), age_through_trait_object(&person, reference));
}
