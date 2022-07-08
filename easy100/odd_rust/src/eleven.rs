use proconio::input;
pub fn main() {
    input! {n:u16}

    let mut max = 0;
    let mut res = 0;
    if n == 1 {
        println!("1")
    } else {
        for i in 1..=n {
            if i % 2 == 0 {
                let mut div = 0;
                let mut num = i;
                while num % 2 == 0 {
                    div += 1;
                    num /= 2;
                }
                if max < div {
                    max = div;
                    res = i;
                }
            }
        }
        println!("{}", res);
    }
}
