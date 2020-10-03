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

type SimpleReject = Reject<SimpleState, SimpleAlphabet>;

#[derive(Automaton)]
#[state = "SimpleState"]
#[alphabet = "SimpleAlphabet"]
#[source = "example/dfa.ron"]
struct Machine {
    current: SimpleState,
}

impl Deterministic<SimpleState, SimpleAlphabet> for Machine {
    fn initial() -> SimpleState {
        SimpleState::Zero
    }

    fn delta(
        state: &SimpleState,
        input: SimpleAlphabet,
    ) -> Result<SimpleState, SimpleReject> {
        match (state, input) {
            (SimpleState::Zero, SimpleAlphabet::A) => Ok(SimpleState::One),
            (SimpleState::Zero, SimpleAlphabet::B) => Ok(SimpleState::Zero),
            (SimpleState::One, SimpleAlphabet::B) => Ok(SimpleState::One),
            _ => Err(Reject::InvalidInput(input.clone())),
        }
    }
}

impl Acceptor<SimpleState> for Machine {
    fn accept(current: &SimpleState) -> Result<SimpleState, Reject<SimpleState, ()>> {
        match current {
            SimpleState::Zero => Err(Reject::NotAccept(current.clone())),
            SimpleState::One => Ok(current.clone()),
        }
    }
}

impl Machine {
    fn new() -> Self {
        Self { current: Self::initial() }
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