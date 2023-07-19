// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/infer_tu.h"

#include <optional>
#include <vector>

#include "nullability/inference/inference.proto.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Index/USRGeneration.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/SmallString.h"
#include "llvm/ADT/StringRef.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
using ast_matchers::hasName;

MATCHER_P2(inferredSlot, I, Nullability, "") {
  return arg.slot() == I && arg.nullability() == Nullability;
}
MATCHER_P2(inferenceMatcher, USR, SlotsMatcher, "") {
  if (arg.symbol().usr() != USR) return false;
  return testing::ExplainMatchResult(SlotsMatcher, arg.slot_inference(),
                                     result_listener);
}

AST_MATCHER(Decl, isCanonical) { return Node.isCanonicalDecl(); }

class InferTUTest : public ::testing::Test {
  std::optional<TestAST> AST;

 protected:
  void build(llvm::StringRef Code) {
    TestInputs Inputs = Code;
    Inputs.ExtraFiles["nullability.h"] = R"cc(
      template <typename T>
      using Nullable [[clang::annotate("Nullable")]] = T;
      template <typename T>
      using Nonnull [[clang::annotate("Nonnull")]] = T;
    )cc";
    Inputs.ExtraArgs.push_back("-include");
    Inputs.ExtraArgs.push_back("nullability.h");
    AST.emplace(Inputs);
  }

  auto infer() { return inferTU(AST->context()); }

  // Returns a matcher for an Inference.
  // The DeclMatcher should uniquely identify the symbol being described.
  // (We use this to compute the USR we expect to find in the inference proto).
  // Slots should describe the slots that were inferred.
  template <typename MatcherT>
  testing::Matcher<const Inference &> inference(
      MatcherT DeclMatcher,
      std::vector<testing::Matcher<const Inference::SlotInference &>> Slots) {
    llvm::SmallString<128> USR;
    auto Matches = ast_matchers::match(
        ast_matchers::namedDecl(isCanonical(), DeclMatcher).bind("decl"),
        AST->context());
    EXPECT_EQ(Matches.size(), 1);
    if (auto *D = ast_matchers::selectFirst<Decl>("decl", Matches))
      EXPECT_FALSE(index::generateUSRForDecl(D, USR));
    return inferenceMatcher(USR, testing::ElementsAreArray(Slots));
  }
};

TEST_F(InferTUTest, UncheckedDeref) {
  build(R"cc(
    void target(int *p, bool cond) {
      if (cond) *p;
    }

    void guarded(int *p) {
      if (p) *p;
    }
  )cc");

  EXPECT_THAT(infer(),
              ElementsAre(inference(hasName("target"),
                                    {inferredSlot(1, Inference::NONNULL)})));
}

TEST_F(InferTUTest, Annotations) {
  build(R"cc(
    Nonnull<int *> target(int *a, int *b);
    Nonnull<int *> target(int *a, Nullable<int *> p) { *p; }
  )cc");

  EXPECT_THAT(infer(),
              ElementsAre(inference(hasName("target"),
                                    {
                                        inferredSlot(0, Inference::NONNULL),
                                        inferredSlot(2, Inference::NULLABLE),
                                    })));
}

TEST_F(InferTUTest, AnnotationsConflict) {
  build(R"cc(
    Nonnull<int *> target();
    Nullable<int *> target();
  )cc");

  EXPECT_THAT(infer(),
              ElementsAre(inference(hasName("target"),
                                    {inferredSlot(0, Inference::UNKNOWN)})));
}

}  // namespace
}  // namespace clang::tidy::nullability
