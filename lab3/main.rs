use std::collections::{HashMap, HashSet};

fn main() {
    let mut args = std::env::args().skip(1);
    let input_path = args.next().expect("no file given");

    let input_file = std::fs::read_to_string(input_path).expect("failed to read file");
    let fa = FiniteAutomaton::from(input_file.as_str());

    loop {
        println!(
            "\
1. set of states
2. alphabet
3. transitions
4. initial state
5. final states
6. check token
0. exit
"
        );

        let mut option = String::new();
        let _ = std::io::stdin().read_line(&mut option);
        match option.trim() {
            "0" => break,
            "1" => fa.print_states(),
            "2" => fa.print_alphabet(),
            "3" => fa.print_transitions(),
            "4" => fa.print_initial_state(),
            "5" => fa.print_final_states(),
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

    fn print_states(&self) {
        print!("states Q = {{");
        for state in self.states.iter() {
            print!(" {}", state);
        }
        println!(" }}");
    }

    fn print_alphabet(&self) {
        print!("alphabet Σ = {{");
        for symbol in self.alphabet.iter() {
            print!(r#" "{}""#, symbol);
        }
        println!(" }}");
    }

    fn print_transitions(&self) {
        print!("transitions");
        for (state, transitions) in self.transitions.iter() {
            println!();
            for (symbol, next_state) in transitions.iter() {
                print!(r#"    δ({}, "{}") = {}"#, state, symbol, next_state);
            }
        }
        println!();
    }

    fn print_initial_state(&self) {
        println!("initial state {}", self.initial_state);
    }

    fn print_final_states(&self) {
        print!("final states F = {{");
        for final_state in self.final_states.iter() {
            print!(" {}", final_state);
        }
        println!(" }}");
    }
}

impl<'a> From<&'a str> for FiniteAutomaton<'a> {
    fn from(s: &'a str) -> Self {
        let mut lines = s
            .split('\n')
            .map(|line| line.trim())
            .filter(|line| !line.is_empty());

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

        let mut transitions = HashMap::new();
        for transition_line in lines {
            let mut transition = transition_line.split(' ');
            let state = transition.next().expect("failed to get transition state");
            let state_transitions = transitions.entry(state).or_insert_with(HashMap::new);
            while let (Some(symbols), Some(next_state)) = (transition.next(), transition.next()) {
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
