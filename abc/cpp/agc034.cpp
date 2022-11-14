#include <iostream>

using namespace std;

int main() {
  int N, A, B, C, D;
  string S;
  cin >> N >> A >> B >> C >> D >> S;

  S = "#" + S + "#";

  auto can_reach = [&](int start, int end) {
    for (int i = start; i + 1 <= end; i++) {
      if (S[i] == '#' && S[i + 1] == '#')
        return false;
    }
    return true;
  };

  if (!can_reach(A, C) || !can_reach(B, D)) {
    cout << "No" << endl;
    return 0;
  }

  if (C > D) {
    bool snuke_can_over = false;
    for (int i = B; i <= D; i++) {
      if (S[i - 1] == '.' && S[i] == '.' && S[i + 1] == '.') {
        snuke_can_over = true;
      }
    }
    if (!snuke_can_over) {
      cout << "No" << endl;
      return 0;
    }
  }

  cout << "Yes" << endl;
  return 0;
}
