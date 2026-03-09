pub fn density(cells: &[u8]) -> f64 {
    let alive: usize = cells.iter().map(|&c| c as usize).sum();
    alive as f64 / cells.len() as f64
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
}
