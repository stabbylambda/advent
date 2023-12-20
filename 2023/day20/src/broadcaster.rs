use crate::Pulse;

#[derive(Debug)]
pub struct Broadcaster {}

impl<'a> Broadcaster {
    pub fn new() -> Self {
        Self {}
    }
    pub fn receive(&'a mut self, _from: &'a str, pulse: Pulse) -> Option<Pulse> {
        Some(pulse)
    }
}

impl Default for Broadcaster {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn broadcaster_test() {
    let mut b = Broadcaster::new();

    assert_eq!(b.receive("button", Pulse::High), Some(Pulse::High));
    assert_eq!(b.receive("button", Pulse::Low), Some(Pulse::Low));
}
