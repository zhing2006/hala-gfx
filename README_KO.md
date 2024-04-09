# hala-gfx
[![License](https://img.shields.io/badge/License-GPL3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0.en.html)
[![MSRV](https://img.shields.io/badge/rustc-1.70.0+-ab6000.svg)](https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html)

[English](README.md) | [中文](README_CN.md) | [日本語](README_JP.md) | [한국어](README_KO.md)

## 소개
`hala-gfx`는 Rust 언어로 개발된 그래픽 라이브러리로, `ash` 크레이트를 이용해 Vulkan API를 래핑합니다. 이 라이브러리는 Rust 개발자들에게 Vulkan의 그래픽 프로그래밍 기능을 간단하고 직관적이며 강력하게 활용할 수 있는 방법을 제공하는 것을 목표로 합니다.

## 기능 특징
- **Vulkan 기능 래핑**: Vulkan 핵심 기능에 대한 Rust 래퍼를 제공합니다.

## 설치
Rust 프로젝트에서 `hala-gfx`를 사용하려면, `Cargo.toml` 파일에 다음 의존성을 추가하여 git 저장소를 직접 참조하십시오:

```toml
[dependencies]
hala-gfx = { git = "https://github.com/zhing2006/hala-gfx.git" }
```

시스템에 Rust 프로그래밍 환경과 cargo 패키지 매니저가 설치되어 있는지 확인하십시오.

## 의존성
`hala-gfx`는 다음 라이브러리에 의존합니다:

- [thiserror](https://github.com/dtolnay/thiserror)：에러 타입을 정의하는 데 사용됩니다.
- [log](https://github.com/rust-lang/log)：로깅 기능을 제공합니다.
- [ash](https://github.com/ash-rs/ash)：Rust로 작성된 Vulkan 바인딩 라이브러리입니다.
- [gpu-allocator](https://github.com/Traverse-Research/gpu-allocator)：Vulkan 메모리 할당을 위해 사용됩니다.
- [winit](https://github.com/rust-windowing/winit)：플랫폼 간 창 생성 및 관리 솔루션을 제공합니다.

`hala-gfx`를 사용하기 전에 이러한 의존성이 올바르게 설치되어 있는지 확인하십시오.

## 기여
버그 보고 또는 코드 기여 등 모든 종류의 기여를 환영합니다.

## 라이선스
`hala-gfx`는 GNU General Public License v3.0을 오픈 소스 라이선스로 사용합니다.

## 연락처
질문이나 제안이 있으시면 issue를 생성하여 연락주십시오.