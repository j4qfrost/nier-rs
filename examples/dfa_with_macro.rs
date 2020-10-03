use nier::*;
use nier_macros::*;

deserialize_dfa!(examples/dfa.ron);

fn main() {
    let mut machine = Machine::new();
    let mut current = Machine::initial();

    assert_eq!(current, SimpleState::Zero);

    current = Machine::delta(&current, SimpleAlphabet::A).unwrap();
    assert_eq!(current, SimpleState::One);
    assert!(Machine::accept(&current).is_ok());

    machine.current = Machine::delta(&current, SimpleAlphabet::B).unwrap();
    assert_eq!(current, SimpleState::One);

    machine.reset();
    assert_eq!(current, SimpleState::Zero);
}