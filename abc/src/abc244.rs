use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, source::line::LineSource};
use std::io::{stdin, stdout, BufReader, Write};
pub fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
          from &mut source,
    n:usize
        }
    let mut av = HashSet::new();
    for i in 1..=n * 2 + 1 {
        av.insert(i);
    }
    loop {
        let a = *av.iter().next().unwrap();
        println!("{}", a);
        stdout().flush().unwrap();
        av.remove(&a);
        input! {
        from &mut source,
        b:usize
        }
        if b == 0 {
            return;
        }
        av.remove(&b);
    }
}
