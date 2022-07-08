use proconio::input;
pub fn main() {
    input! {
       n:usize,
       k:usize,
       p:[u64;n]
    }
    let cumsum = CumSum::new(p.clone());
    let mut res = 0.0;

    for i in 0..=n - k {
        let temp = cumsum.section_sum(i, i + k);
        if res < temp {
            res = temp;
        }
    }
    println!("{:?}", res);
}

#[derive(Debug, Clone)]
pub struct CumSum {
    pub section: Vec<u64>,
    pub cumsum: Vec<u64>,
}
impl CumSum {
    pub fn new(vec: Vec<u64>) -> Self {
        let section = vec.clone();
        //  vec.insert(0,0);
        Self {
            section: section,
            cumsum: vec
                .iter()
                .scan(0, |acc, x| {
                    *acc += x;
                    Some(*acc)
                })
                .collect(),
        }
    }
    pub fn section_sum(&self, left: usize, right: usize) -> f64 {
        let mut sum = 0.0;
        // [left,right)
        for i in left..right {
            let mut prob = 0.0;
            for j in 1..=self.section[i] {
                prob += 1.0 / (self.section[i] as f64) * j as f64;
            }
            sum += prob;
        }
        sum
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
 
 
 
 
 
 
 
 
 
 
 
 
 
 
// int N, K, p[201010];
// double e[201010];
// //---------------------------------------------------------------------------------------------------
// void _main() {
// 	cin >> N >> K;
// 	rep(i, 0, N) cin >> p[i];
 
// 	rep(i, 0, N) e[i] = 1.0 * (1 + p[i]) / 2;
 
// 	double tot = 0;
// 	rep(i, 0, K) tot += e[i];
 
// 	double ans = tot;
// 	rep(i, K, N) {
// 		tot = tot + e[i] - e[i - K];
// 		chmax(ans, tot);
// 	}
// 	printf("%.10f\n", ans);
// }