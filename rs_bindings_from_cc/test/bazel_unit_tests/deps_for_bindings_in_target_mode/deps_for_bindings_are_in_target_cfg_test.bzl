# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A test that the dependencies needed for our generated bindings files are built in target cfg."""

load("@bazel_skylib//lib:unittest.bzl", "analysistest", "asserts")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_make_analysis_test")
load(
    "//rs_bindings_from_cc/test/bazel_unit_tests:defs.bzl",
    "ActionsInfo",
    "attach_aspect",
)

def _filter_by_substring(arr, substring):
    return [x for x in arr if substring in x]

def _negative_filter_by_substring(arr, substring):
    return [x for x in arr if substring not in x]

def _deps_for_bindings_in_target_cfg_impl(ctx):
    env = analysistest.begin(ctx)
    tut = analysistest.target_under_test(env)

    # The compiled binding implicitly depends on a version of the memoffset crate.
    # Here we check that the -Ldependency and -Lextern arguments for this crate point to a
    # non exec path, aka target configuration.

    action = [a for a in tut[ActionsInfo].actions if a.mnemonic == "Rustc"][0]
    memoffset_args = _filter_by_substring(action.argv, "third_party/rust/memoffset")
    exec_cfg_args = _filter_by_substring(memoffset_args, "-exec-")
    target_cfg_args = _negative_filter_by_substring(memoffset_args, "-exec-")

    asserts.equals(env, 0, len(exec_cfg_args))
    asserts.true(env, len(target_cfg_args) > 0)

    return analysistest.end(env)

deps_for_bindings_in_target_cfg_test = crubit_make_analysis_test(
    _deps_for_bindings_in_target_cfg_impl,
)

def _test_deps_for_bindings_in_target_cfg():
    native.cc_library(name = "lib", hdrs = ["lib.h"])
    attach_aspect(name = "lib_with_aspect", dep = ":lib")
    deps_for_bindings_in_target_cfg_test(
        name = "deps_for_bindings_in_target_cfg_test",
        target_under_test = ":lib_with_aspect",
    )

def deps_for_bindings_are_in_target_cfg_test(name):
    """Sets up deps_for_bindings_are_in_target_cfg_test test suite.

    Args:
      name: name of the test suite"""
    _test_deps_for_bindings_in_target_cfg()

    native.test_suite(
        name = name,
        tests = [
            ":deps_for_bindings_in_target_cfg_test",
        ],
    )
