use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Represents a nondeterministic finite automaton.
pub struct NFA<S, A> {
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
    pub fn new() -> NFA<S, A> {
        NFA {
            transitions: HashMap::new()
        }
    }

    /// Adds a transition to the NFA.
    pub fn add_transition(&mut self, from: S, with: A, to: S) {
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
        // Wir bauen uns ein Set mit den Zuständen, in denen der Automat
        // aktuell sein kann. Anfangs ist es nur der gegebene Startzustand.
        let mut start_state = HashSet::new();
        start_state.insert(from);

        // .fold ist eine Akkumulator-Funktion, durch die alle Buchstaben aus
        // dem gegebenen Wort in die möglichen Zustände "reduziert" werden.
        //
        // In anderen Sprachen bekannt als `reduce` oder `aggregate`, etc.
        with.into_iter()
            .fold(start_state, |possible_states, word| {
                // Nun gehen wir die möglichen Zustände des Automaten durch.
                // Wir schauen nach Transitions von unserem aktuellen Zustand
                // über das aktuelle Wort. Falls wir welche finden, vereinen
                // wir die Mengen der gefundenen Zustände zu einer großen,
                // von der wir dann im nächsten `fold`-Schritt wieder ausgehen.
                //
                // .filter_map mappt alle Elemente der Auflistung auf einen
                // anderen, und filtert dabei direkt die `null`-Werte heraus.
                // Somit überspringen wir direkt alle Zustände von denen es
                // keine Transitionen gibt und die Transitionen, die nicht
                // von dem gegebenen Wort ausgelöst werden.
                possible_states.into_iter()
                    .filter_map(|state| self.transitions.get(&state))
                    .filter_map(|trans| trans.get(&word))
                    .fold(HashSet::new(), |set, states| &set | states)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let mut nfa = NFA::new();

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
        let mut nfa = NFA::new();

        nfa.add_transition(1, "b", 2);

        let input = &["a", "a", "a", "b", "a"];
        let expected_result = HashSet::new();

        assert_eq!(nfa.simulate(1, input), expected_result);
    }
}
