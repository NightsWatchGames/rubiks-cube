# Rubik's Cube 魔方
- [x] 3阶魔方
- [x] 随机打乱魔方
- [x] 重置魔方
- [x] 鼠标拖拽魔方旋转
- [x] 游戏UI
- [ ] 相机视角控制（缩放、移动）
- [x] WASM支持

在线游玩：[点这里](https://nightswatchgames.github.io/games/rubiks-cube/)（电脑版Chrome/Firefox打卡）

## 运行
1. 本地运行
```
cargo run
```
2. WASM运行
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

## 游戏截图
![playing cube](https://raw.githubusercontent.com/NightsWatchGames/rubiks-cube/master/screenshots/playing_cube.jpg)

## 参考资料
- [Rubik's Cube - Wikipedia](https://en.wikipedia.org/wiki/Rubik%27s_Cube)
- [Online Rubik's Cube example1](https://rubikscu.be/)
- [Online Rubik's Cube example2](https://cube-solver.com/)