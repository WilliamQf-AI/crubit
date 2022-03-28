# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A wrapper rule around the bindings binary.

We need this wrapper so we can specify a transition and thus avoid the circular dependency
that happens when we try to build the rs_bindings_from_cc:
rust_bindings_from_cc_aspect -> rs_bindings_from_cc -> rust_library
              ^_____________________________________________|

Disclaimer: This project is experimental, under heavy development, and should
not be used yet.
"""

load(
    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_transition.bzl",
    "rust_bindings_from_cc_transition",
)

def _rust_bindings_from_cc_binary_impl(ctx):
    output = ctx.actions.declare_file(ctx.label.name)
    ctx.actions.symlink(output = output, target_file = ctx.executable.binary, is_executable = True)
    return [
        DefaultInfo(
            executable = output,
            runfiles = ctx.runfiles(
                files = [ctx.file.binary, ctx.executable._rustfmt] + ctx.files._rustfmt_cfg,
            ),
        ),
    ]

rust_bindings_from_cc_binary = rule(
    attrs = {
        "binary": attr.label(
            executable = True,
            allow_single_file = True,
            cfg = rust_bindings_from_cc_transition,
            doc = ("Executable StandaloneClangTool binary that generates Rust bindings from " +
                   "C++ code."),
        ),
        "_allowlist_function_transition": attr.label(
            default = "//tools/allowlists/function_transition_allowlist",
        ),
        "_rustfmt": attr.label(
            default = "//third_party/unsupported_toolchains/rust/toolchains/nightly:bin/rustfmt",
            executable = True,
            allow_single_file = True,
            cfg = "exec",
        ),
        "_rustfmt_cfg": attr.label(
            default = "@rustfmt//:rustfmt.toml",
            allow_single_file = True,
        ),
    },
    executable = True,
    implementation = _rust_bindings_from_cc_binary_impl,
)
