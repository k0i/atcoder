use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
        T: Chars,
    }
    let mut start = !0;
    if S.len() < T.len() {
        println!("UNRESTORABLE");
        return;
    }
    for i in 0..=S.len() - T.len() {
        let mut flg = true;
        for j in 0..T.len() {
            if S[i + j] != '?' && S[i + j] != T[j] {
                flg = false;
                break;
            }
        }
        if flg {
            start = i;
        }
    }
    if start == !0 {
        println!("UNRESTORABLE");
        return;
    }
    let mut ans = S
        .iter()
        .map(|&x| if x == '?' { 'a' } else { x })
        .collect::<Vec<char>>();
    for (ix, c) in T.iter().enumerate() {
        ans[start + ix] = *c;
    }
    println!("{}", ans.into_iter().collect::<String>());
}
