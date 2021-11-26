// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_LIFETIME_ANNOTATIONS_LIFETIME_ANNOTATIONS_H_
#define CRUBIT_LIFETIME_ANNOTATIONS_LIFETIME_ANNOTATIONS_H_

#include <memory>

#include "lifetime_annotations/function_lifetimes.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Decl.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Frontend/CompilerInstance.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/Support/Error.h"

namespace devtools_rust {

// Context that is required to obtain lifetime annotations for a function.
struct LifetimeAnnotationContext {
  // Files in which the `lifetime_elision` pragma was specified.
  llvm::DenseSet<clang::FileID> lifetime_elision_files;
};

// Returns the lifetimes annotated on `func`.
// If the file containing the function definition specifies the
// `lifetime_elision` pragma, lifetime elision rules are used to determine
// any unannotated lifetimes.
// Returns an error if the function contains unannotated lifetimes that could
// not be determined through lifetime elision, either because the
// `lifetime_elision`pragma was not specified or because the lifetime elision
// rules were not applicable.
// TODO(mboehme): This function has only been partially implemented.
llvm::Expected<FunctionLifetimes> GetLifetimeAnnotations(
    const clang::FunctionDecl* func, const LifetimeAnnotationContext& context);

// Adds handlers to `compiler` to populate `context`.
// To be able to use GetLifetimeAnnotations(), call this function to add the
// necessary handlers before compiling any code.
void AddLifetimeAnnotationHandlers(
    clang::CompilerInstance& compiler,
    std::shared_ptr<LifetimeAnnotationContext> context);

}  // namespace devtools_rust

#endif  // CRUBIT_LIFETIME_ANNOTATIONS_LIFETIME_ANNOTATIONS_H_
