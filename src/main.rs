use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let mut args = std::env::args().skip(1);
    let input_path = args.next().expect("no file given");

    let input_file = std::fs::read_to_string(input_path).expect("failed to read file");
    let fa = FiniteAutomaton::new(input_file.as_str());

    loop {
        println!(
            "1. print set of states\n\
             2. print alphabet\n\
             3. print transitions\n\
             4. print initial state\n\
             5. print final states\n\
             6. check token\n\
             0. exit"
        );

        let mut option = String::new();
        let _ = std::io::stdin().read_line(&mut option);
        match option.trim() {
            "0" => break,
            "1" => println!("states = {:?}", fa.states),
            "2" => println!("alphabet = {:?}", fa.alphabet),
            "3" => println!("transitions = {:#?}", fa.transitions),
            "4" => println!("initial state = {}", fa.initial_state),
            "5" => println!("final states = {:?}", fa.final_states),
            "6" => {
                let mut token = String::new();
                let _ = std::io::stdin().read_line(&mut token);
                if fa.is_valid_token(token.trim()) {
                    println!("valid");
                } else {
                    println!("invalid");
                }
            }
            _ => eprintln!("invalid option"),
        }
    }
}

struct FiniteAutomaton<'a> {
    states: BTreeSet<&'a str>,
    alphabet: BTreeSet<char>,
    transitions: BTreeMap<&'a str, BTreeMap<char, &'a str>>,
    initial_state: &'a str,
    final_states: BTreeSet<&'a str>,
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
    
    fn new(s: &'a str) -> Self {
        let mut lines = s
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !(line.is_empty() || line.starts_with('#')));

        let states = lines
            .next()
            .expect("failed to get states line")
            .split(' ')
            .collect();
        let initial_state = lines.next().expect("failed to get initial state line");
        let final_states = lines
            .next()
            .expect("failed to get final states line")
            .split(' ')
            .collect();
        let alphabet = lines
            .next()
            .expect("failed to get alphabet line")
            .chars()
            .collect();

        let mut transitions = BTreeMap::new();
        for transition_line in lines {
            let mut transition = transition_line.split(' ');
            let state = transition.next().expect("failed to get transition state");
            let state_transitions = transitions.entry(state).or_insert_with(BTreeMap::new);
            while let (Some(symbols), Some(next_state)) = (transition.next(), transition.next()) {
                for symbol in symbols.chars() {
                    state_transitions.insert(symbol, next_state);
                }
            }
        }

        Self {
            states,
            alphabet,
            transitions,
            initial_state,
            final_states,
        }
    }
}
