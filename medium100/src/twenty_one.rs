use proconio::input;
pub fn main() {
    input! { n:usize,
    a:[u64;n] }
    let mut m = a.clone().into_iter().max().unwrap() as usize;
    let mut ai = vec![0; m];

    for i in 0..n {
        ai[(a[i] - 1) as usize] += 1;
    }
    let mut ans = 0;
    for i in 0..n {
        if ai[(a[i] - 1) as usize] == 0 {
            continue;
        }
let mut comb =ai[(a[i] - 1) as usize];
        if comb <=2{
           println!("{}",0);
           continue;
        }

        println!(
            "{}",
            comb* (comb - 1)/2 - (comb - 1) * (comb - 2) / 2
        );
    }
}

// int N, A[201010];
// ll cnt[201010];
// //---------------------------------------------------------------------------------------------------
// void _main() {
//     cin >> N;
//     rep(i, 0, N) cin >> A[i];

//     ll all = 0;
//     rep(i, 0, N) cnt[A[i]]++;
//     rep(i, 1, N + 1) all += cnt[i] * (cnt[i] - 1) / 2;

//     rep(i, 0, N) {
//         ll ans = all;
//         ans -= cnt[A[i]] * (cnt[A[i]] - 1) / 2;
//         ans += (cnt[A[i]] - 1) * (cnt[A[i]] - 2) / 2;
//         printf("%lld\n", ans);
//     }
// }