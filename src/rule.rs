pub struct Rule {
    number: u8,
}

impl Rule {
    pub fn new(number: u8) -> Self {
        Self { number }
    }

    pub fn apply(&self, left: u8, center: u8, right: u8) -> u8 {
        let index = (left << 2) | (center << 1) | right;
        (self.number >> index) & 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_30_lookup() {
        let rule = Rule::new(30);

        // 30 = 0b00011110
        // pattern: 111 110 101 100 011 010 001 000
        // output:    0   0   0   1   1   1   1   0
        assert_eq!(rule.apply(1, 1, 1), 0);
        assert_eq!(rule.apply(1, 1, 0), 0);
        assert_eq!(rule.apply(1, 0, 1), 0);
        assert_eq!(rule.apply(1, 0, 0), 1);
        assert_eq!(rule.apply(0, 1, 1), 1);
        assert_eq!(rule.apply(0, 1, 0), 1);
        assert_eq!(rule.apply(0, 0, 1), 1);
        assert_eq!(rule.apply(0, 0, 0), 0);
    }

    #[test]
    fn rule_90_is_xor() {
        let rule = Rule::new(90);

        for l in 0..=1u8 {
            for c in 0..=1u8 {
                for r in 0..=1u8 {
                    assert_eq!(rule.apply(l, c, r), l ^ r);
                }
            }
        }
    }
}
