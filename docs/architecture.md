# 腾讯云 Rust SDK 架构设计

发布入口、feature 默认值、产品版本启用方式及公开模型构造策略以 [模块化与按需依赖设计](modularity.md) 的细化方案为准：对外单 facade crate，对内保留 core 与产品 crate，默认不启用产品或具体 HTTP 实现。下文目录和接口示例保留为内核分层草图。

热路径、资源共享、背压、凭证缓存和离线基准的设计见 [性能设计](performance.md)。

## 1. 架构原则

SDK 面向腾讯云 API 3.0，提供通用请求能力和按产品、API 版本组织的强类型封装。协议以腾讯云官方 API 文档为准，Java SDK 作为行为和模型参考，不作为协议规范的替代品。

- 通用内核只实现协议与基础机制，不包含产品业务流程。
- 产品 Client 是通用内核的薄封装，模型和接口通过生成器维护。
- 默认异步，复用 HTTP 连接池；不在 SDK 内创建 Tokio Runtime。
- 采用组合而非继承，不复制 Java 的反射式调用和模型基类。
- 一个 core crate 承载内核，避免按技术概念过度拆包。
- 强类型调用与通用调用使用相同的签名、传输和错误处理链路。
- 不把“查询类命名”当作接口幂等性的证明，不默认重试未知操作。

## 2. Workspace 与依赖关系

```text
tencentcloud-sdk/
├── Cargo.toml                         # workspace、统一依赖、lint
├── Cargo.lock                         # 固定本仓库工具和测试的依赖解析
├── README.md
├── LICENSE
├── AGENTS.md
├── docs/
│   ├── architecture.md
│   └── codegen.md                     # 元数据格式和生成约定
├── crates/
│   ├── core/                         # tencentcloud-sdk-foundation
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── client.rs              # Client、ClientBuilder、execute
│   │   │   ├── config.rs              # 客户端级不可变配置
│   │   │   ├── operation.rs           # Operation、操作描述与请求选项
│   │   │   ├── response.rs            # ApiResponse、ResponseMetadata
│   │   │   ├── error.rs
│   │   │   ├── common.rs              # 动态 Action 与 JSON/二进制调用
│   │   │   ├── endpoint.rs            # URL 解析、端点策略
│   │   │   ├── credentials/
│   │   │   │   ├── mod.rs             # Credentials、ProvideCredentials
│   │   │   │   ├── static_provider.rs
│   │   │   │   ├── environment.rs
│   │   │   │   ├── profile.rs
│   │   │   │   ├── chain.rs
│   │   │   │   ├── cache.rs           # 临时凭证缓存、提前刷新、单次刷新协调
│   │   │   │   ├── sts.rs
│   │   │   │   ├── cvm_role.rs
│   │   │   │   └── oidc.rs
│   │   │   ├── signing/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── tc3.rs             # 默认签名
│   │   │   │   └── legacy.rs          # 可选旧版签名
│   │   │   ├── protocol/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── headers.rs         # 公共参数与受保护 Header
│   │   │   │   ├── json.rs            # 请求编码、统一响应外层
│   │   │   │   ├── query.rs
│   │   │   │   └── multipart.rs
│   │   │   ├── transport/
│   │   │   │   ├── mod.rs             # Transport、传输请求和响应
│   │   │   │   ├── reqwest.rs         # 默认 HTTP 实现
│   │   │   │   └── body.rs            # 字节体、响应字节流
│   │   │   ├── retry.rs
│   │   │   ├── circuit_breaker.rs
│   │   │   ├── pagination.rs
│   │   │   ├── stream/
│   │   │   │   ├── mod.rs
│   │   │   │   └── sse.rs
│   │   │   ├── observability.rs
│   │   │   └── runtime.rs            # 内部 Clock、Sleep、随机源注入
│   │   └── tests/
│   │       ├── signing.rs
│   │       ├── protocol.rs
│   │       ├── credentials.rs
│   │       ├── retry.rs
│   │       ├── pagination.rs
│   │       └── sse.rs
│   ├── cvm/                          # tencentcloud-sdk-cvm
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   └── v20170312/
│   │   │       ├── mod.rs
│   │   │       ├── client.rs
│   │   │       ├── models.rs          # 汇总导出，不集中存放所有结构体
│   │   │       ├── models/
│   │   │       │   ├── describe_instances.rs
│   │   │       │   ├── instance.rs
│   │   │       │   └── filter.rs
│   │   │       ├── operations.rs      # Action 与能力描述
│   │   │       └── pagination.rs      # 有明确元数据的分页封装
│   │   ├── examples/
│   │   └── tests/
│   ├── cynosdb/                      # 其他产品沿用相同结构
│   └── postgres/
├── tools/
│   └── codegen/                      # 不发布的 Rust 生成工具
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           ├── input.rs              # 上游元数据导入
│           ├── ir.rs                 # 统一中间表示
│           ├── normalize.rs
│           ├── validate.rs
│           └── emit/
│               ├── mod.rs
│               ├── models.rs
│               ├── client.rs
│               └── pagination.rs
├── metadata/
│   ├── manifest.json                 # 来源、版本、校验摘要
│   ├── services/                     # 归一化后的产品元数据
│   └── overrides/                    # 可追溯的人工补充
├── tests/
│   ├── fixtures/                     # 固定签名、响应、SSE 样例
│   ├── support/                      # MockTransport 等共享源码
│   └── live/                         # 独立、非 workspace 默认成员
└── rustfmt.toml
```

目录树描述模块归属；文件按实际复杂度拆分，不要求为每个小类型创建文件。共享测试辅助代码通过测试模块引用，不形成对外发布的依赖。

依赖方向：

```text
应用 ──→ 产品 crate ──→ core ──→ HTTP / 序列化 / 密码学基础库
  └──────────────────→ core（通用调用）

codegen ──读取元数据──→ 生成产品 crate
```

- core 不依赖产品 crate；产品之间不互相依赖。
- STS 凭证提供器在 core 内用最小的私有请求/响应结构执行 STS 操作，复用通用协议链路，不反向依赖 sts 产品 crate。
- 产品 crate 的内部模型不跨产品共享，避免字段碰巧相同导致版本耦合。
- 根目录使用 virtual workspace，不提供默认依赖全部产品的聚合包。

## 3. 产品与版本组织

每个产品一个 crate，每个 API 版本一个模块，例如 `tencentcloud_sdk_cvm::v20170312`。

- 不提供隐含变化的 `latest` 模块或版本别名。
- 单版本产品默认启用该版本；多版本产品通过 Cargo feature 选择，默认版本固定并记录在文档中。
- 每个产品版本的 Client 固定 service 和 API version；endpoint、凭证和 Region 从 core Client 取得。
- 一个 core Client 可以被多个产品 Client 复用。共享连接池和凭证缓存，不共享可变的“当前产品”。
- 产品方法使用 Rust snake_case，线上 Action 保持官方大小写。
- 所有生成代码标注来源摘要和生成器版本；修改通过元数据或 overrides 完成，不直接修改生成文件。

## 4. 公开接口模型

### 4.1 核心类型

| 类型 | 职责 |
|---|---|
| `Client` / `ClientBuilder` | 不可变配置、共享资源、统一执行请求 |
| `ProvideCredentials` | 异步取得当前有效的凭证快照 |
| `Credentials` | 一致的 SecretId、SecretKey、Token、过期时间 |
| `Transport` | 发送已签名 HTTP 请求，返回状态、Header 和字节流 |
| `Operation` | 绑定输入、输出、service、version、Action 和编码方式 |
| `RequestOptions` | 单次调用的 Region、超时、语言、扩展字段和 Header |
| `ApiResponse<T>` | 业务数据 `data` 与统一响应元数据 `metadata` |
| `SdkError` | 本地配置、凭证、签名、传输、协议和服务端错误 |
| `CommonClient` | 动态指定 service、version、Action 的调用入口 |
| `EventStream<T>` | 可取消的异步流式响应 |

`Operation` 是静态分派的请求描述，产品 Client 不使用反射发现方法。`Transport` 和 `ProvideCredentials` 是可动态替换的窄接口，要求 `Send + Sync`；返回 Send 的 boxed future，使 core Client 无须暴露复杂泛型，也不必引入 async-trait。

### 4.2 使用形式

以下是公开接口形态示例，不是已经实现的代码：

```rust
use tencentcloud_sdk_core::Client;
use tencentcloud_sdk_cvm::v20170312::{Client as CvmClient, models::DescribeInstancesRequest};

let core = Client::builder()
    .region("ap-shanghai")
    .credentials_from_env()
    .build()?;

let cvm = CvmClient::new(core);
let response = cvm
    .describe_instances(&DescribeInstancesRequest {
        limit: Some(20),
        ..Default::default()
    })
    .await?;

println!("{:?}", response.data.total_count);
println!("{:?}", response.metadata.request_id);
```

普通方法提供默认请求选项；同一操作另提供 `_with_options` 方法接收 `RequestOptions`，不要求用户为一次调用修改全局 Client。

通用调用显式指定 service、version、Action 和 JSON 请求体。原始 JSON 返回和指定类型反序列化共用响应校验；二进制与 multipart 调用使用独立的 body 类型，不能把任意字节错误转换成 JSON。

### 4.3 数据模型

- 请求和响应使用 `serde`，Rust 字段显式映射到官方字段名。
- 请求可选字段使用 `Option<T>`，`None` 默认不发送；必填字段不因方便构造而全部变为 Option。
- 只有必填字段为空的请求实现 `Default`。需要必填值时生成构造函数，避免默认构造掩盖缺失参数。
- 响应字段依据文档的缺省和可空语义建模；不能因为请求字段必填就推导响应也必填。
- 对协议确实区分缺省与显式 null 的请求字段，使用三态字段类型；普通字段不承担这个复杂度。
- 整数依照范围区分 `i64` 和 `u64`，不能仅凭 Java 的 Long 自动判定符号；金额和精度敏感字段保留官方字符串/精确表示，不擅自转换为浮点数。
- 响应默认容忍未知字段；字符串枚举保留未知值，不因服务端新增值而解析失败。
- 请求额外字段通过 `RequestOptions` 提供；与已声明字段或公共参数冲突时返回本地错误，不静默覆盖。
- RequestId 统一进入 `ResponseMetadata`；嵌套业务结构中的同名字段仍按官方模型保留。
- 分页参数不提供擅自猜测的默认值；纯本地校验只覆盖稳定的协议约束和明确约束，不复制复杂服务端业务判断。

## 5. 统一请求执行链路

```text
产品方法 / CommonClient
  → 解析 Operation 和 RequestOptions
  → 确定 service、version、Region、endpoint、deadline
  → 校验字段和 Header 冲突
  → 编码为可重放的请求体
  → 执行尝试循环：
      读取一致的有效凭证快照
      → 获取当前时间，构造公共 Header
      → 对实际发送的 URL、Header、字节体签名
      → Transport 发送
      → 解析 HTTP 状态、Content-Type、腾讯云响应外层
      → 返回响应，或根据显式策略决定是否重试
  → ApiResponse<T> / 流式响应 / SdkError
```

### 5.1 签名和协议

默认 `TC3-HMAC-SHA256 + HTTPS + POST JSON`。旧版签名通过 `legacy-signing` feature 开启，不作为默认依赖。

- 签名层输入包含确定的时间、凭证、service、URI、query、Header 和 body 字节，不自行读取环境或发起网络请求。
- JSON 编码完成后直接对同一份字节签名和发送，禁止二次序列化导致签名不一致。
- TC3 的 service 由操作描述指定，不从自定义代理域名猜测。
- Host 和 canonical URI 按实际协议要求构造，自定义 endpoint 必须重新签名。
- GET query 编码、旧版表单、multipart boundary 和二进制 body 使用独立编码器。
- multipart boundary 和时钟由内部可注入组件提供，测试可固定。
- `UNSIGNED-PAYLOAD` 仅作为显式配置，按官方协议和接口能力校验，不自动启用。
- 公共 Header 如 Authorization、Host、X-TC-Action、X-TC-Version、X-TC-Timestamp、X-TC-Token 不能通过普通扩展 Header 覆盖。TraceId 等非冲突 Header 可以设置。
- 不开放通用 `skip_sign` 开关；匿名操作只有在官方元数据明确声明时才由操作描述选择认证模式。

### 5.2 Endpoint 与 HTTP

默认 endpoint 为 `<service>.tencentcloudapi.com`，Region 与 endpoint 分开建模。Region 是 API 参数，端点切换不隐含跨地域执行资源操作。

- Client 支持默认端点策略和按 service 配置的显式 endpoint；避免复用 core Client 时把 CVM 的域名误用于数据库产品。
- 保留官方网关场景的显式路由配置，区分连接目标与签名 authority，不通过任意 Header 拼装模拟。
- 自动重定向关闭，避免 Authorization 和 Token 被转发至其他目标。
- 默认 TLS 证书校验开启，支持自定义根证书与代理；不提供一键关闭证书校验的便利方法。
- HTTP 仅允许通过明确的本地测试/私有环境配置启用，不允许默认凭证链向任意明文公网地址发送凭证。
- 默认连接超时 10 秒，普通请求总超时 60 秒；允许覆盖。SSE 使用单独的建连和空闲超时，不套用普通响应的总时长限制。
- 普通响应默认限制 32 MiB，可配置；SSE 限制单事件与缓冲区大小，不限制整个流累计长度。
- 请求超时覆盖凭证解析、发送和退避等待。取消 future 或丢弃流会释放当前传输，不启动脱离调用生命周期的业务请求。

## 6. 凭证管理

`Credentials` 是不可变快照，调试输出对 SecretId、SecretKey、Token 全部脱敏。整个签名过程使用同一个快照。

提供器包括静态凭证、环境变量、配置文件、STS AssumeRole、CVM 实例角色、OIDC，以及显式构建的提供器链。

- `credentials_from_env()` 只读取环境变量，不隐含联网。
- `default_credentials()` 使用环境变量 → 配置文件 → CVM 实例角色 → OIDC 的默认顺序，并明确记录元数据访问行为；可显式禁用联网提供器或自定义链。
- “未配置”允许继续探测下一个提供器；“已配置但无效/访问失败”返回对应错误，不静默换用另一个身份。
- 环境变量和配置文件支持临时 Token；配置文件可选择 profile，解析错误不输出凭证原文。
- STS 使用显式来源凭证，不递归调用默认凭证链；OIDC 和 CVM 获取流程有独立超时。
- CVM 元数据访问使用专门的传输配置和固定官方目标，不继承公共代理及任意 endpoint 覆盖。
- 临时凭证按服务端到期时间提前刷新，例如提前 5 分钟；短生命周期凭证按剩余寿命比例计算刷新窗口。
- 同一提供器并发只执行一次刷新。异步互斥只保护刷新路径，快照读取不持有阻塞锁跨越网络 await。
- 提前刷新失败时，仅在旧凭证仍有效的条件下允许使用旧快照，并短暂抑制重复刷新；过期后必须返回错误，不能继续签名。

## 7. 错误、重试和容灾

### 7.1 错误模型

`SdkError` 区分：`Configuration`、`Credentials`、`Signing`、`Transport`、`Timeout`、`Serialization`、`Protocol` 和 `Service`。使用非穷尽枚举保留扩展空间。

服务端错误至少保留：

- 官方 error code 和 message。
- RequestId。
- HTTP status。
- 重试相关响应 Header 的安全子集。

HTTP 非 200 时也尝试解析腾讯云错误外层；无法解析时返回携带状态码和可用 RequestId 的协议/传输错误，而不是丢弃所有上下文。原始响应体不默认进入错误字符串、Debug 或日志。

成功响应缺失必要外层、JSON 损坏、Content-Type 不支持均返回协议错误，不 panic。RequestId 优先取协议约定位置，必要时从响应 Header 补充。

### 7.2 重试策略

默认 `max_attempts = 1`，即关闭 SDK 层自动重试。底层 HTTP 实现也必须关闭自动请求重放，防止绕过 SDK 的幂等控制。

用户显式开启后，重试仍同时满足：

1. 操作在人工审核的安全元数据中明确允许，或调用方显式声明接受重复执行风险；未知操作不自动重试。
2. 失败属于允许的网络暂态、限流或指定服务端错误，认证/签名/参数错误直接返回。
3. body 可重放，且未超过最大尝试次数和总 deadline。

退避采用有上限的指数退避和 full jitter，优先遵守有效的 Retry-After。每次尝试重新取时间和签名，必要时重新取凭证。购买、创建、删除等操作不能因名字或 HTTP POST/GET 自动推导安全性。ClientToken 只有在官方接口明确支持时才使用，不由 SDK 生成一个 Header 假装实现幂等。

### 7.3 熔断和备用端点

默认关闭，用户必须提供经过确认的备用 endpoint。按 service 与 endpoint 维护健康状态，不把业务参数错误记作端点故障。

端点选择与请求重试是两个机制：熔断可以影响下一次请求的目标，但不能授权当前非幂等请求自动重放。切换连接入口时保留原始 Region；跨 Region 的资源操作由应用显式决定。

## 8. SSE、分页和状态等待

### 8.1 SSE

SSE 是独立流式执行入口，例如生成 `chat_completions_stream`，避免一个普通方法动态返回两种不相关类型。

- 以 HTTP 媒体类型解析结果识别 SSE，允许 `text/event-stream; charset=utf-8`，不作字符串完全相等比较。
- 解析支持跨网络分块、UTF-8 边界、CRLF、多行 data、注释、空 data 和无冒号字段。
- 通用入口返回原始 SSE 事件；产品适配层按官方约定解析 JSON、结束标记和错误事件。
- `EventStream<T>` 持有响应体，按需读取，依靠背压控制缓冲；提供消费前可读取的响应元数据。
- SSE 事件开始交付后不自动重连，不因为 `retry` 字段自动重放云 API 请求。

### 8.2 分页

分页器依赖操作元数据明确描述，不从字段名猜测。

支持 Offset/Limit、PageNumber/PageSize 和 Token/Cursor 三类策略。统一提供页级异步接口，业务项扁平化是显式的第二层便利接口。

分页状态包含下一页位置、页大小和停止条件；停止条件使用官方定义的 token、total 或结束标记，仅在协议明确时才使用“少于页大小”。检测重复 token、无进展位置和最大页数，防止无限循环。每页保留自己的 RequestId；某页失败后返回错误，不静默跳过，也不在下次调用时越过失败页。

### 8.3 状态等待

不在通用内核内定义业务状态名。产品若提供 waiter，必须有明确元数据描述查询操作、成功状态、失败状态和 deadline。等待器复用查询与退避能力，但不自动创建、修复或回滚资源。

## 9. 代码生成

生成链路：

```text
官方 API 元数据 / 经核对的 Java 模型信息
  → 导入适配
  → 统一 IR
  → 校验与 overrides
  → Rust Client / 模型 / 操作描述 / 分页器 / 文档
  → rustfmt 与离线测试
```

IR 显式描述 service、API version、Action、认证方式、字段名、类型、必填/缺省/可空、编码方式和响应模式。安全重试、分页、流式能力等元数据不能可靠取得时，采用保守默认值，不以启发式推断开放行为。

官方元数据源通过独立 input 适配器接入，不假定 Java 仓库包含生成器或完整 schema。Java 文档与类型作为导入参考，人工核对的补充保存在 overrides，记录依据与适用版本。

- 生成过程确定化：固定排序，不写每次变化的时间戳。
- Rust 关键字、命名冲突和类型重名统一处理，线上字段名不受重命名影响。
- 生成文档保留官方字段说明和 API 文档链接。
- 不在 build.rs、cargo build 或测试中联网更新模型；生成文件随仓库提交。
- 产品 crate 版本遵循 SemVer；腾讯云 API 日期版本与 crate 发布版本是两个不同维度。
- 新增必填字段、字段类型变化、已有方法签名变化按破坏性变更处理；不能把上游模型更新一概视为 patch。

## 10. 依赖与 feature

核心依赖保持小而明确：

| 用途 | 选择 |
|---|---|
| HTTP 与连接池 | reqwest，关闭其默认 feature，仅开启所需 TLS/流式能力 |
| 异步运行时支持 | Tokio，仅使用必要模块，不创建 Runtime |
| 模型与 JSON | serde、serde_json |
| TC3 密码学 | hmac、sha2 |
| HTTP 公共类型、字节体 | http、bytes |
| 流接口 | futures-core；额外组合工具仅在需要处使用 |
| UTC 日期 | time，仅开启所需能力 |
| 可选日志 | tracing，不安装全局 subscriber |

不为简单十六进制编码、有限 INI 配置解析或 boxed future 单独增加依赖；不自行实现 HMAC/SHA256 等密码算法。旧版签名按需引入 SHA1/Base64 相关依赖。

core 默认启用 reqwest 传输、Rustls、TC3 和普通 JSON 调用。建议 feature：`reqwest-transport`、`legacy-signing`、`sse`、`tracing`。静态/环境/配置凭证属于基础能力，联网凭证提供器按使用调用，不因创建 Client 就访问网络。

Rust edition 使用 2024；最低 Rust 版本不低于 1.85，并在锁定实际依赖版本后以最低版本构建测试确认。依赖在 workspace 中统一管理，不在设计文档中猜测当前最新版本。

## 11. 测试和可观测性

默认 `cargo test --workspace` 只执行本地测试；live 测试作为独立且被 workspace 排除的包，只有显式进入并运行才可能访问真实接口。产品 examples 不纳入默认真实请求测试。

| 层级 | 重点 |
|---|---|
| 签名固定样例 | TC3 canonical request、UTC 日期边界、query 编码、临时 Token |
| 编解码 | 嵌套数组、可选/null、额外字段冲突、未知响应字段、整数范围 |
| HTTP 协议 | 非 200 的结构化错误、无效 JSON、响应体限制、Header 保护 |
| 凭证 | 来源链、配置错误、并发单次刷新、刷新失败与过期 |
| 重试 | 最大尝试数、deadline、jitter、Retry-After、未知操作不重试 |
| SSE | 任意分块、UTF-8、空字段、多行 data、取消、流中错误 |
| 分页 | 三种分页策略、终止条件、重复 token、失败页不跳过 |
| 产品封装 | Action/Version/Service 正确，模型映射与官方固定样例一致 |
| 生成器 | IR 校验、命名冲突、输出快照、重复生成无差异 |

MockTransport 默认拦截全部网络请求，未匹配请求直接失败。只有传输适配器测试使用本地回环 HTTP 服务。时间、sleep 和随机源均可替换，不靠真实等待测试重试或凭证过期。

日志默认不记录请求/响应 body、Authorization、Token、SecretId、代理凭证或完整查询串。可观测字段限定为 service、Action、API version、Region、RequestId、状态、耗时、尝试次数和安全错误码。服务端 message 可能含业务数据，不作为默认日志字段。SDK 不启动 metrics 服务，不安装日志全局配置，不默认上报遥测。
