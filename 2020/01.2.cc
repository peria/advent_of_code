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
  for (int64 i = 0; i < entries.size(); ++i) {
    if (entries[i] * 3 > kTarget)
      break;
    for (int64 j = i + 1; j < entries.size(); ++j) {
      int64 var = kTarget - entries[i] - entries[j];
      if (var < entries[j])
        break;
      auto iter = lower_bound(entries.begin() + j, entries.end(), var);
      if (iter == entries.end() || *iter != var)
        continue;
      cout << entries[i] * entries[j] * var << "\n";
    }
  }

  return 0;
}
