use proconio::input;

const INF: i64 = 1_000_000;

fn get(row: &[i8], i: usize, flip: bool) -> i8 {
    if flip {
        1 - row[i]
    } else {
        row[i]
    }
}

fn okay(above: &[i8], mid: &[i8], below: &[i8], flip: (bool, bool, bool)) -> bool {
    let w = mid.len();
    for i in 0..w {
        if i > 0 && mid[i] == mid[i - 1] {
            continue;
        }
        if i < w - 1 && mid[i] == mid[i + 1] {
            continue;
        }
        let x = get(&mid, i, flip.1);
        if get(&above, i, flip.0) == x || get(&below, i, flip.2) == x {
            continue;
        }
        return false;
    }
    true
}

fn main() {
    input! { h: usize, w: usize, a: [[i8; w]; h], }
    let invalid = vec![2; w];
    let mut num = [[0; 2]; 2];
    num[0][1] = 1;
    num[1][1] = 1;
    for i in 0..h {
        let mut next = [[INF; 2]; 2];
        let above = if i == 0 { &invalid } else { &a[i - 1] };
        let below = if i == h - 1 { &invalid } else { &a[i + 1] };
        for x in 0..2 {
            for y in 0..2 {
                if num[x][y] == INF {
                    continue;
                }
                for z in 0..2 {
                    if okay(above, &a[i], below, (x == 1, y == 1, z == 1)) {
                        next[y][z] = next[y][z].min(num[x][y] + if z == 0 { 0 } else { 1 });
                    }
                }
            }
        }
        num = next;
    }
    let ans = num[0][0].min(num[1][0]);
    println!("{}", if ans == INF { -1 } else { ans });
}
