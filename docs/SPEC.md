# DFTF – Gorbagana\u00a0Spot\u00a0DEX on Solana

> **Draft v0.1 – feel free to edit, expand or prune.**

## \ud83d\ude80 Vision

A single\u2011surface, community\u2011owned DEX that lets anyone **swap, provide liquidity, and bundle LP positions into auto\u2011compounding vault indexes**. Think Hyperliquid\u2019s vault UX, Jupiter\u2019s route depth, Meteora\u2019s flexible pools, and Privy\u2019s web\u20112\u2011easy onboarding, glued together by a buy\u2011back\u2011and\u2011burn token flywheel.

## \u2699\ufe0f Core Architecture

```
programs/
  \u251c\u2500 router_program/        # CPI wrapper around Jupiter + Meteora for swaps & LP ops
  \u251c\u2500 vault_program/         # Mint/redeem vLP shares, on\u2011chain rebalancing
  \u2514\u2500 fee_collector_program/ # Accrues protocol fees, buys back & burns $GDX
app/                         # Next\u00a013 front\u2011end (swap \xb7 LP \xb7 vault)
keeper/                      # TypeScript bot to crank rebalances if needed
scripts/                     # Dev tooling (init pools, airdrop, etc.)
```

_All on\u2011chain code written with **Anchor 0.30+** and compiled to the Gorbagana Solana runtime._

## \ud83d\udcaa Quick\u00a0Start

```bash
# 0. Install deps
brew install anchor-cli solana node@18
npm i -g pnpm

# 1. Clone & bootstrap
pnpm dlx degit Fyuturevizion/DFTF
cd DFTF
pnpm install

# 2. Scaffold programs (one\u2011time)
anchor init router_program    --javascript
anchor init vault_program     --javascript
anchor init fee_collector_program --javascript

# 3. Spin up localnet
anchor test --skip-deploy     # compiles & launches test validator

# 4. Front\u2011end dev
cd app && pnpm dev            # Next\u00a013 on http://localhost:3000
```

## \ud83d\udcda Key Dependencies

| Domain             | Stack                                                                 | Docs        |
| ------------------ | --------------------------------------------------------------------- | ----------- |
| Routing            | [Jupiter Lite API](https://dev.jup.ag/)                               | jup.ag      |
| LP Pools           | [Meteora DLMM](https://docs.meteora.ag/)                              | meteora.ag  |
| Infra edge         | [bloXroute\u00a0Solana\u00a0relay](https://docs.bloxroute.com/)       | bloxroute   |
| Wallet abstraction | [Privy](https://docs.privy.io)                                        | privy.io    |
| Vault inspiration  | [Hyperliquid vaults](https://hyperliquid.gitbook.io/hyperliquid-docs) | hyperliquid |

## \ud83e\uddf9 Tokenomics (draft)

| Stream        | Fee                                   | Destination                                        |
| ------------- | ------------------------------------- | -------------------------------------------------- |
| Swaps         | **0.20%**                             | 0.02% \u2192 buy\u2011back queue, 0.18% \u2192 LPs |
| LP vault perf | **5%** of yield                       | Buy\u2011back queue                                |
| Burn cadence  | Auto once \u2265\u00a01\u00a0SOL fees | `spl_token::burn` of \$GDX                         |

## \ud83d\udd12 Security Notes

- Freeze program IDLs post\u2011audit to block rogue upgrades.
- Static, rent\u2011exempt PDAs for vaults to prevent \u2018tx\u2011bomb\u2019 griefing.
- Use bloXroute\u2019s regulated relay at launch to lower MEV risk.

## \ud83d\udea9 Roadmap Snapshot

| Phase | Deliverable                   | ETA (wks) |
| ----- | ----------------------------- | --------- |
| 0     | Router \u2194 Jupiter PoC     | 1         |
| 1     | Meteora CPI + LP UI           | 2         |
| 2     | VaultProgram MVP              | 3         |
| 3     | Privy onboarding polish       | 2         |
| 4     | Token fee plumbing            | 1         |
| 5     | bX\u2011relay & observability | 1         |
| 6     | Audit window                  | 4         |
| 7     | Mainnet Gorbagana launch      | \u2014    |

## \ud83d\ude4d Contributing

1. Fork \u2192 Branch (`feat/<xyz>`)
2. `anchor fmt` & `prettier` before PR
3. PR template auto\u2011runs Anchor + TS tests in CI

## \ud83d\uddc9 License

MIT or Apache\u00a02.0 \u2013\u00a0TBD.

---

_Questions?_ Open an issue or ping in **#dftf\u2011dev** Discord.
