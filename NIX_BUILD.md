# Reproducbile Builds with Nix

To build ciper-paratime without sgx support:

```console
nix build
```

or

```console
nix build .#nosgx
```

The binary `cipher-paratime` should be under `result/bin/`.

To build cipher-paratime with sgx support:

```console
nix build .#sgx
```

The binary `cipher-paratime.sgxs` should be under `result/bin/`.

Check the hash ...

SHA256:
```console
sha256sum result/bin/cipher-paratime.sgxs
```
```console
f7ff2296f5182cb287a30cab7c41a8a2ab75e72847300b9131f1507e37826322  result/bin/cipher-paratime.sgxs
```

BLAKE2:
```console
b2sum result/bin/cipher-paratime.sgxs
```
```console
09328d5f870ad3332307aff2eac5a23f767c6c2ff8fa98fc05754c410bcf6d1f4561ddf471d5cbe7a7ca957853c3b9f9c8d1063f71aa1f228fb5076d9be8e599  result/bin/cipher-paratime.sgxs
```

To build both `cipher-paratime` (without sgx support) and `cipher-paratime.sgxs`:

```console
nix build .#sgx .#nosgx
```

It's also possible to start a development shell with:

```console
nix develop --ignore-environment
```

Build the `cipher-paratime` binary:

```console
cargo build --release --target x86_64-fortanix-unknown-sgx
```

Convert an the `x86_64-fortanix-unknown-sgx` ELF binary to `SGXS`:

```console
cargo elf2sgxs --release
```

The output will be under `target/x86_64-fortanix-unknown-sgx/release`.

## Building without local repository

```console
nix build github:sbellem/cipher-paratime/nix-flake-2.0.1-alpha1#sgx
```
