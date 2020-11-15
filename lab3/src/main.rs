use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    iter::FromIterator,
};

fn main() {
    let input_path = std::env::args().skip(1).next().expect("no file given");
    let input = std::fs::read_to_string(input_path).expect("failed to read file");
    let fa = FiniteAutomaton::from(input.as_str());
    println!("{}", fa);
}

struct FiniteAutomaton<'a> {
    states: HashSet<&'a str>,
    initial_state: &'a str,
    final_states: HashSet<&'a str>,
    alphabet: HashSet<char>,
    transitions: HashMap<&'a str, HashMap<char, &'a str>>,
}

impl<'a> FiniteAutomaton<'a> {
    fn is_valid_token(&self, token: &str) -> bool {
        let mut current_state = self.initial_state;
        for symbol in token.chars() {
            current_state = self
                .transitions
                .get(current_state)
                .expect("state has no transitions")
                .get(&symbol)
                .expect("unexpected transition symbol");
        }
        self.final_states.contains(current_state)
    }
}

impl<'a> From<&'a str> for FiniteAutomaton<'a> {
    fn from(s: &'a str) -> Self {
        let mut lines = s.split('\n');

        let states = lines
            .next()
            .expect("failed to get states line")
            .trim()
            .split(' ');
        let states = HashSet::from_iter(states);

        let initial_state = lines
            .next()
            .expect("failed to get initial state line")
            .trim();

        let final_states = lines
            .next()
            .expect("failed to get final states line")
            .trim()
            .split(' ');
        let final_states = HashSet::from_iter(final_states);

        let alphabet = lines.next().expect("failed to get alphabet line").chars();
        let alphabet = HashSet::from_iter(alphabet);

        let mut transitions = HashMap::new();
        for transition_line in lines {
            let transition_line = transition_line.trim();
            if transition_line.is_empty() {
                continue;
            }
            let mut state_trans = transition_line.split(' ');
            let state = state_trans.next().expect("failed to get transition state");
            let state_transitions = transitions.entry(state).or_insert(HashMap::new());
            while let (Some(symbols), Some(next_state)) = (state_trans.next(), state_trans.next()) {
                for symbol in symbols.chars() {
                    state_transitions.insert(symbol, next_state);
                }
            }
        }

        Self {
            states,
            initial_state,
            final_states,
            alphabet,
            transitions,
        }
    }
}

impl<'a> Display for FiniteAutomaton<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "states Q = {{")?;
        for state in self.states.iter() {
            write!(f, " {}", state)?;
        }

        write!(f, " }}\nalphabet Σ = {{",)?;
        for symbol in self.alphabet.iter() {
            write!(f, r#" "{}""#, symbol)?;
        }

        write!(f, " }}\ntransitions")?;
        for (state, transitions) in self.transitions.iter() {
            write!(f, "\n")?;
            for (symbol, next_state) in transitions.iter() {
                write!(f, r#"    δ({}, "{}") = {}"#, state, symbol, next_state)?;
            }
        }

        write!(f, "\ninitial state {}", self.initial_state)?;

        write!(f, "\nfinal states F = {{")?;
        for final_state in self.final_states.iter() {
            write!(f, " {}", final_state)?;
        }

        writeln!(f, " }}")
    }
}

/// checks if a sequence of characters is a valid identifier
/// a valid identifier contains only letters, digits or underscore and cannot start with a digit
/// more precisely, it follows the regex [A-Za-z_][A-Za-z0-9_]*
#[test]
fn bonus() {
    let fa_in = "\
Init Invalid Valid
Init
Valid
_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890
Init 1234567890 Invalid _ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz Valid
Invalid _ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 Invalid
Valid _ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 Valid
";

    let fa = FiniteAutomaton::from(fa_in);
    assert!(fa.is_valid_token("valid_123"), r#""valid_123" should be valid"#);
    assert!(!fa.is_valid_token("1_begins_with_digit"), r#""1_begins_with_digit" should be invalid"#);
    assert!(!fa.is_valid_token(""), "empty string should be invalid");
}
