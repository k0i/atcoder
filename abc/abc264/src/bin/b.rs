use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    r:Usize1,
    c:Usize1
        }
    let v = [
        [
            '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
        ],
        [
            '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#',
        ],
        [
            '#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#',
        ],
        [
            '#', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '#',
        ],
        [
            '#', '.', '#', '.', '#', '#', '#', '#', '#', '#', '#', '.', '#', '.', '#',
        ],
        [
            '#', '.', '#', '.', '#', '.', '.', '.', '.', '.', '#', '.', '#', '.', '#',
        ],
        [
            '#', '.', '#', '.', '#', '.', '#', '#', '#', '.', '#', '.', '#', '.', '#',
        ],
        [
            '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#', '.', '#',
        ],
        [
            '#', '.', '#', '.', '#', '.', '#', '#', '#', '.', '#', '.', '#', '.', '#',
        ],
        [
            '#', '.', '#', '.', '#', '.', '.', '.', '.', '.', '#', '.', '#', '.', '#',
        ],
        [
            '#', '.', '#', '.', '#', '#', '#', '#', '#', '#', '#', '.', '#', '.', '#',
        ],
        [
            '#', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '#',
        ],
        [
            '#', '.', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '.', '#',
        ],
        [
            '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#',
        ],
        [
            '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
        ],
    ];
    if v[r][c] == '#' {
        println!("black");
    } else {
        println!("white");
    }
}
