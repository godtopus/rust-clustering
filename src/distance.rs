#[allow(dead_code)]

use std::cmp::Ordering;

pub struct SquaredEuclidean;
pub struct Euclidean;
pub struct Hamming;
pub struct Chebyshev;
pub struct Manhattan;

pub trait Distance {
    fn distance(_: &[f64], _: &[f64]) -> f64 {
        0.0
    }
}

impl Distance for SquaredEuclidean {
    #[inline]
    fn distance(a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y) * (x - y))
            .sum()
    }
}

impl Distance for Euclidean {
    fn distance(a: &[f64], b: &[f64]) -> f64 {
        (a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y) * (x - y))
            .sum::<f64>()).sqrt()
    }
}

impl Distance for Hamming {
    fn distance(a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .filter(|&(x, y)| x != y)
            .count() as f64
    }
}

impl Distance for Chebyshev {
    fn distance(a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).abs())
            .max_by(|x, y| x.partial_cmp(&y).unwrap_or(Ordering::Equal))
            .unwrap()
    }
}

impl Distance for Manhattan {
    #[inline]
    fn distance(a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).abs())
            .sum()
    }
}

fn covariance_matrix(points: &[&[f64]]) -> Vec<Vec<f64>> {
    let row_length = points.len();
    let col_length = points[0].len();
    let means: Vec<f64> = points.iter().fold(vec![0.0; col_length], |mut means, next| {
        for i in 0..col_length {
            means[i] += next[i];
        }

        means
    }).into_iter().map(|a| a / (col_length as f64)).collect();

    let mut covariance_matrix = vec![vec![0.0; col_length]; row_length];

    for i in 0..row_length {
        for j in 0..col_length {
            for k in 0..row_length {
                covariance_matrix[i][j] = (means[i] - points[k][i]) * (means[j] - points[k][j]);
            }

            covariance_matrix[i][j] /= (row_length - 1) as f64;
        }
    }

    covariance_matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squared_euclidean_is_correct_distance() {
        assert_eq!(false, true);
    }

    #[test]
    fn euclidean_is_correct_distance() {
        assert_eq!(false, true);
    }

    #[test]
    fn hamming_is_correct_distance() {
        let expected = 4.0;

        let input_a = vec![0.0, 1.0, 3.0, -8.7, 4.5, 1.0];
        let input_b = vec![-2.3, 1.0, -1.0, 3.0, 4.5, -2.3];

        let output = Hamming::distance(input_a.as_slice(), input_b.as_slice());

        assert_eq!(expected, output);
    }

    #[test]
    fn chebyshev_is_correct_distance() {
        assert_eq!(false, true);
    }

    #[test]
    fn manhattan_is_correct_distance() {
        assert_eq!(false, true);
    }
}