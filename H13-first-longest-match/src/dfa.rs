use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

static COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;

/// Represents a DFA-like NFA.
///
/// To simplify the implementation of the first-longest-match
/// algorithm, we omit transitions instead of using sink states.
/// Strictly speaking, we're now dealing with an NFA because you do not have
/// a transition from every state into another for every symbol
///
/// Other than that, this automaton works like a regular DFA, in that there can
/// only be one state the automaton is in.
/// Alternatively: given a symbol from the alphabet, there can only ever be one
/// transition with that symbol from one state to another.
#[derive(Clone, Debug)]
pub struct DFA {
    end_states: HashMap<State, HashSet<String>>,
    start_state: State,
    transitions: HashMap<State, HashMap<char, State>>
}

/// Represents an automaton state.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct State(usize);

impl DFA {
    /// Creates a new DFA.
    pub fn new(start_state: State) -> DFA {
        DFA {
            end_states: HashMap::new(),
            start_state: start_state,
            transitions: HashMap::new()
        }
    }

    /// Adds a transition to the state.
    pub fn add_transition(&mut self, from: State, with: char, to: State) {
        self.transitions.entry(from)
            .or_insert_with(|| HashMap::new())
            .insert(with, to);
    }

    /// Extends the DFA such that the given word leads to an end state when simulated.
    pub fn extend_with<I>(&mut self, with: I, token: &str)
            where I: IntoIterator<Item = char> {
        // Möchten wir nur ganze Wörter (ohne spezielle Symbole wie +, *, (, )) matchen
        // können wir unserem bestehenden DFA die Fähigkeit verleihen, auch das gegebene
        // Wort zu matchen, ohne über die Potenzmengenkonstruktion zu gehen.
        //
        // Dafür "simulieren" wir einfach das Wort solange bis uns eine Transition fehlt,
        // die für das Wort nötig wäre. Dort fügen wir nun einfach einen Zustand mit der
        // passenden Transition ein. Das ganze wird wiederholt, bis alle Buchstaben
        // als Transitionen im Automaten vorhanden sind.
        //
        // Hier kommt es zum Tragen, dass wir in Wirklichkeit mit einem NFA arbeiten und
        // nicht einem richtigen DFA. In einem DFA müssten wir hier unseren Pfad noch
        // zusätzlich auf das Vorhandensein von Senken prüfen, bzw. backtracing machen und
        // alle Loops verbieten. Denn sonst könnten wir Senken vom bereits bestehenden Automaten
        // zu Endzuständen machen, und das wäre fatal.
        let end = with.into_iter()
            .fold(self.start_state, |cur, letter| {
                *self.transitions.entry(cur)
                    .or_insert_with(|| HashMap::new())
                    .entry(letter)
                    .or_insert_with(|| State::new())
            });

        // Der letzte Zustand wird als Endzustand markiert, damit die Erkennung funktioniert.
        self.mark_end_state(end, token);
    }

    /// Obtains the start state of the automaton.
    pub fn get_start_state(&self) -> State {
        self.start_state
    }

    /// Makes the given state an end state of the automaton.
    pub fn mark_end_state(&mut self, s: State, token: &str) {
        self.end_states.entry(s)
            .or_insert_with(|| HashSet::new())
            .insert(token.to_string());
    }

    /// Performs the first-longest-match algorithm on the automaton.
    ///
    /// This is the algorithm of the example solution of T17.
    pub fn first_longest_match<I>(&self, input: I) -> Vec<(String, HashSet<String>)>
            where I: IntoIterator<Item = char> {
        // Wir brauchen einen speziellen Iterator, bei dem man nachschauen kann, ob
        // noch Elemente vorhanden sind, ohne sie dabei zu entfernen. Dafür das `.peekable()`.
        let mut input = input.into_iter().peekable();
        let mut results = Vec::new();

        while input.peek().is_some() {
            let mut cur_state = self.start_state;
            let mut chars = Vec::new();
            let mut intermediate = None;

            while let Some(&ch) = input.peek() {
                // Eingabe in temporärem Vektor sammeln, damit wir sie später
                // mit dem passenden Token wieder ausgeben können.
                chars.push(ch);

                // Versuche eine Transition vom aktuellen Zustand über das aktuelle
                // Zeichen zu finden
                let maybe_next = self.transitions.get(&cur_state)
                    .and_then(|trans| trans.get(&ch));

                match maybe_next {
                    // Falls es eine Transition gibt, gehe zu diesem Zustand und shifte
                    // den Iterator mit der Eingabe ein Zeichen weiter.
                    Some(&next) => {
                        cur_state = next;
                        input.next();

                        // Prüfe ob wir in einem Endzustand sind. Falls ja, notiere den
                        // aktuellen Input und das zugehörige Token in einer temporären
                        // Variable.
                        if let Some(tokens) = self.end_states.get(&cur_state) {
                            let until_now = chars.iter().cloned().collect();
                            intermediate = Some((until_now, tokens.clone()));
                        }
                    },

                    // Falls nicht, sind wir in einem Senkenzustand. Der Iterator wird
                    // NICHT geshiftet, da wir nun wieder vom Startzustand ausgehen müssen.
                    // Ende dieser Iteration.
                    //
                    // Sind wir noch im Startzustand und landen direkt in einem Senkenzustand
                    // returnen wir hier direkt, damit wir nicht in einem Infinite Loop stecken
                    // bleiben.
                    None => {
                        if cur_state != self.start_state {
                            break;
                        } else {
                            return results
                        }
                    }
                }
            }

            // Haben wir ein temporäres Ergebnis, konnte mindestens eine Zeichenkette (oder
            // ein Zeichen) geparsed werden. Also schieben wir es in den Ergebnisvektor.
            //
            // Haben wir keine temporären Ergebnisse ist die Eingabe ungültig und wir
            // terminieren nach Algorithmus aus T17.
            match intermediate {
                Some(x) => results.push(x),
                None => return results
            }
        }

        // Ergebnisse returnen
        results
    }
}

impl State {
    /// Creates a new State.
    pub fn new() -> State {
        // Jeder Zustand kriegt für die Unterscheidbarkeit einen atomisch
        // inkrementierten Wert (Integer) zugewiesen. Bis auf die mathematischen
        // Operatoren verhält sich `State` danach genau wie der zugrunde
        // liegende Integer selbst.
        State(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
}
