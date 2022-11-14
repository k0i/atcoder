#include <iostream>
#include <string>
#include <vector>
using namespace std;

int main() {
  int n;
  cin >> n;
  vector<string> s(n);

  for (int i = 0; i < n; i++) {
    cin >> s[i];
  }
  bool ans = true;
  for (int i = 0; i < n; i++) {
    for (int j = 0; j < i; j++) {
      if (s[i] == s[j]) {
        ans = false;
      }
    }
  }
  string s1 = "HDCS";
  string s2 = "A23456789TJQK";
  for (int i = 0; i < n; i++) {
    if (s1.find(s[i][0]) == string::npos || s2.find(s[i][1]) == string::npos) {
      ans = false;
    }
  }

  cout << (ans ? "Yes" : "No") << endl;
}
