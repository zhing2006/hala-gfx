# hala-gfx
[![License](https://img.shields.io/badge/License-GPL3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![MSRV](https://img.shields.io/badge/rustc-1.70.0+-ab6000.svg)](https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html)

[English](README.md) | [中文](README_CN.md) | [日本語](README_JP.md) | [한국어](README_KO.md)

## 紹介
`hala-gfx`はRust言語で開発されたグラフィックスライブラリで、`ash`クレートを使用してVulkan APIをラップしています。このライブラリは、Rust開発者にVulkanのグラフィックスプログラミング機能を簡単で直感的かつ強力に活用する方法を提供することを目指しています。

## 機能特徴
- **Vulkan機能のラッピング**：Vulkanのコア機能のRustラッパーを提供します。

## インストール
Rustプロジェクトで`hala-gfx`を使用するには、`Cargo.toml`ファイルに以下の依存関係を追加して、gitリポジトリを直接参照することができます：

```toml
[dependencies]
hala-gfx = { git = "https://github.com/zhing2006/hala-gfx.git" }
```

システムにRustプログラミング環境とcargoパッケージマネージャがインストールされていることを確認してください。

## 依存関係
`hala-gfx`は以下のライブラリに依存しています：

- [thiserror](https://github.com/dtolnay/thiserror)：エラータイプを定義するために使用します。
- [log](https://github.com/rust-lang/log)：ログ記録機能を提供します。
- [ash](https://github.com/ash-rs/ash)：Rustで書かれたVulkanバインディングライブラリです。
- [gpu-allocator](https://github.com/Traverse-Research/gpu-allocator)：Vulkanメモリ割り当て用です。
- [winit](https://github.com/rust-windowing/winit)：クロスプラットフォームのウィンドウ作成と管理のソリューションを提供します。

`hala-gfx`を使用する前にこれらの依存関係が正しくインストールされていることを確認してください。

## 貢献
バグ報告やコードの貢献など、あらゆる種類の貢献を歓迎します。

## ライセンス
`hala-gfx`は[GNU General Public License v3.0](LICENSE)でオープンソース化されています。

## 連絡先
ご質問や提案がある場合は、issueを作成してご連絡ください。