#pragma once

#include "rust/cxx.h"
#include <memory>

namespace org {
namespace sorting {
class Sorter {
public:
  Sorter();

  void sort(rust::Vec<rust::i32> &numbers) const;
};

std::unique_ptr<Sorter> new_sorter();
} // namespace sorting
} // namespace org
