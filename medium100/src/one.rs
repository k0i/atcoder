use proconio::input;

pub fn main() {
    input![n: usize, a: [u64; n]];
    let mut res = 1;
    let mut node = a[0];
    loop {
        if node == 2 {
            println!("{}", res);
            break;
        }
        res += 1;
        node = a[(node as usize) - 1];
        if res == n {
            println!("{}", -1);
            break;
        }
    }
}
