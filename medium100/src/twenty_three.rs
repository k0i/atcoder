use std::rc::Rc;

use proconio::input;
pub fn main() {
    input! {
       n:usize,
       m:usize,
       sc:[(u32,u32);m]
    }
    let a = Rc::new(sc);
    'outer: for i in 0..=999 {
        let i_vec = i
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        if i_vec.len() != n {
            continue;
        }
            let mut ok = true;
        for j in Rc::clone(&a).iter() {
            if i_vec[(j.0 - 1) as usize] != j.1 {
                ok = false;
                continue;
            }
        }
        if ok{
            println!("{}", i);
            break 'outer;
        }
        if i ==999{
            println!("-1");
            break 'outer;
        }
    }
}

// #include<bits/stdc++.h>
// #define rep(i,a,b) for(int i=a;i<b;i++)
// #define rrep(i,a,b) for(int i=a;i>=b;i--)
// #define fore(i,a) for(auto &i:a)
// #define all(x) (x).begin(),(x).end()
// //#pragma GCC optimize ("-O3")
// using namespace std; void _main(); int main() { cin.tie(0); ios::sync_with_stdio(false); _main(); }
// typedef long long ll; const int inf = INT_MAX / 2; const ll infl = 1LL << 60;
// template<class T>bool chmax(T& a, const T& b) { if (a < b) { a = b; return 1; } return 0; }
// template<class T>bool chmin(T& a, const T& b) { if (b < a) { a = b; return 1; } return 0; }
// //---------------------------------------------------------------------------------------------------
// /*---------------------------------------------------------------------------------------------------
// 　　　　　　　　　　　 ∧＿∧
// 　　　　　 ∧＿∧ 　（´<_｀ ）　 Welcome to My Coding Space!
// 　　　　 （ ´_ゝ`）　/　 ⌒i     @hamayanhamayan0
// 　　　　／　　　＼　 　  |　|
// 　　　 /　　 /￣￣￣￣/　　|
// 　 ＿_(__ﾆつ/　    ＿/ .| .|＿＿＿＿
// 　 　　　＼/＿＿＿＿/　（u　⊃
// ---------------------------------------------------------------------------------------------------*/
 
 
 
 
 
 
 
 
 
 
 
 
 
 
// int N, M, S[5], C[5];
// //---------------------------------------------------------------------------------------------------
// void _main() {
// 	cin >> N >> M;
// 	rep(i, 0, M) cin >> S[i] >> C[i], S[i]--;
 
// 	rep(ans, 0, 1000) {
// 		string s = to_string(ans);
// 		if (s.length() != N) continue;
 
// 		bool ok = true;
// 		rep(i, 0, M) rep(j, 0, N) if (j == S[i] && (s[j] - '0') != C[i]) ok = false;
// 		if (ok) {
// 			cout << ans << endl;
// 			return;
// 		}
// 	}
 
// 	cout << -1 << endl;
// }