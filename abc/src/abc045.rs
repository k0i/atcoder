use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    mut s:Chars,
    }
    let mut s: Vec<usize> = s
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let mut ans = 0;
    for i in 0..1 << s.len() {
        let mut temp = String::new();
        let mut op = vec![];
        for j in 0..s.len() {
            if i >> j & 1 == 1 {
                op.push(temp.parse::<usize>().unwrap_or(0));
                temp = String::new();
                continue;
            }
            temp.push_str(&s[j].to_string());
        }
        op.push(temp.parse::<usize>().unwrap_or(0));
        ans += op.into_iter().sum::<usize>();
    }
    println!("{}", ans);
}
fn b() {
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
