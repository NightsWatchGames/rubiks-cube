# Rubik's Cube
- [x] Rubiks cube
- [x] Scramble cube
- [x] Reset cube
- [x] Mouse dragging
- [x] Game UI
- [x] Camera controller (move、zoom)
- [x] WASM support

Play online：[click here](https://nightswatchgames.github.io/games/rubiks-cube/)（Open with PC Chrome/Firefox/Edge）

## Get started
1. Native
```
cargo run
```
2. WASM
```
rustup target install wasm32-unknown-unknown
cargo install wasm-server-runner
cargo run --target wasm32-unknown-unknown
```
```
cargo install wasm-bindgen-cli
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/rubiks-cube.wasm
```

## Screenshots
Game video: [YouTube](https://www.youtube.com/watch?v=hGuDTozFvMk)

![playing cube](https://raw.githubusercontent.com/NightsWatchGames/rubiks-cube/master/screenshots/playing_cube.jpg)

## Reference
- [Rubik's Cube - Wikipedia](https://en.wikipedia.org/wiki/Rubik%27s_Cube)
- [Online Rubik's Cube example1](https://rubikscu.be/)
- [Online Rubik's Cube example2](https://cube-solver.com/)