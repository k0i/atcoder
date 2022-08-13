use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      ha: usize, wa: usize,
      a: [[usize; wa];ha],
      hb: usize, wb: usize,
      b: [[usize; wb]; hb],
    }

    let mut res = "No";
    for h in 0..1 << ha {
        for w in 0..1 << wa {
            let mut v = vec![];
            for i in 0..ha {
                if h >> i & 1 == 1 {
                    let mut temp = vec![];
                    for j in 0..wa {
                        if w >> j & 1 == 1 {
                            temp.push(a[i][j]);
                        }
                    }
                    v.push(temp)
                }
            }
            if v == b {
                res = "Yes";
                break;
            }
        }
    }
    println!("{}", res);
}
