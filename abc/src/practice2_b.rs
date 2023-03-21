#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
pub fn main() {
    b()
}

fn b() {
    ip! {
    n:usize,
    q:usize,
    a:[usize;n],
    xlr:[(usize,usize,usize);q]
        }
    let mut t = Treap::new();
    for i in a {
        t.insert(i);
    }
    for i in 0..q {
        let (x, l, r) = xlr[i];
        match x {
            0 => {
                let current = t.get(l);
                if current.is_none() {
                    continue;
                }
                t.update_by_index(l, current.unwrap().key + r);
            }
            _ => {
                println!("{}", t.sum(l, r));
            }
        }
    }
}

use rand::Rng;
use std::{
    cmp::{Ord, Ordering::Equal, Ordering::Greater, Ordering::Less},
    ops::{Add, Sub},
};

#[derive(Debug, Clone)]
pub struct Treap<T> {
    root: Option<Box<Node<T>>>,
    size: usize,
    _rnd: rand::rngs::ThreadRng,
}

// RangeTree API
pub trait RangeTree<T> {
    fn new() -> Self;
    fn insert(&mut self, key: T);
    fn erase(&mut self, key: &T);
    fn get(&self, index: usize) -> Option<&Node<T>>;
    fn lower_bound(&self, key: &T) -> Option<&Node<T>>;
    fn upper_bound(&self, key: &T) -> Option<&Node<T>>;
    fn max(&self) -> Option<T>;
    fn min(&self) -> Option<T>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn contains(&self, key: &T) -> bool;
    fn update_by_index(&mut self, index: usize, key: T);
    // fn range_min(&self, l: usize, r: usize) -> T;
    // fn range_max(&self, l: usize, r: usize) -> T;
    fn sum(&mut self, l: usize, r: usize) -> T;
    //fn range_reverse(&mut self, l: usize, r: usize);
}

impl<T> RangeTree<T> for Treap<T>
where
    T: Ord + Copy + std::fmt::Debug + Add<Output = T> + Default + Sub<Output = T>,
{
    fn new() -> Treap<T> {
        Self {
            root: None,
            size: 0,
            _rnd: rand::thread_rng(),
        }
    }

    /// insert the key into the treap
    /// ```
    /// use k0i::treap::Treap;
    /// use crate::k0i::treap::RangeTree;
    /// let mut t = Treap::new();
    /// t.insert(1);
    /// t.insert(2);
    /// t.insert(3);
    /// t.insert(4);
    /// assert_eq!(true, t.contains(&1));
    /// assert_eq!(true, t.contains(&2));
    /// assert_eq!(true, t.contains(&3));
    /// assert_eq!(true, t.contains(&4));
    /// ```

    fn insert(&mut self, key: T) {
        // if key duplicates, update frequency
        if let Some(n) = self.get_mut(&key) {
            n.frequency += 1;
            self.size += 1;
            return;
        }
        let priority = self._rnd.gen::<u64>();
        let node = Node::new(key, priority);
        let root = self.root.take();
        let (left, right) = self.split(root, &node.key);
        let new_root = self.merge(left, Some(Box::new(node)));
        let new_root = self.merge(new_root, right);
        self.root = new_root;
        self.size += 1;
    }
    /// erase the key from the treap. If there are duplicates, decrease the frequency
    /// ```
    /// use k0i::treap::Treap;
    /// use crate::k0i::treap::RangeTree;
    /// let mut t = Treap::new();
    /// t.insert(1);
    /// t.insert(2);
    /// t.insert(3);
    /// t.insert(4);
    /// assert_eq!(true, t.contains(&1));
    /// t.erase(&1);
    /// assert_eq!(false, t.contains(&1));
    /// // duplicates
    /// let mut t = Treap::new();
    /// t.insert(1);
    /// t.insert(1);
    /// t.insert(1);
    /// assert_eq!(true, t.contains(&1));
    /// t.erase(&1);
    /// assert_eq!(true, t.contains(&1));
    /// t.erase(&1);
    /// assert_eq!(true, t.contains(&1));
    /// t.erase(&1);
    /// assert_eq!(false, t.contains(&1));
    /// ```
    fn erase(&mut self, key: &T) {
        // if duplicates, decrease frequency
        if let Some(n) = self.get_mut(key) {
            if n.frequency > 1 {
                n.frequency -= 1;
                self.size -= 1;
                return;
            }
        }
        let root = self.root.take();
        let (left, right) = self.split(root, key);
        let (left, _) = self.split(left, key);
        let new_root = self.merge(left, right);
        self.root = new_root;
        self.size -= 1;
    }
    /// get the node by index
    /// ```
    /// use k0i::treap::Treap;
    /// use crate::k0i::treap::RangeTree;
    /// let mut t = Treap::new();
    /// t.insert(1);
    /// t.insert(2);
    /// t.insert(3);
    /// t.insert(4);
    /// ```
    fn get(&self, index: usize) -> Option<&Node<T>> {
        self.root.as_ref().and_then(|n| n.get_by_index(index))
    }

    fn lower_bound(&self, key: &T) -> Option<&Node<T>> {
        self.root.as_ref().and_then(|n| n.lower_bound(key))
    }

    fn upper_bound(&self, key: &T) -> Option<&Node<T>> {
        self.root.as_ref().and_then(|n| n.upper_bound(key))
    }

    /// return the max key in the treap.
    /// ```
    /// use k0i::treap::Treap;
    /// use crate::k0i::treap::RangeTree;
    /// let mut t = Treap::new();
    /// t.insert(1);
    /// t.insert(2);
    /// t.insert(3);
    /// t.insert(4);
    /// assert_eq!(4, t.max().unwrap());
    /// t.erase(&1);
    /// assert_eq!(4, t.max().unwrap());
    /// t.erase(&4);
    /// assert_eq!(3, t.max().unwrap());
    /// ```
    fn max(&self) -> Option<T> {
        if let Some(n) = self.root.as_ref() {
            Some(n.max())
        } else {
            None
        }
    }
    /// return the min key in the treap.
    /// ```
    /// use k0i::treap::Treap;
    /// use crate::k0i::treap::RangeTree;
    /// let mut t = Treap::new();
    /// t.insert(1);
    /// t.insert(2);
    /// t.insert(3);
    /// t.insert(4);
    /// assert_eq!(1, t.min().unwrap());
    /// t.erase(&1);
    /// assert_eq!(2, t.min().unwrap());
    /// t.insert(1);
    /// assert_eq!(1, t.min().unwrap());
    /// ```

    fn min(&self) -> Option<T> {
        if let Some(n) = self.root.as_ref() {
            Some(n.min())
        } else {
            None
        }
    }
    // return length of the treap
    /// ```
    /// use k0i::treap::Treap;
    /// use crate::k0i::treap::RangeTree;
    /// let mut t = Treap::new();
    /// assert_eq!(0, t.len());
    /// t.insert(1);
    /// assert_eq!(1, t.len());
    /// t.insert(2);
    /// assert_eq!(2, t.len());
    /// t.insert(3);
    /// assert_eq!(3, t.len());
    /// t.erase(&2);
    /// assert_eq!(2, t.len());
    /// t.erase(&1);
    /// assert_eq!(1, t.len());
    /// t.erase(&3);
    /// assert_eq!(0, t.len());
    /// // if duplicates
    /// let mut t = Treap::new();
    /// t.insert(1);
    /// t.insert(1);
    /// t.insert(1);
    /// t.insert(2);
    /// assert_eq!(4, t.len());
    /// t.erase(&1);
    /// assert_eq!(3, t.len());
    /// t.erase(&1);
    /// assert_eq!(2, t.len());
    /// t.erase(&1);
    /// assert_eq!(1, t.len());
    /// ```
    fn len(&self) -> usize {
        self.size
    }

    /// check if the treap is empty
    /// ```
    /// use k0i::treap::Treap;
    /// use crate::k0i::treap::RangeTree;
    /// let mut t = Treap::new();
    /// assert!(t.is_empty());
    /// t.insert(1);
    /// assert!(!t.is_empty());
    /// t.erase(&1);
    /// assert!(t.is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// check if the treap contains the key
    /// ```
    /// use k0i::treap::Treap;
    /// use crate::k0i::treap::RangeTree;
    /// let mut t = Treap::new();
    /// t.insert(1);
    /// t.insert(2);
    /// t.insert(3);
    /// t.insert(4);
    /// assert_eq!(true, t.contains(&1));
    /// assert_eq!(true, t.contains(&2));
    /// assert_eq!(true, t.contains(&3));
    /// assert_eq!(true, t.contains(&4));
    /// assert_eq!(false, t.contains(&5));
    /// t.erase(&1);
    /// assert_eq!(false, t.contains(&1));
    /// assert_eq!(true, t.contains(&2));
    /// ```
    fn contains(&self, key: &T) -> bool {
        self.root.as_ref().map_or(false, |n| n.cointains(key))
    }

    /// return sum in the range [l, r)
    /// ```
    /// use k0i::treap::Treap;
    /// use crate::k0i::treap::RangeTree;
    /// let mut t = Treap::new();
    /// t.insert(1);
    /// t.insert(2);
    /// t.insert(3);
    /// t.insert(4);
    /// // (index,value) = (0,1), (1,2), (2,3), (3,4)
    /// assert_eq!(10, t.sum(0, 4));
    /// assert_eq!(5, t.sum(1, 3));
    /// assert_eq!(2, t.sum(1, 2));
    /// ```
    fn sum(&mut self, l: usize, r: usize) -> T {
        let mut ret = T::default();
        let root = self.root.take();
        let (left, right) = self.split_by_index(root, l);
        let (mut ll, rr) = self.split_by_index(right, r - l);
        if let Some(n) = ll.as_mut() {
            ret = n.sum;
        }
        let new_right = self.merge(ll, rr);
        self.root = self.merge(left, new_right);
        ret
    }

    /// update the key at index `index` to `key`
    fn update_by_index(&mut self, index: usize, key: T) {
        let root = self.root.take();
        let (left, right) = self.split_by_index(root, index);
        let (mut ll, rr) = self.split_by_index(right, 1);
        if let Some(n) = ll.as_mut() {
            n.key = key;
            n.update();
        }
        let new_right = self.merge(ll, rr);
        self.root = self.merge(left, new_right);
    }
}

impl<T> Treap<T>
where
    T: Ord + Copy + std::fmt::Debug + Add<Output = T> + Default + Sub<Output = T>,
{
    fn merge(
        &mut self,
        mut left: Option<Box<Node<T>>>,
        mut right: Option<Box<Node<T>>>,
    ) -> Option<Box<Node<T>>> {
        match (left.as_mut(), right.as_mut()) {
            (Some(l), Some(r)) => {
                if l.priority > r.priority {
                    let r = l.right.take();
                    let rr = self.merge(r, right);
                    l.set_right(rr);
                    left
                } else {
                    let l = r.left.take();
                    let ll = self.merge(left, l);
                    r.set_left(ll);
                    right
                }
            }
            (Some(_), None) => left,
            (None, Some(_s)) => right,
            (None, None) => None,
        }
    }

    fn split(
        &mut self,
        node: Option<Box<Node<T>>>,
        key: &T,
    ) -> (Option<Box<Node<T>>>, Option<Box<Node<T>>>) {
        match node {
            Some(mut n) => match key.cmp(&n.key) {
                Equal => {
                    let right = n.right.take();
                    let left = n.left.take();
                    n.update();
                    (left, right)
                }
                Less => {
                    let l = n.left.take();
                    let (left, right) = self.split(l, key);
                    n.set_left(right);
                    (left, Some(n))
                }
                Greater => {
                    let r = n.right.take();
                    let (left, right) = self.split(r, key);
                    n.set_right(left);
                    (Some(n), right)
                }
            },
            None => (None, None),
        }
    }
    fn split_by_index(
        &mut self,
        node: Option<Box<Node<T>>>,
        index: usize,
    ) -> (Option<Box<Node<T>>>, Option<Box<Node<T>>>) {
        match node {
            Some(mut n) => {
                let lsize = n.left.as_ref().map_or(0, |l| l.count());
                if index <= lsize {
                    let l = n.left.take();
                    let (left, right) = self.split_by_index(l, index);
                    n.set_left(right);
                    (left, Some(n))
                } else {
                    let r = n.right.take();
                    let (left, right) = self.split_by_index(r, index - lsize - 1);
                    n.set_right(left);
                    (Some(n), right)
                }
            }
            None => (None, None),
        }
    }

    fn get_mut(&mut self, key: &T) -> Option<&mut Node<T>> {
        self.root.as_mut().and_then(|n| n.get_mut(key))
    }
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub key: T,
    priority: u64,
    count: usize,
    frequency: usize,
    min: T,
    max: T,
    sum: T,
    reverse: bool,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: Ord + Copy + std::fmt::Debug + Add<Output = T> + Default + Sub<Output = T>,
{
    pub fn new(key: T, priority: u64) -> Node<T> {
        Node {
            key,
            priority,
            count: 1,
            frequency: 1,
            sum: key,
            min: key,
            max: key,
            reverse: false,
            left: None,
            right: None,
        }
    }

    pub fn count(&self) -> usize {
        self.count + self.frequency - 1
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Node<T>> {
        let lsize = self.left.as_ref().map_or(0, |l| l.count());
        if index < lsize {
            self.left.as_ref().and_then(|n| n.get_by_index(index))
        } else if index == lsize {
            Some(self)
        } else {
            self.right
                .as_ref()
                .and_then(|n| n.get_by_index(index - lsize - 1))
        }
    }

    pub fn get(&self, key: &T) -> Option<&Node<T>> {
        match key.cmp(&self.key) {
            Equal => Some(self),
            Less => self.left.as_ref().and_then(|n| n.get(key)),
            Greater => self.right.as_ref().and_then(|n| n.get(key)),
        }
    }

    pub fn get_mut(&mut self, key: &T) -> Option<&mut Node<T>> {
        match key.cmp(&self.key) {
            Equal => Some(&mut *self),
            Less => self.left.as_mut().and_then(|n| n.get_mut(key)),
            Greater => self.right.as_mut().and_then(|n| n.get_mut(key)),
        }
    }

    pub fn cointains(&self, key: &T) -> bool {
        self.get(key).is_some()
    }
    fn set_left(&mut self, left: Option<Box<Node<T>>>) {
        self.left = left;
        self.update();
    }
    fn set_right(&mut self, right: Option<Box<Node<T>>>) {
        self.right = right;
        self.update();
    }

    fn update(&mut self) {
        match (self.left.as_ref(), self.right.as_ref()) {
            (Some(left), Some(right)) => {
                self.min = left.min;
                self.max = right.max;
                self.count = left.count + right.count + self.frequency;
                self.sum = left.sum + right.sum + self.key;
            }
            (Some(left), None) => {
                self.min = left.min;
                self.max = self.key;
                self.count = left.count + self.frequency;
                self.sum = left.sum + self.key;
            }
            (None, Some(right)) => {
                self.min = self.key;
                self.max = right.max;
                self.count = right.count + self.frequency;
                self.sum = right.sum + self.key;
            }
            (None, None) => {
                self.min = self.key;
                self.max = self.key;
                self.count = self.frequency;
                self.sum = self.key;
            }
        }
    }

    fn max(&self) -> T {
        self.max
    }

    fn min(&self) -> T {
        self.min
    }

    fn lower_bound(&self, key: &T) -> Option<&Node<T>> {
        match key.cmp(&self.key) {
            Equal => Some(self),
            Less => self.left.as_ref().and_then(|n| n.lower_bound(key)),
            Greater => self.right.as_ref().and_then(|n| {
                if let Some(n) = n.lower_bound(key) {
                    Some(n)
                } else {
                    Some(self)
                }
            }),
        }
    }
    fn upper_bound(&self, key: &T) -> Option<&Node<T>> {
        match key.cmp(&self.key) {
            Equal => self.right.as_ref().and_then(|n| n.lower_bound(key)),
            Less => self.left.as_ref().and_then(|n| n.lower_bound(key)),
            Greater => self.right.as_ref().and_then(|n| n.lower_bound(key)),
        }
    }

    fn sum(&self) -> T {
        self.sum
    }
}
