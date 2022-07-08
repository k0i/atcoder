use proconio::input;
pub fn main() {
    input! {
    n:usize,m:usize,
    s:[(i64,i64);n],
    c:[(i64,i64);m]
       }
    let mut res: Vec<(i64, usize)> = vec![(i64::MAX, 51); n];

    for i in 0..s.len() {
        for j in 0..c.len() {
            res[i] = match (s[i].0 - c[j].0).abs() + (s[i].1 - c[j].1).abs() < res[i].0 {
                true => ((s[i].0 - c[j].0).abs() + (s[i].1 - c[j].1).abs(), j + 1),
                false => res[i],
            }
        }
        println!("{}", res[i].1);
    }
}
