//! Integration tests for the invoice tool core, exercised exactly as the wasm
//! `execute` entry point drives it: JSON args + flat `__config` map → pure
//! `execute_invoice`. Runs on the host with a plain `cargo test`.

use std::collections::HashMap;

use brl_usdc_invoice::invoice_tool::{execute_from_args, execute_invoice, ExecuteArgs};

const MERCHANT: &str = "11111111111111111111111111111112";
// Token Program — valid base58 pubkey distinct from MERCHANT.
const OTHER_MERCHANT: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

fn base_config() -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("pix_key".into(), "merchant@example.com".into());
    m.insert("pix_name".into(), "Loja Demo".into());
    m.insert("pix_city".into(), "Sao Paulo".into());
    m.insert("merchant_solana".into(), MERCHANT.into());
    m.insert("max_amount_brl".into(), "1000".into());
    m.insert("max_amount_usdc".into(), "200".into());
    m.insert("recipient_locked".into(), "true".into());
    m.insert("brl_per_usdc".into(), "5.5".into());
    m
}

fn args_json(amount_brl: &str, invoice_id: &str, config: &HashMap<String, String>) -> String {
    serde_json::json!({
        "amount_brl": amount_brl,
        "invoice_id": invoice_id,
        "description": "Pedido teste",
        "__config": config,
    })
    .to_string()
}

#[test]
fn empty_config_fails_clearly() {
    let err = execute_invoice(
        r#"{"amount_brl":"10.00","invoice_id":"inv-empty","__config":{}}"#,
    )
    .unwrap_err();
    assert!(
        err.contains("pix_key")
            || err.contains("pix_name")
            || err.contains("pix_city")
            || err.contains("merchant_solana")
            || err.contains("required"),
        "empty config should fail clearly, got: {err}"
    );
}

#[test]
fn over_max_amount_brl_fails() {
    let cfg = base_config();
    let json = args_json("1000.01", "inv-over", &cfg);
    let err = execute_invoice(&json).unwrap_err();
    assert!(
        err.contains("exceeds max_amount_brl"),
        "expected cap error, got: {err}"
    );
}

#[test]
fn happy_path_pix_and_solana_url() {
    let cfg = base_config();
    let json = args_json("150.00", "inv-001", &cfg);
    let out = execute_invoice(&json).expect("happy path should succeed");

    assert!(
        out.contains("000201"),
        "PIX payload should start with 000201 EMV header, got:\n{out}"
    );
    assert!(out.contains("PIX (Copia e Cola"), "PT section header missing:\n{out}");
    assert!(out.contains("USDC (Solana Pay)"), "USDC section missing:\n{out}");
    assert!(out.contains("qr_payload") || out.contains("Copia e Cola"));
    assert!(
        out.contains(&format!("solana:{MERCHANT}")),
        "Solana Pay should target config merchant, got:\n{out}"
    );
    assert!(out.contains("REF:"), "should report REF");
    assert!(out.contains("AMOUNT: R$ 150.00"));
    assert!(out.contains("recipient_locked=true"));
}

#[test]
fn prompt_injection_huge_amount_fails_closed() {
    // Malicious channel message tries to overcharge past operator cap.
    let cfg = base_config();
    let json = args_json("999999999.99", "inv-inject", &cfg);
    let err = execute_invoice(&json).unwrap_err();
    assert!(
        err.contains("exceeds max_amount_brl") || err.contains("exceeds"),
        "prompt_injection huge amount must fail closed, got: {err}"
    );
}

#[test]
fn prompt_injection_merchant_override_ignored_when_locked() {
    let cfg = base_config();
    let args = ExecuteArgs {
        amount_brl: "10.00".into(),
        invoice_id: Some("inv-lock".into()),
        description: Some("locked".into()),
        payer_name: None,
        usdc_amount: None,
        merchant_override: Some(OTHER_MERCHANT.into()),
        mint_override: None,
        config: cfg,
    };

    let out = execute_from_args(args).expect("locked override should still build");
    assert!(
        out.contains(&format!("solana:{MERCHANT}")),
        "recipient_locked must keep config merchant, got:\n{out}"
    );
    assert!(
        !out.contains(OTHER_MERCHANT),
        "merchant_override must be ignored when locked, got:\n{out}"
    );
}

#[test]
fn auto_invoice_id_when_omitted() {
    let cfg = base_config();
    let json = serde_json::json!({
        "amount_brl": "25.00",
        "description": "cafe",
        "__config": cfg,
    })
    .to_string();
    let out = execute_invoice(&json).expect("auto id");
    assert!(out.contains("INVOICE #INV-") || out.contains("INV-"));
    assert!(out.contains("MEMO: PIX|BRL|INV-"));
}

#[test]
fn invalid_json_fails() {
    let err = execute_invoice("not-json").unwrap_err();
    assert!(
        err.contains("invalid arguments"),
        "expected parse error, got: {err}"
    );
}
