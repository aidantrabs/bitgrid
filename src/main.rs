mod analysis;
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
    let show_density = args.iter().any(|a| a == "--density");
    let show_entropy = args.iter().any(|a| a == "--entropy");
    let positional: Vec<&String> = args.iter().skip(1).filter(|a| !a.starts_with("--")).collect();

    let rule_number: u8 = positional.first()
        .expect("usage: bitgrid <rule> [width] [generations] [--density] [--entropy]")
        .parse()
        .expect("rule must be 0-255");

    let width: usize = positional.get(1).map_or(81, |s| s.parse().expect("invalid width"));
    let generations: usize = positional.get(2).map_or(40, |s| s.parse().expect("invalid generations"));

    let rule = Rule::new(rule_number);
    let mut automaton = Automaton::new(rule, width);

    for _ in 0..generations {
        let mut prefix = String::new();
        if show_density {
            prefix.push_str(&format!("d={:.4} ", analysis::density(&automaton.cells)));
        }
        if show_entropy {
            prefix.push_str(&format!("h={:.4} ", analysis::entropy(&automaton.cells)));
        }
        println!("{}{}", prefix, render(&automaton.cells));
        automaton.step();
    }
}
