use proconio::input;
pub fn main() {
    input! {
    w:usize,
    h:usize
    }
    let mut point = 0;
    if w % 4 == 0 {
        point = w / 4;
    }
    if h / point == 3 {
        println!("4:3");
    } else {
        println!("16:9");
    }
}
