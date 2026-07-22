# PixZClaw

**A Brazilian merchant charges in BRL from Telegram and gets paid on two rails: PIX (BRL) or USDC (Solana Pay).**

Three WebAssembly tool plugins (`wasm32-wasip2`) for the [ZeroClaw](https://github.com/zeroclaw-labs) agent.
Custody **T0/T1 only** — the agent never signs anything and never sees a private key.

| | |
|---|---|
| **Landing / setup guide** | https://capitv.github.io/pixzclaw-pi/ |
| **Upstream PR** | [zeroclaw-labs/zeroclaw-plugins#123](https://github.com/zeroclaw-labs/zeroclaw-plugins/pull/123) |
| **Release** | [`v0.5.1-plugins`](https://github.com/capitv/pixzclaw-pi/releases/tag/v0.5.1-plugins) |
| **Runs on** | A Raspberry Pi 3B+ in production, Telegram bot live |

---

## The three tools

| Tool | Tier | What it does |
|---|---|---|
| `brl_usdc_invoice` | T1 | Builds a BRL invoice with **both** rails: a PIX EMV "Copia e Cola" payload (CRC16-CCITT validated) and a Solana Pay URL carrying a deterministic `reference`. Builds only — it never signs. |
| `invoice_status` | T0 | Reads the Solana chain and verifies **how much the merchant actually received**. |
| `pixzclaw_brief` | T0 | 24h close-of-day: transactions, invoices, ids, sparkline. |

Two rails, not an on-ramp and not an exchange. They carry the same `invoice_id`
independently: PIX goes to the merchant's bank, USDC goes to the merchant's wallet.

## Why `invoice_status` is the interesting one

Most "did it get paid?" checks answer *is there a transaction?*. That is not the
same question, and answering it wrongly costs a merchant real money.

This one derives the reference deterministically —
`bs58(sha256("zc-inv-v1" || invoice_id || "|" || merchant)[0..32])` — pulls the
signatures on it, and reads the actual token-balance delta for the merchant on
the USDC mint. Then it says `PAID ✅`, `UNDERPAID ⚠️ (faltam 9)`, `OVERPAID`, or
that it could not tell.

Three properties it holds that are easy to get wrong:

**Exact integer arithmetic, no tolerance band.** Both the issuing and the
verifying side compare in minimum units. There is no "close enough" window a
payer can sit inside.

**Every signature on the reference is scanned**, not just the newest, with an
early stop once the invoice is covered. Otherwise a handful of dust
transactions would push the real payment out of the window and a paid invoice
would read as pending.

**An incomplete scan never asserts a shortfall.** When the RPC rate-limits or
the network drops, the sum is a *lower bound*. A lower bound is enough to
confirm a payment it already covers, but it is not enough to claim money is
missing — that would be the same lie as claiming PAID without checking, pointed
the other way. It degrades to "amount not verified" and says so.

## What it does not know

**PIX settlement is not verifiable by software here.** The PIX rail produces a
valid, scannable payload, but confirmation happens in the merchant's bank —
outside the agent's reach. The product says this out loud instead of implying
otherwise, and only a human operator can mark the PIX leg as received.

---

## Install on a host that already runs ZeroClaw

```bash
curl -fsSL https://raw.githubusercontent.com/capitv/pixzclaw-pi/main/scripts/force-reinstall-plugins.sh | bash -s -- v0.5.1-plugins
```

Then configure and restart — full walkthrough on the
[landing page](https://capitv.github.io/pixzclaw-pi/#instalacao) and in
[PI_INSTALL.md](./PI_INSTALL.md).

## Why this repo builds in CI

A Pi 3 (~1 GB RAM) crashes building ZeroClaw + wasmtime (`SIGSEGV`,
undervoltage). GitHub Actions builds the aarch64 binary and the WASM
components; the Pi only installs them.

| Artifact | Use on the Pi |
|---|---|
| `zeroclaw-aarch64-plugins` | Replaces `zeroclaw` so `zeroclaw plugin` exists |
| `pixzclaw-plugins` | `plugin install` folders |

Push to `main`, or **Actions → build-pi → Run workflow**, then follow
[PI_INSTALL.md](./PI_INSTALL.md).

## Layout

```
crates/solana-wasm-core/   shared pure core (single source of truth)
plugins/brl-usdc-invoice/  PIX EMV + Solana Pay URL          (T1)
plugins/invoice-status/    on-chain USDC amount verification (T0)
plugins/pixzclaw-brief/    24h close-of-day                  (T0)
skills/                    agent skills: onboarding, daily brief, payment watch
docs/                      the landing page (GitHub Pages)
scripts/                   install / force-reinstall helpers
wit/                       ZeroClaw WIT contract (v0)
ci/build-pi.yml            aarch64 + wasm build
```

The core is vendored into each plugin at `plugins/<name>/vendor/solana-wasm-core/`
so every plugin builds standalone — upstream CI copies one plugin directory at a
time, so a path dependency pointing outside it would not resolve. `crates/` stays
the single source of truth; `tools/vendor-core.sh` syncs and `--check` proves no
drift.

## Testing

Everything is host-testable with no network: the RPC layer is behind an injected
`HttpTransport`, so tests feed recorded JSON. The wasm shim is the only place
that touches HTTP or the clock; the core is pure.

```bash
cd plugins/invoice-status && cargo test
```

135 tests across the core and the three plugins.

## Enable GitHub Actions (one-time)

The automation token lacks the `workflow` scope, so the workflow file lives at
[`ci/build-pi.yml`](./ci/build-pi.yml). Copy it into place:

```bash
gh auth refresh -s workflow,repo
mkdir -p .github/workflows
cp ci/build-pi.yml .github/workflows/build-pi.yml
git add .github/workflows/build-pi.yml && git commit -m "ci: enable build-pi" && git push
```

Or through the web UI: **Add file → Create new file →**
`.github/workflows/build-pi.yml`, paste the contents, commit, then
**Actions → build-pi → Run workflow**.

## License

MIT ([LICENSE](./LICENSE)). ZeroClaw itself is built from upstream at CI time
under its own license.
