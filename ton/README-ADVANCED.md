# 🧠 TON 合约开发进阶指南

---

## 📖 目录

- [1. 📚 架构与设计模式](#1-架构与设计模式)
- [2. 🔁 消息序列化与交互范式](#2-消息序列化与交互范式)
- [3. 🔍 安全建议与常见陷阱](#3-安全建议与常见陷阱)
- [4. 🔐 多签机制设计](#4-多签机制设计)
- [5. 🧩 与链上钱包或 Jetton 的集成建议](#5-与链上钱包或-jetton-的集成建议)
- [6. 🛠 链下调用脚本结构（TypeScript）](#6-链下调用脚本结构typescipt)
- [7. 🧪 单元测试与模拟环境](#7-单元测试与模拟环境)
- [8. 🎛 Gas 成本与优化技巧](#8-gas-成本与优化技巧)
- [9. ⚙️ 工程化建议](#9-工程化建议)
- [10. 🔄 合约升级机制](#10-合约升级机制)

---

## 1. 📚 架构与设计模式

### FunC 模块结构推荐

- 使用 `lib.fc` 拆分工具函数
- 避免单文件千行，推荐将逻辑划分为操作型函数（`op_` 前缀）与状态处理函数
- 状态布局推荐使用 `slice`+`builder` 分层组织数据

### Tact 项目结构建议

- 使用 `struct` 定义资源模型
- 使用 `contract` 包装接口函数，自动生成 ABI 与客户端类型
- 利用 `extends` 实现继承和接口组合

---

## 2. 🔁 消息序列化与交互范式

TON 使用 Cell 作为消息载体，关键在于：

- `builder.storeUint/storeRef` 构建 Cell
- `slice.loadUint/loadRef` 解析 Cell
- 所有外部消息必须满足 ABI（特别是 `op` 编码对齐）

可借助 `@ton/core` 中的 `beginCell()` 工具类快速构造序列化数据：

```ts
import { beginCell } from '@ton/core';

const body = beginCell()
  .storeUint(0x1234abcd, 32) // op code
  .storeUint(1000, 64)       // amount
  .endCell();
```

---

## 3. 🔍 安全建议与常见陷阱

- 所有 Cell 构造需确保 ABI 编码一致，避免偏移错乱；
- 防止重入型消息调用（特别是 token 回调）；
- 使用 op code 签名验证调用者身份；
- 合约入口必须判定 `msg.sender`, `msg.value`, `msg.body` 等字段合法性。

---

## 4. 🔐 多签机制设计

TON 无内置多签，但可通过自定义逻辑实现“链上签名人状态+阈值判断”：

- 使用 `map<address, bool>` 表示已签名者集合
- 存储 `threshold: int` 与 `signedCount: int`
- 每次外部调用附带签名地址，合约验证后更新状态
- 达到 `threshold` 才执行真正逻辑（如转账）

可参考 TonSafe 多签钱包开源逻辑，或构建 DAO 模块自行实现。

---

## 5. 🧩 与链上钱包或 Jetton 的集成建议

- 钱包交互建议使用官方钱包 ABI 或 TON Connect 协议；
- 与 Jetton 兼容的合约需要正确实现 `receive_transfer`, `get_wallet_data` 等接口；
- 推荐封装 token 调用函数为工具模块，统一构造转账消息。

---

## 6. 🛠 链下调用脚本结构（TypeScript）

推荐结构：

```
scripts/
├── deploy.ts           # 编译 & 初始化合约
├── call.ts             # 执行合约交互
├── multisig.ts         # 模拟多签投票与执行
└── utils/              # 工具类与消息构造器
```

脚本可使用 `@ton/core`, `@ton/cli`, `dotenv` 配置环境变量，并结合 `ton-http-client` 访问主网或 testnet。

---

## 7. 🧪 单元测试与模拟环境

Tact 提供内建测试框架，可直接编写 `.spec.ts` 或 `.spec.tact` 文件实现模拟交互测试：

```ts
import { createSandbox, expect } from '@tact-lang/testkit';
import { MyContract } from '../build/MyContract';

test('should increment counter', async () => {
  const sandbox = await createSandbox();
  const contract = await sandbox.deploy(MyContract);
  await contract.increment();
  const state = await contract.getState();
  expect(state.count).toEqual(1);
});
```

> `@tact-lang/testkit` 支持 snapshot 测试、事件断言、Gas 测量等，适合进行 CI 集成。

---

## 8. 🎛 Gas 成本与优化技巧

- 使用 `optional`, `slice` 等类型减少链上空间占用；
- 合理压缩链上状态，避免冗余存储；
- 避免在合约内做复杂数学运算，复杂逻辑应尽量链下执行；
- `cell_depth_limit`, `cell_bit_limit` 是写复杂逻辑时的重要限制参数。

---

## 9. ⚙️ 工程化建议

- 使用 `.env` 管理合约地址、私钥、RPC 节点
- 将 Cell 序列化与解析逻辑拆分为工具库
- Tact 支持模拟器测试 (`tact test`) 与合约类型检查
- GitHub Actions 可执行自动构建与部署发布

---

> TON 开发强调“极致低成本”与“链上数据压缩能力”，建议在写合约前先明确数据布局、交互方式与 gas 预算。合理使用 Tact 可大幅减少编码复杂度与序列化冗余。

---

## 10. 🔄 合约升级机制

TON 合约不可直接升级代码，但可采用：

1. Proxy 模式：通过一个中转合约转发消息至实际逻辑；
2. 状态迁移：部署新合约后，通过 admin 权限迁移旧状态并冻结旧合约；
3. DNS 指针：通过链上 DNS 合约记录指向可升级逻辑的地址。

---

> 建议高复杂合约设计前优先绘制状态机图与消息时序图，以便统一协作、规范安全边界并降低 gas 消耗。