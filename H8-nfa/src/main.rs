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

        for word in with {
            let mut new_states = HashSet::new();

            // Nun gehen wir die möglichen Zustände des Automaten durch.
            // Wir schauen nach Transitions von unserem aktuellen Zustand
            // über das aktuelle Wort. Falls wir welche finden, fügen wir
            // die neuen möglichen Zustände in die Liste der aktuell möglichen
            // Zustände ein.
            for state in possible_states {
                self.transitions.get(&state)
                    .and_then(|dict| dict.get(&word))
                    .map(|set| new_states.extend(set.clone()));
            }

            possible_states = new_states;

            // Kleine Optimierung: Falls es keine Zustände gibt, in denen
            // wir aktuell sein können, können wir die Schleife unterbrechen,
            // denn es gibt von hier aus keine Möglichkeit mehr, wieder in
            // einen Zustand zu gelangen.
            if possible_states.is_empty() {
                break;
            }
        }

        possible_states
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
        let states = vec![1, 2, 3, 4];
        let mut nfa = NFA::new(states);

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
        let states = vec![1, 2];
        let mut nfa = NFA::new(states);

        nfa.add_transition(1, "b", 2);

        let input = &["a", "a", "a", "b", "a"];
        let expected_result = HashSet::new();

        assert_eq!(nfa.simulate(1, input), expected_result);
    }
}