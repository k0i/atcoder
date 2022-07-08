use proconio::input;
pub fn main() {
    input! {
        n: usize,
        a:[u64;n]
    }
    let mut res = 0;
    let mut asc =a[0] <a[1];

    for mut i in 0..n {
      if i + 1 < n && a[i] == a[i + 1] {
         continue;
        }
        if i + 1 < n && a[i] < a[i + 1] {
        if asc{continue;}
        asc=true
        } else if i + 1 < n && a[i] > a[i + 1] {
       if !asc{continue;}
       asc=false
        }
        res += 1;
    }
    println!("{}", res);
}


// #include <iostream>
// #include <vector>
// using namespace std;

// int main() {
//     int N; cin >> N;
//     vector<int> A(N);
//     for (int i = 0; i < N; ++i) cin >> A[i];

//     int res = 0;
//     for (int i = 0; i < N; ++i) {
//         // same を抜ける
//         while (i+1 < N && A[i] == A[i+1]) ++i;

//         // up
//         if (i+1 < N && A[i] < A[i+1]) {
//             while (i+1 < N && A[i] <= A[i+1]) ++i;
//         }
//         // down
//         else if (i+1 < N && A[i] > A[i+1]) {
//             while (i+1 < N && A[i] >= A[i+1]) ++i;
//         }
//         ++res;
//     }
//     cout << res << endl;
// }