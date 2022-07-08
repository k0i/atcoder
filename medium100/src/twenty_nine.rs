use proconio::input;
use proconio::marker::Chars;

pub fn main() {
    input! {
      mut s:Chars,
      mut t:Chars
    }
    s.sort();
    t.sort();
    t.reverse();

  let order = s.cmp(&t);
 match order {
    std::cmp::Ordering::Equal => println!("No"),
    std::cmp::Ordering::Less => println!("Yes"),
    _ => println!("No")
 }
}
