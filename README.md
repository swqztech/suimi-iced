# suimi-iced

[随密](https://github.com/swqztech/suimi) 的 **iced** 实现：与 `suimi`（eframe/egui）、`suimi-gpui` 功能相同，使用 [iced](https://github.com/iced-rs/iced)（Elm 风格、wgpu 渲染）。

## 构建

```bash
cd suimi-iced
cargo build --release
target\release\suimi-iced.exe
```

Windows 下会自动尝试加载系统微软雅黑等字体以显示中文。

## 说明

- 密码 / 用户名 / 网名、强度估计、复制、一次六个等与其它随密客户端一致
- 单文件 exe 绿色运行（无需额外 DLL）

## 打包（绿色版 / 安装包）

```bash
bash packaging/build.sh
```

产物：绿色版 ZIP + 可选 NSIS 安装包 `*-setup.exe`。见 `packaging/README.md`。
