use crate::Pulse;

#[derive(Debug)]
pub struct FlipFlop {
    state: bool,
}

impl<'a> FlipFlop {
    pub fn new() -> Self {
        Self { state: false }
    }
    pub fn receive(&'a mut self, _from: &'a str, pulse: Pulse) -> Option<Pulse> {
        match (pulse, self.state) {
            (Pulse::High, _) => None,
            (Pulse::Low, true) => {
                self.state = false;
                Some(Pulse::Low)
            }
            (Pulse::Low, false) => {
                self.state = true;
                Some(Pulse::High)
            }
        }
    }
}

impl Default for FlipFlop {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn flip_flop_test() {
    let mut f = FlipFlop::new();
    // starts off
    assert!(!f.state);

    // ignores High
    assert_eq!(f.receive("foo", Pulse::High), None);

    // If it was off, it turns on and sends a high pulse
    assert_eq!(f.receive("foo", Pulse::Low), Some(Pulse::High));
    assert!(f.state);

    // If it was on, it turns off and sends a low pulse.
    assert_eq!(f.receive("foo", Pulse::Low), Some(Pulse::Low));
    assert!(!f.state);
}
