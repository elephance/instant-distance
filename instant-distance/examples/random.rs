use std::collections::HashSet;

use ordered_float::OrderedFloat;
use rand::rngs::{StdRng, ThreadRng};
use rand::{Rng, SeedableRng};

use instant_distance::{Builder, Point as _, Search};

#[test]
fn random_heuristic() {
    let (seed, recall) = randomized(Builder::default());
    println!("heuristic (seed = {seed}) recall = {recall}");
    assert!(recall > 97, "expected at least 98, got {recall}");
}

#[test]
fn random_simple() {
    let (seed, recall) = randomized(Builder::default().select_heuristic(None));
    println!("simple (seed = {seed}) recall = {recall}");
    assert!(recall > 90, "expected at least 90, got {recall}");
}

#[allow(dead_code)]
fn randomized(builder: Builder) -> (u64, usize) {
    let seed = ThreadRng::default().random::<u64>();
    let mut rng = StdRng::seed_from_u64(seed);
    let points = (0..1024)
        .map(|_| Point(rng.random(), rng.random()))
        .collect::<Vec<_>>();

    let query = Point(rng.random(), rng.random());
    let mut nearest = Vec::with_capacity(256);
    for (i, p) in points.iter().enumerate() {
        nearest.push((OrderedFloat::from(query.distance(p)), i));
        if nearest.len() >= 200 {
            nearest.sort_unstable();
            nearest.truncate(100);
        }
    }

    let (hnsw, pids) = builder.seed(seed).build_hnsw(points);
    let mut search = Search::default();
    let results = hnsw.search(&query, &mut search);
    assert!(results.len() >= 100);

    nearest.sort_unstable();
    nearest.truncate(100);
    let forced = nearest
        .iter()
        .map(|(_, i)| pids[*i])
        .collect::<HashSet<_>>();
    let found = results
        .take(100)
        .map(|item| item.pid)
        .collect::<HashSet<_>>();
    (seed, forced.intersection(&found).count())
}

// A simple 2-d vector
#[derive(Clone, Copy, Debug)]
struct Point(f32, f32);

impl instant_distance::Point for Point {
    fn distance(&self, other: &Self) -> f32 {
        // Euclidean distance metric
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }
}

// can we use this crate's distance calculation implementations?
// https://docs.rs/anndists/0.1.2/anndists/dist/distances/struct.DistL2.html#impl-Distance%3Cf32%3E-for-DistL2

const NUM_VECS: usize = 1024; //1_000_000; // 1024
const K_NEAREST: usize = 10;
const NEAREST_SIZE: usize = K_NEAREST *  2 + K_NEAREST / 2;
const RECALL: usize = 9; //90;

fn main() {
    // let points = vec![Point(255, 0, 0), Point(0, 255, 0), Point(0, 0, 255)];
    // let values = vec!["red", "green", "blue"];

    // from tests above
    // let (seed, recall) = randomized(Builder::default().select_heuristic(None));
    let builder = Builder::without_seed().select_heuristic(None);

    // seeded using OS RNG
    // let seed = ThreadRng::default().gen::<u64>();
    
    let seed = 42;
    // uses ChaCha12
    let mut rng = StdRng::seed_from_u64(seed);
    
    let builder = builder.seed(rng.random());

    // generate some random points
    println!("randomly generating points ...");
    let points = (0..NUM_VECS)
        .map(|_| Point(rng.random(), rng.random()))
        .collect::<Vec<_>>();

    // randomly generate a query vector
    let query = Point(rng.random(), rng.random());

    // search for the 100 nearest vectors manually
    // nearest contains a distance and the index/id of the point
    println!("manually finding the k nearest points");
    let mut nearest = Vec::with_capacity(NEAREST_SIZE);
    for (i, p) in points.iter().enumerate() {
        nearest.push((OrderedFloat::from(query.distance(p)), i));
        // when we have about 200 vectors queue'd up
        if nearest.len() >= K_NEAREST * 2 {
            // sort the 
            nearest.sort_unstable();
            // keep the 100 nearest neighbors
            nearest.truncate(K_NEAREST);
        }
    }

    // build the hnsw index
    println!("building the index ...");
    // returns the index, and the ids of the points specified in order passed in
    let (hnsw, pids) = builder.seed(seed).build_hnsw(points);
    
    // now search the index for atleast K of the nearest neighbors
    let mut search = Search::default();
    let mut results = hnsw.search(&query, &mut search);
    // I don't understand how this API bounds the number of neighbors to search
    println!("found {} nearest neighbors", results.len());
    assert!(results.len() >= K_NEAREST);

    // do one final sort and truncate
    nearest.sort_unstable();
    nearest.truncate(K_NEAREST);

    // find the point ids of all the nearest neighbors
    let forced = nearest
        .iter()
        .map(|(_, i)| pids[*i])
        .collect::<HashSet<_>>();
    
    // the closest point is which vector, the first one I guess
    let closest_point = results.next().unwrap();

    println!("{:?}", closest_point.point);

    // find the K nearest neighbors (PointIds)
    let mut found = results
        .take(K_NEAREST - 1)
        .map(|item| item.pid)
        .collect::<HashSet<_>>();
    found.insert(closest_point.pid);

    // the recall is the intersection of these points
    let (seed, recall) = (seed, forced.intersection(&found).count());

    println!("simple (seed = {seed}) recall = {recall}");
    assert!(recall > RECALL, "expected at least {RECALL}, got {recall}");

    // hold off on this for now ...
    // the closest point is which vector, the first one I guess
    // let closest_point = results.next().unwrap().point;

    // println!("{:?}", closest_point);
}

