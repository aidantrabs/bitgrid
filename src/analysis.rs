pub fn density(cells: &[u8]) -> f64 {
    let alive: usize = cells.iter().map(|&c| c as usize).sum();
    alive as f64 / cells.len() as f64
}

pub fn entropy(cells: &[u8]) -> f64 {
    let p = density(cells);
    if p == 0.0 || p == 1.0 {
        return 0.0;
    }
    -(p * p.log2() + (1.0 - p) * (1.0 - p).log2())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn density_single_cell() {
        let cells = vec![0, 0, 1, 0, 0];
        assert!((density(&cells) - 0.2).abs() < f64::EPSILON);
    }

    #[test]
    fn density_all_alive() {
        let cells = vec![1, 1, 1, 1];
        assert!((density(&cells) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn density_empty() {
        let cells = vec![0, 0, 0];
        assert!(density(&cells).abs() < f64::EPSILON);
    }

    #[test]
    fn entropy_uniform() {
        let cells = vec![0, 1, 0, 1, 0, 1];
        assert!((entropy(&cells) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn entropy_homogeneous() {
        assert!(entropy(&vec![0, 0, 0]).abs() < f64::EPSILON);
        assert!(entropy(&vec![1, 1, 1]).abs() < f64::EPSILON);
    }
}
