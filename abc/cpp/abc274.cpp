#include <iomanip>
#include <iostream>
using namespace std;
int main() {
  int a, b;
  cin >> a >> b;
  double c = double(b) / double(a);
  cout << fixed << std::setprecision(3) << c;
}
