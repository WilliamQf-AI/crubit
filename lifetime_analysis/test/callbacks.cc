// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Tests for propagating pointees through function calls.
//
// Not every test that contains a function call should go here -- just those
// that test some specific aspect of the logic that propagates pointees through
// function calls.

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "lifetime_analysis/test/lifetime_analysis_test.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

TEST_F(LifetimeAnalysisTest, SimpleCallback) {
  EXPECT_THAT(GetLifetimes(R"(
    void target(int* a, void (*f)(int*)) {
      return f(a);
    }
  )"),
              LifetimesAre({{"target", "a, (a)"}}));
}

TEST_F(LifetimeAnalysisTest, SimpleReturningCallback) {
  EXPECT_THAT(GetLifetimes(R"(
    int* target(int* a, int* (*f)(int*)) {
      return f(a);
    }
  )"),
              LifetimesAre({{"target", "a, (a -> b) -> b"}}));
}

TEST_F(LifetimeAnalysisTest, CallWithCallback) {
  EXPECT_THAT(GetLifetimes(R"(
    int* fun(int* a) {
      return a;
    }

    int* with_cb(int* a, int* (*f)(int*)) {
      return f(a);
    }

    int* target(int* a) {
      return with_cb(a, fun);
    }
  )"),
              LifetimesContain({{"target", "a -> a"}}));
}

TEST_F(LifetimeAnalysisTest, CallWithCallbackReturningStatic) {
  EXPECT_THAT(GetLifetimes(R"(
    int* fun(int*) {
      static int a;
      return &a;
    }

    int* with_cb(int* a, int* (*f)(int*)) {
      return f(a);
    }

    int* target(int* a) {
      return with_cb(a, fun);
    }
  )"),
              LifetimesContain({{"target", "a -> b"}}));
}

}  // namespace
}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
