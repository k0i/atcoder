use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        h: usize, w: usize,
        mut map: [Chars; h]
    }
    let mut ret = -1;
    for i in 0..h {
        for j in 0..w {
            if map[i][j] == '.' {
                map[i][j] = 's';
                ret = ret.max(dfs(&mut map, i, j, 1));
                map[i][j] = '.';
            }
        }
    }
    println!("{}", ret);
}

fn dfs(map: &mut Vec<Vec<char>>, y: usize, x: usize, l: i32) -> i32 {
    let h = map.len();
    let w = map[0].len();
    let neighbors = [
        (y.wrapping_sub(1), x),
        (y.wrapping_add(1), x),
        (y, x.wrapping_sub(1)),
        (y, x.wrapping_add(1)),
    ];
    let mut ret = -1;
    for &(ny, nx) in &neighbors {
        if ny >= h || nx >= w {
            continue;
        }
        match map[ny][nx] {
            's' => {
                if l > 2 {
                    ret = ret.max(l);
                }
            }
            '.' => {
                map[ny][nx] = 'x';
                ret = ret.max(dfs(map, ny, nx, l + 1));
                map[ny][nx] = '.';
            }
            _ => (),
        }
    }
    ret
}
