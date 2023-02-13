use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::{cmp::Ordering, collections::VecDeque};

#[fastout]
pub fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    };

    let mut que = VecDeque::new();
    let mut d = vec![vec![std::i64::MAX; w]; h];
    d[0][0] = 0;
    que.push_back((0isize, 0isize));

    while let Some((x, y)) = que.pop_back() {
        let mut dis = d[x as usize][y as usize];
        for &(dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            let nx = (dx + x) as usize;
            let ny = (dy + y) as usize;
            if nx < h && ny < w && s[nx][ny] == '.' && d[nx][ny] > dis {
                que.push_back((nx as isize, ny as isize));
                d[nx][ny] = dis;
            }
        }
        dis += 1;
        for dx in -2isize..=2 {
            for dy in -2isize..=2 {
                if dx.abs() + dy.abs() == 4 {
                    continue;
                }
                let nx = (dx + x) as usize;
                let ny = (dy + y) as usize;
                if nx < h && ny < w && d[nx][ny] > dis {
                    que.push_front((nx as isize, ny as isize));
                    d[nx][ny] = dis;
                }
            }
        }
    }
    println!("{}", d[h - 1][w - 1]);
}

fn d() {
    input! {
        n: usize,
    }
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        input! {
            a: Usize1,
            b: Usize1,
        }
        g[a].push(b);
        g[b].push(a);
    }
    for i in 0..n {
        g[i].sort();
    }
    let mut res = vec![];
    println!("{:?}", g);
    dfs(0, n, &g, &mut res);
    println!("{}", res.iter().map(|x| (x + 1).to_string()).join(" "));
}

fn dfs(v: usize, p: usize, g: &Vec<Vec<usize>>, res: &mut Vec<usize>) {
    res.push(v);
    println!("outer:{:?}", res);
    for &u in &g[v] {
        if p != u {
            dfs(u, v, g, res);
            res.push(v);
            println!("inner{:?}", res);
        }
    }
}

fn c() {
    input! {
    h:usize,
    w:usize,
    n:usize,
    ab:[(usize,usize);n]
        }

    let mut row = vec![];
    let mut col = vec![];
    let mut row_compressed = vec![];
    let mut col_compressed = vec![];
    for i in 0..n {
        let (x, y) = ab[i];
        row.push(x);
        col.push(y);
    }
    row.sort();
    col.sort();
    row.dedup();
    col.dedup();
    for i in 0..n {
        let (x, y) = ab[i];
        let a = row.lower_bound(x).unwrap();
        let b = col.lower_bound(y).unwrap();
        row_compressed.push(a);
        col_compressed.push(b);
    }
    for i in 0..n {
        println!("{} {}", row_compressed[i] + 1, col_compressed[i] + 1);
    }
}

// Returns an iterator to specified bound that pointing to the first element in the range.
pub trait Bound<T> {
    fn lower_bound(&self, x: T) -> Result<usize, usize>;
    fn upper_bound(&self, x: T) -> Result<usize, usize>;
}

impl<T: Ord> Bound<T> for [T] {
    /// Returns an iterator pointing to the first element in the range [first,last) which does not compare less than val.
    /// If the iterator is empty, the function returns Err(0).
    /// If the given element larger than all elements in the range, the function returns Err(length of the range).
    /// ```
    /// use k0i::bounds::Bound;
    /// let vec = vec![1, 2, 4, 6];
    /// assert_eq!(vec.lower_bound(2), Ok(1));
    /// assert_eq!(vec.lower_bound(3), Ok(2));
    /// assert_eq!(vec.lower_bound(0), Ok(0));
    /// assert_eq!(vec.lower_bound(6), Ok(3));
    /// assert_eq!(vec.lower_bound(7), Err(4));
    /// ```
    fn lower_bound(&self, x: T) -> Result<usize, usize> {
        if self.is_empty() {
            return Err(0);
        }
        if self.iter().last().unwrap() < &x {
            return Err(self.len());
        }
        let mut low = 0;
        let mut high = self.len();

        while low < high {
            let mid = (low + high) / 2;
            match self[mid].cmp(&x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                _ => {
                    high = mid;
                }
            }
        }
        Ok(low)
    }
    /// Returns an iterator pointing to the first element in the range [first,last) which compares greater than val.
    /// If the iterator is empty, the function returns Err(0).
    /// If the given element larger than all elements in the range, the function returns Err(length of the range).
    /// ```
    /// use k0i::bounds::Bound;
    /// let vec = vec![1, 2, 4, 6];
    /// assert_eq!(vec.upper_bound(4), Ok(3));
    /// assert_eq!(vec.upper_bound(0), Ok(0));
    /// assert_eq!(vec.upper_bound(3), Ok(2));
    /// assert_eq!(vec.upper_bound(6), Err(4));
    /// assert_eq!(vec.upper_bound(7), Err(4));
    /// ```
    fn upper_bound(&self, x: T) -> Result<usize, usize> {
        if self.is_empty() {
            return Err(0);
        }
        if self.iter().last().unwrap() <= &x {
            return Err(self.len());
        }
        let mut low = 0;
        let mut high = self.len();

        while low < high {
            let mid = (low + high) / 2;
            match self[mid].cmp(&x) {
                Ordering::Greater => {
                    high = mid;
                }
                _ => {
                    low = mid + 1;
                }
            }
        }
        Ok(low)
    }
}
