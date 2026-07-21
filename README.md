# PixZClaw (ZeroClaw PIX ↔ USDC) — Pi deploy

**Brand:** PixZClaw  
**Tools:** `brl_usdc_invoice` (T1) + `invoice_status` (T0)  
**Host:** ZeroClaw with `plugins-wasm` (built for **aarch64** Linux / Raspberry Pi)

This repo is for **CI + install**, not for compiling on a Pi 3.

## Why CI?

Pi 3 (~1 GB RAM) crashes building ZeroClaw + wasmtime (`SIGSEGV` / undervoltage).  
GitHub Actions builds the **binary** and **WASM plugins**; you only install on the Pi.

## What Actions produces

| Artifact | Use on Pi |
|---|---|
| `zeroclaw-aarch64-plugins` | Replace `zeroclaw` so `zeroclaw plugin` exists |
| `pixzclaw-plugins` | `plugin install` folders |

## Trigger build

1. Push to `main` or **Actions → build-pi → Run workflow**
2. Wait for green
3. Download both artifacts
4. Follow [PI_INSTALL.md](./PI_INSTALL.md)

## Layout

```
crates/solana-wasm-core/   # shared pure core
plugins/brl-usdc-invoice/  # PIX + Solana Pay
plugins/invoice-status/    # USDC settlement check
wit/                       # ZeroClaw WIT contract
.github/workflows/build-pi.yml
```

## License

Plugin code: MIT. ZeroClaw is built from upstream at CI time (their license).

## Enable GitHub Actions (one-time)

The GitHub token used for automation lacks the `workflow` scope, so the
workflow file lives at [`ci/build-pi.yml`](./ci/build-pi.yml).

**You (or re-auth with workflow scope):** copy it into Actions:

1. Open https://github.com/capitv/pixzclaw-pi
2. Add file ? Create new file ? path: `.github/workflows/build-pi.yml`
3. Paste contents of `ci/build-pi.yml` ? Commit
4. Actions ? **build-pi** ? Run workflow

Or locally after `gh auth refresh -s workflow,repo`:

```bash
mkdir -p .github/workflows
cp ci/build-pi.yml .github/workflows/build-pi.yml
git add .github/workflows/build-pi.yml && git commit -m "ci: enable build-pi" && git push
```
