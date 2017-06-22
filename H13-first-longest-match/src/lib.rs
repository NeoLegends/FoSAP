//! An excersise regarding the evaluation of multiple regexes at the same time.

use std::char as ch;
use std::collections::HashSet;

mod dfa;

pub use dfa::*;

/// Simulates the DFA from the exercise with the given RegEx / Token
/// combo and some input string.
pub fn simulate(reg_exs: &[(&str, &str)], input: &str) -> Vec<(String, HashSet<String>)> {
    let mut dfa = DFA::new(State::new());

    // Füge die hardkodierten Regular Expressions ein, die Teil jeder Eingabe sein sollen.
    add_default_states(&mut dfa);
    for &(regex, token) in reg_exs {
        dfa.extend_with(regex.chars(), token);
    }

    dfa.first_longest_match(input.chars())
}

/// This function adds the default states that are part of any input
/// to the given automaton.
fn add_default_states(a: &mut DFA) {
    let end_id = State::new();
    let end_num = State::new();
    let sink = State::new();
    let start = a.get_start_state();

    for i in 0..10 { // Numbers 0...9
        let ch = ch::from_digit(i, 10).unwrap();
        a.add_transition(start, ch, end_num);
        a.add_transition(end_num, ch, end_num);
        a.add_transition(sink, ch, sink);
    }
    for x in 97..123 { // Digits a...z
        let ch = x as u8 as char;
        a.add_transition(end_num, ch, sink);
        a.add_transition(sink, ch, sink);
    }

    for i in 0..10 {
        a.add_transition(end_id, ch::from_digit(i, 10).unwrap(), end_id);
    }
    for x in 97..123 {
        let ch = x as u8 as char;
        a.add_transition(start, ch, end_id);
        a.add_transition(end_id, ch, end_id);
    }

    a.mark_end_state(end_id, "rId");
    a.mark_end_state(end_num, "rNum");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sim1() {
        static REGEXES_WITH_TOKENS: &'static[(&'static str, &'static str)] = &[
            ("abc", "ABC"),
            ("<", "SMALLER"),
            ("<<", "DOUBLE SMALLER")
        ];

        run_test(REGEXES_WITH_TOKENS, "abc<<dd");
    }

    #[test]
    fn sim2() {
        static REGEXES_WITH_TOKENS: &'static[(&'static str, &'static str)] = &[
            ("<", "SMALLER")
        ];

        run_test(REGEXES_WITH_TOKENS, "<<<<<");
    }

    #[test]
    fn sim3() {
        static REGEXES_WITH_TOKENS: &'static[(&'static str, &'static str)] = &[
            ("for", "FOR"),
            ("if", "IF"),
            ("while", "WHILE"),
            (";", "SEMICOLON"),
            ("<", "SMALLER"),
            ("+", "PLUS"),
            ("++", "PLUSPLUS"),
            ("(", "ROUND_BRACE_OPEN"),
            (")", "ROUND_BRACE_CLOSE"),
            ("{", "CURLY_BRACE_OPEN"),
            ("}", "CURLY_BRACE_CLOSE"),
            ("=", "EQUALS"),
        ];

        run_test(REGEXES_WITH_TOKENS, "for(i=0;i<10;i++){print(i);}");
    }

    #[test]
    fn sim4() {
        static REGEXES_WITH_TOKENS: &'static[(&'static str, &'static str)] = &[
            ("for", "FOR"),
            ("if", "IF"),
            ("while", "WHILE"),
            (";", "SEMICOLON"),
            ("<", "SMALLER"),
            ("+", "PLUS"),
            ("++", "PLUSPLUS"),
            ("(", "ROUND_BRACE_OPEN"),
            (")", "ROUND_BRACE_CLOSE"),
            ("{", "CURLY_BRACE_OPEN"),
            ("}", "CURLY_BRACE_CLOSE"),
            ("=", "EQUALS"),
            (" ", "SPACE"),
            ("\t", "TAB"),
        ];

        run_test(REGEXES_WITH_TOKENS, "for (i = 0; i < 10; i++) {\tprint(i); }");
    }

    fn run_test(regexps: &[(&str, &str)], input: &str) {
        println!(
            "Input: '{}'\nRegular Expressions:\n{:#?}\nOutput:\n{:#?}\n",
            input, regexps, simulate(regexps, input)
        );
    }
}