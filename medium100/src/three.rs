// #include <bits/stdc++.h>
// using namespace std;
// template<class T> inline bool chmax(T& a, T b) { if (a < b) { a = b; return 1; } return 0; }

// int main() {
//     string S;
//     cin >> S;
//     int N = S.size() + 1;
//     vector<int> A(N, 0);
//     for (int i = 0; i < N-1; ++i) {
//         if (S[i] == '<') chmax(A[i+1], A[i]+1);
//     }
//     for (int i = N-2; i >= 0; --i) {
//         if (S[i] == '>') chmax(A[i], A[i+1]+1);
//     }
//     long long res = 0;
//     for (auto v : A) res += v;
//     cout << res << endl;
// }
