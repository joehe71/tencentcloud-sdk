# Core 最低版本与离线性能实测

## 1. 验证环境

- 日期：2026-10-15。
- 平台：aarch64-apple-darwin，macOS 27.0.1（26A434）。
- 硬件：Apple M5，10 个逻辑核心，16 GiB 内存。
- 当前工具链：Rust/Cargo 1.98.1（Homebrew）。
- 最低版本工具链：rustc 1.85.0（4d91de4e4 2025-02-17）、cargo 1.85.0（d73d2caf9 2024-12-31）。
- 被测运行时实现：提交 `1f4008f` 的 core。本次增加内部基准、开发依赖与验证文档，未修改运行时调用链。
- 依赖使用本次提交的 Cargo.lock，所有验证命令使用 `--locked`。

## 2. Rust 1.85.0 实测

Rust 2024 edition 所需的最低稳定版本为 1.85；本项目的 rust-version 保持 1.85。验证的是 1.85.0 本身，不是通过新编译器的兼容性选项模拟。

隔离工具链安装在 `target/msrv`，编译产物在 `target/msrv-build`，未修改全局 Rust 默认版本或 PATH；该目录被 Git 忽略。

已执行并通过：

| 命令 | 结果 |
|---|---|
| `cargo check --workspace --no-default-features --locked` | 两个 lib crate 构建通过 |
| `cargo test --workspace --no-default-features --locked` | 22 项通过，1 项性能基准默认忽略 |
| `cargo test --workspace --all-features --locked` | 25 项通过，1 项性能基准默认忽略 |

两组测试都包含单元、集成及 doctest 检查；当前 doctest 数量为 0。全 feature 额外执行 4 项本地 HTTP 测试，不调用腾讯云。

复现时需显式使用 1.85.0 的 cargo、rustc 和 rustdoc，不能只切换 cargo 而继续调用新版 rustc。使用 rustup 管理工具链的环境可运行：

```bash
cargo +1.85.0 check --workspace --no-default-features --locked
cargo +1.85.0 test --workspace --no-default-features --locked
cargo +1.85.0 test --workspace --all-features --locked
```

本次实际使用项目内工具链的绝对路径，通过 RUSTC、RUSTDOC 指定对应二进制，并设置独立 CARGO_TARGET_DIR。验证结果仅适用于已锁定依赖与 macOS ARM；不声称所有平台或未来任意依赖更新均已验证。更新依赖后需重跑最低版本测试。

## 3. 性能测量方法

基准位于 `crates/core/src/benchmarks.rs`，通过 cfg(test) 编译为默认忽略的内部测试，使用 Criterion 0.5.1：

- 直接调用实际 `protocol::encode`、TC3 签名、`protocol::decode` 和 `Client::execute`，不复制实现，也不增加公开 benchmark feature。
- Criterion 仅为 dev-dependency，关闭其默认 feature，不进入 SDK 正常依赖图。
- 每个用例预热 1 秒，测量 2 秒，采集 50 个样本，bootstrap 重采样 10,000 次，置信水平 95%。
- release opt-level=3，LTO=false，codegen-units=16，debug=0，incremental=0；使用 Rust 1.98.1。
- 单个 libtest 测试串行执行全部用例，Tokio 使用一个 current-thread Runtime；不在计时期间编译其他目标。
- 使用 std::hint::black_box，避免将输入或结果优化掉。正常输出的构造和释放计入迭代开销。
- 结果文件写入 `target/criterion-core`；new/estimates.json 保存时间估计，new/sample.json 保存样本。

输入由一个 Data 字符串组成，不含转义字符，JSON 总字节数严格为 1 KiB、64 KiB、1 MiB。响应还包含固定 RequestId。该数据适合建立线性体积基线，不代表复杂嵌套、Unicode/转义密集的所有产品模型。

| 用例 | 包含的工作 | 不包含的工作 |
|---|---|---|
| encode | 有界 writer、serde JSON 编码及返回字节 | 构造请求模型 |
| tc3_sign | payload SHA256、UTC 日期、HMAC 派生、Authorization | JSON 编码、网络 |
| decode | 外层扫描、公共字段解析、强类型业务解码 | 网络读取、构造原始响应 |
| mock_execute | 请求编码、凭证快照、签名、Header、Mock 传输、body 收集和业务解码 | DNS、TLS、网络与服务端处理 |
| client_clone | 共享 Client 的克隆及释放 | 新建客户端或连接池 |
| static_credentials | boxed future、Arc 快照读取及 Runtime::block_on | 环境变量读取、刷新或 STS |
| decode_service_error | 解析固定限流错误及保留元数据 | 成功模型解码 |

Mock 完整调用的请求和响应分别为表中尺寸；其吞吐字节数按两者之和统计。计时外创建 Client、连接资源抽象、模型、操作描述、固定时钟和原始响应。每次迭代仍通过真实核心执行路径；Mock 不进行网络等待。

复现命令：

```bash
CARGO_INCREMENTAL=0 \
CARGO_PROFILE_RELEASE_OPT_LEVEL=3 \
CARGO_PROFILE_RELEASE_LTO=false \
CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 \
CARGO_PROFILE_RELEASE_DEBUG=0 \
cargo test -p tencentcloud-sdk-foundation --release --no-default-features --locked \
  --lib benchmarks::core_performance_baseline \
  -- --ignored --exact --nocapture --test-threads=1
```

不传 --ignored 时不会执行耗时基准；若以 debug 模式显式运行，基准会拒绝执行，避免将未优化结果误记为 release 性能。

## 4. 两轮实测结果

两轮使用完全相同的二进制、配置和输入。下表统一取 estimates.json 的 mean，而非混用 Criterion 控制台默认展示的 slope/mean；95% CI 和标准差来自第二轮。标准差描述 50 个批次各自的每迭代平均耗时分布，不是单次请求的尾延迟。

单位均为微秒（µs）：

| 用例 | JSON 尺寸 | 第一轮均值 | 第二轮均值 | 第二轮 95% CI | 第二轮标准差 |
|---|---:|---:|---:|---:|---:|
| encode | 1 KiB | 0.765 | 0.758 | 0.757–0.760 | 0.005 |
| encode | 64 KiB | 36.470 | 32.325 | 32.260–32.397 | 0.251 |
| encode | 1 MiB | 507.176 | 509.957 | 508.957–510.964 | 3.594 |
| tc3_sign | 1 KiB | 9.558 | 9.502 | 9.488–9.517 | 0.051 |
| tc3_sign | 64 KiB | 224.827 | 224.595 | 222.923–227.663 | 10.056 |
| tc3_sign | 1 MiB | 3507.386 | 3477.035 | 3474.121–3479.937 | 10.579 |
| decode | 1 KiB | 0.791 | 0.779 | 0.778–0.780 | 0.004 |
| decode | 64 KiB | 25.952 | 26.455 | 26.352–26.563 | 0.389 |
| decode | 1 MiB | 399.479 | 403.673 | 402.834–404.604 | 3.314 |
| mock_execute | 1 KiB | 13.239 | 13.166 | 13.139–13.197 | 0.106 |
| mock_execute | 64 KiB | 286.911 | 284.217 | 283.757–284.671 | 1.662 |
| mock_execute | 1 MiB | 4413.375 | 4398.551 | 4395.410–4401.584 | 11.399 |

以下单位为纳秒（ns）：

| 用例 | 第一轮均值 | 第二轮均值 | 第二轮 95% CI | 第二轮标准差 |
|---|---:|---:|---:|---:|
| client_clone | 6.652 | 6.648 | 6.638–6.659 | 0.037 |
| static_credentials | 121.791 | 121.727 | 121.356–122.293 | 1.783 |
| decode_service_error | 421.115 | 419.255 | 418.074–420.959 | 5.366 |

数据解读：

- 1 KiB 请求/响应的完整本地调用约 13.2 µs；1 MiB 约 4.40 ms。后者不能被表述为真实腾讯云请求只需 4.40 ms。
- 第一轮 64 KiB 编码存在明显噪声：均值 36.470 µs、中位数 32.313 µs、标准差 7.133 µs；第二轮均值 32.325 µs、标准差 0.251 µs。保留两轮结果，不将第二轮较快值解释为代码优化收益。
- 第二轮 64 KiB 签名也有高侧离群值，均值 224.595 µs、中位数 223.247 µs，标准差 10.056 µs；置信区间已反映此波动。
- Criterion 自动选择采样模式：多数用例使用 Linear；1 MiB 签名和完整调用使用 Flat，仍各有 50 个样本。未将批次均值的 p95/p99 误当作真实请求尾延迟。
- 两轮没有修改代码。控制台可能提示 Performance has improved，这只是统计差异，不构成优化结论。桌面系统未做独占或 CPU 频率锁定，区间也不涵盖跨机器、跨编译器和系统负载变化。
- 签名的独立用例在大包场景成本最突出，可作为后续 profiling 的定位依据；独立微基准不是严格可加的阶段追踪，本次不据此改动算法或添加手写 SIMD/unsafe。

原始第一轮结果保存在 `target/criterion-core-run1`，第二轮保存在 `target/criterion-core`；这两个路径均不提交。上述表格永久记录两轮摘要，重跑会更新 target 中的 Criterion 结果。

本次 Cargo.lock SHA-256：`e10888ccfa2114e622e2504f3c1f66bd79e99a7151da94879ba8fa31107e260f`。

## 5. 验证边界

已完成的是当前 core 的最低编译器兼容性与单线程离线耗时基线。没有调用真实腾讯云接口，也没有测量真实网络 QPS、TLS/DNS 延迟、并发尾延迟、分配次数、峰值内存、复杂产品模型或未来 SSE/凭证刷新。性能数据不充当这些能力的验证结果，也不据此设定跨环境的绝对性能门槛。
