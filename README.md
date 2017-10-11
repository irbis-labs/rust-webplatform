# rust-webplatform

A Rust library for use with emscripten to access the DOM and API.

[Read the documentation](https://docs.rs/crate/webplatform), read [brson's post on how
Rust works with emscripten](https://users.rust-lang.org/t/compiling-to-the-web-with-rust-and-emscripten/7627),
or see an example app with [rust-todomvc](http://github.com/tcr/rust-todomvc).

```rust
extern crate webplatform;

fn main() {
    let document = webplatform::init();
    let body = document.element_query("body").unwrap();
    body.html_set("<h1>HELLO FROM RUST</h1> <button>CLICK ME</button>");
    let button = document.element_query("button").unwrap();
    button.on("click", |_| webplatform::alert("WITNESS ME"));
}
```

Used with `cargo build --target=asmjs-unknown-emscripten` or `cargo build --target=wasm32-unknown-emscripten`.


## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
