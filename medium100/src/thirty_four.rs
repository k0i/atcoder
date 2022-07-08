use proconio::input;
pub fn main() {
input!{
   n:usize,
   k:usize,
   _a:[u64;n]
}

let ans = (n - 1 + k - 2) / (k - 1);
println!("{}",ans);
}