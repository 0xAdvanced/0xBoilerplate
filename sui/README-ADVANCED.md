# 🧠 Sui Move 合约进阶指南

---

## 📖 目录

- [1. 📚 模块组织与设计范式](#1-模块组织与设计范式)
- [2. 🧮 Gas 优化技巧](#2-gas-优化技巧)
- [3. 🔐 权限与安全模式](#3-权限与安全模式)
- [4. 🛠 链下脚本与交互实践（TypeScript）](#4-链下脚本与交互实践typescript)
- [5. 🔁 多签控制机制](#5-多签控制机制)
- [6. ⚙️ 工程化与 CI/CD 实践](#6-工程化与-cicd-实践)
- [7. 📘 示例模块：NFT 合约](#7-示例模块nft-合约)
- [8. 📜 脚本调用示例（TypeScript）](#8-脚本调用示例typescript)

---

## 1. 📚 模块组织与设计范式

- 单模块 vs 多模块拆分
- 使用 `friend` 关键字做内部模块协作
- `has key`, `store`, `drop` 权限管理

---

## 2. 🧮 Gas 优化技巧

- 减少过多嵌套与循环调用
- 避免使用大数据结构（如 vector）频繁修改
- 用事件代替资源写入进行链上记录

---

## 3. 🔐 权限与安全模式

- 使用 `ObjectID` 管理资源所有权
- 自定义 `AdminCap` / `OwnerCap` 控制权限迁移
- 防止重放攻击与 `object reuse`

---

## 4. 🛠 链下脚本与交互实践（TypeScript）

- 使用 Sui TypeScript SDK（`@mysten/sui.js`）
- 脚本目录结构推荐：
  ```
  scripts/
  ├── publish.ts
  ├── mint-nft.ts
  ├── vote-proposal.ts
  └── utils/
  ```
- 支持 `.env` 读取私钥与节点地址
- 配合 yargs 构建 CLI 工具脚本

---

## 5. 🔁 多签控制机制

Sui 中暂无内建多签模块，但可通过以下方式构建自定义多签逻辑：

- 自定义 `MultisigCap` 资源，记录签名列表
- 基于资源状态判断是否达到阈值后执行动作
- 配合事件记录每次签名与操作轨迹

---

## 6. ⚙️ 工程化与 CI/CD 实践

- 测试使用 `sui move test`
- CI 推荐 GitHub Actions:
  ```yaml
  steps:
    - run: sui move build && sui move test
  ```

- 推荐补充：
  - 文档自动化：使用 DocC 或自定义解析器导出模块说明
  - 集成 CLI 脚本自动部署与调用
  - 使用本地 devnet 镜像做集成测试

---

> 持续打磨 Move 合约工程架构是高质量项目的核心竞争力，建议结合团队实际构建模块化、可复用的 Move 开发体系。

---

项目结构说明更新：

```
├── examples/                 # 合约使用示例（如 NFT、DAO 等模块）
│   ├── nft.move              # 示例：简单 NFT 模块
│   └── README.md             # 使用说明
├── scripts/                  # 链下调用脚本（TypeScript）
│   ├── mint-nft.ts           # 示例：NFT 铸造
│   └── init.ts               # 示例：初始化配置调用
```

---

## 7. 📘 示例模块：NFT 合约

为了帮助开发者理解资源定义与合约函数调用的模式，我们提供一个简单的 NFT 合约示例，位于 `examples/nft.move`，主要功能包括：

- 定义 `SimpleNFT` 资源结构，含有名称与 URI；
- 实现 `mint_nft` 方法：可公开创建一个新的 NFT 资源；
- 演示 `object::new` 与 `transfer::public_share_object` 的典型用法。

示例部署步骤：

```bash
sui move build
sui client publish --gas-budget 100000000
```

---

## 8. 📜 脚本调用示例（TypeScript）

项目中也包含了配套的链下脚本，用于合约交互与部署测试。位于 `scripts/` 文件夹中：

### `mint-nft.ts`

```ts
import { SuiClient, getFullnodeUrl } from '@mysten/sui.js/client';
import { Ed25519Keypair } from '@mysten/sui.js/keypairs';
import dotenv from 'dotenv';
dotenv.config();

const keypair = Ed25519Keypair.fromSecretKey(
  Uint8Array.from(Buffer.from(process.env.SUI_SECRET!, 'base64'))
);
const client = new SuiClient({ url: getFullnodeUrl('devnet') });

async function main() {
  const tx = {
    packageObjectId: '0xNFT_MODULE_PACKAGE_ID',
    module: 'nft',
    function: 'mint_nft',
    arguments: ['My NFT', 'https://img.example.com/nft.png'],
    gasBudget: 100000000,
    signer: keypair,
  };
  const result = await client.call(tx);
  console.log('NFT 已铸造：', result);
}

main();
```

> 替换其中 `0xNFT_MODULE_PACKAGE_ID` 为你的合约包地址

### `init.ts`

```ts
// 示例：初始化配置资源
// TODO：根据你的合约初始化函数定义进行补充调用逻辑
```

---

> 下一步建议拓展方向：加入 DAO 模块示例、多签执行流程模拟、复杂资源迁移机制等，逐步完善 Sui 合约实战知识体系。
