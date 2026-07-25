//! Verify a PixZClaw invoice against the live Solana chain, from any host.
//!
//! The point of this binary is that it is not a re-implementation. It calls
//! `invoice_status::status_tool::fetch_and_status` — the same function the
//! WebAssembly component calls — and only supplies a different `HttpTransport`.
//! The plugin gives it `waki` over `wasi:http`; this gives it `ureq`. Every
//! decision about what counts as paid is made by the identical code.
//!
//! So a reviewer with neither ZeroClaw nor a Raspberry Pi can still watch the
//! real verdict come off the real chain:
//!
//! ```text
//! cargo run --release
//! ```

use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, ExitCode, Stdio};

use invoice_status::status_tool::{fetch_and_status, StatusConfig, StatusRequest};
use serde_json::Value;
use solana_wasm_core::{HttpTransport, RpcError};

/// The invoice recorded in the demo. It is settled on Solana mainnet and the
/// payment is deliberately short, because a verifier that only ever prints
/// `PAID` has not demonstrated anything.
mod demo {
    pub const INVOICE_ID: &str = "INV-DEMO-A";
    pub const MERCHANT: &str = "";
    pub const EXPECTED_USDC: &str = "10";
}

/// Blocking JSON-RPC over `curl`.
///
/// A Rust HTTP client would drag in a TLS stack and roughly two hundred crates
/// for two POST requests. `curl` ships with Windows 10+, macOS and every Linux,
/// builds nothing, and leaves the exact bytes on the wire inspectable — which
/// matters more here than elegance, because the whole point is that a reviewer
/// can check the tool is not inventing its evidence.
struct CurlTransport;

impl HttpTransport for CurlTransport {
    fn post_json(&self, url: &str, body: &Value) -> Result<Value, RpcError> {
        let mut child = Command::new("curl")
            .args([
                "--silent",
                "--show-error",
                "--fail",
                "--location",
                "--proto",
                "=https",
                "--max-time",
                "25",
                "--retry",
                "2",
                "--header",
                "content-type: application/json",
                "--data-binary",
                "@-",
                url,
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| RpcError {
                message: format!("could not run `curl` (is it on PATH?): {e}"),
            })?;

        // The request body goes over stdin rather than argv so a long payload
        // cannot hit a command-line length limit.
        let payload = body.to_string();
        child
            .stdin
            .take()
            .expect("stdin was piped")
            .write_all(payload.as_bytes())
            .map_err(|e| RpcError {
                message: format!("could not write the request body: {e}"),
            })?;

        let output = child.wait_with_output().map_err(|e| RpcError {
            message: format!("curl did not finish: {e}"),
        })?;

        if !output.status.success() {
            return Err(RpcError {
                message: format!(
                    "curl failed ({}): {}",
                    output.status,
                    String::from_utf8_lossy(&output.stderr).trim()
                ),
            });
        }

        serde_json::from_slice(&output.stdout).map_err(|e| RpcError {
            message: format!("response was not JSON: {e}"),
        })
    }
}

/// Long options only, so a mistyped flag is an error rather than a silent
/// default that would make the output a lie.
fn parse_args() -> Result<HashMap<String, String>, String> {
    const KNOWN: [&str; 7] = [
        "--invoice",
        "--merchant",
        "--reference",
        "--expected",
        "--rpc",
        "--mint",
        "--lookback",
    ];

    let mut parsed = HashMap::new();
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        if flag == "--help" || flag == "-h" {
            return Err(usage());
        }
        if !KNOWN.contains(&flag.as_str()) {
            return Err(format!("unknown flag `{flag}`\n\n{}", usage()));
        }
        let value = args
            .next()
            .ok_or_else(|| format!("`{flag}` needs a value\n\n{}", usage()))?;
        parsed.insert(flag, value);
    }
    Ok(parsed)
}

fn usage() -> String {
    format!(
        "verify-live — run invoice_status against the live Solana chain\n\
         \n\
         Usage: cargo run --release -- [options]\n\
         \n\
         Options (all optional; defaults reproduce the recorded demo):\n\
         \x20 --invoice   <id>     invoice id                  [{invoice}]\n\
         \x20 --merchant  <pubkey> merchant wallet             [{merchant}]\n\
         \x20 --reference <pubkey> read this address instead of deriving one\n\
         \x20 --expected  <amount> expected USDC, decimal       [{expected}]\n\
         \x20 --rpc       <url>    Solana JSON-RPC endpoint\n\
         \x20 --mint      <pubkey> SPL mint to verify           [USDC mainnet]\n\
         \x20 --lookback  <n>      signatures to scan\n\
         \n\
         Pointing --reference at any address that has received the mint, with\n\
         --merchant set to the wallet that received it, runs the real amount\n\
         verification over real transactions without needing a PixZClaw invoice.\n",
        invoice = demo::INVOICE_ID,
        merchant = if demo::MERCHANT.is_empty() {
            "unset — pass --merchant"
        } else {
            demo::MERCHANT
        },
        expected = demo::EXPECTED_USDC,
    )
}

fn run() -> Result<String, String> {
    let args = parse_args()?;

    let merchant = args
        .get("--merchant")
        .map(String::as_str)
        .unwrap_or(demo::MERCHANT);
    if merchant.trim().is_empty() {
        return Err(
            "no merchant wallet: this build has no recorded demo wallet yet, \
             so pass one with --merchant <pubkey>"
                .to_string(),
        );
    }

    let mut cfg = StatusConfig {
        merchant_solana: merchant.to_string(),
        ..StatusConfig::default()
    };
    if let Some(rpc) = args.get("--rpc") {
        cfg.rpc_url = rpc.clone();
    }
    if let Some(mint) = args.get("--mint") {
        cfg.usdc_mint = mint.clone();
    }

    let mut req = StatusRequest {
        invoice_id: args
            .get("--invoice")
            .cloned()
            .unwrap_or_else(|| demo::INVOICE_ID.to_string()),
        reference: args.get("--reference").cloned(),
        expected_usdc: Some(
            args.get("--expected")
                .cloned()
                .unwrap_or_else(|| demo::EXPECTED_USDC.to_string()),
        ),
        ..StatusRequest::default()
    };
    if let Some(raw) = args.get("--lookback") {
        req.lookback = raw
            .parse()
            .map_err(|_| format!("`--lookback` expects a whole number, got `{raw}`"))?;
    }

    // Printing the reference before the verdict lets a reviewer open it on any
    // explorer and confirm the transactions being read are the ones the tool
    // claims to be reading.
    let (reference, origin) = match req.reference.as_deref() {
        Some(given) => (given.to_string(), "given"),
        None => (
            solana_wasm_core::derive_reference(&req.invoice_id, &cfg.merchant_solana),
            "derived",
        ),
    };
    println!("invoice    {}", req.invoice_id);
    println!("merchant   {}", cfg.merchant_solana);
    println!("reference  {reference}  ({origin})");
    println!("explorer   https://solscan.io/account/{reference}");
    println!("rpc        {}", cfg.rpc_url);
    println!("mint       {}", cfg.usdc_mint);
    println!("expected   {} USDC", req.expected_usdc.as_deref().unwrap_or("—"));
    println!();

    fetch_and_status(&req, &cfg, CurlTransport)
}

fn main() -> ExitCode {
    match run() {
        Ok(report) => {
            println!("{report}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
