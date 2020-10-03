use std::collections::HashSet;
use std::hash::Hash;

pub trait State {}
pub trait Alphabet {}

impl<T: Eq + PartialEq + Hash + Copy + Clone> State for T {}
impl<T: Copy + Clone> Alphabet for T {}

pub trait Epsilon: Alphabet {
    fn epsilon() -> Self;
}

#[derive(Debug)]
pub enum Reject<S: State, I> {
    NotAccept(S),
    InvalidInput(I),
    ThisCannotContinue,
}

pub trait Automaton<S>
where
    S: State,
{
}

pub trait Deterministic<S: State, I: Alphabet>: Automaton<S> {
    fn initial() -> S;
    fn delta(state: &S, input: I) -> Result<S, Reject<S, I>>;
}

pub trait NonDeterministic<S: State, I: Alphabet>: Automaton<S> {
    fn inital() -> HashSet<S>;
    fn delta(states: &HashSet<S>, input: I) -> Result<HashSet<S>, Reject<S, I>>;
}

pub trait Acceptor<S: State>: Automaton<S> {
    fn accept(state: &S) -> Result<S, Reject<S, ()>>;
}

pub trait Transducer<S: State, I: Alphabet, O>: Automaton<S> {
    fn omega(state: &S, input: O) -> Result<O, Reject<S, I>>;
}
