use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    a:Usize1,
    b:Usize1,
        }
    let tree = vec![
        vec![1, 2],
        vec![0, 3, 4],
        vec![0, 5, 6],
        vec![1, 7, 8],
        vec![1, 9, 10],
        vec![2, 11, 12],
        vec![2, 13, 14],
        vec![3, 15, 16],
        vec![3, 17, 18],
        vec![4, 19, 20],
        vec![4, 21, 22],
        vec![5, 23, 24],
        vec![5, 25, 26],
        vec![6, 27, 28],
        vec![6, 29, 30],
        vec![7, 31, 32],
        vec![7, 33, 34],
        vec![8, 35, 36],
        vec![8, 37, 38],
        vec![9, 39, 40],
        vec![9, 41, 42],
        vec![10, 43, 44],
        vec![10, 45, 46],
        vec![11, 47, 48],
        vec![11, 49, 50],
        vec![12, 51, 52],
        vec![12, 53, 54],
        vec![13, 55, 56],
        vec![13, 57, 58],
        vec![14, 59, 60],
        vec![14, 61, 62],
        vec![15, 63, 64],
        vec![15, 65, 66],
        vec![16, 67, 68],
        vec![16, 69, 70],
        vec![17, 71, 72],
        vec![17, 73, 74],
        vec![18, 75, 76],
        vec![18, 77, 78],
        vec![19, 79, 80],
        vec![19, 81, 82],
        vec![20, 83, 84],
        vec![20, 85, 86],
        vec![21, 87, 88],
        vec![21, 89, 90],
        vec![22, 91, 92],
        vec![22, 93, 94],
        vec![23, 95, 96],
        vec![23, 97, 98],
        vec![24, 99, 100],
        vec![24, 101, 102],
        vec![25, 103, 104],
        vec![25, 105, 106],
        vec![26, 107, 108],
        vec![26, 109, 110],
        vec![27, 111, 112],
        vec![27, 113, 114],
        vec![28, 115, 116],
        vec![28, 117, 118],
        vec![29, 119, 120],
        vec![29, 121, 122],
        vec![30, 123, 124],
        vec![30, 125, 126],
        vec![31, 127, 128],
        vec![31, 129, 130],
        vec![32, 131, 132],
        vec![32, 133, 134],
        vec![33, 135],
    ];

    if tree[a].contains(&b) {
        println!("Yes");
    } else {
        println!("No");
    }
}
