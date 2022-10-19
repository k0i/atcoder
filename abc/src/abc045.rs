use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        sa: String,
        sb: String,
        sc: String,
    };
    let ma = sa.chars().map(m).collect::<Vec<usize>>();
    let mb = sb.chars().map(m).collect::<Vec<usize>>();
    let mc = sc.chars().map(m).collect::<Vec<usize>>();
    let mm = vec![ma, mb, mc];

    let mut turn = 0;
    let mut pos = vec![0; 3];
    while pos[turn] < mm[turn].len() {
        let old_turn = turn;
        turn = mm[turn][pos[turn]];
        pos[old_turn] += 1;
    }
    println!(
        "{}",
        if turn == 0 {
            "A"
        } else if turn == 1 {
            "B"
        } else {
            "C"
        }
    );
}
fn m(c: char) -> usize {
    match c {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        _ => unreachable!(),
    }
}
