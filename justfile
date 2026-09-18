# -------- BC-RUST JUSTFILE HELP --------
# tl;dr - use "just" command to run default "recipe" for validation builds and tests (equivalent to "just validate-all")
# use "just --list" to see all available recipes

# The logic to generate all test combos using simple lists may not be obvious.
# It relies on recursive dependencies on smaller recipes,
# which eventually call a single recipe that builds, tests, or benchmarks a single cargo package using "-p".
# This design requires the unstable lists feature to invoke recipe dependencies more than once,
# using the list variables defined below as dependency arguments.
# The docs say: "Dependencies may be invoked once per element of a list with *(recipe *argument)"
# https://github.com/casey/just/tree/master#lists

set unstable
set lists

# every crate should go here
all-crates := [
  "bouncycastle-base64",
  "bouncycastle-core",
  "bouncycastle-core-test-framework",
  "bouncycastle-factory",
  "bouncycastle-hex",
  "bouncycastle-hkdf",
  "bouncycastle-hmac",
  "bouncycastle-mldsa",
  "bouncycastle-mldsa-lowmemory",
  "bouncycastle-mlkem",
  "bouncycastle-mlkem-lowmemory",
  "bouncycastle-rng",
  "bouncycastle-sha2",
  "bouncycastle-sha3",
  "bouncycastle-utils",
  "cli",
  "bouncycastle",
]

# crates that should be able to build with --no-default-features (to disable "alloc" feature) go here
# this is to catch regressions in known-working crates, and quickly test other crates as they are worked on
# for now, this list should be added to incrementally as crates and their dependencies
# are updated to build with --no-default-features
no-std-crates := [
  "bouncycastle-core",
  "bouncycastle-core-test-framework",
]


# do a thorough validation of all crates, individually and as a workspace
[default]
validate-all: test-all-variants build-release-all-variants

# -------- TESTS SECTION --------

# test all crates, individually and as a workspace, with std and with no_std
test-all-variants: test-workspace test-all-crates-no-std test-all-crates

test-workspace:
  cargo test --workspace

test-all-crates: *(test-crate *all-crates)

test-all-crates-no-std: *(test-crate-no-std *no-std-crates)

test-crate crate:
  cargo test -p {{crate}}

test-crate-no-std crate:
  cargo test -p {{crate}} --no-default-features

# -------- BENCHMARKS SECTION --------
  
# benchmark all crates, individually and as a workspace
bench-all-variants: bench-workspace bench-all-crates-no-std bench-all-crates

bench-workspace:
  cargo bench --workspace

bench-all-crates: *(bench-crate *all-crates)

bench-all-crates-no-std: *(bench-crate-no-std *no-std-crates)

bench-crate crate:
  cargo bench -p {{crate}}

bench-crate-no-std crate:
  cargo bench -p {{crate}} --no-default-features

# -------- RELEASE BUILDS SECTION --------

# build all crates in release, individually and as a workspace
build-release-all-variants: build-release-workspace build-release-all-crates-no-std build-release-all-crates

build-release-workspace:
  cargo build --release --workspace

build-release-all-crates: *(build-release-crate *all-crates)

build-release-all-crates-no-std: *(build-release-crate-no-std *no-std-crates)

build-release-crate crate:
  cargo build --release -p {{crate}}

build-release-crate-no-std crate:
  cargo build --release -p {{crate}} --no-default-features