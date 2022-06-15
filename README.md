# The Cipher ParaTime

[![CI lint status][github-ci-lint-badge]][github-ci-lint-link]
[![CI audit status][github-ci-audit-badge]][github-ci-audit-link]

<!-- markdownlint-disable line-length -->
[github-ci-lint-badge]: https://github.com/oasisprotocol/cipher-paratime/workflows/ci-lint/badge.svg
[github-ci-lint-link]: https://github.com/oasisprotocol/cipher-paratime/actions?query=workflow:ci-lint+branch:main
[github-ci-audit-badge]: https://github.com/oasisprotocol/cipher-paratime/workflows/ci-audit/badge.svg
[github-ci-audit-link]: https://github.com/oasisprotocol/cipher-paratime/actions?query=workflow:ci-audit+branch:main
<!-- markdownlint-enable line-length -->

This is the Cipher ParaTime, an official [Oasis Protocol Foundation]'s ParaTime
for the [Oasis Network] built using the [Oasis SDK].

[Oasis Protocol Foundation]: https://oasisprotocol.org/
[Oasis Network]: https://docs.oasis.dev/oasis-network-primer/
[Oasis SDK]: https://github.com/oasisprotocol/oasis-sdk

## Note

* **This ParaTime currently depends on an unreleased version of [Oasis SDK].**
* **The code has not yet been audited.**

## SGX and Non-SGX Variants of the Binary

The non-SGX variant is a regular ELF binary that can be used by Oasis nodes
without SGX support to operate as client nodes.

This allows (non-SGX) Oasis nodes to interact with the Cipher ParaTime (e.g.
perform non-confidential queries and validate transactions they send out) but
they cannot participate in the execution of Cipher ParaTime's transactions and
they cannot see its confidential state.

## Building

### Prerequisites

#### Rust

Ensure you have [Rust] and [rustup] installed on your system.
For more details, see [Oasis Core's Development Setup Prerequisites]
documentation, the Rust section.

The version of the Rust toolchain we use for the Cipher ParaTime is specified in
the [rust-toolchain] file.

The rustup-installed versions of `cargo`, `rustc` and other tools will
[automatically detect this file and use the appropriate version of the Rust
toolchain][rust-toolchain-precedence] when invoked from the Cipher ParaTime git
checkout directory.

To install the appropriate version of the Rust toolchain, make sure you are
in an Cipher ParaTime git checkout directory and run:

```
rustup show
```

This will automatically install the appropriate Rust toolchain (if not
present) and output something similar to:

```
...

active toolchain
----------------

nightly-2021-08-17-x86_64-unknown-linux-gnu (overridden by '/code/rust-toolchain')
rustc 1.56.0-nightly (0035d9dce 2021-08-16)
```

Then add the Fortanix SGX Rust target to this version of the Rust toolchain by
running:

```
rustup target add x86_64-fortanix-unknown-sgx
```

[Rust]: https://www.rust-lang.org/
[rustup]: https://rustup.rs/
[Oasis Core's Development Setup Prerequisites]:
  https://docs.oasis.dev/oasis-core/development-setup/prerequisites
[rust-toolchain]: rust-toolchain
[rust-toolchain-precedence]:
  https://github.com/rust-lang/rustup/blob/master/README.md#override-precedence

#### System Packages

Building Cipher ParaTime requires the following system packages:

- [GCC]
- [Clang] (for compiling the [wasm3-rs] crates)

_NOTE: On Ubuntu/Debian systems, compiling [wasm3-rs] crates when building the
SGX binary requires having the `gcc-multilib` package installed._

On Fedora 35+, you can install the above with:

```
sudo dnf install gcc clang
```

On Ubuntu 20.04+, you can install the above with:

```
sudo apt install gcc gcc-multilib clang
```

[GCC]: http://gcc.gnu.org/
[Clang]: https://clang.llvm.org/
[wasm3-rs]: https://github.com/wasm3/wasm3-rs

### Non-SGX Binary

To build the non-SGX binary of the Cipher ParaTime, checkout the appropriate
version and run:

```
cargo build --release
```

The resulting ELF binary is located at `target/release/cipher-paratime`.

_NOTE: The non-SGX binary is dynamically linked so it may not be portable
between machines with different versions of shared libraries._

### SGX Binary

To build the SGX binary of the Cipher ParaTime, checkout the appropriate version
and run:

```
cargo build --release --target x86_64-fortanix-unknown-sgx
cargo elf2sgxs --release
```

The resulting SGX binary is located at
`target/x86_64-fortanix-unknown-sgx/release/cipher-paratime.sgxs`.

_NOTE: The SGX binary is always statically linked so it doesn't exhibit the
portability issues the ELF binary has._
