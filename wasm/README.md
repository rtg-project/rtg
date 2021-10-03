# rtg Wasm

To build:

```
wasm-pack build --target web
http-server -c-1 .
```

See https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

# Size optimization

You can re optimize the output binary size even more.

See https://rustwasm.github.io/book/reference/code-size.html#compiling-with-link-time-optimizations-lto

```
brew install binaryen
```