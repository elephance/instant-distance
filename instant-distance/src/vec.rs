// pub use std::vec::Vec;
use std::marker::PhantomData;

use rayon::iter::{FromParallelIterator, ParallelIterator};

use crate::{Layer, NearestIter, Point, PointId, PointMgr, UpperNode, ZeroNode};

pub type Vec<T> = SegmentedVector<T>;

// TODO: implement Clone
#[derive(Debug, Clone, PartialEq)]
pub struct SegmentedVector<T> {
    length: usize,
    segment_size: usize,
    segments: std::vec::Vec<SegmentArray<T>>,
}

impl<T> SegmentedVector<T> {
    pub const DEFAULT_SEGMENT_SIZE: usize = 1024 * 1024 * 1024;
    pub fn new() -> Self {
        Self { 
            length: 0,
            segment_size: Self::DEFAULT_SEGMENT_SIZE,
            segments: std::vec::Vec::new(),
        }
    }

    /// length in the number of elements
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn iter(&self) -> Iter<'_, T> {
        // self.segments.iter()
        todo!()
    }
    
    pub fn segment_size(&self) -> usize {
        self.segment_size
    }

    fn segments(&self) -> std::slice::Iter<SegmentArray<T>> {
        self.segments.iter()
    }

    // TODO: should we hide this?
    pub fn num_segments(&self) -> usize {
        self.segments.len()
    }

    #[inline]
    pub fn get(&self, idx: usize) -> Option<&T> {
        if std::intrinsics::likely(idx < self.length) {
            Ok(self.get_slice_containing_unchecked(idx).get(rel))
        } else {
            Err("index out of bounds")
        }

    }

    pub fn push(&mut self, value: T) {
        // self.segments.push(value);
        todo!()
    }

}

// list things to implement:
// - ref (&) -> slice
// - iter()
// - as_slice()
// - collect()

pub struct Iter<'a, T> {
    vector: &'a SegmentedVector<T>,
    next: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.next;
        if idx < self.vector.len() {
            self.next += 1;
            self.vector.get(idx)
        } else {
            None
        }
    }
}

// TODO: maybe not change all vec to be segmented vec ...
#[derive(Debug, Clone, PartialEq)]
struct SegmentArray<T> {
    // TODO: do we make this a vec with the option to have a different allocator?
    _data: PhantomData<T>,
}

impl<'a> Layer for &'a SegmentedVector<UpperNode> {
    type Slice = &'a [PointId];

    fn nearest_iter(&self, _pid: PointId) -> NearestIter<Self::Slice> {
        todo!()
        // TODO: get unchecked ...
        // NearestIter::new(&self[pid.0 as usize].0)
    }
}

impl<'a> Layer for &'a SegmentedVector<ZeroNode> {
    type Slice = &'a [PointId];

    fn nearest_iter(&self, _pid: PointId) -> NearestIter<Self::Slice> {
        todo!()
        // NearestIter::new(&self[pid.0 as usize])
    }
}

impl<'a, P: Point> PointMgr<'a, P> for &'a SegmentedVector<P> {
    type R = &'a P;

    fn calc_distance(&self, _a: PointId, _b: PointId) -> f32 {
        // let a = &self[a];
        // let b = &self[b];
        // a.distance(b)
        todo!()
    }

    fn calc_distance_from(&self, _a: PointId, _b: &P) -> f32 {
        // let a = &self[a];
        // TODO: implement [] operator
        // a.distance(b)
        todo!()
    }

    fn get(&'a self, _idx: PointId) -> Self::R {
        // &self[idx]
        todo!()
    }

    fn num_vectors(&self) -> usize {
        self.len()
    }
}

impl<T> FromIterator<T> for SegmentedVector<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec = SegmentedVector::new();
        iter.into_iter().for_each(|item| vec.push(item));
        vec
    }
}

impl<T: Send> FromParallelIterator<T> for SegmentedVector<T> {
    fn from_par_iter<I>(par_iter: I) -> Self
        where
            I: rayon::prelude::IntoParallelIterator<Item = T> 
    {
        // todo!()
        // could use a variety of methods
        // - use collect_vec_list -> copy each individually
        // - fold/fold_with (does not preserve order)
        // - for_each (does not preserve order)
        // - map items with index, then insert in specified place?
        let ordered_list = par_iter.into_par_iter().collect_vec_list();
        ordered_list.into_iter().flatten().collect()
 
        // TODO: better impl, use reserve method, add insert as well

        // not the best since it probs uses a ton of memory...
    }
}