# hala-gfx

## 简介
`hala-gfx`是一个基于Rust语言开发的图形库，它利用`ash`库对Vulkan API进行了封装。该库旨在为Rust开发者提供一个简单、直观且功能强大的方式来利用Vulkan的图形编程能力。

## 功能特点
- **Vulkan功能封装**：提供了Vulkan核心功能的Rust封装。

## 安装
要在你的Rust项目中使用`hala-gfx`，你可以通过在`Cargo.toml`文件中添加以下依赖来直接引用git仓库：

```toml
[dependencies]
hala-gfx = { git = "https://github.com/zhing2006/hala-gfx.git" }
```

确保你的系统已经安装了Rust编程环境和cargo包管理器。

## 依赖关系
hala-gfx依赖于以下库：

- [thiserror](https://github.com/dtolnay/thiserror)：用于定义错误类型。
- [log](https://github.com/rust-lang/log)：提供日志记录功能。
- [ash](https://github.com/ash-rs/ash)：一个Rust编写的Vulkan绑定库。
- [gpu-allocator](https://github.com/Traverse-Research/gpu-allocator)：用于Vulkan内存分配。
- [winit](https://github.com/rust-windowing/winit)：提供了一个跨平台的窗口创建和管理的解决方案。

请确保这些依赖项在使用hala-gfx之前已正确安装。

## 贡献
欢迎任何形式的贡献，无论是bug报告或是代码贡献。

## 许可证
hala-gfx根据《[GNU General Public License v3.0许可证](LICENSE)》开源。

## 联系方式
如果你有任何问题或建议，请通过创建一个issue来联系。