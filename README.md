[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Travis CI](https://travis-ci.com/KizzyCode/mbedcrypto_sys-rust.svg?branch=master)](https://travis-ci.com/KizzyCode/mbedcrypto_sys-rust)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/mbedcrypto_sys-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/mbedcrypto-sys-rust)


# mbedcrypto_sys
Welcome to `mbedcrypto_sys` 🎉

This crate builds the crypto parts of mbedTLS and provides low-level bindings to them.


## Requirements
The following build tools are required:
 - A unix-like `tar` binary with `gzip` support. You can specify the path manually by setting `MBEDCRYPTO_TAR`.
 - A working `cmake` toolchain (used to build mbedTLS).
 - A working `python3` installation (used to build mbedTLS).
 - A working `llvm/clang` toolchain (used to generate the bindings).


## Included 3rd-Party Products and Licenses
 - mbedTLS: [Apache License 2.0](./mbedtls/LICENSE)