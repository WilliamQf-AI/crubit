// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_GENERATE_BINDINGS_AND_METADATA_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_GENERATE_BINDINGS_AND_METADATA_H_

#include <string>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/status/statusor.h"
#include "rs_bindings_from_cc/cmdline.h"
#include "rs_bindings_from_cc/collect_namespaces.h"
#include "rs_bindings_from_cc/ir.h"

namespace crubit {
// Contains generated bindings and all related metadata, such as the IR.
struct BindingsAndMetadata {
  // Intermediate representation of the Clang AST from which we generated
  // bindings.
  IR ir;
  // Generated Rust source code.
  std::string rs_api;
  // Generated C++ source code.
  std::string rs_api_impl;
  // A hierarchy tree for all C++ namespaces used in the target.
  NamespacesHierarchy namespaces;
  // C++ class templates explicitly instantiated in this TU and their Rust
  // struct name.
  absl::flat_hash_map<Identifier, Identifier> instantiations;
  // A JSON error report, if requested.
  std::string error_report;
};

// Returns `BindingsAndMetadata` as requested by the user on the command line.
absl::StatusOr<BindingsAndMetadata> GenerateBindingsAndMetadata(
    Cmdline& cmdline, std::vector<std::string> clang_args,
    absl::flat_hash_map<HeaderName, std::string>
        virtual_headers_contents_for_testing = {});

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_GENERATE_BINDINGS_AND_METADATA_H_
