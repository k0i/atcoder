use proconio::{fastout, input};
#[fastout]
pub fn main() {
    input! {
    mut h:usize,
    mut m:usize
        }

    loop {
        let mut hs = format!("{:0>2}", h.to_string());
        let mut ms = format!("{:0>2}", m.to_string());

        let sh = format!(
            "{}{}",
            hs.chars().next().unwrap(),
            ms.chars().next().unwrap()
        );
        let sm = format!(
            "{}{}",
            hs.to_string().chars().nth(1).unwrap(),
            ms.to_string().chars().nth(1).unwrap()
        );

        if sh.parse::<usize>().unwrap() < 24 && sm.parse::<usize>().unwrap() < 60 {
            println!("{} {}", h, m);
            return;
        }
        m += 1;
        if m == 60 {
            m = 0;
            h += 1;
        }
        if h == 24 {
            h = 0;
        }
    }
}
