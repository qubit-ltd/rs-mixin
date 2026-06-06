use qubit_mixin::WithUdid;

struct Device {
    udid: String,
}

impl WithUdid for Device {
    fn udid(&self) -> &str {
        &self.udid
    }
    fn set_udid(&mut self, udid: &str) {
        self.udid = udid.to_owned();
    }
}

#[test]
fn test_with_udid_gets_and_sets_udid() {
    let mut device = Device {
        udid: "u1".to_owned(),
    };
    device.set_udid("u2");
    assert_eq!("u2", device.udid());
}
