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
    fn check_token(&self, token: &str) -> bool {
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
            while let (Some(symbol), Some(next_state)) = (state_trans.next(), state_trans.next()) {
                state_transitions.insert(
                    symbol.chars().next().expect("failed to get symbol"),
                    next_state,
                );
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
                write!(f, "    δ({}, \"{}\") = {}", state, symbol, next_state)?;
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

#[test]
/// checks if a sequence of characters is a valid identifier
/// a valid identifier contains only letters, digits or underscore and cannot start with a digit
/// more precisely, it follows the regex [A-Za-z_][A-Za-z0-9_]*
fn bonus() {
    let fa_in = "\
q1 q2 q3
q1
q3
_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890
q1 1 q2 2 q2 3 q2 4 q2 5 q2 6 q2 7 q2 8 q2 9 q2 0 q2
q2 1 q2 2 q2 3 q2 4 q2 5 q2 6 q2 7 q2 8 q2 9 q2 0 q2
q3 1 q3 2 q3 3 q3 4 q3 5 q3 6 q3 7 q3 8 q3 9 q3 0 q3
q1 _ q3 A q3 B q3 C q3 D q3 E q3 F q3 G q3 H q3 I q3 J q3 K q3 L q3 M q3 N q3 O q3 P q3 Q q3 R q3 S q3 T q3 U q3 V q3 W q3 X q3 Y q3 Z q3 a q3 b q3 c q3 d q3 e q3 f q3 g q3 h q3 i q3 j q3 k q3 l q3 m q3 n q3 o q3 p q3 q q3 r q3 s q3 t q3 u q3 v q3 w q3 x q3 y q3 z q3
q3 _ q3 A q3 B q3 C q3 D q3 E q3 F q3 G q3 H q3 I q3 J q3 K q3 L q3 M q3 N q3 O q3 P q3 Q q3 R q3 S q3 T q3 U q3 V q3 W q3 X q3 Y q3 Z q3 a q3 b q3 c q3 d q3 e q3 f q3 g q3 h q3 i q3 j q3 k q3 l q3 m q3 n q3 o q3 p q3 q q3 r q3 s q3 t q3 u q3 v q3 w q3 x q3 y q3 z q3
q2 _ q2 A q2 B q2 C q2 D q2 E q2 F q2 G q2 H q2 I q2 J q2 K q2 L q2 M q2 N q2 O q2 P q2 Q q2 R q2 S q2 T q2 U q2 V q2 W q2 X q2 Y q2 Z q2 a q2 b q2 c q2 d q2 e q2 f q2 g q2 h q2 i q2 j q2 k q2 l q2 m q2 n q2 o q2 p q2 q q2 r q2 s q2 t q2 u q2 v q2 w q2 x q2 y q2 z q2
";

    let fa = FiniteAutomaton::from(fa_in);
    assert!(fa.check_token("_a1askfhjbJHKGBKJHG__21342"));
    assert!(!fa.check_token("1adad2132_a"));
    assert!(!fa.check_token(""));
}
