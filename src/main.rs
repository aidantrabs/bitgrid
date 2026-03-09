mod automaton;
mod rule;

use automaton::Automaton;
use rule::Rule;

fn render(cells: &[u8]) -> String {
    cells.iter().map(|&c| if c == 1 { '█' } else { ' ' }).collect()
}

fn main() {
    let rule = Rule::new(30);
    let mut automaton = Automaton::new(rule, 81);

    for _ in 0..40 {
        println!("{}", render(&automaton.cells));
        automaton.step();
    }
}
