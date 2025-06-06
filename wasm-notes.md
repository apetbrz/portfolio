# Learning WASM

## my notes

### Compilation:

`cargo build --target=wasm32-unknown-unknown`

`wasm-bindgen target/wasm32-unknown-unknown/release/[projectname].wasm --out-dir pkg`

Move `pkg/*` to `lib/wasm/` in front-end
