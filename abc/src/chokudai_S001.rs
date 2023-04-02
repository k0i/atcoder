#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::{
    collections::{HashMap, HashSet},
    ops::Sub,
};
#[fastout]
pub fn main() {
    ip! {
    n:usize,
    a:[usize;n]
        }
    let mut t = Treap::new();
    for i in 0..n {
        t.push_back(0usize);
    }
    t.push_back(0);
    let mut ans = 0;
    for i in 0..n {
        let range = t.range_sum(0, a[i]).unwrap_or(0);
        ans += i - range;
        let update = t.get(a[i]).unwrap() + 1;
        t.update(a[i], update);
    }
    println!("{}", ans);
}

use rand::Rng;
use std::{
    cmp::{Ord, Ordering::Equal, Ordering::Greater, Ordering::Less},
    ops::{Add, Index},
};

#[derive(Debug, Clone)]
pub struct Treap<T> {
    root: Option<Box<Node<T>>>,
    size: usize,
    _rnd: rand::rngs::ThreadRng,
}

pub trait RangeTree<T> {
    fn new() -> Self;
    fn get(&mut self, index: usize) -> Option<T>;
    fn insert(&mut self, index: usize, key: T);
    fn push_back(&mut self, key: T);
    fn push_front(&mut self, key: T);
    fn remove(&mut self, index: usize) -> Option<T>;
    fn erase(&mut self, key: &T) -> Option<T>;
    fn pop_back(&mut self) -> Option<T>;
    fn pop_front(&mut self) -> Option<T>;
    fn pop_max(&mut self) -> Option<T>;
    fn pop_min(&mut self) -> Option<T>;
    fn update(&mut self, index: usize, key: T);
    fn min(&self) -> Option<T>;
    fn max(&self) -> Option<T>;
    fn sum(&self) -> Option<T>;
    fn lower_bound(&mut self, key: T, l: usize, r: usize) -> Option<usize>;
    fn upper_bound(&mut self, key: T, l: usize, r: usize) -> Option<usize>;
    fn range_min(&mut self, l: usize, r: usize) -> Option<T>;
    fn range_max(&mut self, l: usize, r: usize) -> Option<T>;
    fn range_sum(&mut self, l: usize, r: usize) -> Option<T>;
    fn contains(&self, key: &T) -> bool;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

impl<T> RangeTree<T> for Treap<T>
where
    T: Ord + Copy + std::fmt::Debug + Add<Output = T> + Sub<Output = T>,
{
    fn new() -> Treap<T> {
        Self {
            root: None,
            size: 0,
            _rnd: rand::thread_rng(),
        }
    }

    fn get(&mut self, index: usize) -> Option<T> {
        self.query(index, index + 1)
    }

    fn insert(&mut self, index: usize, key: T) {
        let priority = self._rnd.gen::<u64>();
        let node = Node::new(key, priority);
        let root = self.root.take();
        let (left, right) = self.split(root, index);
        let new_root = self.merge(left, Some(Box::new(node)));
        let new_root = self.merge(new_root, right);
        self.root = new_root;
        self.size += 1;
    }

    fn push_back(&mut self, key: T) {
        self.insert(self.size, key);
    }

    fn push_front(&mut self, key: T) {
        self.insert(0, key);
    }

    fn remove(&mut self, index: usize) -> Option<T> {
        let root = self.root.take();
        let (l, r) = self.split(root, index + 1);
        let (left, ret) = self.split(l, index);
        let new_root = self.merge(left, r);
        self.root = new_root;
        self.size -= 1;
        ret.map(|n| n.key)
    }
    /// Erase the first element that is equal to key.
    /// Return the erased element.
    /// If there is no such element, return None.
    /// Time complexity: O(logN)
    /// Space complexity: O(logN)
    /// # Examples
    /// ```
    /// use k0i::treap::Treap;
    /// use k0i::treap::RangeTree;
    /// let mut t = Treap::new();
    /// t.push_back(1);
    /// t.push_back(1);
    /// t.push_back(2);
    /// t.push_back(2);
    /// t.push_back(3);
    /// t.erase(&1);
    /// assert_eq!(t.get(0), Some(1));
    /// assert_eq!(t.get(1), Some(2));
    /// assert_eq!(t.len(), 4);
    /// t.erase(&2);
    /// assert_eq!(t.get(1), Some(2));
    /// assert_eq!(t.len(), 3);
    /// t.erase(&2);
    /// assert_eq!(t.get(1), Some(3));
    /// assert_eq!(t.len(), 2);
    ///```
    fn erase(&mut self, key: &T) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let root = self.root.take().unwrap();
        let index = root.as_ref().get(key).map(|(_, ind)| ind);
        self.root = Some(root);
        match index {
            None => None,
            Some(index) => self.remove(index),
        }
    }
    fn pop_back(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            let key = self.get(self.size - 1);
            self.remove(self.size - 1);
            key
        }
    }
    fn pop_front(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            let key = self.get(0);
            self.remove(0);
            key
        }
    }
    fn pop_max(&mut self) -> Option<T> {
        match self.max() {
            None => None,
            Some(max) => self.erase(&max),
        }
    }

    fn pop_min(&mut self) -> Option<T> {
        match self.min() {
            None => None,
            Some(min) => self.erase(&min),
        }
    }

    fn update(&mut self, index: usize, key: T) {
        self.remove(index);
        self.insert(index, key);
    }
    fn min(&self) -> Option<T> {
        self.root.as_ref().map(|n| n.min)
    }

    fn max(&self) -> Option<T> {
        self.root.as_ref().map(|n| n.max)
    }
    fn sum(&self) -> Option<T> {
        self.root.as_ref().map(|n| n.sum)
    }

    /// ```
    /// use k0i::treap::Treap;
    /// use k0i::treap::RangeTree;
    /// let mut treap = Treap::new();
    /// treap.insert(0,1);
    /// treap.insert(1,2);
    /// treap.insert(2,3);
    /// treap.insert(3,2);
    /// treap.insert(4,1);
    /// assert_eq!(treap.lower_bound(2,0,5),Some(1));
    /// assert_eq!(treap.lower_bound(3,1,5),Some(2));
    /// assert_eq!(treap.lower_bound(4,0,5),None);
    /// assert_eq!(treap.lower_bound(0,0,5),Some(0));
    /// assert_eq!(treap.lower_bound(3,1,5),Some(2));
    /// assert_eq!(treap.lower_bound(2,2,5),Some(2));
    /// assert_eq!(treap.lower_bound(2,3,5),Some(3));
    /// treap.update(0, 3);
    /// treap.update(1, 5);
    /// assert_eq!(treap.lower_bound(3,0,5),Some(0));
    /// assert_eq!(treap.lower_bound(4,0,5),Some(1));
    /// ```
    fn lower_bound(&mut self, key: T, l: usize, r: usize) -> Option<usize> {
        if l >= r {
            return None;
        }
        let root = self.root.take();
        let (left, right) = self.split(root, l);
        let (ll, rr) = self.split(right, r - l);

        fn cmp_func<'a, 'b: 'a, T: Ord>(a: &'a T, b: &'b T) -> &'a T {
            if a >= b {
                a
            } else {
                b
            }
        }
        let ret = ll.as_ref().unwrap().find(&key, l, &cmp_func);
        let new_root = self.merge(left, ll);
        let new_root = self.merge(new_root, rr);
        self.root = new_root;
        ret
    }
    fn upper_bound(&mut self, key: T, l: usize, r: usize) -> Option<usize> {
        if l >= r {
            return None;
        }
        let root = self.root.take();
        let (left, right) = self.split(root, l);
        let (ll, rr) = self.split(right, r - l);
        fn cmp_func<'a, 'b: 'a, T: Ord>(a: &'a T, b: &'b T) -> &'a T {
            if a > b {
                a
            } else {
                b
            }
        }
        let ret = ll.as_ref().unwrap().find(&key, l, &cmp_func);
        let new_root = self.merge(left, ll);
        let new_root = self.merge(new_root, rr);
        self.root = new_root;
        ret
    }

    fn range_min(&mut self, l: usize, r: usize) -> Option<T> {
        if l == r {
            return None;
        }
        let root = self.root.take();
        let (left, right) = self.split(root, l);
        let (ll, rr) = self.split(right, r - l);
        let ret = ll.as_ref().map(|n| n.min);
        let new_root = self.merge(left, ll);
        let new_root = self.merge(new_root, rr);
        self.root = new_root;
        ret
    }
    /// return max value in [l, r)
    /// if l == r, return None
    /// ```
    /// use k0i::treap::Treap;
    /// use k0i::treap::RangeTree;
    /// ```
    /// let mut treap = Treap::new();
    /// treap.push_back(1);
    /// treap.push_back(2);
    /// treap.push_back(3);
    /// assert_eq!(treap.range_max(0, 3), Some(3));
    /// assert_eq!(treap.range_max(0, 2), Some(2));
    /// assert_eq!(treap.range_max(1, 2), Some(2));
    /// assert_eq!(treap.range_max(1, 3), Some(3));
    /// assert_eq!(treap.range_max(2, 3), Some(3));
    /// treap.update(1, 4);
    /// assert_eq!(treap.range_max(0, 3), Some(4));
    /// assert_eq!(treap.range_max(0, 0), None);
    /// assert_eq!(treap.range_max(0, 1), Some(1));
    /// treap.update(0, 5);
    /// assert_eq!(treap.range_max(0, 3), Some(5));
    /// assert_eq!(treap.range_max(0, 1), Some(5));
    /// ```
    fn range_max(&mut self, l: usize, r: usize) -> Option<T> {
        if l == r {
            return None;
        }
        let root = self.root.take();
        let (left, right) = self.split(root, l);
        let (ll, rr) = self.split(right, r - l);
        let ret = ll.as_ref().map(|n| n.max);
        let new_root = self.merge(left, ll);
        let new_root = self.merge(new_root, rr);
        self.root = new_root;
        ret
    }

    fn range_sum(&mut self, l: usize, r: usize) -> Option<T> {
        if l == r {
            return None;
        }
        let root = self.root.take();
        let (left, right) = self.split(root, l);
        let (ll, rr) = self.split(right, r - l);
        let ret = ll.as_ref().map(|n| n.sum);
        let new_root = self.merge(left, ll);
        let new_root = self.merge(new_root, rr);
        self.root = new_root;
        ret
    }

    fn contains(&self, key: &T) -> bool {
        self.root.as_ref().map_or(false, |n| n.cointains(key))
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<T> Treap<T>
where
    T: Ord + Copy + std::fmt::Debug + Add<Output = T> + Sub<Output = T>,
{
    fn query(&mut self, l: usize, r: usize) -> Option<T> {
        if l == r {
            return None;
        }
        let root = self.root.take();
        let (left, right) = self.split(root, l);
        let (ll, rr) = self.split(right, r - l);
        let ret = ll.as_ref().map(|n| n.key);
        let new_root = self.merge(left, ll);
        let new_root = self.merge(new_root, rr);
        self.root = new_root;
        ret
    }
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
        index: usize,
    ) -> (Option<Box<Node<T>>>, Option<Box<Node<T>>>) {
        match node {
            Some(mut n) => {
                let implicit_key = n.left.as_ref().map_or(0, |l| l.count) + 1;
                if index < implicit_key {
                    let l = n.left.take();
                    let (left, right) = self.split(l, index);
                    n.set_left(right);
                    (left, Some(n))
                } else {
                    let r = n.right.take();
                    let (left, right) = self.split(r, index - implicit_key);
                    n.set_right(left);
                    (Some(n), right)
                }
            }
            None => (None, None),
        }
    }
}

impl<T> Index<usize> for Treap<T>
where
    T: Ord + Copy + std::fmt::Debug + Add<Output = T> + Sub<Output = T>,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.root
            .as_ref()
            .map(|n| n.get_index(index))
            .flatten()
            .map(|n| &n.key)
            .unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct Node<T> {
    key: T,
    priority: u64,
    count: usize,
    min: T,
    max: T,
    sum: T,
    rev: bool,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: Ord + Copy + std::fmt::Debug + Add<Output = T>,
{
    pub fn new(key: T, priority: u64) -> Node<T> {
        Node {
            key,
            priority,
            count: 1,
            min: key,
            max: key,
            sum: key,
            rev: false,
            left: None,
            right: None,
        }
    }

    pub fn get_index(&self, index: usize) -> Option<&Node<T>> {
        let left_count = self.left.as_ref().map_or(0, |n| n.count);
        match index.cmp(&left_count) {
            Equal => Some(self),
            Less => self.left.as_ref().and_then(|n| n.get_index(index)),
            Greater => self
                .right
                .as_ref()
                .and_then(|n| n.get_index(index - left_count - 1)),
        }
    }

    pub fn get_index_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
        let left_count = self.left.as_ref().map_or(0, |n| n.count);
        match index.cmp(&left_count) {
            Equal => Some(&mut *self),
            Less => self.left.as_mut().and_then(|n| n.get_index_mut(index)),
            Greater => self
                .right
                .as_mut()
                .and_then(|n| n.get_index_mut(index - left_count - 1)),
        }
    }

    pub fn get(&self, key: &T) -> Option<(&Node<T>, usize)> {
        match key.cmp(&self.key) {
            Equal => {
                let index = self.left.as_ref().map_or(0, |n| n.count);
                Some((self, index))
            }
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
                self.min = self.key.min(left.min).min(right.min);
                self.max = self.key.max(left.max).max(right.max);
                self.count = left.count + right.count + 1;
                self.sum = left.sum + right.sum + self.key;
            }
            (Some(left), None) => {
                self.min = self.key.min(left.min);
                self.max = self.key.max(left.max);
                self.count = left.count + 1;
                self.sum = left.sum + self.key;
            }
            (None, Some(right)) => {
                self.min = self.key.min(right.min);
                self.max = self.key.max(right.max);
                self.count = right.count + 1;
                self.sum = right.sum + self.key;
            }
            (None, None) => {
                self.min = self.key;
                self.max = self.key;
                self.count = 1;
                self.sum = self.key;
            }
        }
    }

    fn find<'a, 'b: 'a>(
        &'b self,
        target: &'b T,
        offset: usize,
        monoid: &dyn Fn(&'a T, &'b T) -> &'a T,
    ) -> Option<usize> {
        if std::ptr::eq(monoid(&self.max, target), target) {
            return None;
        }
        if self.left.is_some()
            && !std::ptr::eq(monoid(&self.left.as_ref().unwrap().max, target), target)
        {
            return self.left.as_ref().unwrap().find(target, offset, monoid);
        } else {
            if !std::ptr::eq(monoid(&self.key, target), target) {
                return Some(offset + self.left.as_ref().map_or(0, |l| l.count));
            }
            return self.right.as_ref()?.find(
                target,
                offset + self.left.as_ref().map_or(0, |l| l.count) + 1,
                monoid,
            );
        }
    }
}
