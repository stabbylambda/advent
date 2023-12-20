use std::fmt::Display;

use broadcaster::Broadcaster;
use conjunction::Conjunction;
use flip_flop::FlipFlop;

pub mod broadcaster;
pub mod conjunction;
pub mod flip_flop;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pulse {
    High,
    Low,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::High => write!(f, "-high->"),
            Pulse::Low => write!(f, "-low->"),
        }
    }
}

#[derive(Debug)]
pub enum ModuleKind {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster(Broadcaster),
}

impl<'a> ModuleKind {
    pub fn receive(&mut self, from: &'a str, pulse: Pulse) -> Option<Pulse> {
        match self {
            ModuleKind::FlipFlop(m) => m.receive(from, pulse),
            ModuleKind::Conjunction(m) => m.receive(from, pulse),
            ModuleKind::Broadcaster(m) => m.receive(from, pulse),
        }
    }
}
