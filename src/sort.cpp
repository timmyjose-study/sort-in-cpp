#include "sort-in-cpp/include/sort.h"
#include "sort-in-cpp/src/main.rs.h"
#include <algorithm>

namespace org {
namespace sorting {
std::unique_ptr<Sorter> new_sorter() { return std::make_unique<Sorter>(); }

Sorter::Sorter() {}

void Sorter::sort(rust::Vec<rust::i32> &numbers) const {
  std::sort(numbers.begin(), numbers.end());
}
} // namespace sorting
} // namespace org
