use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let mut b = (0..=n).collect::<Vec<usize>>();
    for i in 0..m {
        b.swap(a[i], a[i] + 1);
    }
    let mut pos = vec![0; n + 1];
    for j in 1..=n {
        pos[b[j]] = j;
    }

    let mut b = (0..=n).collect::<Vec<usize>>();
    for i in 0..m {
        if b[a[i]] == 1 {
            println!("{}", pos[b[a[i] + 1]]);
        } else if b[a[i] + 1] == 1 {
            println!("{}", pos[b[a[i]]]);
        } else {
            println!("{}", pos[1]);
        }
        b.swap(a[i], a[i] + 1);
    }
}
