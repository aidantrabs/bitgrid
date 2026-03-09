use crate::rule::Rule;

pub struct Automaton {
    pub cells: Vec<u8>,
    rule: Rule,
}

impl Automaton {
    pub fn new(rule: Rule, width: usize) -> Self {
        let mut cells = vec![0u8; width];
        cells[width / 2] = 1;
        Self { cells, rule }
    }

    pub fn run(&mut self, generations: usize) -> Vec<Vec<u8>> {
        let mut history = Vec::with_capacity(generations);
        for _ in 0..generations {
            history.push(self.cells.clone());
            self.step();
        }
        history
    }

    pub fn step(&mut self) {
        let len = self.cells.len();
        let prev = self.cells.clone();
        for i in 0..len {
            let left = if i == 0 { 0 } else { prev[i - 1] };
            let center = prev[i];
            let right = if i == len - 1 { 0 } else { prev[i + 1] };
            self.cells[i] = self.rule.apply(left, center, right);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_step_rule_90() {
        let rule = Rule::new(90);
        let mut automaton = Automaton::new(rule, 5);

        // initial: [0, 0, 1, 0, 0]
        assert_eq!(automaton.cells, vec![0, 0, 1, 0, 0]);

        automaton.step();
        // rule 90 = xor(left, right)
        // cell 0: xor(0, 0) = 0
        // cell 1: xor(0, 1) = 1
        // cell 2: xor(0, 0) = 0
        // cell 3: xor(1, 0) = 1
        // cell 4: xor(0, 0) = 0
        assert_eq!(automaton.cells, vec![0, 1, 0, 1, 0]);
    }
}
