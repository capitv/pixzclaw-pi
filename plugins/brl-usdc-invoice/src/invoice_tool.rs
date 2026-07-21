//! Pure invoice-tool core. No wit-bindgen or wasm dependency so it compiles and
//! tests on the host with a plain `cargo test`, while the wasm component reuses
//! the exact same logic through `lib.rs`.

use std::collections::HashMap;

use serde::Deserialize;
use solana_wasm_core::invoice::{
    build_invoice, InvoiceConfig, InvoiceRequest, InvoiceResult,
};
use solana_wasm_core::solana_pay::url_encode;

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
    let args: ExecuteArgs =
        serde_json::from_str(args_json).map_err(|e| format!("invalid arguments: {e}"))?;
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
    Ok(format_invoice_result(
        &result,
        cfg.recipient_locked,
        &cfg.max_amount_brl,
        &cfg.max_amount_usdc,
    ))
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

/// Public QR image URL (tap-friendly in Telegram). Encodes `data` as query param.
pub fn qr_image_url(data: &str) -> String {
    // api.qrserver.com — no API key; fine for demo / self-host ops
    format!(
        "https://api.qrserver.com/v1/create-qr-code/?size=320x320&margin=8&data={}",
        url_encode(data)
    )
}

/// Telegram-friendly dual-rail invoice card.
///
/// Rules for the agent (also in skills):
/// - Send this text **almost literal** — do not run redact on it.
/// - Do not invent a second PIX string.
/// - QR links open the image when tapped (Telegram preview).
pub fn format_invoice_result(
    r: &InvoiceResult,
    recipient_locked: bool,
    max_brl: &str,
    max_usdc: &str,
) -> String {
    let pix_qr = qr_image_url(&r.pix_payload);
    let sol_qr = qr_image_url(&r.solana_pay_url);

    // Invoice id from summary or memo
    let inv = r
        .memo
        .split('|')
        .nth(2)
        .unwrap_or("invoice")
        .trim();

    format!(
        "\
🦞 *PixZClaw · Cobrança pronta*

💰 *Valor:* R$ {brl}  ≈  {usdc} USDC
🧾 *Fatura:* `{inv}`
📝 *Memo:* `{memo}`

━━━━━━━━━━━━━━━━━━━━
🇧🇷 *Pagar com PIX (real)*
1. Toque no link do QR *ou* copie o código abaixo
2. Abra o app do banco → Pix → Copia e Cola / ler QR

📷 *QR PIX (toque):*
{pix_qr}

📋 *PIX Copia e Cola* (segure para copiar):
`{pix}`

━━━━━━━━━━━━━━━━━━━━
💎 *Pagar com USDC (Solana)*
1. Toque no QR *ou* abra o link `solana:` no Phantom/Solflare
2. Confirme o envio de *{usdc} USDC*

📷 *QR Solana Pay (toque):*
{sol_qr}

🔗 *Link Solana Pay* (inteiro — não redija):
`{solana}`

🔖 *Reference (status):* `{reference}`

━━━━━━━━━━━━━━━━━━━━
✅ Depois de pagar em USDC: diga *\"fatura {inv} pagou?\"*
ℹ️ PIX do banco o agent não vê sozinho — confira no extrato se for PIX.

_AGENT: envie esta mensagem quase literal. NÃO use redact. NÃO invente outro PIX. NÃO mascare pubkey/mint/reference._
🔒 teto R${max_brl} · USDC max {max_usdc} · lock={locked}",
        brl = r.amount_brl,
        usdc = r.amount_usdc,
        inv = inv,
        memo = r.memo,
        pix_qr = pix_qr,
        pix = r.pix_payload,
        sol_qr = sol_qr,
        solana = r.solana_pay_url,
        reference = r.reference,
        max_brl = max_brl,
        max_usdc = max_usdc,
        locked = recipient_locked,
    )
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn format_includes_qr_and_clean_blocks() {
        let r = InvoiceResult {
            pix_payload: "000201TESTPIX".into(),
            solana_pay_url: "solana:11111111111111111111111111111112?amount=1&spl-token=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v&reference=RefTest1111111111111111111111111".into(),
            reference: "RefTest1111111111111111111111111".into(),
            memo: "PIX|BRL|INV-1|x".into(),
            amount_brl: "10.00".into(),
            amount_usdc: "1.818181".into(),
            summary: "INVOICE #INV-1 · R$ 10.00".into(),
        };
        let s = format_invoice_result(&r, true, "1000", "200");
        assert!(s.contains("QR PIX"));
        assert!(s.contains("api.qrserver.com"));
        assert!(s.contains("000201TESTPIX"));
        assert!(s.contains("solana:11111111111111111111111111111112"));
        assert!(s.contains("NÃO use redact"));
        assert!(s.contains("INV-1"));
        assert!(!s.contains("[REDACTED"));
    }

    #[test]
    fn qr_url_encodes_payload() {
        let u = qr_image_url("solana:abc?amount=1");
        assert!(u.contains("api.qrserver.com"));
        assert!(u.contains("data="));
        assert!(u.contains("solana"));
    }
}
