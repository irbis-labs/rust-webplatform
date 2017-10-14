# rust-webplatform

[![Travis Build Status][travis-build-status-svg]][travis-build-status]

A Rust library for use with emscripten to access the DOM and API.

[Read the documentation](https://docs.rs/crate/webplatform), read [brson's post on how
Rust works with emscripten](https://users.rust-lang.org/t/compiling-to-the-web-with-rust-and-emscripten/7627),
or see an example app with [rust-todomvc](http://github.com/tcr/rust-todomvc).


## Example

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


## Roadmap

### v 0.5. Establish a development process

* [x] CI
* [ ] Tests
* [ ] Split Document and Window
* [ ] Extract event manager
* [ ] Make intuitive method names similar to the original ones in JS

### v 0.6. Support Essential Event Types

* [ ] FocusEvent
* [ ] KeyboardEvent
* [ ] MouseEvent
* [ ] UiEvent
* [ ] WheelEvent
 
### v 0.7. More APIs 

* [ ] File
* [ ] WebSocket
* [ ] XmlHttpRequest

### v 0.x. Keep doing good

* [ ] Fix memory leaks


## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

[travis-build-status]: https://travis-ci.org/irbis-labs/rust-webplatform?branch=master
[travis-build-status-svg]: https://travis-ci.org/irbis-labs/rust-webplatform.svg?branch=master
