//! Integration tests for the invoice-status pure core, exercised the same way
//! the wasm `execute` entry point drives it: build a `StatusConfig` from a flat
//! config section, then evaluate status from fixture signatures. Runs on the
//! host with a plain `cargo test` — no network.

use std::collections::HashMap;

use invoice_status::status_tool::{
    evaluate_status, fixture_success_sig, resolve_reference, StatusConfig, StatusRequest,
    DEFAULT_LOOKBACK,
};
use solana_wasm_core::derive_reference;

fn section(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

const MERCHANT: &str = "11111111111111111111111111111112";
const INVOICE_ID: &str = "inv-001";

#[test]
fn resolve_reference_is_deterministic() {
    let a = resolve_reference(INVOICE_ID, None, MERCHANT).unwrap();
    let b = resolve_reference(INVOICE_ID, None, MERCHANT).unwrap();
    assert_eq!(a, b);
    assert_eq!(a, derive_reference(INVOICE_ID, MERCHANT));

    let explicit = resolve_reference(INVOICE_ID, Some("ExplicitRef111"), MERCHANT).unwrap();
    assert_eq!(explicit, "ExplicitRef111");
}

#[test]
fn evaluate_status_unpaid() {
    let cfg = StatusConfig::from_map(&section(&[("merchant_solana", MERCHANT)]));
    let req = StatusRequest {
        invoice_id: INVOICE_ID.into(),
        reference: None,
        expected_usdc: Some("10".into()),
        pix_marked_paid: false,
        lookback: DEFAULT_LOOKBACK,
    };
    let s = evaluate_status(&req, &cfg, &[]);
    assert!(s.contains("USDC: PENDING"), "unexpected unpaid text: {s}");
    assert!(s.contains("PIX: PENDING"), "{s}");
    assert!(s.contains("OVERALL: PENDING"), "{s}");
    assert!(s.contains(INVOICE_ID), "{s}");
}

#[test]
fn evaluate_status_with_successful_sig() {
    let cfg = StatusConfig::from_map(&section(&[("merchant_solana", MERCHANT)]));
    let req = StatusRequest {
        invoice_id: INVOICE_ID.into(),
        reference: None,
        expected_usdc: Some("12.5".into()),
        pix_marked_paid: false,
        lookback: DEFAULT_LOOKBACK,
    };
    let sigs = vec![fixture_success_sig(
        "VeryLongSignatureSuccess111",
        Some("PIX|BRL|inv-001|x"),
    )];
    let s = evaluate_status(&req, &cfg, &sigs);
    assert!(s.contains("USDC: PAID"), "{s}");
    assert!(s.contains("solscan.io/tx/"), "{s}");
    assert!(s.contains("PIX: PENDING") || s.contains("PIX não confirmado"), "{s}");
    assert!(s.contains("esperado=12.5") || s.contains("12.5"), "{s}");
}

#[test]
fn evaluate_status_pix_marked_paid() {
    let cfg = StatusConfig::from_map(&section(&[("merchant_solana", MERCHANT)]));
    let req = StatusRequest {
        invoice_id: "inv-2".into(),
        reference: Some("RefXYZ".into()),
        expected_usdc: None,
        pix_marked_paid: true,
        lookback: DEFAULT_LOOKBACK,
    };
    let sigs = vec![fixture_success_sig("SigOK", None)];
    let s = evaluate_status(&req, &cfg, &sigs);
    assert!(s.contains("USDC: PAID"), "{s}");
    assert!(s.contains("PIX: PAID"), "{s}");
    assert!(s.contains("ambos trilhos") || s.contains("OVERALL:"), "{s}");
}

#[test]
fn missing_merchant_fails_resolve_when_no_reference() {
    let err = resolve_reference(INVOICE_ID, None, "").unwrap_err();
    assert!(
        err.contains("merchant_solana") || err.contains("reference"),
        "unexpected error: {err}"
    );

    let cfg = StatusConfig::from_map(&HashMap::new());
    let req = StatusRequest {
        invoice_id: INVOICE_ID.into(),
        reference: None,
        expected_usdc: None,
        pix_marked_paid: false,
        lookback: DEFAULT_LOOKBACK,
    };
    let s = evaluate_status(&req, &cfg, &[]);
    assert!(
        s.contains("merchant_solana") || s.contains("reference"),
        "unexpected: {s}"
    );
}

#[test]
fn config_defaults_and_overrides() {
    let empty = StatusConfig::from_map(&HashMap::new());
    assert!(empty.rpc_url.contains("solana.com"));
    assert!(empty.merchant_solana.is_empty());

    let cfg = StatusConfig::from_map(&section(&[
        ("rpc_url", "https://rpc.example/"),
        ("merchant_solana", MERCHANT),
        ("usdc_mint", "Mint111"),
    ]));
    assert_eq!(cfg.rpc_url, "https://rpc.example/");
    assert_eq!(cfg.merchant_solana, MERCHANT);
    assert_eq!(cfg.usdc_mint, "Mint111");
}
