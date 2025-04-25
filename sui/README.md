# 🚀 Sui 合约开发入门指南

本指南将带你从零开始构建 Sui Move 合约，适用于希望快速上手 Sui 智能合约开发的开发者。

---

## 📖 目录

- [1. 📦 项目结构说明](#1-项目结构说明)
- [2. ✨ 快速开始](#2-快速开始)
- [3. 🧪 测试合约](#3-测试合约)
- [4. 🔗 后续阅读建议](#4-🔗-后续阅读建议)
- [5. 📘 示例模块：NFT 合约](#5-示例模块nft-合约)
- [6. 📜 脚本调用示例（TypeScript）](#6-脚本调用示例typescript)

---

## 1. 📦 项目结构说明

```
sui/
├── move.toml                 # Move 包配置文件
├── sources/                  # Move 合约源代码
│   ├── my_module.move
│   └── ...
├── tests/                    # Move 合约单元测试
├── examples/                 # 合约使用示例（如 NFT、DAO 等模块）
│   ├── nft.move              # 示例：简单 NFT 模块
│   └── README.md             # 使用说明
├── scripts/                  # 链下调用脚本（TypeScript）
│   ├── mint-nft.ts           # 示例：NFT 铸造
│   └── init.ts               # 示例：初始化配置调用
└── sui.devnet.key            # 开发账户密钥（可选）
```

---

## 2. ✨ 快速开始

1. 安装依赖：
   ```bash
   brew install sui
   ```

2. 初始化项目：
   ```bash
   sui move init --name my_project
   ```

3. 编译合约：
   ```bash
   sui move build
   ```

4. 发布合约到 devnet：
   ```bash
   sui client publish --gas-budget 100000000
   ```

---

## 3. 🧪 测试合约

编写 `tests/xxx.move` 文件后：

```bash
sui move test
```

---

## 4. 🔗 后续阅读建议

本文件仅覆盖 Sui 合约开发的基本流程。若希望深入了解模块架构设计、跨模块调用、安全性及链下交互等高级主题，请参考 [README-ADVANCED.md](./README-ADVANCED.md)。

---

## 5. 📘 示例模块：NFT 合约

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

## 6. 📜 脚本调用示例（TypeScript）

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
