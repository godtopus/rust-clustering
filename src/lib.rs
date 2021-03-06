#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

extern crate rand;
extern crate time;
extern crate itertools;
extern crate rayon;
extern crate nalgebra;

pub mod clustering {
    pub mod agglomerative;
    pub mod fuzzy_cmeans;
    pub mod kmeans;
    pub mod kmedians;
    pub mod kmedoids;
    pub mod mini_batch_kmeans;
}

pub mod statistics {
    pub mod statistics;
    pub mod distance;
}

pub mod experimental {
    pub mod kdtree;
    pub mod cure;
    pub mod xmeans;
    pub mod clarans;
    pub mod cluster;
}

pub mod point;
pub mod datasets;