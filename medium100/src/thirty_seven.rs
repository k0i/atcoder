use proconio::input;

pub fn main() {
    input! {
       n:usize,
       mut x:[u16;n],
    }
    let mut res = 0;
    'outer: loop {
        if x.iter().max().unwrap() == &0 {
            break 'outer;
        } else {
            let mut i = 0;
            while i < n {
                if x[i] == 0 {
                    i += 1;
                    continue;
                } else {
                    res += 1;
                    while i < n && x[i] > 0 {
                        x[i] -= 1;
                        i += 1;
                    }
                }
            }
        }
    }
    println!("{}", res);
}