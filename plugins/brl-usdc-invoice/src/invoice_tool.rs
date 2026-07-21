//! Pure invoice-tool core — Telegram card: QR first, minimal raw secrets in text
//! (ZeroClaw host redacts high-entropy base58 as [REDACTED_…]).

use std::collections::HashMap;

use serde::Deserialize;
use solana_wasm_core::invoice::{
    build_invoice, InvoiceConfig, InvoiceRequest, InvoiceResult,
};
use solana_wasm_core::solana_pay::url_encode;

#[derive(Debug, Deserialize)]
pub struct ExecuteArgs {
    pub amount_brl: String,
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

pub fn execute_invoice(args_json: &str) -> Result<String, String> {
    let args: ExecuteArgs =
        serde_json::from_str(args_json).map_err(|e| format!("invalid arguments: {e}"))?;
    execute_from_args(args)
}

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

pub fn qr_image_url(data: &str) -> String {
    format!(
        "https://api.qrserver.com/v1/create-qr-code/?size=320x320&margin=8&data={}",
        url_encode(data)
    )
}

/// Short invoice id for display (from memo).
fn invoice_label(r: &InvoiceResult) -> String {
    r.memo
        .split('|')
        .nth(2)
        .unwrap_or("invoice")
        .trim()
        .to_string()
}

/// Telegram-friendly card.
///
/// - PIX: QR link + full copia-e-cola (banks need the string).
/// - Solana: **QR only** in chat — host redacts base58 as [REDACTED_HIGH_ENTROPY_TOKEN]
///   and breaks the solana: line; the QR image still encodes the full Solana Pay URL.
pub fn format_invoice_result(
    r: &InvoiceResult,
    recipient_locked: bool,
    max_brl: &str,
) -> String {
    let pix_qr = qr_image_url(&r.pix_payload);
    let sol_qr = qr_image_url(&r.solana_pay_url);
    let inv = invoice_label(r);

    format!(
        "\
🦞 PixZClaw · Cobrança pronta

💰 Valor: *R$ {brl}*  (≈ {usdc} USDC)
🧾 Fatura: *{inv}*

━━━━━━━━━━━━━━━━
🇧🇷 *PIX (real)*
1. Toque no link → abre o QR
2. Ou copie o código no app do banco

📷 QR PIX (toque):
{pix_qr}

📋 PIX Copia e Cola:
`{pix}`

━━━━━━━━━━━━━━━━
💎 *USDC (Solana)*
Toque no link e escaneie no *Phantom / Solflare*
(o texto solana: é omitido de propósito — o ZeroClaw mascara pubkey no chat)

📷 QR Solana Pay (toque):
{sol_qr}

━━━━━━━━━━━━━━━━
✅ Depois do USDC: «fatura {inv} pagou?»
ℹ️ PIX no banco: confira no extrato.
🔒 teto R$ {max_brl} · destino travado={locked}

_Repasse este card sem apagar os links do QR._",
        brl = r.amount_brl,
        usdc = r.amount_usdc,
        inv = inv,
        pix_qr = pix_qr,
        pix = r.pix_payload,
        sol_qr = sol_qr,
        max_brl = max_brl,
        locked = if recipient_locked { "sim" } else { "não" },
    )
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn format_has_qr_no_raw_solana_line() {
        let r = InvoiceResult {
            pix_payload: "000201TEST".into(),
            solana_pay_url: "solana:11111111111111111111111111111112?amount=1&spl-token=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v&reference=RefTest1111111111111111111111111".into(),
            reference: "RefTest1111111111111111111111111".into(),
            memo: "PIX|BRL|INV-1|x".into(),
            amount_brl: "10.00".into(),
            amount_usdc: "1.82".into(),
            summary: "x".into(),
        };
        let s = format_invoice_result(&r, true, "1000");
        assert!(s.contains("api.qrserver.com"));
        assert!(s.contains("000201TEST"));
        assert!(s.contains("QR Solana"));
        // Must NOT put a plain solana: line (gets REDACTED by host)
        assert!(
            !s.lines().any(|l| l.trim().starts_with("solana:")),
            "should not expose raw solana: line, got:\n{s}"
        );
        assert!(s.contains("PixZClaw"));
    }
}
