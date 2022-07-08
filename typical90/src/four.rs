use proconio::input;
pub fn main() {
    input! {
    h:usize,
    w:usize,
    a:[[u64;w];h]
    }
    let mut vert = vec![0; h];
    let mut hori = vec![0; w];

    for i in 0..h {
        for j in 0..w {
            vert[i] += a[i][j];
            hori[j] += a[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            if j == w - 1 {
                println!("{}", vert[i] + hori[j] - a[i][j]);
            } else {
                print!("{} ", vert[i] + hori[j] - a[i][j]);
            }
        }
    }
}
