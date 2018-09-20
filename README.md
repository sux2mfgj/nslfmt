nslfmt
---
[![Build Status](https://travis-ci.org/sux2mfgj/nslfmt.svg?branch=master)](https://travis-ci.org/sux2mfgj/nslfmt)
[![Coverage Status](https://coveralls.io/repos/github/sux2mfgj/nslfmt/badge.svg?branch=master)](https://coveralls.io/github/sux2mfgj/nslfmt?branch=master)
[![gitmoji badge](https://img.shields.io/badge/gitmoji-%20😜%20😍-FFDD67.svg?style=flat-square)](https://github.com/carloscuesta/gitmoji)

##### What's this?
A nslfmt is a code fomatter for [NSL](http://www.overtone.co.jp/products/overture/) which is one of the HDL and a succsesor of [SFL](https://ja.wikipedia.org/wiki/SFL).

##### Requirements
- rust
You can build the nslfmt by __stable rust__ and toolchains, but, [tarpaulin](https://github.com/xd009642/tarpaulin) is required  for a coverage test, because we use a tarpauli which require it.

- tarpaulin
```
$ rustup run nightly RUSTFLAGS="--cfg procmacro2_semver_exempt" cargo install cargo-tarpaulin
$ rustup run nightly cargo run
```

##### References
- [Writting a Simple Parser in Rust](https://adriann.github.io/rust_parser.html)

##### License
[MIT](./LICENSE)
