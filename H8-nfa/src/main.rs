//! A naive implementation of an NFA in Rust.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FromIterator;

/// Represents a nondeterministic finite automaton.
struct NFA<S, A> {
    /// The states the NFA can be in.
    ///
    /// This field is only used for validation when inserting
    /// new transitions.
    states: HashSet<S>,

    /// This contains the transitions from state S over A
    /// to some other given state(s) S.
    ///
    /// Since we're simulating an NFA we can have more than one
    /// destination state for a given transition. Using a HashSet
    /// makes sure we don't store duplicates.
    transitions: HashMap<S, HashMap<A, HashSet<S>>>
}

impl<S, A> NFA<S, A>
        where S: Clone + Eq + Hash,
              A: Eq + Hash {
    /// Creates a new NFA with the given states.
    pub fn new<I>(states: I) -> NFA<S, A>
            where I: IntoIterator<Item = S> {
        NFA {
            states: HashSet::from_iter(states),
            transitions: HashMap::new()
        }
    }

    /// Adds a transition to the NFA.
    pub fn add_transition(&mut self, from: S, with: A, to: S) {
        assert!(
            self.states.contains(&from) && self.states.contains(&to),
            "Transition from or to unknown state."
        );

        // .entry(...) holt sich den Wert aus der HashMap mit dem jeweiligen Key.
        // Falls er nicht existiert, wird er mit .or_insert_with(...) eingefügt.
        // Damit stellen wir sicher, dass alles nötige initialisiert ist.
        self.transitions.entry(from)
            .or_insert_with(|| HashMap::new())
            .entry(with)
            .or_insert_with(|| HashSet::new())
            .insert(to);
    }

    /// Simulates the NFA from the given starting point.
    pub fn simulate(&self, from: S, with: &[A]) -> HashSet<S> {
        assert!(self.states.contains(&from), "Unknown state!");

        // Wir bauen uns ein Set mit den Zuständen, in denen der Automat
        // aktuell sein kann. Anfangs ist es nur der gegebene Startzustand.
        let mut possible_states = HashSet::new();
        possible_states.insert(from);

        // .fold ist eine Akkumulator-Funktion, durch die alle Buchstaben aus
        // dem gegebenen Wort in die möglichen Zustände "reduziert" werden.
        //
        // In anderen Sprachen bekannt als .reduce, .aggregate, etc.
        with.into_iter()
            .fold(possible_states, |possible_states, word| {
                // Nun gehen wir die möglichen Zustände des Automaten durch.
                // Wir schauen nach Transitions von unserem aktuellen Zustand
                // über das aktuelle Wort. Falls wir welche finden, vereinen
                // wir die Mengen der gefundenen Zustände zu einer großen,
                // von der wir dann im nächsten 'fold'-Schritt wieder ausgehen.
                possible_states.into_iter()
                    .filter_map(|state| self.transitions.get(&state))
                    .filter_map(|trans| trans.get(&word))
                    .fold(HashSet::new(), |set, states| &set | states)
            })
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let mut nfa = NFA::new(vec![1, 2, 3, 4]);

        nfa.add_transition(1, "a", 1);

        nfa.add_transition(1, "a", 4);
        nfa.add_transition(4, "a", 4);
        nfa.add_transition(4, "b", 4);

        nfa.add_transition(1, "b", 2);
        nfa.add_transition(2, "b", 3);

        let input = &["a", "a", "a", "b"];
        let expected_result = {
            let mut set = HashSet::new();
            set.insert(2);
            set.insert(4);
            set
        };

        assert_eq!(nfa.simulate(1, input), expected_result);
    }

    #[test]
    fn inaccessible() {
        let mut nfa = NFA::new(vec![1, 2]);

        nfa.add_transition(1, "b", 2);

        let input = &["a", "a", "a", "b", "a"];
        let expected_result = HashSet::new();

        assert_eq!(nfa.simulate(1, input), expected_result);
    }

    #[test]
    #[should_panic]
    fn simulation_validation() {
        let mut nfa = NFA::new(vec![1, 2]);
        nfa.add_transition(1, "a", 2);

        nfa.simulate(3, &[]);
    }

    #[test]
    #[should_panic]
    fn transition_validation() {
        let mut nfa = NFA::new(vec![1, 2]);
        nfa.add_transition(4, "a", 5);
    }
}