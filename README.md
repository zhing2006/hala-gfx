# hala-gfx
[![License](https://img.shields.io/badge/License-GPL3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![MSRV](https://img.shields.io/badge/rustc-1.70.0+-ab6000.svg)](https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html)

[English](README.md) | [中文](README_CN.md) | [日本語](README_JP.md) | [한국어](README_KO.md)

## Introduction
`hala-gfx` is a graphics library developed in Rust, which provides a wrapper around the Vulkan API using the `ash` crate. This library aims to offer Rust developers a simple, intuitive, and powerful way to harness the graphics programming capabilities of Vulkan.

## Features
- **Vulkan Feature Wrapping**: Offers Rust wrappers for core Vulkan functionalities.

## Installation
To use `hala-gfx` in your Rust project, you can directly reference the git repository by adding the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
hala-gfx = { git = "https://github.com/zhing2006/hala-gfx.git" }
```

Make sure that you have the Rust programming environment and the cargo package manager installed on your system.

## Dependencies
`hala-gfx` depends on the following libraries:

- [thiserror](https://github.com/dtolnay/thiserror)：For defining error types.
- [log](https://github.com/rust-lang/log)：Provides logging capabilities.
- [ash](https://github.com/ash-rs/ash)：A Rust-written Vulkan binding library.
- [gpu-allocator](https://github.com/Traverse-Research/gpu-allocator)：For Vulkan memory allocation.
- [winit](https://github.com/rust-windowing/winit)：Offers a cross-platform solution for window creation and management.

Please ensure these dependencies are correctly installed before using `hala-gfx`.

## Contribution
Contributions of any kind are welcome, whether it's bug reporting or code contributions.

## License
`hala-gfx` is open-sourced under the [GNU General Public License v3.0](LICENSE).

## Contact
If you have any questions or suggestions, please contact us by creating an issue.