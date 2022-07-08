use proconio::input;
pub fn main() {
    input! {
    a:u64,
    b:u64,
    c:u64,
    d:u64
    }
    let first = b / c - (a - 1) / c;
    let second = b / d - (a - 1) / d;
    let lcm = lcm(c, d);
    let third = b / lcm - (a - 1) / lcm;
    let res = (b - a) + 1 - first - second + third;

    println!("{}", res);
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
