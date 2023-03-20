use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
pub fn main() {
    input! {
    n:usize,
    q:usize,
    ab:[(usize,Usize1);n],
    cd:[(Usize1,Usize1);q]
        }
    let mut a = vec![0; n];
    let mut b = vec![0; n];

    let mut garden = vec![TreapSet::new(); 200000];
    for i in 0..n {
        a[i] = ab[i].0;
        b[i] = ab[i].1;
        let (a, b) = ab[i];
        garden[b].insert(a);
    }
    let mut max_treap = TreapSet::new();
    for i in 0..200000 {
        if garden[i].is_empty() {
            continue;
        }
        let garden_max = garden[i].max().unwrap().key;
        max_treap.insert(garden_max);
    }

    for (c, d) in cd {
        let rate = a[c];
        let now = b[c];
        let temp = garden[now].max().unwrap().key;
        if temp == rate {
            garden[now].erase(&rate);
            max_treap.erase(&rate);
            if !garden[now].is_empty() {
                let new_max = garden[now].max().unwrap().key;
                max_treap.insert(new_max);
            }
        } else {
            garden[now].erase(&rate);
        }
        if !garden[d].is_empty() {
            let new_max = garden[d].max().unwrap().key;
            if rate > new_max {
                max_treap.erase(&new_max);
                max_treap.insert(rate);
            }
        } else {
            max_treap.insert(rate);
        }
        garden[d].insert(rate);
        b[c] = d;
        let ans = max_treap.min().unwrap().key;
        println!("{}", ans);
    }
}

use std::cmp::Ordering;

use rand::Rng;
use std::cmp::{Ord, Ordering::Equal, Ordering::Greater, Ordering::Less};

#[derive(Debug, Clone)]
pub struct TreapSet<T> {
    root: Option<Box<Node<T, ()>>>,
    size: usize,
    _rnd: rand::rngs::ThreadRng,
}

impl<T: Ord> TreapSet<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
            _rnd: rand::thread_rng(),
        }
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn merge(
        &mut self,
        mut left: Option<Box<Node<T, ()>>>,
        mut right: Option<Box<Node<T, ()>>>,
    ) -> Option<Box<Node<T, ()>>> {
        match (left.as_mut(), right.as_mut()) {
            (Some(l), Some(r)) => {
                if l.priority > r.priority {
                    let rr = self.merge(l.right.take(), right);
                    l.set_right(rr);
                    left
                } else {
                    let ll = self.merge(left, r.left.take());
                    r.set_left(ll);
                    right
                }
            }
            (Some(_), None) => left,
            (None, Some(_s)) => right,
            (None, None) => None,
        }
    }

    pub fn split(
        &mut self,
        node: Option<Box<Node<T, ()>>>,
        key: &T,
    ) -> (Option<Box<Node<T, ()>>>, Option<Box<Node<T, ()>>>) {
        match node {
            Some(mut n) => match key.cmp(&n.key) {
                Equal => {
                    let right = n.right.take();
                    (n.left.take(), right)
                }
                Less => {
                    let (left, right) = self.split(n.left.take(), key);
                    n.set_left(right);
                    (left, Some(n))
                }
                Greater => {
                    let (left, right) = self.split(n.right.take(), key);
                    n.set_right(left);
                    (Some(n), right)
                }
            },
            None => (None, None),
        }
    }

    pub fn insert(&mut self, key: T) {
        // if key duplicates, update frequency
        if let Some(n) = self.get_mut(&key) {
            n.frequency += 1;
            return;
        }
        let priority = self._rnd.gen::<u64>();
        let node = Node::new(key, (), priority);
        let root = self.root.take();
        let (left, right) = self.split(root, &node.key);
        let new_root = self.merge(left, Some(Box::new(node)));
        let new_root = self.merge(new_root, right);
        self.root = new_root;
        self.size += 1;
    }

    pub fn erase(&mut self, key: &T) {
        // if duplicates, decrease frequency
        if let Some(n) = self.get_mut(key) {
            if n.frequency > 1 {
                n.frequency -= 1;
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

    pub fn contains(&self, key: &T) -> bool {
        self.root.as_ref().map_or(false, |n| n.cointains(key))
    }
    pub fn get(&self, key: &T) -> Option<&Node<T, ()>> {
        self.root.as_ref().and_then(|n| n.get(key))
    }

    pub fn get_mut(&mut self, key: &T) -> Option<&mut Node<T, ()>> {
        self.root.as_mut().and_then(|n| n.get_mut(key))
    }

    pub fn min(&self) -> Option<&Node<T, ()>> {
        let mut node = self.root.as_ref();
        while let Some(n) = node {
            if n.left.is_none() {
                return Some(n);
            }
            node = n.left.as_ref();
        }
        None
    }
    pub fn max(&self) -> Option<&Node<T, ()>> {
        let mut node = self.root.as_ref();
        while let Some(n) = node {
            if n.right.is_none() {
                return Some(n);
            }
            node = n.right.as_ref();
        }
        None
    }
    pub fn min_range(&self, left: &T, right: &T) -> Option<&Node<T, ()>> {
        let mut node = self.root.as_ref();
        let mut min = None;
        while let Some(n) = node {
            match n.key.cmp(left) {
                Ordering::Equal => return Some(n),
                Ordering::Less => {
                    node = n.right.as_ref();
                }
                Ordering::Greater => {
                    min = Some(n);
                    node = n.left.as_ref();
                }
            }
        }
        min.and_then(|n| n.get(right))
    }
    pub fn max_range(&self, left: &T, right: &T) -> Option<&Node<T, ()>> {
        let mut node = self.root.as_ref();
        let mut max = None;
        while let Some(n) = node {
            match n.key.cmp(right) {
                Equal => return Some(n),
                Less => {
                    max = Some(n);
                    node = n.right.as_ref();
                }
                Greater => {
                    node = n.left.as_ref();
                }
            }
        }
        max.and_then(|n| n.get(left))
    }
}

#[derive(Debug, Clone)]
pub struct Treap<T, U> {
    root: Option<Box<Node<T, U>>>,
    size: usize,
    _rnd: rand::rngs::ThreadRng,
}

impl<T: Ord, U> Treap<T, U> {
    pub fn new() -> Treap<T, U> {
        Self {
            root: None,
            size: 0,
            _rnd: rand::thread_rng(),
        }
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn merge(
        &mut self,
        mut left: Option<Box<Node<T, U>>>,
        mut right: Option<Box<Node<T, U>>>,
    ) -> Option<Box<Node<T, U>>> {
        match (left.as_mut(), right.as_mut()) {
            (Some(l), Some(r)) => {
                if l.priority > r.priority {
                    let rr = self.merge(l.right.take(), right);
                    l.set_right(rr);
                    left
                } else {
                    let ll = self.merge(left, r.left.take());
                    r.set_left(ll);
                    right
                }
            }
            (Some(_), None) => left,
            (None, Some(_s)) => right,
            (None, None) => None,
        }
    }

    pub fn split(
        &mut self,
        node: Option<Box<Node<T, U>>>,
        key: &T,
    ) -> (Option<Box<Node<T, U>>>, Option<Box<Node<T, U>>>) {
        match node {
            Some(mut n) => match key.cmp(&n.key) {
                Equal => {
                    let right = n.right.take();
                    (n.left.take(), right)
                }
                Less => {
                    let (left, right) = self.split(n.left.take(), key);
                    n.set_left(right);
                    (left, Some(n))
                }
                Greater => {
                    let (left, right) = self.split(n.right.take(), key);
                    n.set_right(left);
                    (Some(n), right)
                }
            },
            None => (None, None),
        }
    }

    pub fn insert(&mut self, key: T, value: U) {
        // if key duplicates, update frequency
        if let Some(n) = self.get_mut(&key) {
            n.value = value;
            n.frequency += 1;
            return;
        }
        let priority = self._rnd.gen::<u64>();
        let node = Node::new(key, value, priority);
        let root = self.root.take();
        let (left, right) = self.split(root, &node.key);
        let new_root = self.merge(left, Some(Box::new(node)));
        let new_root = self.merge(new_root, right);
        self.root = new_root;
        self.size += 1;
    }

    pub fn erase(&mut self, key: &T) {
        // if duplicates, decrease frequency
        if let Some(n) = self.get_mut(key) {
            if n.frequency > 1 {
                n.frequency -= 1;
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

    pub fn contains(&self, key: &T) -> bool {
        self.root.as_ref().map_or(false, |n| n.cointains(key))
    }
    pub fn get(&self, key: &T) -> Option<&Node<T, U>> {
        self.root.as_ref().and_then(|n| n.get(key))
    }

    pub fn get_mut(&mut self, key: &T) -> Option<&mut Node<T, U>> {
        self.root.as_mut().and_then(|n| n.get_mut(key))
    }

    pub fn min(&self) -> Option<&Node<T, U>> {
        let mut node = self.root.as_ref();
        while let Some(n) = node {
            if n.left.is_none() {
                return Some(n);
            }
            node = n.left.as_ref();
        }
        None
    }
    pub fn max(&self) -> Option<&Node<T, U>> {
        let mut node = self.root.as_ref();
        while let Some(n) = node {
            if n.right.is_none() {
                return Some(n);
            }
            node = n.right.as_ref();
        }
        None
    }
    pub fn min_range(&self, left: &T, right: &T) -> Option<&Node<T, U>> {
        let mut node = self.root.as_ref();
        let mut min = None;
        while let Some(n) = node {
            match n.key.cmp(left) {
                Ordering::Equal => return Some(n),
                Ordering::Less => {
                    node = n.right.as_ref();
                }
                Ordering::Greater => {
                    min = Some(n);
                    node = n.left.as_ref();
                }
            }
        }
        min.and_then(|n| n.get(right))
    }
    pub fn max_range(&self, left: &T, right: &T) -> Option<&Node<T, U>> {
        let mut node = self.root.as_ref();
        let mut max = None;
        while let Some(n) = node {
            match n.key.cmp(right) {
                Equal => return Some(n),
                Less => {
                    max = Some(n);
                    node = n.right.as_ref();
                }
                Greater => {
                    node = n.left.as_ref();
                }
            }
        }
        max.and_then(|n| n.get(left))
    }
}

#[derive(Debug, Clone)]
pub struct Node<T, U> {
    key: T,
    priority: u64,
    value: U,
    count: usize,
    frequency: usize,
    pub left: Option<Box<Node<T, U>>>,
    pub right: Option<Box<Node<T, U>>>,
}

impl<T: Ord, U> Node<T, U> {
    pub fn new(key: T, value: U, priority: u64) -> Node<T, U> {
        Node {
            key,
            priority,
            value,
            count: 1,
            frequency: 1,
            left: None,
            right: None,
        }
    }

    pub fn get(&self, key: &T) -> Option<&Node<T, U>> {
        match key.cmp(&self.key) {
            Equal => Some(self),
            Less => self.left.as_ref().and_then(|n| n.get(key)),
            Greater => self.right.as_ref().and_then(|n| n.get(key)),
        }
    }

    pub fn get_mut(&mut self, key: &T) -> Option<&mut Node<T, U>> {
        match key.cmp(&self.key) {
            Equal => Some(&mut *self),
            Less => self.left.as_mut().and_then(|n| n.get_mut(key)),
            Greater => self.right.as_mut().and_then(|n| n.get_mut(key)),
        }
    }

    pub fn cointains(&self, key: &T) -> bool {
        self.get(key).is_some()
    }
    fn set_left(&mut self, left: Option<Box<Node<T, U>>>) {
        self.left = left;
        self.update();
    }
    fn set_right(&mut self, right: Option<Box<Node<T, U>>>) {
        self.right = right;
        self.update();
    }

    fn update(&mut self) {
        self.count = 1;
        if let Some(l) = self.left.as_ref() {
            self.count += l.count;
        }
        if let Some(r) = self.right.as_ref() {
            self.count += r.count;
        }
    }
}
