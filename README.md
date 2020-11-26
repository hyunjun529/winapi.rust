# winapi.rust
winapi &lt;> rust simple example.

![image](/_res/run.png)

## build & run
- VS2019 for cpp
- npm
- Rust
  - wasm-pack : https://rustwasm.github.io/docs/wasm-pack/quickstart.html
  - `cargo build` at `/`

### cpp (C++/WinAPI32)
- excute cpp.sln
- `F5`

### rust_winapi (Rust, wasm-pack)
- `cargo run -p rust_winapi` at `/`

### rust_wasm (Rust, winapi crate)
- install
```bash
wasm-pack build rust_wasm
cd rust_wasm
npm install
npm run serve
```
- `http://localhost:8080/`