# 腾讯云 Rust SDK

按 Rust 惯用法设计的腾讯云 API 3.0 SDK：对外一个入口 crate，对内将共享内核与产品封装分离。Java SDK 只用于协议和模型参考，不移植其类层次或反射调用。

当前实现的是 `tencentcloud-sdk-core`，尚无产品 crate；两个包均暂不发布。架构文档描述的产品 feature、分页、SSE 等能力不代表已经实现。

## 已实现

- Rust 2024 workspace，根入口重导出 core，默认 feature 为空。
- 共享、不可变配置的异步 Client；需要启用时间驱动的 Tokio Runtime，SDK 不创建 Runtime。
- 静态凭证与显式环境凭证加载；支持临时 Token、自定义异步凭证提供器。
- 可选凭证文件、CVM 实例角色、STS AssumeRole、TKE/OIDC、显式默认凭证链，以及共享缓存和按需自动刷新。
- TC3-HMAC-SHA256、POST JSON、固定操作描述和动态通用调用。
- 可替换 Transport；可选 reqwest/Rustls 连接池实现。
- 按产品的 HTTPS 根端点覆盖、单次 Region/语言/Header/超时选项。
- JSON 统一响应外层、强类型解析、保留官方错误码与 RequestId。
- 有界请求编码与实际响应字节读取，凭证和请求的默认调试输出脱敏。

## 依赖与使用

仓库尚未发布，使用本地路径依赖：

```toml
[dependencies]
tencentcloud-sdk = { path = "../tencentcloud-sdk", default-features = false, features = ["transport-reqwest"] }
serde_json = "1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

以下示例会发起真实查询，需由应用自行授权运行；本仓库默认测试不会执行它：

```rust,no_run
use tencentcloud_sdk::{ApiResponse, Client, CommonClient};
use serde_json::{Value, json};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .credentials_from_env()
        .region("ap-shanghai")
        .build()?;
    let cvm = CommonClient::new(client, "cvm", "2017-03-12")?;
    let response: ApiResponse<Value> = cvm
        .call("DescribeInstances", &json!({"Limit": 1}))
        .await?;
    println!("RequestId: {:?}", response.metadata.request_id);
    Ok(())
}
```

`credentials_from_env()` 仅在 build 时读取 `TENCENTCLOUD_SECRET_ID`、`TENCENTCLOUD_SECRET_KEY` 和可选的 `TENCENTCLOUD_TOKEN`，不探测实例元数据、不调用 STS，也不在每次请求重新读取环境。该静态环境快照没有自动有效期管理；需要角色换证和刷新时，可显式启用 `credentials-roles` 并选择相应提供器，见 [Credentials guide](docs/credentials.md)。

也可以使用 `Operation::new(service, version, action)` 搭配 `Client::execute` 和自定义 serde 请求/响应结构。该入口与 CommonClient 使用同一调用链路，所有请求必须编码为 JSON 对象。

未启用 `transport-reqwest` 时需通过构造器注入 `Transport`，否则 build 返回配置错误。core 不隐藏开启网络依赖。

## 默认行为与限制

- 每次调用只发送一次请求，不自动重试；默认传输关闭重定向和环境代理，开启 TLS 证书校验。
- 自定义 Transport 必须遵守禁止重定向、自动重放和请求重编码的约束。
- 默认连接超时 10 秒，调用预算 60 秒，请求和响应体各限制 32 MiB。
- 预算包含凭证获取与响应体读取；同步编码/解码无法被中途抢占，会在阶段边界检查是否超时。
- endpoint 只支持 HTTPS 根 URL，不支持任意路径/查询串；HTTP 仅通过显式数值回环端点供本地测试使用。
- HTTP 非 200 也解析腾讯云 JSON 错误；缺失 Content-Type 时尝试 JSON，存在时必须为 application/json，可带 charset。
- 服务错误 message 可能含业务信息，仅通过字段显式访问，不进入默认 Display/Debug；调用方显式访问错误来源时仍需注意其中的敏感信息。
- 默认不读取凭证文件或探测元数据；文件和角色凭证必须显式启用并选择，SDK 不持久化密钥。
- 业务 API 仍仅支持 POST JSON；角色元数据使用 GET。当前不支持旧版签名、二进制/multipart、SSE、自动分页、并发限额和容灾切换。

## 本地校验

```bash
cargo fmt --all --check
cargo test --workspace --no-default-features
cargo test --workspace --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo doc --workspace --all-features --no-deps
cargo tree -e normal --no-default-features
bash scripts/check-coverage.sh
```

全 feature 生产代码行覆盖率要求 **≥90%**，当前实测 **92.25%**；口径、工具配置和报告命令见 [Test coverage](docs/coverage.md)。

测试使用固定假凭证与 MockTransport；HTTP 适配器测试只访问 127.0.0.1 临时端口，不依赖真实腾讯云凭证。签名预期值通过独立 Python hashlib/hmac 实现核对。

最低 Rust 版本 **1.85.0** 已在 macOS ARM 上验证。离线性能基准使用 release 配置，显式运行，不进入默认测试或 SDK 运行时依赖；环境、结果和复现命令见 [实测报告](docs/validation.md)。

## 设计文档

- [Credentials guide](docs/credentials.md)
- [整体架构](docs/architecture.md)
- [模块化与按需依赖](docs/modularity.md)
- [性能与离线基准](docs/performance.md)
