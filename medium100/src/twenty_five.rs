use proconio::input;
pub fn main() {
    input! {
        n: usize,
        k: usize,
        a:[u64;n]
    }
    let mut hs = vec![0; n];
    for i in a {
        hs[(i - 1) as usize] += 1;
    }
    let mut fil = hs.into_iter().filter(|x| *x != 0).collect::<Vec<u64>>();
    fil.sort();
    if fil.len() <= k {
        println!("{}", 0);
    } else {
        let mut ans = 0;
        for i in 0..fil.len() - k {
            ans += fil[i];
        }
        println!("{}", ans);
    }
}
