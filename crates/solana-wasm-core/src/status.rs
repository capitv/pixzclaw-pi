//! Invoice status shaping from Solana signature lists (+ optional PIX flag).

use crate::rpc::SignatureInfo;
use crate::shape::short_label;

/// Build a short LLM-oriented status string from signature query results.
///
/// Honesty rules:
/// - USDC leg is inferred only from successful signatures on `reference`.
/// - PIX bank settlement is **not** visible on-chain; only confirmed when
///   `pix_marked_paid` is true (operator / PSP signal).
pub fn status_from_signatures(
    invoice_id: &str,
    reference: &str,
    sigs: &[SignatureInfo],
    expected_usdc: Option<&str>,
    pix_marked_paid: bool,
) -> String {
    let id = if invoice_id.trim().is_empty() {
        "(unknown)"
    } else {
        invoice_id.trim()
    };
    let ref_short = short_label(reference, 12);

    let successful: Vec<&SignatureInfo> = sigs.iter().filter(|s| s.is_success()).collect();
    let failed = sigs.len().saturating_sub(successful.len());
    let usdc_paid = !successful.is_empty();

    let usdc_status = if !usdc_paid {
        if sigs.is_empty() {
            "USDC: PENDING (nenhuma assinatura no reference)".to_string()
        } else {
            format!("USDC: PENDING ({failed} sig(s) com erro; nenhuma sucesso)")
        }
    } else {
        let latest = successful[0];
        let sig = &latest.signature;
        let sig_short = short_label(sig, 12);
        let conf = latest
            .confirmation_status
            .as_deref()
            .unwrap_or("unknown");
        let explorer = format!("https://solscan.io/tx/{sig}");
        let amt_note = expected_usdc
            .map(|a| format!(" esperado={a} USDC"))
            .unwrap_or_default();
        format!(
            "USDC: PAID ({n} sig ok) latest={sig_short} conf={conf}{amt_note}\nEXPLORER: {explorer}",
            n = successful.len(),
        )
    };

    let pix_status = if pix_marked_paid {
        "PIX: PAID (marcado pelo operador — SPI/banco NÃO verificado por esta tool)".to_string()
    } else {
        "PIX: PENDING (tool não vê SPI do banco; use pix_marked_paid=true se confirmou)".to_string()
    };

    let overall = match (pix_marked_paid, usdc_paid) {
        (true, true) => "OVERALL: ambos trilhos com indício de pagamento",
        (true, false) => "OVERALL: PIX marcado; USDC PENDING",
        (false, true) => "OVERALL: USDC PAID; PIX não confirmado",
        (false, false) => "OVERALL: PENDING nos dois trilhos",
    };

    format!("INVOICE: {id}\nREF: {ref_short}\n{usdc_status}\n{pix_status}\n{overall}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn sig(signature: &str, ok: bool, memo: Option<&str>) -> SignatureInfo {
        SignatureInfo {
            signature: signature.into(),
            slot: 1,
            err: if ok {
                None
            } else {
                Some(json!({"InstructionError": [0, "Custom"]}))
            },
            memo: memo.map(|s| s.into()),
            block_time: Some(1),
            confirmation_status: Some("finalized".into()),
        }
    }

    #[test]
    fn unpaid_when_empty() {
        let s = status_from_signatures("inv-1", "RefABC123456", &[], Some("10"), false);
        assert!(s.contains("USDC: PENDING"));
        assert!(s.contains("PIX: PENDING"));
        assert!(s.contains("OVERALL: PENDING"));
    }

    #[test]
    fn usdc_paid_pix_open() {
        let sigs = vec![sig("VeryLongSignature111ABCDEF", true, Some("PIX|BRL|inv-1|x"))];
        let s = status_from_signatures("inv-1", "RefABC123456", &sigs, Some("10"), false);
        assert!(s.contains("USDC: PAID"));
        assert!(s.contains("solscan.io/tx/"));
        assert!(s.contains("PIX: PENDING") || s.contains("PIX não confirmado"));
    }

    #[test]
    fn both_rails() {
        let sigs = vec![sig("SigOK", true, None)];
        let s = status_from_signatures("inv-2", "RefXYZ", &sigs, None, true);
        assert!(s.contains("ambos trilhos") || s.contains("USDC: PAID"));
        assert!(s.contains("PIX: PAID"));
    }
}
