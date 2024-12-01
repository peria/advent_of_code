#include <algorithm>
#include <cstdint>
#include <iostream>
#include <vector>
using namespace std;

using int64 = int64_t;

int main() {
  vector<int64> entries;
  for (int64 entry; cin >> entry;)
    entries.push_back(entry);
  sort(entries.begin(), entries.end());

  static constexpr int64 kTarget = 2020;
  for (int64 i = 0, j = entries.size() - 1; i < j;) {
    while (i < j && entries[i] + entries[j] < kTarget)
      ++i;
    while (i < j && entries[i] + entries[j] > kTarget)
      --j;
    if (i < j && entries[i] + entries[j] == kTarget) {
      cout << entries[i] * entries[j] << " " << entries[i] << " " <<  entries[j] << "\n";
      ++i;
    }
  }

  return 0;
}
