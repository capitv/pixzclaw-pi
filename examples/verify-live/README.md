# verify-live

Run the `invoice_status` verification against the live Solana chain from any
machine. No ZeroClaw, no wasmtime, no Raspberry Pi.

```bash
cd examples/verify-live
cargo run -- --merchant <wallet> --invoice <id> --expected 10
```

Needs `curl` on PATH (Windows 10+, macOS and Linux all ship it) and nothing else
— there is no HTTP or TLS crate in the dependency tree.

## Why this is not a demo script

It calls `invoice_status::status_tool::fetch_and_status`, which is the exact
function the WebAssembly component calls. The only thing swapped is the
`HttpTransport` implementation: the plugin hands it `waki` over `wasi:http`,
this hands it `curl`. Every decision about what counts as paid — the reference
derivation, the signature scan, the integer amount comparison, the refusal to
assert a shortfall on an incomplete scan — is made by the same code, compiled
from the same source.

If it prints `UNDERPAID` here, the plugin prints `UNDERPAID` on the Pi.

## Check it against a real payment you already know

You do not need a PixZClaw invoice to exercise the amount verification. Point
`--reference` at any address that has received the mint, and `--merchant` at the
wallet that received it:

```bash
cargo run -- \
  --merchant  <wallet that received USDC> \
  --reference <address the payment carried> \
  --expected  1 \
  --lookback  5
```

The tool prints the address it is about to read and a Solscan link to it, before
it prints a verdict. Open the link, count the transfers yourself, and compare.
That is the whole point: the evidence is checkable, not asserted.

## Reading the output

The plugin speaks to Brazilian merchants, so the verdict lines are Portuguese.
They map to:

| Line | Meaning |
|---|---|
| `USDC: PAID ✅` | The merchant's USDC balance rose by at least the expected amount |
| `USDC: UNDERPAID ⚠️ (recebido 1 de 10 USDC — faltam 9)` | Received 1 of 10, 9 short. Exact integer arithmetic, no tolerance band |
| `USDC: OVERPAID` | More arrived than was invoiced |
| `USDC: SIG OK` | Transactions exist, but the amount could not be verified — the scan was incomplete, so the sum is only a lower bound and claiming a shortfall would be a guess |
| `USDC: PENDING (nenhuma assinatura no reference)` | Nothing has touched the reference |
| `USDC: PENDING (assinatura sem transferência de USDC ao lojista)` | Something touched it, but no USDC reached the merchant |
| `PIX: PENDING (tool não vê SPI do banco…)` | PIX settlement happens inside the merchant's bank and is not verifiable here. Only a human operator can mark it received |

## Options

| Flag | Default |
|---|---|
| `--invoice <id>` | `INV-DEMO-A` |
| `--merchant <pubkey>` | required |
| `--reference <pubkey>` | derived as `bs58(sha256("zc-inv-v1" ‖ invoice_id ‖ "\|" ‖ merchant)[0..32])` |
| `--expected <amount>` | `10` |
| `--rpc <url>` | `https://api.mainnet-beta.solana.com` |
| `--mint <pubkey>` | USDC mainnet |
| `--lookback <n>` | `25` |

The public mainnet endpoint rate-limits. Under a rate limit the tool degrades to
`SIG OK` rather than inventing a verdict, which is the behaviour worth watching
— pass your own `--rpc` to see the full scan complete.
