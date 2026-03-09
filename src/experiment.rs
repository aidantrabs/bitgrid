use crate::analysis;
use crate::automaton::Automaton;
use crate::rule::Rule;

pub struct Summary {
    pub rule_number: u8,
    pub final_density: f64,
    pub final_entropy: f64,
    pub mean_density: f64,
    pub mean_entropy: f64,
}

pub fn run(rule_number: u8, width: usize, generations: usize) -> Summary {
    let rule = Rule::new(rule_number);
    let mut automaton = Automaton::new(rule, width);
    let history = automaton.run(generations);

    let densities: Vec<f64> = history.iter().map(|row| analysis::density(row)).collect();
    let entropies: Vec<f64> = history.iter().map(|row| analysis::entropy(row)).collect();

    let n = densities.len() as f64;

    Summary {
        rule_number,
        final_density: *densities.last().unwrap(),
        final_entropy: *entropies.last().unwrap(),
        mean_density: densities.iter().sum::<f64>() / n,
        mean_entropy: entropies.iter().sum::<f64>() / n,
    }
}

pub fn print_table(summaries: &[Summary]) {
    println!(
        "{:<6} {:>10} {:>10} {:>10} {:>10}",
        "rule", "ρ final", "ρ mean", "h final", "h mean"
    );
    println!("{}", "-".repeat(50));
    for s in summaries {
        println!(
            "{:<6} {:>10.4} {:>10.4} {:>10.4} {:>10.4}",
            s.rule_number, s.final_density, s.mean_density, s.final_entropy, s.mean_entropy
        );
    }
}
