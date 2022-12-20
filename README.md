# Rubik's Cube 魔方
- [x] 3阶魔方
- [x] 随机打乱魔方
- [x] 重置魔方
- [x] 鼠标拖拽魔方旋转
- [x] 游戏UI
- [ ] 相机视角控制（缩放、移动）
- [ ] WASM支持
- [ ] N阶魔方

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

## 游戏截图
![playing cube](https://raw.githubusercontent.com/NightsWatchGames/rubiks-cube/master/screenshots/playing_cube.jpg)

## 参考资料
- [Rubik's Cube - Wikipedia](https://en.wikipedia.org/wiki/Rubik%27s_Cube)
- [Online Rubik's Cube example1](https://rubikscu.be/)
- [Online Rubik's Cube example2](https://cube-solver.com/)