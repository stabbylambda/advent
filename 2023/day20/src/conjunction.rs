use std::collections::BTreeMap;

use crate::Pulse;

#[derive(Debug)]
pub struct Conjunction {
    last_pulses: BTreeMap<String, Pulse>,
}

impl<'a> Conjunction {
    pub fn new(incoming: &[&str]) -> Self {
        Self {
            last_pulses: incoming
                .iter()
                .map(|name| (name.to_string(), Pulse::Low))
                .collect(),
        }
    }

    pub fn receive(&mut self, from: &'a str, pulse: Pulse) -> Option<Pulse> {
        self.last_pulses.insert(from.to_string(), pulse);

        if self.last_pulses.values().all(|p| *p == Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }
}

#[test]
fn conjunction_test() {
    let mut c = Conjunction::new(&["foo", "bar", "baz"]);

    assert_eq!(c.receive("foo", Pulse::High), Some(Pulse::High));
    assert_eq!(c.receive("bar", Pulse::High), Some(Pulse::High));
    assert_eq!(c.receive("baz", Pulse::High), Some(Pulse::Low));
    assert_eq!(c.receive("baz", Pulse::Low), Some(Pulse::High));
    assert_eq!(c.receive("baz", Pulse::High), Some(Pulse::Low));
    assert_eq!(c.receive("bar", Pulse::High), Some(Pulse::Low));
}
