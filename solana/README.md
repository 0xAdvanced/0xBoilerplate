# Solana 智能合约开发（Rust + Anchor）

---

## 📖 目录

- [1. 📦 模块结构](#1-模块结构)
- [2. 🚀 快速开始](#2-快速开始)
- [3. 🧠 合约核心知识点](#3-合约核心知识点)
  - [3.1 ✅ Solana 开发模型理解](#31-solana-开发模型理解)
  - [3.2 ✅ Anchor 开发要点](#32-anchor-开发要点)
- [4. 📚 推荐学习示例（可建于 examples/）](#4-推荐学习示例可建于-examples)
- [5. 🔗 实用资源](#5-实用资源)

---

## 1. 📦 模块结构

- `programs/`：Anchor 合约代码目录
- `tests/`：合约测试用例
- `scripts/`：部署与初始化脚本
- `migrations/`：部署过程配置
- `anchor.toml`：Anchor 项目配置文件
- 其他配置项如 `tsconfig.json`、`package.json`

---

## 2. 🚀 快速开始

1. 安装 Anchor CLI：

   ```bash
   cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
   ```

2. 初始化工程
   ```bash
   anchor init 工程名称
   ```

3. 编译合约：

   ```bash
   anchor build
   ```

4. 运行测试：

   ```bash
   anchor test
   ```

---

## 3. 🧠 合约核心知识点

### 3.1 ✅ Solana 开发模型理解

- 所有状态数据通过账户读取与写入，程序与状态完全解耦
- 使用 PDA（Program Derived Account）生成确定性账户地址
- 使用 CPI（跨程序调用）实现模块化合约交互
- 账户空间/租约模型与初始化成本机制

### 3.2 ✅ Anchor 开发要点

- `#[account]` 自动序列化数据结构
- `#[derive(Accounts)]` 实现账户验证与授权控制
- `#[constraint]` 宏定义用于账户权限与状态断言

---

## 4. 📚 推荐学习示例（可建于 `examples/`）

### 1. `basic-counter/`
- 简单计数器合约，管理单账户状态
- 位置：`examples/basic-counter/`
- 初始化：`ts-node scripts/init.ts`
- 调用示例：`ts-node scripts/increment.ts`

---

## 🧱 合约代码结构（Rust）

文件位置：`examples/basic-counter/programs/basic_counter/src/lib.rs`

```rust
use anchor_lang::prelude::*;

declare_id!("YourProgramIDHere11111111111111111111111111111");

#[program]
pub mod basic_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.counter.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        ctx.accounts.counter.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8, seeds = [b"counter", user.key().as_ref()], bump)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [b"counter", user.key().as_ref()], bump)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

#[account]
pub struct Counter {
    pub count: u64,
}
```

---

## 📜 TypeScript 脚本集成

文件路径：`examples/basic-counter/scripts/init.ts`

```ts
import * as anchor from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import dotenv from "dotenv";
dotenv.config();

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.BasicCounter;

async function main() {
  const [counterPda, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("counter"), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  await program.methods
    .initialize()
    .accounts({
      counter: counterPda,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .rpc();

  console.log("Counter account initialized at:", counterPda.toBase58());
}

main();
```

---

## 🧪 CLI 与测试整合建议

项目根目录建议：

- `.env` 文件配置：

```
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com
ANCHOR_WALLET=~/.config/solana/id.json
```

- `package.json` 中配置：

```json
{
  "scripts": {
    "build": "anchor build",
    "test": "anchor test",
    "deploy": "anchor deploy",
    "init": "ts-node scripts/init.ts",
    "increment": "ts-node scripts/increment.ts"
  }
}
```

- 使用 `ts-node` + `yargs` 封装统一入口（可参考 CLI 工具化模板）

### 2. `token-airdrop/`
- 使用 SPL Token 程序完成空投操作
- 位置：`examples/token-airdrop/`
- 初始化并铸造 token：`ts-node scripts/mint-airdrop.ts`

### 3. `dao-voting/`
- 建议账户结构治理合约，实现提案与投票流程
- 位置：`examples/dao-voting/`
- 提案初始化：`ts-node scripts/create-proposal.ts`
- 投票调用：`ts-node scripts/vote.ts`

---

## 5. 🔗 实用资源

- [Solana 中文文档](https://solana.wiki/)
- [Anchor 官方文档](https://book.anchor-lang.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Metaplex Token Metadata](https://docs.metaplex.com/)
