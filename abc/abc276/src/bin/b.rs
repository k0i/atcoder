use proconio::{fastout, input, marker::Usize1};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    m:usize,
    ab:[(Usize1,Usize1);m],
        }

    let mut res = vec![vec![]; n];
    for i in ab {
        let (a, b) = i;
        res[a].push(b);
        res[b].push(a);
    }
    res.iter_mut().for_each(|x| x.sort());

    for i in res {
        print!("{} ", i.len());
        for j in i {
            print!("{} ", j + 1);
        }
    }
}
