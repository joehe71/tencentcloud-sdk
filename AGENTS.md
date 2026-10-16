# 空间工作规则

## 代码语言

- Code comments, Rust documentation comments, package descriptions, and SDK-generated diagnostic messages must be written in English. Preserve official service messages and protocol values verbatim; existing Chinese design documents may remain in Chinese.

## 用途与边界

- 本空间用于开发腾讯云 SDK：编写、维护和测试腾讯云接口封装，不作为日常云资源运维入口。
- 以腾讯云官方 API 文档为准，不将具体业务流程塞进 SDK。
- 代码设计与实现始终以 Rust 语言规范和惯用法为准；Java SDK 仅作协议、字段和功能参考，不照搬其类层次、反射、getter/setter 或同步调用结构。

## 测试与真实接口

- 默认仅运行本地测试，不调用真实腾讯云接口。
- 使用 Mock、固定样例或本地模拟服务开展测试，测试凭证使用假值。
- 即使环境中已有腾讯云凭证，也不视为真实接口调用授权。
- 真实接口联调必须由用户另行明确授权；涉及资源变更或费用的操作，必须明确确认。
- 重点覆盖签名、请求序列化、响应解析、错误映射和分页；时间、随机数等影响测试的因素应可控。

## 凭证安全

- 不硬编码密钥，不将凭证写入日志、测试样例或 Git。

## 接口与错误处理

- 修改公开接口、默认行为或依赖版本前，先说明影响，不顺手做破坏性重构。
- 保留腾讯云错误码和 RequestId。
- 重试策略必须明确，不默认重试可能产生重复副作用的请求。

## 改动与验证

- 优先复用现有实现，尽量少引入第三方依赖。
- 不夹带无关重构，不擅自提交或推送 Git 改动。
- 完成后说明改动内容、实际运行的测试以及尚未验证的部分。
- 不将本地测试通过表述为真实接口验证通过。
