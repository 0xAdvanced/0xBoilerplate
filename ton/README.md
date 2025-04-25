

# 🚀 TON 合约开发入门指南（FunC & Tact）

---

## 📖 目录

- [1. 📦 项目结构说明](#1-项目结构说明)
- [2. 🛠 安装依赖](#2-安装依赖)
- [3. ✨ 合约构建与部署](#3-合约构建与部署)
- [4. 🔗 合约调用示例](#4-合约调用示例)
- [5. 🧪 合约测试](#5-合约测试)
- [6. 🔗 高级阅读推荐](#6-高级阅读推荐)

---

本指南涵盖使用 FunC / Tact 编写、部署和交互 TON 智能合约的入门知识，适用于刚接触 TON 智能合约开发的工程师。

---

## 1. 📦 项目结构说明

```
ton/
├── contracts/               # FunC / Tact 合约源代码
│   ├── counter.fc
│   └── my_contract.tact
├── build/                   # 编译输出
├── scripts/                 # 链下交互脚本（TS / bash）
│   ├── deploy.ts
│   └── call.ts
├── test/                    # 单元测试
├── .env                     # 密钥与 RPC 配置
├── ton-core.config.ts       # Ton-Access 网络配置（可选）
└── package.json             # 项目依赖与脚本命令
```

---

## 2. 🛠 安装依赖

```bash
npm install
```

依赖包含：

- `@ton/core`, `@ton/cli`: 合约编译与链交互
- `tact-lang`: 若使用 Tact 编写合约
- `dotenv`: 管理环境变量

---

## 3. ✨ 合约构建与部署

以 `counter.fc` 为例：

1. 编译：
   ```bash
   npx func -o build/counter.cell contracts/counter.fc
   ```

2. 部署脚本：
   ```bash
   ts-node scripts/deploy.ts
   ```

---

## 4. 🔗 合约调用示例

以 `call.ts` 为例，通过 `@ton/core` 调用已部署合约函数，获取或设置链上状态。

---

## 5. 🧪 合约测试

可结合 `jest` 或 `mocha` 编写测试，或使用 Tact 自带模拟环境。

---

## 6. 🔗 高级阅读推荐

请参考 [README-ADVANCED.md](./README-ADVANCED.md)，深入了解 TON 的多签模式、链上钱包交互、安全模型、消息格式与序列化细节等。