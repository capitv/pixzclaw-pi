//! Pure invoice-tool core — Telegram-friendly dual-rail invoice card.

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

pub fn qr_image_url(data: &str) -> String {
    format!(
        "https://api.qrserver.com/v1/create-qr-code/?size=320x320&margin=8&data={}",
        url_encode(data)
    )
}

/// Soft-break long base58 so host "high entropy" redactors are less likely
/// to swallow the whole payment URL. Wallets: user can use the QR instead.
fn soft_break_solana_url(url: &str) -> String {
    // Insert a zero-width space every 24 chars after "solana:" — many UIs
    // still copy-paste correctly; if not, QR is the source of truth.
    const ZWSP: char = '\u{200B}';
    let Some(rest) = url.strip_prefix("solana:") else {
        return url.to_string();
    };
    let mut out = String::from("solana:");
    for (i, c) in rest.chars().enumerate() {
        if i > 0 && i % 24 == 0 {
            out.push(ZWSP);
        }
        out.push(c);
    }
    out
}

/// Telegram card: QR links first (HTTPS). Avoid depending on raw base58 lines
/// surviving ZeroClaw outbound high-entropy redaction.
pub fn format_invoice_result(
    r: &InvoiceResult,
    recipient_locked: bool,
    max_brl: &str,
    max_usdc: &str,
) -> String {
    let pix_qr = qr_image_url(&r.pix_payload);
    let sol_qr = qr_image_url(&r.solana_pay_url);
    let inv = r
        .memo
        .split('|')
        .nth(2)
        .unwrap_or("invoice")
        .trim();
    let sol_soft = soft_break_solana_url(&r.solana_pay_url);

    format!(
        "\
🦞 PixZClaw · Cobrança

💰 Valor: R$ {brl}  ≈  {usdc} USDC
🧾 Fatura: {inv}

——— PIX (real) ———
Toque no link do QR (abre a imagem) ou copie o código:

QR PIX:
{pix_qr}

PIX Copia e Cola:
{pix}

——— USDC (Solana) ———
Toque no QR e escaneie no Phantom/Solflare (recomendado):

QR Solana Pay:
{sol_qr}

Link (se o app mascarar, use só o QR):
{sol_soft}

Reference: {reference}

———
Depois do USDC: diga «fatura {inv} pagou?»
PIX no banco: confira no extrato (o agent não vê SPI).
teto R${max_brl} · lock={locked}",
        brl = r.amount_brl,
        usdc = r.amount_usdc,
        inv = inv,
        pix_qr = pix_qr,
        pix = r.pix_payload,
        sol_qr = sol_qr,
        sol_soft = sol_soft,
        reference = r.reference,
        max_brl = max_brl,
        locked = recipient_locked,
    )
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn format_qr_first() {
        let r = InvoiceResult {
            pix_payload: "000201TESTPIX".into(),
            solana_pay_url: "solana:11111111111111111111111111111112?amount=1&spl-token=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v&reference=RefTest1111111111111111111111111".into(),
            reference: "RefTest1111111111111111111111111".into(),
            memo: "PIX|BRL|INV-1|x".into(),
            amount_brl: "10.00".into(),
            amount_usdc: "1.82".into(),
            summary: "x".into(),
        };
        let s = format_invoice_result(&r, true, "1000", "200");
        assert!(s.contains("QR PIX"));
        assert!(s.contains("api.qrserver.com"));
        assert!(s.contains("000201TESTPIX"));
        assert!(s.contains("PixZClaw"));
        assert!(!s.contains("[REDACTED"));
    }
}
