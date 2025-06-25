# DFTF – Gorbagana Spot DEX on Solana

> **Draft v0.1 – feel free to edit, expand or prune.**

## 🚀 Vision

A single‑surface, community‑owned DEX that lets anyone **swap, provide liquidity, and bundle LP positions into auto‑compounding vault indexes**. Think Hyperliquid’s vault UX, Jupiter’s route depth, Meteora’s flexible pools, and Privy’s web‑2‑easy onboarding, glued together by a buy‑back‑and‑burn token flywheel.

## ⚙️ Core Architecture

```
programs/
  ├─ router_program/        # CPI wrapper around Jupiter + Meteora for swaps & LP ops
  ├─ vault_program/         # Mint/redeem vLP shares, on‑chain rebalancing
  └─ fee_collector_program/ # Accrues protocol fees, buys back & burns $GDX
app/                         # Next 13 front‑end (swap · LP · vault)
keeper/                      # TypeScript bot to crank rebalances if needed
scripts/                     # Dev tooling (init pools, airdrop, etc.)
```

*All on‑chain code written with ****Anchor 0.30+**** and compiled to the Gorbagana Solana runtime.*

## 🛠 Quick Start

```bash
# 0. Install deps
brew install anchor-cli solana node@18
npm i -g pnpm

# 1. Clone & bootstrap
pnpm dlx degit Fyuturevizion/DFTF
cd DFTF
pnpm install

# 2. Scaffold programs (one‑time)
anchor init router_program    --javascript
anchor init vault_program     --javascript
anchor init fee_collector_program --javascript

# 3. Spin up localnet
anchor test --skip-deploy     # compiles & launches test validator

# 4. Front‑end dev
cd app && pnpm dev            # Next 13 on http://localhost:3000
```

## 📚 Key Dependencies

| Domain             | Stack                                                                 | Docs        |
| ------------------ | --------------------------------------------------------------------- | ----------- |
| Routing            | [Jupiter Lite API](https://dev.jup.ag/)                               | jup.ag      |
| LP Pools           | [Meteora DLMM](https://docs.meteora.ag/)                              | meteora.ag  |
| Infra edge         | [bloXroute Solana relay](https://docs.bloxroute.com/)                 | bloxroute   |
| Wallet abstraction | [Privy](https://docs.privy.io)                                        | privy.io    |
| Vault inspiration  | [Hyperliquid vaults](https://hyperliquid.gitbook.io/hyperliquid-docs) | hyperliquid |

## 🧩 Tokenomics (draft)

| Stream        | Fee                    | Destination                           |
| ------------- | ---------------------- | ------------------------------------- |
| Swaps         | **0.20 %**             | 0.02 % → buy‑back queue, 0.18 % → LPs |
| LP vault perf | **5 %** of yield       | Buy‑back queue                        |
| Burn cadence  | Auto once ≥ 1 SOL fees | `spl_token::burn` of \$GDX            |

## 🔒 Security Notes

- Freeze program IDLs post‑audit to block rogue upgrades.
- Static, rent‑exempt PDAs for vaults to prevent ‘tx‑bomb’ griefing.
- Use bloXroute’s regulated relay at launch to lower MEV risk.

## 🗺 Roadmap Snapshot

| Phase | Deliverable              | ETA (wks) |
| ----- | ------------------------ | --------- |
| 0     | Router ↔ Jupiter PoC     | 1         |
| 1     | Meteora CPI + LP UI      | 2         |
| 2     | VaultProgram MVP         | 3         |
| 3     | Privy onboarding polish  | 2         |
| 4     | Token fee plumbing       | 1         |
| 5     | bX‑relay & observability | 1         |
| 6     | Audit window             | 4         |
| 7     | Mainnet Gorbagana launch | —         |

## 🤝 Contributing

1. Fork → Branch (`feat/<xyz>`)
2. `anchor fmt` & `prettier` before PR
3. PR template auto‑runs Anchor + TS tests in CI

## 🪪 License

MIT or Apache 2.0 – TBD.

---

*Questions?* Open an issue or ping in **#dftf‑dev** Discord.

