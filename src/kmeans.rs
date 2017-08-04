// http://ilpubs.stanford.edu:8090/778/1/2006-13.pdf

use rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use std::cmp::Ordering;
use std::usize;
use point::Point;
use distance::*;
use std::collections::HashMap;
use itertools::Itertools;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use kmeans::KMeansInitialization::*;
use statistics::Statistics;

pub enum KMeansInitialization {
    Random,
    KMeansPlusPlus,
    Precomputed
}

pub struct KMeans {
    assignments: Vec<usize>,
    centroids: Vec<Point>,
    iterations: usize,
    converged: bool
}

impl KMeans {
    pub fn run(points: &[Point], no_clusters: usize, max_iterations: usize, init_method: KMeansInitialization, precomputed: Option<&[Vec<f64>]>) -> Self {
        let dimension = points[0].coordinates().len() as f64;

        let mut centroids = Self::initial_centroids(points, no_clusters, init_method, precomputed);

        let mut previous_round: HashMap<usize, usize> = HashMap::with_capacity(points.len());

        let mut i = 0;

        while i < max_iterations {
            let mut has_converged = true;
            let mut new_centroids: Vec<Vec<&[f64]>> = vec![vec![]; no_clusters];

            for (index_p, p) in points.iter().enumerate() {
                let (index_c, _) = Self::closest_centroid(p.coordinates(), centroids.as_slice());

                new_centroids[index_c].push(points[index_p].coordinates());

                match previous_round.entry(index_p) {
                    Occupied(ref o) if o.get() == &index_c => (),
                    Occupied(mut o) => {
                        o.insert(index_c);
                        has_converged = false;
                    },
                    Vacant(v) => {
                        v.insert(index_c);
                        has_converged = false;
                    }
                }
            }

            if has_converged {
                break;
            }

            centroids = new_centroids.into_iter().map(|ref v| {
                Statistics::mean(&v)
            }).collect();

            i += 1;
        }

        KMeans {
            assignments: previous_round.into_iter().sorted_by(|&(p1, _), &(p2, _)| p1.partial_cmp(&p2).unwrap_or(Ordering::Equal)).into_iter().map(|(_, c)| c).collect(),
            centroids: centroids.into_iter().map(|c| Point::new(c)).collect(),
            iterations: i,
            converged: i < max_iterations
        }
    }

    fn initial_centroids(points: &[Point], no_clusters: usize, init_method: KMeansInitialization, precomputed: Option<&[Vec<f64>]>) -> Vec<Vec<f64>> {
        match init_method {
            Random => {
                let mut rng = rand::thread_rng();
                let between = Range::new(0, points.len());

                (0..no_clusters).map(|_| {
                    points[between.ind_sample(&mut rng)].coordinates().to_vec()
                }).collect()
            },
            KMeansPlusPlus => {
                let mut rng = rand::thread_rng();
                let between = Range::new(0, points.len());

                let mut distances: Vec<f64> = vec![0.0; points.len()];
                let mut centroids: Vec<Vec<f64>> = vec![points[between.ind_sample(&mut rng)].coordinates().to_vec()];

                for _ in 1..no_clusters {
                    let mut sum = points.iter().enumerate().fold(0.0, |sum, (index_p, p)| {
                        let (_, distance_c) = Self::closest_centroid(p.coordinates(), centroids.as_slice());
                        distances[index_p] = distance_c;
                        sum + distance_c
                    });

                    sum *= rng.next_f64();
                    for (index_p, d) in distances.iter().enumerate() {
                        sum -= *d;

                        if sum <= 0f64 {
                            centroids.push(points[index_p].coordinates().to_vec());
                            break;
                        }
                    }
                }

                centroids
            },
            Precomputed => {
                precomputed.expect("Expected a slice of clusters, on the form Vec<f64>").to_vec()
            }
        }
    }

    #[inline]
    fn closest_centroid(point: &[f64], centroids: &[Vec<f64>]) -> (usize, f64) {
        match centroids.iter().enumerate().map(|(index_c, c)| {
            (index_c, SquaredEuclidean::distance(point, c))
        }).min_by(|&(_, a), &(_, b)| {
            a.partial_cmp(&b).unwrap_or(Ordering::Equal)
        }) {
            Some(closest) => closest,
            None => panic!()
        }
    }

    pub fn centroids(&self) -> &[Point] {
        &self.centroids
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand;
    use rand::Rng;
    use time;
    //use test::Bencher;

    /*#[test]
    fn can_run() {
        let expected: Vec<KMeansCluster> = vec![
            KMeansCluster::from(vec![Point::new(vec![0.0, 1.0]), Point::new(vec![1.0, 2.0]), Point::new(vec![2.0, 3.0])]),
            KMeansCluster::from(vec![Point::new(vec![3.0, 4.0]), Point::new(vec![4.0, 5.0])])
        ];
        let mut input = vec![Point::new(vec![0.0, 1.0]), Point::new(vec![1.0, 2.0]), Point::new(vec![2.0, 3.0]), Point::new(vec![3.0, 4.0]), Point::new(vec![4.0, 5.0])];

        let output = kmeans(input.as_mut_slice(), 2, usize::max_value());

        assert_eq!(expected, output);
    }*/

    #[test]
    fn bench_100000_points() {
        let mut rng = rand::thread_rng();
        let mut points: Vec<Point> = (0..100000).map(|_| {
            Point::new((0..2).into_iter().map(|_| rng.next_f64()).collect())
        }).collect();

        let repeat_count = 10_u8;
        let mut total = 0_u64;
        for _ in 0..repeat_count {
            let start = time::precise_time_ns();
            KMeans::run(points.as_mut_slice(), 10, 15, KMeansInitialization::Random, None);
            let end = time::precise_time_ns();
            total += end - start
        }

        let avg_ns: f64 = total as f64 / repeat_count as f64;
        let avg_ms = avg_ns / 1.0e6;

        println!("{} runs, avg {}", repeat_count, avg_ms);
    }

    /*#[bench]
    fn bench_100000_points(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let points = Vec::from_fn(100000, |_| Point::new(vec![rng.next_f64(), rng.next_f64()]));

        b.iter(|| {
            kmeans(points, 50, usize::max_value());
        });
    }*/
}