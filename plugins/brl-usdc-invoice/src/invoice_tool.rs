//! Pure invoice-tool core. No wit-bindgen or wasm dependency so it compiles and
//! tests on the host with a plain `cargo test`, while the wasm component reuses
//! the exact same logic through `lib.rs`.

use std::collections::HashMap;

use serde::Deserialize;
use solana_wasm_core::invoice::{
    build_invoice, InvoiceConfig, InvoiceRequest, InvoiceResult,
};

/// Arguments injected by the host into `tool::execute` (plus optional `__config`).
#[derive(Debug, Deserialize)]
pub struct ExecuteArgs {
    pub amount_brl: String,
    /// Optional; empty → core generates `INV-XXXXXXXX`.
    #[serde(default)]
    pub invoice_id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub payer_name: Option<String>,
    #[serde(default)]
    pub usdc_amount: Option<String>,
    #[serde(default)]
    pub merchant_override: Option<String>,
    #[serde(default)]
    pub mint_override: Option<String>,
    #[serde(rename = "__config", default)]
    pub config: HashMap<String, String>,
}

/// Parse execute JSON, enforce caps via core, and return LLM-shaped text.
pub fn execute_invoice(args_json: &str) -> Result<String, String> {
    let args: ExecuteArgs = serde_json::from_str(args_json)
        .map_err(|e| format!("invalid arguments: {e}"))?;
    execute_from_args(args)
}

/// Host-testable path that skips JSON when callers already have typed args.
pub fn execute_from_args(args: ExecuteArgs) -> Result<String, String> {
    let cfg = InvoiceConfig::from_map(&args.config);

    let req = InvoiceRequest {
        amount_brl: args.amount_brl,
        invoice_id: empty_to_none(args.invoice_id).unwrap_or_default(),
        description: empty_to_none(args.description),
        payer_name: empty_to_none(args.payer_name),
        usdc_amount: empty_to_none(args.usdc_amount),
        merchant_override: empty_to_none(args.merchant_override),
        mint_override: empty_to_none(args.mint_override),
    };

    let result = build_invoice(&req, &cfg)?;
    Ok(format_invoice_result(&result, cfg.recipient_locked, &cfg.max_amount_brl, &cfg.max_amount_usdc))
}

fn empty_to_none(v: Option<String>) -> Option<String> {
    v.and_then(|s| {
        let t = s.trim();
        if t.is_empty() {
            None
        } else {
            Some(t.to_string())
        }
    })
}

/// Shape dual-rail invoice into PT-BR blocks for Telegram (~200 tokens).
pub fn format_invoice_result(
    r: &InvoiceResult,
    recipient_locked: bool,
    max_brl: &str,
    max_usdc: &str,
) -> String {
    format!(
        "{summary}\n\
         \n\
         INVOICE: (ver summary)\n\
         \n\
         PIX (Copia e Cola / qr_payload):\n\
         {pix}\n\
         \n\
         USDC (Solana Pay):\n\
         {solana}\n\
         \n\
         REF: {reference}\n\
         MEMO: {memo}\n\
         AMOUNT: R$ {brl} ≈ {usdc} USDC\n\
         GUARDS: max_brl={max_brl} max_usdc={max_usdc} recipient_locked={locked}\n\
         \n\
         Não substitui o app do banco: o agente só emite a fatura dual-rail.\n\
         Use invoice_status com REF/invoice para checar USDC on-chain.",
        summary = r.summary,
        pix = r.pix_payload,
        solana = r.solana_pay_url,
        reference = r.reference,
        memo = r.memo,
        brl = r.amount_brl,
        usdc = r.amount_usdc,
        max_brl = max_brl,
        max_usdc = max_usdc,
        locked = recipient_locked,
    )
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn format_includes_sections() {
        let r = InvoiceResult {
            pix_payload: "000201...".into(),
            solana_pay_url: "solana:abc?".into(),
            reference: "ref123".into(),
            memo: "PIX|BRL|id|x".into(),
            amount_brl: "10.00".into(),
            amount_usdc: "1.818181".into(),
            summary: "INVOICE #id · R$ 10.00".into(),
        };
        let s = format_invoice_result(&r, true, "1000", "200");
        assert!(s.contains("PIX (Copia e Cola"));
        assert!(s.contains("USDC (Solana Pay)"));
        assert!(s.contains("000201"));
        assert!(s.contains("solana:abc?"));
        assert!(s.contains("REF:"));
        assert!(s.contains("recipient_locked=true"));
    }
}
