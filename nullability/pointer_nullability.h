// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_POINTER_NULLABILITY_H_
#define CRUBIT_NULLABILITY_POINTER_NULLABILITY_H_

// This file extends the dataflow framework's Value model to track nullability.
// We attach two boolean properties to each modeled pointer value:
//  - is_null: whether the pointer may actually be null
//    If this is false, dereferencing is safe.
//  - from_nullable: whether the originating expression was considered nullable
//    (e.g. a nullptr literal, or a reference to a Nullable-annotated variable)
//    If this is false, dereferencing may be safe: we don't know the contract.

#include <optional>

#include "nullability/type_nullability.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/ASTDumper.h"
#include "clang/AST/Expr.h"
#include "clang/Analysis/FlowSensitive/DataflowAnalysisContext.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Formula.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Basic/Specifiers.h"

namespace clang::tidy::nullability {

/// Returns the `PointerValue` allocated to `PointerExpr` if available.
/// Otherwise, returns nullptr.
dataflow::PointerValue *getPointerValueFromExpr(
    const Expr *PointerExpr, const dataflow::Environment &Env);

// Returns true if the pointer has all properties necessary for representing
// complete nullness information.
// Otherwise, returns false.
//
// Pointers that are the value of some expression always have null state once
// that expression has been analyzed. Other pointers, like the values of unused
// parameters, may lack this state. This state is only set by
// PointerNullabilityAnalysis, not by the dataflow framework.
bool hasPointerNullState(const dataflow::PointerValue &PointerVal);

/// The properties representing nullness information for a pointer.
///
/// We attach these properties to every PointerValue taken by an expression.
struct PointerNullState {
  /// Did the pointer come from a known-nullable source?
  const dataflow::Formula &FromNullable;
  /// Is the pointer's value null?
  const dataflow::Formula &IsNull;
  // These are independent: sources with unknown nullability can yield nullptr!
};

/// Returns the properties representing the nullness information of a pointer.
PointerNullState getPointerNullState(const dataflow::PointerValue &PointerVal);

/// Creates the nullness properties on `PointerVal` if not already initialised.
///
/// We call this when the framework produces a PointerValue for an expression.
/// This ensures that the variable has usable "from nullable" and "is null"
/// boolean variables, and that they are constrained based on the *original*
/// source of the PointerValue.
///
/// For example:
///    Unknown<int> *x = makeNullable();
///                      ~~~~~~~~~~~~~~ <-- initPointerNullState(Nullable)
///    *x;
///     ~ <-- initPointerNullState(Unknown) - no effect, already initialized
///
/// The constraints are added to the context as a non-flow-sensitive invariant,
/// so the source nullability may not depend on flow-sensitive information.
///
/// (We assume that the framework will not provide the same pointer from
/// different initial sources, so the `Source` nullability is the same
/// regardless of block evaluation order).
void initPointerNullState(
    dataflow::PointerValue &PointerVal, dataflow::DataflowAnalysisContext &Ctx,
    std::optional<PointerTypeNullability> Source = std::nullopt);

/// Variant of initPointerNullState, where the pointer is guaranteed null.
/// (This is flow-insensitive, but PointerTypeNullability can't represent it).
void initNullPointer(dataflow::PointerValue &PointerVal,
                     dataflow::DataflowAnalysisContext &Ctx);

/// Returns true if there is evidence that `PointerVal` may hold a nullptr.
bool isNullable(const dataflow::PointerValue &PointerVal,
                const dataflow::Environment &Env,
                const dataflow::Formula *AdditionalConstraints = nullptr);

/// Returns the strongest provable assertion we can make about `PointerVal`.
/// If PointerVal may not be null, returns Nonnull.
/// If PointerVal may be both null and known-nullability, returns Nullable.
/// Otherwise, returns Unspecified.
clang::NullabilityKind getNullability(
    const dataflow::PointerValue &PointerVal, const dataflow::Environment &Env,
    const dataflow::Formula *AdditionalConstraints = nullptr);

/// Returns the strongest provable assertion we can make about the value of
/// `E` in `Env`.
inline clang::NullabilityKind getNullability(
    const Expr *E, const dataflow::Environment &Env,
    const dataflow::Formula *AdditionalConstraints = nullptr) {
  if (auto *P = getPointerValueFromExpr(E, Env))
    return getNullability(*P, Env, AdditionalConstraints);
  return clang::NullabilityKind::Unspecified;
}

// Work around the lack of Expr.dump() etc with an ostream but no ASTContext.
template <typename T>
void dump(const T &Node, llvm::raw_ostream &OS) {
  clang::ASTDumper(OS, /*ShowColors=*/false).Visit(Node);
}

}  // namespace clang::tidy::nullability

#endif  // CRUBIT_NULLABILITY_POINTER_NULLABILITY_H_
