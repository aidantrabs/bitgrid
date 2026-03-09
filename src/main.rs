mod analysis;
mod automaton;
mod experiment;
mod render;
mod rule;

use automaton::Automaton;
use rule::Rule;
use std::env;
use std::path::Path;

fn render_line(cells: &[u8]) -> String {
    cells.iter().map(|&c| if c == 1 { '█' } else { ' ' }).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let show_density = args.iter().any(|a| a == "--density");
    let show_entropy = args.iter().any(|a| a == "--entropy");
    let save_png = args.iter().any(|a| a == "--png");
    let compare = args.iter().any(|a| a == "--compare");
    let positional: Vec<&String> = args.iter().skip(1).filter(|a| !a.starts_with("--")).collect();

    if compare {
        let width: usize = positional.first().map_or(201, |s| s.parse().expect("invalid width"));
        let generations: usize = positional.get(1).map_or(100, |s| s.parse().expect("invalid generations"));
        let rules = [30, 90, 110, 184, 0, 255];
        let summaries: Vec<_> = rules.iter().map(|&r| experiment::run(r, width, generations)).collect();
        experiment::print_table(&summaries);
        return;
    }

    let rule_number: u8 = positional.first()
        .expect("usage: bitgrid <rule> [width] [generations] [--density] [--entropy] [--png] [--compare]")
        .parse()
        .expect("rule must be 0-255");

    let width: usize = positional.get(1).map_or(81, |s| s.parse().expect("invalid width"));
    let generations: usize = positional.get(2).map_or(40, |s| s.parse().expect("invalid generations"));

    let rule = Rule::new(rule_number);
    let mut automaton = Automaton::new(rule, width);

    if save_png {
        let history = automaton.run(generations);
        let filename = format!("output/rule_{}.png", rule_number);
        std::fs::create_dir_all("output").expect("failed to create output directory");
        render::save_png(&history, Path::new(&filename), 4);
        println!("saved {}", filename);
    } else {
        for _ in 0..generations {
            let mut prefix = String::new();
            if show_density {
                prefix.push_str(&format!("d={:.4} ", analysis::density(&automaton.cells)));
            }
            if show_entropy {
                prefix.push_str(&format!("h={:.4} ", analysis::entropy(&automaton.cells)));
            }
            println!("{}{}", prefix, render_line(&automaton.cells));
            automaton.step();
        }
    }
}
