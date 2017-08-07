use distance::*;
use point::Point;
use std::f64::consts::PI;
use nalgebra::*;

pub struct Statistics;

impl Statistics {
    #[inline]
    pub fn mean(centroids: &[&[f64]]) -> Vec<f64> {
        match centroids.len() {
            0 => vec![],
            _ => {
                let dimension = centroids[0].len() as f64;

                centroids.iter().fold(vec![0.0; centroids[0].len()], |mut acc, next| {
                    for i in 0..next.len() {
                        acc[i] += next[i];
                    }

                    acc
                }).into_iter().map(|x| x / dimension).collect()
            }
        }
    }

    pub fn variance(centroid: &[f64], points: &[Point]) -> f64 {
        points.iter().map(|p| SquaredEuclidean::distance(centroid, p.coordinates())).sum()
    }

    pub fn inverse_covariance(matrix: &[&[f64]]) -> Vec<Vec<f64>> {
        let cols = matrix[0].len();

        let mut matrix = DMatrix::from_row_vector(matrix.len(), cols, &matrix.into_iter().flat_map(|m| m.to_vec()).into_iter().collect::<Vec<f64>>()).covariance();
        matrix.inverse_mut();
        matrix.transpose_mut();

        matrix.into_vector().chunks(cols).map(|chunk| chunk.to_vec()).collect()
    }

    pub fn covariance(matrix: &[&[f64]]) -> Vec<Vec<f64>> {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let divisor = rows as f64 - 1.0;

        let means: Vec<f64> = matrix.iter().fold(vec![0.0; cols], |mut means, next| {
            for i in 0..cols {
                means[i] += next[i];
            }

            means
        }).into_iter().map(|a| a / (rows as f64)).collect();

        let mut covariance_matrix = vec![vec![0.0; cols]; cols];
        for i in 0..cols {
            for j in i..cols {
                let sum = (0..rows).into_iter()
                                   .map(|k| (matrix[k][j] - means[j]) * (matrix[k][i] - means[i]))
                                   .sum::<f64>() / divisor;

                covariance_matrix[i][j] = sum;
                covariance_matrix[j][i] = sum;
            }
        }

        return covariance_matrix;
    }

    /**
     * Calculates the BIC for single cluster.
     * @param n the total number of samples.
     * @param d the dimensionality of data.
     * @param distortion the distortion of clusters (i.e. variance).
     * @return the BIC score.
     */
    fn bic_single(n: f64, d: f64, distortion: f64) -> f64 {
        let variance = distortion / (n - 1.0);

        let p1 = -n * (2.0 * PI).ln();
        let p2 = -n * d * variance.ln();
        let p3 = -(n - 1.0);

        ((p1 + p2 + p3) / 2.0) - 0.5 * (d + 1.0) * n.ln()
    }

    /**
     * Calculates the BIC for the given set of centers.
     * @param k the number of clusters.
     * @param n the total number of samples.
     * @param d the dimensionality of data.
     * @param distortion the distortion of clusters.
     * @param cluster_size the number of samples in each cluster.
     * @return the BIC score.
     */
    fn bic(k: f64, n: f64, d: f64, distortion: f64, cluster_size: &[f64]) -> f64 {
        let variance = distortion / (n - k);

        (0..k as usize).into_iter().map(|i| Self::log_likelihood(k, n, cluster_size[i], d, variance)).sum::<f64>() - 0.5 * (k + k * d) * n.ln()
    }

    /**
     * Estimate the log-likelihood of the data for the given model.
     *
     * @param k the number of clusters.
     * @param n the total number of samples.
     * @param ni the number of samples belong to this cluster.
     * @param d the dimensionality of data.
     * @param variance the estimated variance of clusters.
     * @return the likelihood estimate
     */
    fn log_likelihood(k: f64, n: f64, ni: f64, d: f64, variance:f64) -> f64 {
        let p1 = -ni * (2.0 * PI).ln();
        let p2 = -ni * d * variance.ln();
        let p3 = -(ni - k);
        let p4 = ni * ni.ln();
        let p5 = -ni * n.ln();
        (p1 + p2 + p3) / 2.0 + p4 + p5
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}