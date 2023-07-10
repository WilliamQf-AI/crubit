// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_METHOD_PARAMS_METHOD_PARAMS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_METHOD_PARAMS_METHOD_PARAMS_H_

#pragma clang lifetime_elision

template <typename T>
class MyTemplate final {
 public:
  static MyTemplate Create(T value) {
    MyTemplate result;
    result.value_ = value;
    return result;
  }

  T AddOneOtherItem(const MyTemplate<T>& other) const {
    return value_ + other.value_;
  }

  T AddThreeOtherItems(const MyTemplate<T>& a, const MyTemplate<T>& b,
                       const MyTemplate<T>& c) const {
    return value_ + a.value_ + b.value_ + c.value_;
  }

 private:
  T value_;
};

using MyTypeAlias = MyTemplate<int>;

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_METHOD_PARAMS_METHOD_PARAMS_H_
