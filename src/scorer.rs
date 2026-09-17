// scorer.rs

use std::cmp;

pub fn g4hunter_scorer(nucl_byte_slice: &[u8]) -> (f64, usize) {
    // Score a nucleotide slice based on g4hunter scores.
    // Returns a tuple (g4hunter score as a float, scaled g4hunter score (0,1000) for BED files)
    if nucl_byte_slice.is_empty() {
        return (0.0, 0);
    }
    let mut score: i32 = 0;
    let chunks = nucl_byte_slice.chunk_by(|a, b| a.eq_ignore_ascii_case(b));
    for nucl_runs in chunks {
        let size: i32 = nucl_runs.len().try_into().unwrap();
        let current_score: i32 = match nucl_runs.first() {
            Some(65) => 0,                           // A
            Some(84) => 0,                           // T
            Some(71) => cmp::min(size, 4) * size,    // G
            Some(67) => -(cmp::min(size, 4) * size), // C
            Some(103) => cmp::min(size, 4) * size,   // lowercase g
            Some(99) => -(cmp::min(size, 4) * size), // lowercase c
            _ => 0,
        };
        score += current_score
    }

    let length: f64 = nucl_byte_slice.len() as f64;
    let float_score: f64 = score as f64 / length;

    let scaled_score: usize = if float_score.abs() as usize <= 4 {
        (float_score.abs() * 250.0).trunc() as usize
    } else {
        1000
    };

    (float_score, scaled_score)
}

pub fn filter_g4score(score: f64, filter: f64) -> bool {
    score.abs() >= filter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_g4hunter_scorer() {
        assert_eq!(
            g4hunter_scorer(b"CAATCGGATCGAATTCGATCCGATTGAAAAAAA"),
            (0.0, 0)
        );
        assert_eq!(g4hunter_scorer(b"GGGTTAGGG"), (2.0, 500));
        assert_eq!(g4hunter_scorer(b""), (0.0, 0));
        assert_eq!(
            g4hunter_scorer(b"GGGTTAGGGTTAGGGTTAGGG"),
            (1.7142857142857142, 428)
        );
        assert_eq!(
            g4hunter_scorer(b"CCCTTACCCTTACCCTTACCC"),
            (-1.7142857142857142, 428)
        );
        assert_eq!(g4hunter_scorer(b"GGGAGGGAGGGAGGG"), (2.4, 600));
    }

    #[test]
    fn test_g4score_filter() {
        assert!(filter_g4score(-1.714, 1.0));
        assert!(!filter_g4score(0.5, 1.0));
        assert!(filter_g4score(-2.4, 0.0));
        assert!(filter_g4score(1.0, 1.0));
    }
}
