# Rust SDK 模块化与按需依赖设计

运行时热路径、内存、并发与构建成本的细化约束见 [性能设计](performance.md)。模块化提供隔离边界，不能替代运行时性能验证。

## 1. 设计结论

采用“对外一个入口 crate、对内 core 与产品 crate 分离”的结构：

- 用户通常只依赖 `tencentcloud-sdk`，通过 `features = ["cvm"]` 启用产品。
- 入口 crate 是 facade，只做条件重导出，不承担产品实现。
- 每个产品是独立 lib crate，以 optional dependency 接入入口。
- 对体积或依赖边界要求严格的用户，也可以直接依赖产品 crate。
- core 包含协议和共享机制，不依赖任何完整产品。

“一个使用入口”与“整个仓库只能有一个 Cargo package”不是同一个要求。前者既能提供简单 API，也能实现依赖级隔离。单 package 加 cfg 可以减少编译范围，但不能减少该 package 的源码下载量。

本文细化 architecture.md 的发布入口设计：根目录由纯 virtual workspace 调整为同时包含 facade package 的 workspace；原有 core 和产品实现分层不变。

## 2. 三种机制的区别

| 机制 | 能做到的事 | 不能保证的事 |
|---|---|---|
| 普通 `mod` / `pub mod` | 名称空间和可见性隔离 | 排除模块编译和依赖 |
| `#[cfg(feature = "cvm")]` | 未启用时不编译该模块 | 排除无条件依赖、减少同包下载量 |
| optional 产品依赖 + 条件重导出 | 未启用产品不参与正常依赖构建 | 禁止其他上游依赖启用同一产品 |

最终程序还会进行代码消除，但不能依赖链接器代替 feature 设计。编译时间、下载量、rlib 体积、最终 executable 体积是四个不同指标。Cargo.lock 中出现某个可选包，也不等于它参与了当前构建。

## 3. 目录和依赖方向

```text
tencentcloud-sdk/
├── Cargo.toml                 # facade package + workspace
├── src/
│   └── lib.rs                 # 只有公开重导出
├── crates/
│   ├── core/
│   │   ├── Cargo.toml
│   │   └── src/               # Client、凭证、签名、HTTP、错误
│   ├── cvm/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── v20170312/
│   │           ├── mod.rs
│   │           ├── client.rs
│   │           ├── models.rs
│   │           ├── models/
│   │           └── operations.rs
│   └── cynosdb/
├── tools/codegen/
├── metadata/
└── docs/
```

```text
应用
 └─ tencentcloud-sdk [cvm, transport-reqwest]
     ├─ tencentcloud-sdk-foundation
     └─ tencentcloud-sdk-cvm
         └─ tencentcloud-sdk-foundation

没有开启 cynosdb → 不构建 tencentcloud-sdk-cynosdb
```

产品之间不互相依赖。CVM 需要调用 STS 获取凭证时，由 core 的最小 STS 协议适配完成，不自动开启完整 STS 产品。core 的凭证、错误和 transport 类型保持唯一来源，避免各产品重复定义不兼容的类型。

## 4. Cargo feature 设计

以下 Cargo 片段用于说明结构；示例版本不代表已发布的包。

根 Cargo.toml：

```toml
[package]
name = "tencentcloud-sdk"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"

[workspace]
resolver = "3"
members = ["crates/*", "tools/codegen"]
default-members = [".", "crates/core"]
exclude = ["tests/live"]

[features]
default = []
cvm = ["dep:tencentcloud-sdk-cvm", "tencentcloud-sdk-cvm/v20170312"]
cynosdb = ["dep:tencentcloud-sdk-cynosdb", "tencentcloud-sdk-cynosdb/v20190107"]
transport-reqwest = ["tencentcloud-sdk-foundation/transport-reqwest"]
sse = ["tencentcloud-sdk-foundation/sse"]
tracing = ["tencentcloud-sdk-foundation/tracing"]
legacy-signing = ["tencentcloud-sdk-foundation/legacy-signing"]

[dependencies]
tencentcloud-sdk-foundation = { version = "0.1.0", path = "crates/core", default-features = false }
tencentcloud-sdk-cvm = { version = "0.1.0", path = "crates/cvm", optional = true, default-features = false }
tencentcloud-sdk-cynosdb = { version = "0.1.0", path = "crates/cynosdb", optional = true, default-features = false }
```

`default-members` 只控制不带 `--workspace` 时的根命令，不是 workspace 测试隔离机制。`cargo test --workspace` 仍会构建各产品成员；真实接口测试通过排除 live 包隔离。

facade 的 src/lib.rs：

```rust
#![forbid(unsafe_code)]

pub use tencentcloud_sdk_core::{
    ApiResponse, Client, ClientBuilder, Credentials, RequestOptions, SdkError,
};

pub use tencentcloud_sdk_core::credentials;
pub use tencentcloud_sdk_core::transport;

#[cfg(feature = "cvm")]
pub use tencentcloud_sdk_cvm as cvm;

#[cfg(feature = "cynosdb")]
pub use tencentcloud_sdk_cynosdb as cynosdb;
```

用户依赖配置：

```toml
[dependencies]
tencentcloud-sdk = { version = "0.1", default-features = false, features = ["cvm", "transport-reqwest"] }
```

导入路径：

```rust
use tencentcloud_sdk::Client;
use tencentcloud_sdk::cvm::v20170312::{
    Client as CvmClient,
    models::DescribeInstancesRequest,
};
```

未启用 `cvm` 时，该模块根本不存在，而不是运行时返回“功能未启用”。

### 4.1 产品与基础能力正交

- 产品 feature 不自动开启日志、旧版签名或其他产品。
- `transport-reqwest` 开启默认 HTTP 实现及所需 TLS，但不要求用户只能使用这一实现。
- core 的可替换 Transport 接口始终存在。无默认 transport 时，自定义 transport 可以正常构建客户端；未提供任何 transport 则 build 返回配置错误。
- core 同样 `default = []`。产品依赖 core 时必须设置 `default-features = false`，避免默认能力从内部重新被启用。
- 请求/响应 body 的基础字节流接口与 SSE 事件解析区分开；仅 `sse` 才引入事件解码逻辑。
- 重试、分页等轻量公共机制不为了“所有东西都能关”无限细拆 feature。

reqwest 和 TLS 依赖必须关闭各自不需要的默认 feature，不隐含引入 cookies、压缩、多种 TLS 后端等能力。具体依赖版本锁定后，用 `cargo tree -e features` 检查完整 feature 传播。

### 4.2 Feature 必须是加法

Cargo 对同一个包的 feature 进行合并，不能用 `no-cvm`、`no-async` 等负向开关抵消其他依赖启用的能力。

- 一个调用方启用 cvm，另一个调用方启用 cynosdb，最终可能同时启用两者。
- `default-features = false` 只约束当前依赖边，不能关闭其他边已经启用的 feature。
- feature 不改变已有类型的字段布局、方法返回类型或错误语义；只增加模块、实现与入口。
- 避免互斥 feature。默认只选定一个 TLS 实现；需要额外实现时保持类型与行为选择明确。
- 不提供普通用户容易误用的 `full` 默认项。

### 4.3 产品与版本 feature 分层

facade 一产品一个 feature，不为每个 Action 建 feature，也不在 facade 扁平展开全部 API 日期版本。

- 产品 crate 内用 `v20170312` 等版本模块，版本 feature 只在产品包内部管理。
- `cvm` 入口启用固定的已选 API 版本；不使用跟随发布静默变化的 latest。
- 需要非默认历史版本时，用户直接依赖产品包并启用对应版本，入口不为所有版本提供转发 feature。
- 单个产品启用后，其内部源码通常仍要整体检查和构建；不声称“只调用一个方法就只编译一个方法”。

crates.io 当前每个包最多 300 个 feature；参考 Java 仓库已有 262 个产品。facade 在发布时必须检查 feature 数量和包大小。超过发布限制时，不继续堆积转发 feature：新增产品直接使用独立产品 crate，并在文档给出等价用法。产品包不依赖 facade，因此不会受到 facade 容量限制。

## 5. 控制泛型和单态化体积

公共产品接口保持简单的具体 Client，而不是：

```text
Client<Transport, CredentialsProvider, Signer, RetryPolicy, Clock, ...>
```

Client 内部共享不可变配置和连接资源，Transport、凭证提供器在少数可扩展边界使用 `Arc<dyn Trait + Send + Sync>`。采用可对象化的 boxed Send future 接口，不为所有产品传播实现类型参数。

签名、HTTP、重试和错误解析形成非泛型的公共执行主体。泛型仅放在请求编码和响应解码边界：

```text
具体请求类型
  → serde 编码（按类型实例化）
  → 字节级公共执行链路（复用）
  → serde 解码（按响应类型实例化）
  → 具体响应类型
```

不把完整重试、凭证刷新和网络链路写进 `async fn execute<O: Operation>` 的泛型函数体中，否则每个操作都有可能复制整套状态机。泛型包装函数只连接薄的编解码逻辑与共享执行主体。

动态分派在 HTTP I/O 边界的开销通常远小于网络成本，但不能称为零开销。也不为追求静态分派而牺牲依赖结构和编译体积；最终以本地构建与测量验证。

## 6. Rust API 与可见性规范

- 模块目录负责源码组织；public re-export 负责稳定的用户入口，两者不必一一对应。
- core 的签名实现、缓存结构、HTTP 响应解包等默认私有，跨内部模块用 `pub(crate)`，不暴露实现细节。
- 官方日期版本是显式公共路径；类型命名 PascalCase，方法和字段 snake_case，线上字段用 serde rename 映射。
- 请求和响应使用具体类型，不用无约束 JSON 替代所有模型；通用调用另有显式 JSON 入口。
- 有效状态尽可能用类型表达，必填字段通过构造函数提供；公开模型通过构造函数/访问器和非穷尽策略控制演进。
- 输出结构使用 `#[non_exhaustive]`，保留访问字段的便利性；请求构造通过函数或 builder，不把所有字段暴露为必须被下游穷尽构造的稳定承诺。architecture.md 中的直接 struct literal 仅为最初接口草图，实际公开 API 按此规则收敛。
- 错误是可匹配的类型，保留来源错误、官方错误码与 RequestId；lib 不向用户返回 anyhow::Error。
- SDK 不安装日志 subscriber，不读取与任务无关的全局配置，不在构建时联网，不创建运行时或后台资源操作线程。
- 不要求所有类型都 Clone，不对凭证派生泄露内容的 Debug，不为所有生成类型自动派生不必要的 trait。
- 只有真正允许下游扩展的 trait 对外开放。内部操作描述使用 sealed trait 或私有描述对象，避免为了内部优化破坏第三方实现。

## 7. 验证按需构建

验证配置至少包括：

```text
cargo check --no-default-features
cargo check --no-default-features --features cvm,transport-reqwest
cargo check --no-default-features --features cynosdb,transport-reqwest
cargo check --no-default-features --features cvm,cynosdb,transport-reqwest
cargo tree -e normal --no-default-features --features cvm,transport-reqwest
cargo tree -e features --no-default-features --features cvm,transport-reqwest
```

检查使用 CVM 的正常依赖图里没有 cynosdb、postgres 等未启用产品。另用独立的最小消费项目验证，避免 workspace 开发依赖掩盖缺失 feature。

用相同工具链、目标平台、release profile 和使用方式，对比无产品、仅 CVM、多个产品的编译时间、产物大小和依赖图。不把启用 feature 后的源码量直接等同于最终二进制大小。release profile 由最终应用决定，不在 SDK lib 内强制 LTO、strip 或 panic=abort。

文档按产品生成，清楚标注 feature 要求；examples 使用 required-features 避免最小构建失败。全产品文档/测试是维护操作，不让所有产品成为普通消费方默认构建的一部分。

## 8. 严格单 package 的形式

如果发布形式严格限定为一个 Cargo package，则改为同包产品模块：

```rust
#[cfg(feature = "cvm")]
pub mod cvm;

#[cfg(feature = "cynosdb")]
pub mod cynosdb;
```

同时把产品独有第三方依赖声明为 optional；仅使用 cfg! 分支不够，因为分支中的代码仍需类型检查。

这种形式能够排除未启用产品的模块编译，适合产品数量较少的 SDK；但所有产品源码仍属于一个发布包，共用 feature 配额和发布体积限制，所有产品一起发布、一起演进版本。对于覆盖数百个腾讯云产品的 SDK，facade + 产品子 crate 的依赖隔离更稳健。

## 9. 规范依据

- Cargo Features：https://doc.rust-lang.org/cargo/reference/features.html
- Cargo Resolver：https://doc.rust-lang.org/cargo/reference/resolver.html
- Rust Conditional Compilation：https://doc.rust-lang.org/reference/conditional-compilation.html
- Rust API Guidelines：https://rust-lang.github.io/api-guidelines/
