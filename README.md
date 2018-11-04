nslfmt
---
[![Build Status](https://travis-ci.org/sux2mfgj/nslfmt.svg?branch=master)](https://travis-ci.org/sux2mfgj/nslfmt)
[![Coverage Status](https://coveralls.io/repos/github/sux2mfgj/nslfmt/badge.svg?branch=master)](https://coveralls.io/github/sux2mfgj/nslfmt?branch=master)
[![gitmoji badge](https://img.shields.io/badge/gitmoji-%20😜%20😍-FFDD67.svg?style=flat-square)](https://github.com/carloscuesta/gitmoji)

### What's this?
A nslfmt is a code fomatter for [NSL](http://www.overtone.co.jp/products/overture/) which is one of the HDL and a succsesor of [SFL](https://ja.wikipedia.org/wiki/SFL).

### How to Use
TBD

### Requirements for Developpers
- rust  
You can build the nslfmt by __stable rust__ and toolchains. If you want to run a coverage test, nightly rust is required, because we use [tarpaulin](https://github.com/xd009642/tarpaulin).

- tarpaulin  
```
$ rustup run nightly RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin
$ rustup run nightly cargo run
```

### Run the tests
- all
```
$ cargo test
```

- unit test
    - e.g run a test of wire_02 in parser_test.rs
```
$ cargo test module::wire_02 --test parser_test
```

### [TODO](./task_list.md)

### References
- [Writting a Simple Parser in Rust](https://adriann.github.io/rust_parser.html)

### License
[MIT](./LICENSE)
