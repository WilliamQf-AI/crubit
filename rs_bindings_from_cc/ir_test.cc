// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir.h"

#include <string>

#include "testing/base/public/gunit.h"
#include "third_party/json/src/json.hpp"

namespace rs_bindings_from_cc {

namespace {

TEST(IrTest, TypeToJson) {
  nlohmann::json expected = nlohmann::json::parse(R"j({
      "rs_name": "CompoundRs",
      "cc_name": "CompoundCc",
      "type_params": [
          { "rs_name": "i32", "cc_name": "int", "type_params": []}
      ]
  })j");
  auto type = Type{.rs_name = "CompoundRs",
                   .cc_name = "CompoundCc",
                   .type_params = {Type{"i32", "int"}}};
  EXPECT_EQ(type.ToJson(), expected);
}

TEST(IrTest, IR) {
  nlohmann::json expected = nlohmann::json::parse(
      R"j({
            "used_headers": [{ "name": "foo/bar.h" }],
            "functions": [{
              "identifier": { "identifier": "hello_world" },
              "mangled_name": "#$mangled_name$#",
              "return_type": { "rs_name": "i32", "cc_name": "int", "type_params": [] },
              "params": [
                {
                  "identifier": {"identifier": "arg" },
                  "type": { "rs_name": "i32", "cc_name": "int", "type_params": [] }
                }
              ],
              "is_inline": false
            }],
            "records": [
              {
                "identifier": {"identifier": "SomeStruct" },
                "fields": [
                  {
                    "identifier": {"identifier": "first_field" },
                    "type": {"rs_name": "i32", "cc_name": "int", "type_params": [] }
                  },
                  {
                    "identifier": {"identifier": "second_field" },
                    "type": {"rs_name": "i32", "cc_name": "int", "type_params": [] }
                  }
                ]
              }
            ]
      })j");
  EXPECT_EQ(
      IR({HeaderName(std::string("foo/bar.h"))},
         {Func{
             .identifier = Identifier(std::string("hello_world")),
             .mangled_name = std::string("#$mangled_name$#"),
             .return_type = Type{std::string("i32"), std::string("int")},
             .params = {FuncParam{Type{std::string("i32"), std::string("int")},
                                  Identifier(std::string("arg"))}},
             .is_inline = false}},
         {Record(Identifier(std::string("SomeStruct")),
                 {
                     Field{.identifier = Identifier("first_field"),
                           .type = Type{"i32", "int"}},
                     Field{.identifier = Identifier("second_field"),
                           .type = Type{"i32", "int"}},
                 })})
          .ToJson(),
      expected);
}

}  // namespace
}  // namespace rs_bindings_from_cc
