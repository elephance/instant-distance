// pub use std::vec::Vec;
use std::marker::PhantomData;

use crate::{Layer, NearestIter, Point, PointId, PointMgr, UpperNode, ZeroNode};

pub type Vec<T> = SegmentedVector<T>;

// TODO: implement Clone
#[derive(Clone)]
pub(crate) struct SegmentedVector<T> {
    length: usize,
    inner: std::vec::Vec<T>,
    // _data: PhantomData<T>,
}

impl<T> SegmentedVector<T> {
    pub fn new() -> Self { todo!() }

    pub fn len(&self) -> usize { todo!() }

    pub fn iter(&self) -> Iter<'_, T> {
        todo!()
    }
    
    // fn segments(&self) -> SegmentIter {
    //     // self.inner.iter()
    //     todo!()
    // }

    fn num_segments(&self) -> usize {
        todo!()
    }

    #[inline]
    pub fn get(&self, _idx: usize) -> Option<&T> {
        todo!()
        // if std::intrinsics::likely(idx < self.length) {
            // Ok(self.get_slice_containing_unchecked(idx).get(rel))
        // } else {
        //     Err("index out of bounds")
        // }

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
struct SegmentArray<T> {
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