// pub use std::vec::Vec;
use std::alloc::Layout;

use rayon::iter::{FromParallelIterator, ParallelIterator};

use crate::{Layer, NearestIter, Point, PointId, PointMgr, UpperNode, ZeroNode};

pub type Vec<T> = SegmentedVector<T>;

// TODO: implement Clone
#[derive(Debug, Clone)]
pub struct SegmentedVector<T> {
    length: usize,
    // size of each segment in bytes
    segment_size: usize,
    // segment capacity (number of elements)
    segment_capacity: usize,
    segments: std::vec::Vec<Segment<T>>,
}

impl<T> SegmentedVector<T> {
    pub const DEFAULT_SEGMENT_SIZE: usize = 1024 * 1024 * 1024;
    pub fn new() -> Self {
        Self { 
            length: 0,
            segment_size: Self::DEFAULT_SEGMENT_SIZE,
            segment_capacity: Self::DEFAULT_SEGMENT_SIZE / std::mem::size_of::<T>(),
            segments: std::vec::Vec::new(),
        }
    }

    /// length in the number of elements
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn iter(&self) -> SegmentIter<'_, T> {
        SegmentIter {
            vector: self,
            next: 0
        }
    }
    
    pub fn segment_size(&self) -> usize {
        self.segment_size
    }

    #[allow(dead_code)]
    fn segments(&self) -> std::slice::Iter<Segment<T>> {
        self.segments.iter()
    }

    // TODO: hide this
    pub fn num_segments(&self) -> usize {
        self.segments.len()
    }

    // returns the slice containing the desired index for an element,
    // and the relative index for that element
    unsafe fn get_slice_containing_unchecked(&self, idx: usize) -> (&[T], usize) {
        // determine segment from slice
        // 1. find segment for index
        let segment_num = idx / self.segment_capacity;
        // 2. calculate the relative index
        let elem_idx = idx % self.segment_capacity;
        let segment = self.segments.get_unchecked(segment_num);
        (segment.as_slice(), elem_idx)
    }

    #[inline]
    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx < self.length {
            unsafe {
                let (slice, rel) = self.get_slice_containing_unchecked(idx);
                Some(slice.get_unchecked(rel))
            }
        } else {
            None
        }
    }

    #[inline]
    pub fn get_unchecked(&self, idx: usize) -> &T {
        unsafe {
            let (slice, rel) = self.get_slice_containing_unchecked(idx);
            slice.get_unchecked(rel)
        }
    }

    fn add_segment(&mut self) {
        let segment = Segment::new(self.segment_capacity);
        self.segments.push(segment);
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        let segment_idx = self.length / self.segment_capacity;
        if segment_idx >= self.num_segments() {
            self.add_segment();
        }
        let segment = unsafe { self.segments.get_unchecked_mut(segment_idx) };
        // TODO: push unchecked? since we already bounds checked before ...
        let _ = segment.push_unchecked(value);
        self.length += 1;
    }

}

// list things to implement:
// - ref (&) -> slice

impl<T: PartialEq> PartialEq for SegmentedVector<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.length != other.length {
            false
        } else {
            // compare elements side by side
            let zip = self.iter().zip(other.iter());
            for (x, y) in zip.into_iter() {
                if x != y {
                    return false
                }
            }
            true
        }
    }
}

pub struct SegmentIter<'a, T> {
    vector: &'a SegmentedVector<T>,
    next: usize,
}

impl<'a, T> Iterator for SegmentIter<'a, T> {
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

#[derive(Debug, Clone, PartialEq)]
struct Segment<T> {
    // TODO: do we make this a vec with the option to have a different allocator?
    start: *mut T,
    length: usize,
    capacity: usize,
}

impl<T> Segment<T> {
    fn new(capacity: usize) -> Self {
        // ask the memory allocator for memory
        let layout = Layout::array::<T>(capacity).expect("bad layout");
        let ptr = unsafe { std::alloc::alloc(layout) };
        Segment { start: ptr.cast(), length: 0, capacity }
    }
    fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.start, self.length) }
    }
    
    fn as_slice_mut(&self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.start, self.length) }
    }

    // pushes T, returning an error when the segment is full, otherwise index
    #[inline]
    pub(crate) fn push(&mut self, elem: T) -> Result<usize, ()> {
        // 1. the place we want is the last one
        let place = self.length;
        // check bounds
        if self.length >= self.capacity {
            return Err(());
        }
        // increment length
        self.length += 1;
        // 2. index that object
        let slice = self.as_slice_mut();
        // 3. store that element
        slice[place] = elem;
        // return place where we stored it
        Ok(place)
    }

    // pushes T returning the index it was inserted in
    #[inline]
    pub(crate) fn push_unchecked(&mut self, elem: T) -> usize {
        // 1. the place we want is the last one
        let place = self.length;
        // increment length
        self.length += 1;
        // 2. index that object
        let slice = self.as_slice_mut();
        // 3. store that element
        slice[place] = elem;
        // return place where we stored it
        place
    }
}

unsafe impl<T> Sync for Segment<T> {}

impl<'a> Layer for &'a SegmentedVector<UpperNode> {
    type Slice = &'a [PointId];

    fn nearest_iter(&self, pid: PointId) -> NearestIter<Self::Slice> {
        let node = self.get_unchecked(pid.0 as usize);
        NearestIter::new(&node.0)
    }
}

impl<'a> Layer for &'a SegmentedVector<ZeroNode> {
    type Slice = &'a [PointId];

    fn nearest_iter(&self, pid: PointId) -> NearestIter<Self::Slice> {
        let node = self.get_unchecked(pid.0 as usize);
        NearestIter::new(&node)
        // NearestIter::new(&self[pid.0 as usize])
    }
}

impl<'a, P: Point> PointMgr<'a, P> for &'a SegmentedVector<P> {
    type R = &'a P;

    fn calc_distance(&self, a: PointId, b: PointId) -> f32 {
        let b = self.get(b);
        self.calc_distance_from(a, &*b)
    }

    fn calc_distance_from(&self, a: PointId, b: &P) -> f32 {
        let a = self.get(a);
        a.distance(b)
    }

    fn get(&'a self, idx: PointId) -> Self::R {
        <SegmentedVector<P>>::get_unchecked(self, idx.0 as usize)
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
        // could use a variety of methods
        // - use collect_vec_list -> copy each individually
        // - reserve memory, map items with index, then insert in specified place
        let ordered_list = par_iter.into_par_iter().collect_vec_list();
        ordered_list.into_iter().flatten().collect()
    }
}