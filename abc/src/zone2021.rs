use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet, VecDeque};
#[fastout]
pub fn main() {
    d()
}

fn d() {
    input! {
    s:Chars
        }
    let mut ans = VecDeque::new();

    let mut reversed = false;
    let mut last = '1';
    for i in 0..s.len() {
        let mut cur = s[i];
        if last == cur {
            if reversed {
                ans.pop_front();
                last = *ans.front().unwrap_or(&'1');
            } else {
                ans.pop_back();
                last = *ans.back().unwrap_or(&'1');
            }
            continue;
        }
        if cur == 'R' {
            reversed = !reversed;
            if reversed {
                last = *ans.front().unwrap_or(&'1');
            } else {
                last = *ans.back().unwrap_or(&'1');
            }
            continue;
        } else {
            if reversed {
                ans.push_front(cur);
            } else {
                ans.push_back(cur);
            }
        }
        last = cur;
    }
    if reversed {
        for i in ans.iter().rev() {
            print!("{}", i);
        }
        return;
    }
    for i in ans.iter() {
        print!("{}", i);
    }
}
