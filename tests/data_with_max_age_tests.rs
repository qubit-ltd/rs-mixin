use qubit_mixin::DataWithMaxAge;
use std::time::Duration;

struct CacheEntry {
    max_age: Duration,
}

impl DataWithMaxAge for CacheEntry {
    fn max_age(&self) -> Duration {
        self.max_age
    }
    fn set_max_age(&mut self, age: Duration) {
        self.max_age = age;
    }
}

#[test]
fn test_data_with_max_age_gets_and_sets_duration() {
    let mut entry = CacheEntry {
        max_age: Duration::from_secs(30),
    };
    entry.set_max_age(Duration::from_secs(90));
    assert_eq!(entry.max_age(), Duration::from_secs(90));
}
