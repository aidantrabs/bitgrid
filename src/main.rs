mod automaton;
mod rule;

use automaton::Automaton;
use rule::Rule;
use std::env;

fn render(cells: &[u8]) -> String {
    cells.iter().map(|&c| if c == 1 { '█' } else { ' ' }).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let rule_number: u8 = args.get(1)
        .expect("usage: bitgrid <rule> [width] [generations]")
        .parse()
        .expect("rule must be 0-255");

    let width: usize = args.get(2).map_or(81, |s| s.parse().expect("invalid width"));
    let generations: usize = args.get(3).map_or(40, |s| s.parse().expect("invalid generations"));

    let rule = Rule::new(rule_number);
    let mut automaton = Automaton::new(rule, width);

    for _ in 0..generations {
        println!("{}", render(&automaton.cells));
        automaton.step();
    }
}
