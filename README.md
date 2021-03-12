Reproduction of [kornelski/mozjpeg-sys#23]
==========================================

Example repo which reproduces [kornelski/mozjpeg-sys#23].




## Reproduce steps

1. `cargo run` should compile and run OK.

2. Change `mozjpeg-sys` to `"=0.10.13"` in `Cargo.toml`.

3. `cargo run` should compile, but panic with `libjpeg fatal error: Sampling factors too large for interleaved scan` message.





[kornelski/mozjpeg-sys#23]: https://github.com/kornelski/mozjpeg-sys/issues/23
