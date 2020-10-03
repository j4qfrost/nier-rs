use nier::*;
use nier_macros::*;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum SimpleState {
    Zero,
    One,
}

#[derive(Debug, Copy, Clone)]
enum SimpleAlphabet {
    A,
    B,
}

#[derive(Automaton, Deterministic, Acceptor)]
// #[state = "SimpleState"]
// #[alphabet = "SimpleAlphabet"]
// #[source = "examples/dfa.ron"]
#[nier(
    state = "SimpleState",
    alphabet = "SimpleAlphabet",
    source = "examples/dfa.ron"
)]
struct Machine {
    current: SimpleState,
}

impl Machine {
    fn new() -> Self {
        Self {
            current: Self::initial(),
        }
    }

    fn reset(&mut self) {
        self.current = Self::initial();
    }
}

fn main() {
    let mut machine = Machine::new();

    assert_eq!(machine.current, SimpleState::Zero);

    machine.current = Machine::delta(&machine.current, SimpleAlphabet::A).unwrap();
    assert_eq!(machine.current, SimpleState::One);
    assert!(Machine::accept(&machine.current).is_ok());

    machine.current = Machine::delta(&machine.current, SimpleAlphabet::B).unwrap();
    assert_eq!(machine.current, SimpleState::One);

    machine.reset();
    assert_eq!(machine.current, SimpleState::Zero);
}
