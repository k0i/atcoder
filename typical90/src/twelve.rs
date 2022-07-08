use proconio::{input, marker::Usize1};
pub fn main() {
    input! { (h, w): (usize, usize), q: usize }
    let mut p = vec![-1i64; h * w];
    let mut c = vec![0; h * w];
    for _ in 0..q {
        input! { t: usize }
        if t == 1 {
            input! { (r1,c1):(Usize1,Usize1) }
            let idx1 = w * r1 + c1;
            c[idx1] = 1;
            let di: Vec<i64> = vec![-1, 0, 1, 0];
            let dj: Vec<i64> = vec![0, 1, 0, -1];
            for k in 0..4 {
                let ni = r1 as i64 + di[k];
                let nj = c1 as i64 + dj[k];
                if ni >= 0 && ni < h as i64 && nj >= 0 && nj < w as i64 {
                    let idx2 = w * ni as usize + nj as usize;
                    if c[idx2] == 1 {
                        unite(idx1, idx2, &mut p)
                    }
                }
            }
        } else {
            input! {
                (r1, c1): (Usize1, Usize1),
                (r2, c2): (Usize1, Usize1)
            }
            let idx1 = w * r1 + c1;
            let idx2 = w * r2 + c2;
            let ans;
            if same(idx1, idx2, &mut p) && c[idx1] == 1 && c[idx2] == 1 {
                ans = "Yes";
            } else {
                ans = "No"
            }
            println!("{}", ans);
        }
    }
}

fn root(x: usize, p: &mut Vec<i64>) -> usize {
    if p[x] < 0 {
        return x;
    }
    p[x] = root(p[x] as usize, p) as i64;
    p[x] as usize
}
fn same(x: usize, y: usize, mut p: &mut Vec<i64>) -> bool {
    root(x, &mut p) == root(y, &mut p)
}
fn unite(x: usize, y: usize, mut p: &mut Vec<i64>) {
    let mut rx = root(x, &mut p);
    let mut ry = root(y, &mut p);
    if rx == ry {
        return;
    }
    if rx > ry {
        std::mem::swap(&mut rx, &mut ry)
    }
    p[ry] += p[rx];
    p[rx] = ry as i64;
}
