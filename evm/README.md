# EVM 合约开发（Solidity）

本模块聚焦于以太坊虚拟机（EVM）兼容链的智能合约开发，主要使用 Solidity 编程语言，结合 Hardhat 或 Foundry 开发框架，适合初学者逐步深入学习，也方便进阶开发者快速原型开发和测试。

---

## 📖 目录

- [1. 📦 模块结构](#1-模块结构)
- [2. 🚀 快速开始](#2-快速开始)
- [3. 🧠 合约核心知识点](#3-合约核心知识点)
  - [3.1 ✅ 安全性设计](#31-安全性设计)
  - [3.2 ✅ 合约优化](#32-合约优化)
  - [3.3 ✅ 调试与测试](#33-调试与测试)
- [4. 📚 推荐学习示例](#4-推荐学习示例)
- [5. 🔗 实用资源](#5-实用资源)

---

## 1. 📦 模块结构

- `contracts/`：存放智能合约源码（如 ERC20、ERC721、自定义合约等）
- `scripts/`：合约部署和交互脚本（JavaScript 或 TypeScript）
- `tests/`：单元测试代码，使用 Hardhat（Mocha/Chai）或 Foundry 测试框架
- `deployments/`：合约部署配置和历史记录（可选）
- `hardhat.config.ts`：Hardhat 项目配置
- `foundry.toml`：Foundry 项目配置（若使用 Foundry）

---

## 2. 🚀 快速开始

以 Hardhat 为例：

```bash
npm install
npx hardhat compile
npx hardhat test
npx hardhat run scripts/deploy.ts --network goerli
```

---

## 3. 🧠 合约核心知识点

以下是你在合约开发过程中需要深入理解与掌握的重要模块与概念：

### 3.1 ✅ 安全性设计

- **重入攻击防御**：使用 Checks-Effects-Interactions 模式或 ReentrancyGuard
- **访问控制**：使用 `Ownable` / `AccessControl` 进行权限管理
- **整数溢出保护**：使用 `SafeMath`（Solidity ≥0.8 可内置处理）
- **合约升级机制**：使用 UUPS / Transparent Proxy（OpenZeppelin 提供实现）
- **拒绝服务攻击防范**：Gas 上限、fallback 处理、Loop 安全性

### 3.2 ✅ 合约优化

- 精简存储结构（e.g. 位运算打包布尔值）
- 减少 SLOAD/SSTORE 次数
- 使用 `immutable` 和 `constant` 减少部署成本
- 使用 `unchecked` 块节省 gas（注意安全性）

### 3.3 ✅ 调试与测试

- 使用 `console.log` (Hardhat) 或 `vm.debug` (Foundry)
- 多链测试环境模拟（Hardhat + Anvil）
- Mock 合约与 Fuzz Testing 技术

---

## 4. 📚 推荐学习示例

你可以根据以下例子，在 `examples/` 目录下建立子目录，每个示例都对应一个完整的工程模板（含合约、部署、测试）：

### 1. `erc20-token-basic/`

- 实现一个最小化的可转账代币
- 加入铸造/销毁功能
- 加入白名单转账逻辑

### 2. `nft-marketplace/`

- 实现 ERC721 代币
- 支持基础 NFT 市场上架、购买功能
- 加入版税 royalty 与支付分发

### 3. `multisig-wallet/`

- 多签钱包基本结构
- 动态签名阈值设定
- 支持任意合约调用与资产管理

### 4. `upgradable-dao/`

- Proxy 可升级架构
- 简单 DAO 提案 + 投票治理机制
- Token 权重投票模型

### 5. `onchain-games/`

- 编写一个 Rock-Paper-Scissors 游戏合约
- 使用链上状态记录、隐藏提交/揭示机制（commit-reveal）
- 增加下注与奖池结算逻辑

---

## 5. 🔗 实用资源

- [Solidity 官方文档](https://docs.soliditylang.org/)
- [OpenZeppelin Contracts](https://github.com/OpenZeppelin/openzeppelin-contracts)
- [Solidity Gas Optimization](https://github.com/ChilliBits/solidity-gas-optimization-handbook)
- [Hardhat 中文教程](https://learnblockchain.cn/docs/hardhat/)
- [Foundry Book](https://book.getfoundry.sh/)

---

📘 **想要进一步掌握合约升级、模块化架构、工程化与高级 EIP 技术？**

请继续阅读 [README-ADVANCED.md](./README-ADVANCED.md)，深入迈向 Solidity 架构设计师之路。

---

> 本目录既是开发模板，也是一套系统化的练习平台，适用于希望全面掌握 EVM 合约编写与优化的开发者。