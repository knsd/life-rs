/// Vector that can be extended in both directions
/// thus indices could be positive or negative. It is represented as
/// two independent Vecs inside so both sides extensions are
/// very fast (approx constant time), indexing is also a constant.
/// ```
/// let mut v: SymVec<i32> = SymVec::new();
/// v.push_back(10);
/// v.push_back(20);
/// v.push_front(5);
/// v.push_front(6);
/// v.push_front(7);
/// ```
/// Will create a SymVec containing elements: 20, 10, 5, 6, 7
/// with indices:                             -2  -1  0  1  2

use std::ops::{Index, IndexMut};

pub struct SymVec<T> {
    pub vec_neg: Vec<T>,
    pub vec_pos: Vec<T>,
}

impl<T> Index<isize> for SymVec<T> {

    type Output = T;

    fn index(&self, idx: isize) -> &T {
        if idx < 0 {
            let abs_idx = -(1 + idx) as usize;
            if abs_idx >= self.vec_neg.len() {
                panic!("No element with index {}", abs_idx);
            }
            &self.vec_neg[abs_idx]

        } else {
            let abs_idx = idx as usize;
            if abs_idx >= self.vec_pos.len() {
                panic!("No element with index {}", idx);
            }
            &self.vec_pos[abs_idx]
        }
    }

}

impl<T> IndexMut<isize> for SymVec<T> {

    fn index_mut<'a>(&'a mut self, idx: isize) -> &'a mut T {
        if idx < 0 {
            let abs_idx = -(1 + idx) as usize;
            if abs_idx >= self.vec_neg.len() {
                panic!("No element with index {}", abs_idx);
            }
            &mut self.vec_neg[abs_idx]

        } else {
            let abs_idx = idx as usize;
            if abs_idx >= self.vec_pos.len() {
                panic!("No element with index {}", idx);
            }
            &mut self.vec_pos[abs_idx]
        }
    }

}

impl<'a, T: 'a> IntoIterator for &'a SymVec<T> {

    type Item = &'a T;
    type IntoIter = SymVecIntoIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        SymVecIntoIterator{symvec: self, idx: -(self.len_neg() as isize) - 1}
    }

}

pub struct SymVecIntoIterator<'a, T: 'a> {
    symvec: &'a SymVec<T>,
    idx: isize,
}

impl<'a, T> Iterator for SymVecIntoIterator<'a, T> {

    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.idx += 1;
        if self.idx < (self.symvec.len_pos() as isize) {
            Some(&self.symvec[self.idx])
        } else {
            None
        }

    }

}

impl<T> SymVec<T> {

    pub fn new() -> Self {
        SymVec{vec_neg: Vec::new(),
               vec_pos: Vec::new()}
    }

    pub fn push_front(&mut self, e: T) {
        self.vec_pos.push(e);
    }

    pub fn push_back(&mut self, e: T) {
        self.vec_neg.push(e);
    }

    pub fn len_pos(&self) -> usize {
        self.vec_pos.len()
    }

    pub fn len_neg(&self) -> usize {
        self.vec_neg.len()
    }

    pub fn len(&self) -> usize {
        self.len_pos() + self.len_neg()
    }

    pub fn need_extend_pos(&self, idx: isize) -> bool {
        idx >= (self.len_pos() as isize)
    }

    pub fn need_extend_neg(&self, idx: isize) -> bool {
        -idx >= (self.len_neg() as isize) + 1
    }

    pub fn need_extend_pos_cnt(&self, idx: isize) -> Option<usize> {
        if self.need_extend_pos(idx - 1) {
            Some((idx - (self.len_pos() as isize)) as usize)
        } else {
            None
        }
    }

    pub fn need_extend_neg_cnt(&self, idx: isize) -> Option<usize> {
        if self.need_extend_neg(idx) {
            Some(-(idx + self.len_neg() as isize) as usize)
        } else {
            None
        }
    }

    pub fn is_available(&self, idx: isize) -> bool {
        if idx >= 0 {
            !self.need_extend_pos(idx)
        } else {
            !self.need_extend_neg(idx)
        }
    }

}


#[test]
fn test_push_front_back() {

    let mut v: SymVec<i32> = SymVec::new();

    v.push_front(1);
    v.push_front(2);
    v.push_back(-1);

    assert!(v.len() == 3);
    assert!(v[-1] == -1);

    v[-1] = 20;
    assert!(v[-1] == 20);

}

#[test]
fn test_extend()
{

    let mut v: SymVec<i32> = SymVec::new();

    assert!(v.need_extend_pos(0) == true);

    v.push_front(1);

    assert!(v.need_extend_pos(0) == false);
    assert!(v.need_extend_pos(1) == true);
    assert!(v.need_extend_pos(5) == true);

    assert!(v.need_extend_neg(-1) == true);

    v.push_back(-2);

    assert!(v.need_extend_neg(-1) == false);
    assert!(v.need_extend_neg(-2) == true);

}

#[test]
fn test_need_extend_cnt() {

    let mut v: SymVec<i32> = SymVec::new();

    assert_eq!(v.need_extend_neg_cnt(-1), Some(1));
    assert_eq!(v.need_extend_pos_cnt(0), None);

    assert_eq!(v.need_extend_neg_cnt(-20), Some(20));
    assert_eq!(v.need_extend_neg_cnt(15), None);
    assert_eq!(v.need_extend_pos_cnt(15), Some(15));
    assert_eq!(v.need_extend_pos_cnt(-20), None);

    v.push_front(1);
    assert_eq!(v.need_extend_pos_cnt(15), Some(14));

    v.push_back(1);
    assert_eq!(v.need_extend_neg_cnt(-20), Some(19));

    assert_eq!(v.need_extend_neg_cnt(-2), Some(1));
    assert_eq!(v.need_extend_neg_cnt(-1), None);

    assert_eq!(v.need_extend_pos_cnt(2), Some(1));
    assert_eq!(v.need_extend_pos_cnt(1), None);

}

#[test]
fn test_iterator() {

    let mut v: SymVec<i32> = SymVec::new();
    v.push_back(-1);
    v.push_back(-2);
    v.push_front(1);
    v.push_front(2);
    v.push_front(3);

    let mut v2: Vec<&i32> = v.into_iter().collect();
    assert!(*v2[0] == -2);
    assert!(*v2[1] == -1);
    assert!(*v2[2] == 1);
    assert!(*v2[3] == 2);
    assert!(*v2[4] == 3);

}
