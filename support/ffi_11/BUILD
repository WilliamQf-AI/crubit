load("@rules_rust//rust:defs.bzl", "rust_library")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "ffi_11",
    srcs = glob(["src/*.rs"]),
    crate_features = ["crubit"],
    visibility = [
        "//:__subpackages__",
    ],
)
