// #include<bits/stdc++.h>
// #define rep(i,a,b) for(int i=a;i<b;i++)
// #define rrep(i,a,b) for(int i=a;i>=b;i--)
// #define fore(i,a) for(auto &i:a)
// #define all(x) (x).begin(),(x).end()
// #pragma GCC optimize ("-O3")
// using namespace std; void _main(); int main() { cin.tie(0); ios::sync_with_stdio(false); _main(); }
// typedef long long ll; const int inf = INT_MAX / 2; const ll infl = 1LL << 60;
// template<class T>bool chmax(T &a, const T &b) { if (a<b) { a = b; return 1; } return 0; }
// template<class T>bool chmin(T &a, const T &b) { if (b<a) { a = b; return 1; } return 0; }
// //---------------------------------------------------------------------------------------------------
// /*---------------------------------------------------------------------------------------------------
// 　　　　　　　　　　　 ∧＿∧  
// 　　　　　 ∧＿∧ 　（´<_｀ ）　 Welcome to My Coding Space!
// 　　　　 （ ´_ゝ`）　/　 ⌒i     
// 　　　　／　　　＼　 　  |　|     
// 　　　 /　　 /￣￣￣￣/　　|  
// 　 ＿_(__ﾆつ/　    ＿/ .| .|＿＿＿＿  
// 　 　　　＼/＿＿＿＿/　（u　⊃  
// ---------------------------------------------------------------------------------------------------*/
 
 
 
 
// int H, W;
// string A[101];
// //---------------------------------------------------------------------------------------------------
// int check() {
//     rep(y, 0, H) {
//         int ok = 1;
//         rep(x, 0, W) if (A[y][x] == '#') ok = 0;
//         if (ok) {
//             rep(yy, y + 1, H) swap(A[yy - 1], A[yy]);
//             H--;
//             return 1;
//         }
//     }
 
//     rep(x, 0, W) {
//         int ok = 1;
//         rep(y, 0, H) if (A[y][x] == '#') ok = 0;
//         if (ok) {
//             rep(yy, 0, H) A[yy] = A[yy].substr(0, x) + A[yy].substr(x + 1);
//             W--;
//             return 1;
//         }
//     }
 
//     return 0;
// }
// //---------------------------------------------------------------------------------------------------
// void _main() {
//     cin >> H >> W;
//     rep(y, 0, H) cin >> A[y];
//     while (check());
//     rep(y, 0, H) cout << A[y] << endl;
// }