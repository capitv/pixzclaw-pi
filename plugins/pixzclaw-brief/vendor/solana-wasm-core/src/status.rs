//! Invoice status shaping from Solana signature lists (+ optional PIX flag).

use std::cmp::Ordering;

use crate::amount::{compare_units_to_decimal, format_minor_units};
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
        let conf = latest.confirmation_status.as_deref().unwrap_or("unknown");
        let amt_note = expected_usdc
            .map(|a| format!(" esperado={a} USDC"))
            .unwrap_or_default();
        format!(
            "USDC: PAID ({n} sig ok) conf={conf}{amt_note}",
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

    // Same shape as the verified builder: verdict outside, checkable
    // identifiers inside a fence. Divergence between the two would be a trap
    // for whoever reaches for this one next.
    let mut out =
        format!("INVOICE: {id}\n{usdc_status}\n{pix_status}\n{overall}\n```\nREF: https://solscan.io/account/{reference}");
    if let Some(sig) = successful.first().map(|s| s.signature.as_str()) {
        out.push_str(&format!("\nEXPLORER: https://solscan.io/tx/{sig}"));
    }
    out.push_str("\n```\n");
    out.push_str(VERBATIM_HINT);
    out
}

/// Verified USDC settlement detail extracted from `getTransaction`.
///
/// `received_units` is the net amount of the invoice mint **received by the
/// merchant** (`post − pre` token balances), in **minor units** at `decimals`.
/// It is an exact integer on purpose: this is the number the merchant is told
/// they were paid, so no part of its path may go through floating point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsdcReceipt {
    /// Net amount received by the merchant for the invoice mint, minor units.
    pub received_units: u128,
    /// Decimals of the invoice mint, as reported by the RPC.
    pub decimals: u32,
    /// Block time (unix seconds) of the paying transaction, if known.
    pub block_time: Option<i64>,
}

/// Value-aware invoice status.
///
/// Unlike [`status_from_signatures`] (which marks USDC PAID on the mere
/// existence of a successful signature), this checks the **amount actually
/// received by the merchant**:
///
/// - `USDC: PAID ✅` when received **equals** expected, to the minor unit.
/// - `USDC: UNDERPAID ⚠️` when `0 < received < expected` — including by one
///   minor unit. There is no tolerance band: a shortfall is a shortfall.
/// - `USDC: OVERPAID` when received > expected (still counts as paid).
/// - `USDC: RECEBIDO X` when **no** expected amount was provided but funds
///   arrived.
/// - `USDC: PENDING` when nothing arrived.
/// - `USDC: SIG OK (valor não verificado …)` when a successful signature exists
///   but `getTransaction` could not confirm the amount (`verified == None`),
///   **or** when an `expected_usdc` was supplied that cannot be used (a
///   wrong-locale `"27,27"`, a stray currency symbol, a zero). An unusable
///   expectation is not the same as no expectation: answering it with
///   `RECEBIDO` would hand a receipt to anyone who sent one dust unit.
///   This **never** claims PAID without a confirmed value.
///
/// The comparison is exact integer arithmetic on minor units — the same
/// discipline the issuing side ([`crate::amount`]) already used.
///
/// When paid with a confirmed value a shareable PT-BR receipt block is appended
/// for the merchant to forward to the customer.
pub fn status_from_signatures_verified(
    invoice_id: &str,
    reference: &str,
    sigs: &[SignatureInfo],
    verified: Option<UsdcReceipt>,
    expected_usdc: Option<&str>,
    pix_marked_paid: bool,
) -> String {
    let id = if invoice_id.trim().is_empty() {
        "(unknown)"
    } else {
        invoice_id.trim()
    };

    let successful: Vec<&SignatureInfo> = sigs.iter().filter(|s| s.is_success()).collect();
    let failed = sigs.len().saturating_sub(successful.len());

    // (usdc status text, confirmed-paid flag, optional receipt block)
    let (usdc_status, usdc_confirmed, receipt) = if successful.is_empty() {
        let text = if sigs.is_empty() {
            "USDC: PENDING (nenhuma assinatura no reference)".to_string()
        } else {
            format!("USDC: PENDING ({failed} sig(s) com erro; nenhuma sucesso)")
        };
        (text, false, None)
    } else {
        let latest = successful[0];
        let sig = latest.signature.as_str();
        let block_time = verified
            .as_ref()
            .and_then(|v| v.block_time)
            .or(latest.block_time);

        match &verified {
            // getTransaction unavailable / no meta → honest degrade.
            None => {
                let text =
                    format!("USDC: SIG OK (valor não verificado — RPC não retornou a transação)");
                (text, false, None)
            }
            Some(v) => {
                let recv_str = format_minor_units(v.received_units, v.decimals);
                // Three distinct states, and collapsing the last two is how a
                // dust payment used to buy a receipt: *no* expectation given
                // (report what arrived), an expectation that parses exactly and
                // is positive (compare), or an expectation that was given and
                // cannot be used — a wrong-locale `"27,27"`, a stray `R$`, a
                // zero. That last case must not be answered with a verdict.
                let expected_raw = expected_usdc.map(str::trim).filter(|s| !s.is_empty());
                let expected = expected_raw.and_then(|s| {
                    compare_units_to_decimal(v.received_units, v.decimals, s)
                        .ok()
                        .filter(|c| c.expected_units > 0)
                });
                let expected_unusable = expected_raw.is_some() && expected.is_none();

                if v.received_units == 0 {
                    // Successful signature but no USDC reached the merchant.
                    let text =
                        format!("USDC: PENDING (assinatura sem transferência de USDC ao lojista)");
                    (text, false, None)
                } else if expected_unusable {
                    // An expectation was stated and could not be used. There is
                    // nothing to compare against, so there is no verdict — and
                    // emphatically no receipt for whatever did arrive.
                    let bad = short_label(&echo_safe(expected_raw.unwrap_or("")), 16);
                    let text = format!(
                        "USDC: SIG OK (recebido {recv_str}, mas expected_usdc inválido: \
                         {bad} — valor não comparado)"
                    );
                    (text, false, None)
                } else if let Some(cmp) = expected {
                    let exp_str = &cmp.expected_fmt;
                    match cmp.ordering {
                        Ordering::Less => {
                            let missing = &cmp.diff;
                            let text = format!(
                                "USDC: UNDERPAID ⚠️ (recebido {recv_str} de {exp_str} USDC — faltam {missing})"
                            );
                            (text, false, None)
                        }
                        Ordering::Greater => {
                            let excess = &cmp.diff;
                            let text = format!(
                                "USDC: OVERPAID (recebido {recv_str}, esperado {exp_str}; excedente {excess}) ✅"
                            );
                            let rc = build_receipt(id, &recv_str, block_time, sig);
                            (text, true, Some(rc))
                        }
                        Ordering::Equal => {
                            let text =
                                format!("USDC: PAID ✅ (recebido {recv_str} de {exp_str} USDC)");
                            let rc = build_receipt(id, &recv_str, block_time, sig);
                            (text, true, Some(rc))
                        }
                    }
                } else {
                    // Funds arrived but no expected amount to compare against.
                    let text =
                        format!("USDC: RECEBIDO {recv_str} (sem valor esperado para comparar)");
                    let rc = build_receipt(id, &recv_str, block_time, sig);
                    (text, true, Some(rc))
                }
            }
        }
    };

    let pix_status = if pix_marked_paid {
        "PIX: PAID (marcado pelo operador — SPI/banco NÃO verificado por esta tool)".to_string()
    } else {
        "PIX: PENDING (tool não vê SPI do banco; use pix_marked_paid=true se confirmou)".to_string()
    };

    let overall = match (pix_marked_paid, usdc_confirmed) {
        (true, true) => "OVERALL: ambos trilhos com indício de pagamento",
        (true, false) => "OVERALL: PIX marcado; USDC não confirmado",
        (false, true) => "OVERALL: USDC PAID (valor conferido); PIX não confirmado",
        (false, false) => "OVERALL: PENDING (USDC não confirmado por valor)",
    };

    // The verdict is prose the agent may rewrite. Everything a reader could
    // *check* goes inside a fenced block instead, because asking nicely lost:
    // v0.3.3 shipped an instruction naming REF: and EXPLORER: as untouchable
    // and the agent still answered "a fatura está PENDING" in prose with the
    // reference dropped. A fence does not ask. In every invoice card ever
    // emitted, including the versions where the agent rewrote everything
    // around it, the fenced PIX block came through byte-for-byte.
    //
    // The cost is real and taken deliberately: a URL inside a fence is not
    // tappable in Telegram. A link that cannot be tapped can still be copied;
    // a link the agent deleted is not evidence of anything.
    let mut out =
        format!("INVOICE: {id}\n{usdc_status}\n{pix_status}\n{overall}\n```\nREF: https://solscan.io/account/{reference}");
    if let Some(url) = successful.first().map(|s| s.signature.as_str()) {
        out.push_str(&format!("\nEXPLORER: https://solscan.io/tx/{url}"));
    }
    out.push_str("\n```");
    if let Some(rc) = receipt {
        out.push('\n');
        out.push_str(&rc);
    }
    out.push('\n');
    out.push_str(VERBATIM_HINT);
    // Settled with a confirmed amount → tell the agent to stop any watcher.
    // Never emitted on PENDING / UNDERPAID / SIG OK: the reminder must keep
    // running until the value is actually confirmed.
    if usdc_confirmed {
        out.push('\n');
        out.push_str(SETTLED_CRON_HINT);
    }
    out
}

/// A payment that reached the merchant without carrying the invoice reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnreferencedMatch {
    /// Signature of the transaction that moved the funds.
    pub signature: String,
    /// Net minor units the merchant received in it.
    pub received_units: u128,
    /// Decimals of the mint.
    pub decimals: u32,
    /// Block time (unix seconds), if the RPC reported one.
    pub block_time: Option<i64>,
}

/// Status for a payment that matches the invoice **by amount only**.
///
/// Phantom reads a Solana Pay URI, takes the recipient and the amount — it
/// shows the exact figure on screen — and then builds a plain SPL transfer,
/// dropping the `reference` account. Measured on mainnet: `3UQpJTip…` moved
/// exactly the invoiced 0.181818 USDC to the right merchant, carrying neither
/// the reference nor a memo. The reference scan cannot see that payment, so
/// the merchant was told PENDING while the money was already in their account.
///
/// The answer is a **separate and weaker verdict**, never `PAID`. What is known
/// is that the right amount of the right mint reached the right account inside
/// the lookback window. What is not known is that it belongs to *this* invoice:
/// two invoices for the same amount on the same day are indistinguishable this
/// way, which is exactly the ambiguity the reference existed to remove.
///
/// So it says so, in the verdict, in the merchant's language — and emits no
/// receipt and no watcher teardown, because there is nothing here worth handing
/// a customer as proof, and the watch must keep running until something is.
///
/// It also closes: the merchant is the one person who *does* know whether this
/// payment is theirs — they issued the invoice, watched it scanned, saw the
/// amount — so the block ends by naming the sentence that settles it. Stating a
/// doubt and leaving the reader holding it is not honesty, it is just an
/// unfinished answer; the first version of this verdict stopped there and read
/// as strange for exactly that reason.
pub fn status_unreferenced_match(
    invoice_id: &str,
    reference: &str,
    m: &UnreferencedMatch,
    total_matches: usize,
    pix_marked_paid: bool,
) -> String {
    let id = if invoice_id.trim().is_empty() {
        "(unknown)"
    } else {
        invoice_id.trim()
    };
    let recv_str = format_minor_units(m.received_units, m.decimals);
    let when = match m.block_time {
        Some(ts) => format_unix_utc(ts),
        None => "data indisponível".to_string(),
    };

    let pix_status = if pix_marked_paid {
        "PIX: PAID (marcado pelo operador — SPI/banco NÃO verificado por esta tool)"
    } else {
        "PIX: PENDING (tool não vê SPI do banco; use pix_marked_paid=true se confirmou)"
    };

    // More than one transfer of the same amount is the case where matching by
    // amount is worth least, so it is the one the merchant most needs told.
    // Naming a count is also checkable: they can look at their own account.
    let ambiguity = if total_matches > 1 {
        format!(
            " ATENÇÃO: {total_matches} transferências deste mesmo valor entraram na \
             janela consultada — a mostrada abaixo é a mais recente, e NÃO há como \
             saber qual delas é desta fatura."
        )
    } else {
        " Se você emitiu outra fatura do mesmo valor, este pagamento pode ser da outra.".to_string()
    };

    let mut out = format!(
        "INVOICE: {id}\n\
         USDC: PROVÁVEL ⚠️ (recebido {recv_str} em {when} — valor, moeda e destino batem \
         com esta fatura, mas a transação NÃO carrega a reference: a carteira do pagador \
         não a incluiu. Isso é indício, não prova.{ambiguity})\n\
         {pix_status}\n\
         OVERALL: PENDING (indício de pagamento, sem prova de vínculo com esta fatura)\n\
         ```\n\
         REF: https://solscan.io/account/{reference}\n\
         EXPLORER: https://solscan.io/tx/{sig}\n\
         ```\n\
         👉 Só você pode fechar isso: se confere que este pagamento é da fatura {id}, \
         responda “confirmo o pagamento da {id}” e eu registro com comprovante.\n",
        sig = m.signature,
    );
    out.push_str(VERBATIM_HINT);
    out
}

/// Status for a transaction the operator named, rather than one the tool found.
///
/// The natural human move when a wallet drops the reference: the payer sends
/// the transaction hash. Phantom shows it, it is one paste, and it is strictly
/// better evidence than matching by amount — each customer hands over *their*
/// transaction, so two invoices of the same value stop colliding.
///
/// What the chain proves here is the money: this transaction moved exactly this
/// many units of this mint into this merchant's account. What it does not prove
/// is the invoice: nothing on-chain ties the transfer to this id, because the
/// reference is not in it. That link is the operator's assertion — the same
/// standing as `pix_marked_paid`, and labelled as such rather than blurred into
/// the verdict.
///
/// `received` is `None` when `getTransaction` could not be read exactly. That is
/// not "no payment", it is "no answer", and it degrades instead of guessing.
pub fn status_from_declared_tx(
    invoice_id: &str,
    reference: &str,
    signature: &str,
    received: Option<UsdcReceipt>,
    expected_usdc: Option<&str>,
    pix_marked_paid: bool,
) -> String {
    let id = if invoice_id.trim().is_empty() {
        "(unknown)"
    } else {
        invoice_id.trim()
    };
    let origin = "vínculo com a fatura informado por você, não pela chain: \
                  esta transação não carrega a reference";

    let (usdc_status, settled, receipt) = match &received {
        None => (
            "USDC: SIG OK (transação informada não pôde ser lida pelo RPC — valor não conferido)"
                .to_string(),
            false,
            None,
        ),
        Some(v) if v.received_units == 0 => (
            "USDC: NÃO CONFERE ❌ (a transação informada existe, mas não transferiu \
             esta moeda para a sua carteira. Confira se colou o hash certo.)"
                .to_string(),
            false,
            None,
        ),
        Some(v) => {
            let recv_str = format_minor_units(v.received_units, v.decimals);
            let expected_raw = expected_usdc.map(str::trim).filter(|s| !s.is_empty());
            let cmp = expected_raw.and_then(|s| {
                compare_units_to_decimal(v.received_units, v.decimals, s)
                    .ok()
                    .filter(|c| c.expected_units > 0)
            });
            match cmp {
                None => (
                    format!("USDC: RECEBIDO {recv_str} na transação informada ({origin})"),
                    true,
                    Some(build_receipt(id, &recv_str, v.block_time, signature)),
                ),
                Some(c) => match c.ordering {
                    Ordering::Less => (
                        format!(
                            "USDC: UNDERPAID ⚠️ (a transação informada trouxe {recv_str} de \
                             {exp} USDC — faltam {miss})",
                            exp = c.expected_fmt,
                            miss = c.diff
                        ),
                        false,
                        None,
                    ),
                    Ordering::Greater => (
                        format!(
                            "USDC: OVERPAID ✅ (a transação informada trouxe {recv_str}, \
                             esperado {exp}; excedente {ex} — {origin})",
                            exp = c.expected_fmt,
                            ex = c.diff
                        ),
                        true,
                        Some(build_receipt(id, &recv_str, v.block_time, signature)),
                    ),
                    Ordering::Equal => (
                        format!(
                            "USDC: PAID ✅ (valor conferido na transação informada: {recv_str} \
                             de {exp} USDC — {origin})",
                            exp = c.expected_fmt
                        ),
                        true,
                        Some(build_receipt(id, &recv_str, v.block_time, signature)),
                    ),
                },
            }
        }
    };

    let pix_status = if pix_marked_paid {
        "PIX: PAID (marcado pelo operador — SPI/banco NÃO verificado por esta tool)"
    } else {
        "PIX: PENDING (tool não vê SPI do banco; use pix_marked_paid=true se confirmou)"
    };
    let overall = if settled {
        "OVERALL: USDC conferido na transação informada; vínculo com a fatura afirmado pelo operador"
    } else {
        "OVERALL: PENDING (transação informada não fecha esta fatura)"
    };

    let mut out = format!(
        "INVOICE: {id}\n{usdc_status}\n{pix_status}\n{overall}\n\
         ```\nREF: https://solscan.io/account/{reference}\n\
         EXPLORER: https://solscan.io/tx/{signature}\n```"
    );
    if let Some(rc) = receipt {
        out.push('\n');
        out.push_str(&rc);
    }
    out.push('\n');
    out.push_str(VERBATIM_HINT);
    if settled {
        out.push('\n');
        out.push_str(SETTLED_CRON_HINT);
    }
    out
}

/// Agent-facing instruction appended to every status block.
///
/// The block is read by a model before it reaches a human, and a model
/// summarising a status into friendly prose is normally the right thing. It is
/// the wrong thing for two of these lines. `REF:` and the Solscan URL are
/// *verifiable identifiers*: one wrong character points at a different
/// transaction, or at none, and the reader has no way to tell. A status answer
/// that says "recebi 1 de 10, faltam 9" without the explorer link has dropped
/// the only part a merchant — or a judge — can check independently, and it
/// reads exactly as convincing as one that kept it.
///
/// Measured on a live host: asked whether an invoice had been paid, the agent
/// rewrote this block as prose and silently dropped `REF:`. Same failure the
/// invoice card had, same fix.
///
/// Deliberately narrow. It does not demand the whole block verbatim, because
/// prose around the verdict is genuinely better for the merchant; it pins only
/// the lines that stop being evidence when they are retyped.
pub const VERBATIM_HINT: &str = "[sistema] Resuma o veredito com suas palavras — é melhor \
assim. Os blocos de código acima são saída de máquina: repasse-os inteiros, sem reescrever, \
sem encurtar, sem redigitar. São identificadores verificáveis — um caractere trocado aponta \
para outra transação, ou para nenhuma, e quem lê não tem como perceber.";

/// Agent-facing instruction appended after the shareable receipt when the
/// invoice is settled with a confirmed amount. Always the last line, outside
/// the receipt block — it must stay below [`VERBATIM_HINT`], whose "as linhas
/// acima" would otherwise scope over it and invite the agent to echo an
/// internal cron instruction at the merchant.
pub const SETTLED_CRON_HINT: &str = "[sistema] Fatura liquidada: se existir um lembrete cron desta fatura, remova-o (cron_remove) e não agende novos.";

/// Narrow a caller-supplied string down to characters that are safe to echo
/// back into the output.
///
/// The status block is a fixed shape of lines that a model reads; an
/// `expected_usdc` arriving from a tool call must not be able to introduce
/// newlines or markup into it just because it was rejected.
fn echo_safe(s: &str) -> String {
    s.chars()
        .filter(|&c| c.is_ascii_alphanumeric() || matches!(c, '.' | ',' | '-' | '+'))
        .collect()
}

/// Build the shareable PT-BR receipt block for a confirmed USDC payment.
fn build_receipt(
    invoice_id: &str,
    received_str: &str,
    block_time: Option<i64>,
    sig: &str,
) -> String {
    let date = match block_time {
        Some(ts) => format_unix_utc(ts),
        None => "data indisponível".to_string(),
    };
    // Fenced for the same reason the evidence block is, and for one more: the
    // merchant forwards this to whoever paid. A receipt the agent reworded on
    // the way out is a receipt the customer cannot match against their own
    // wallet history. The fence also replaces the drawn rules that used to
    // bound the block.
    //
    // The instruction to forward stays *outside* — it is addressed to the
    // merchant, and it must not travel to the customer along with the receipt.
    // It leads rather than trails so that a line of prose always separates this
    // fence from the evidence fence above it: two fences on consecutive lines
    // read as one empty code block, which would spill the receipt back into
    // plain text and undo the protection.
    format!(
        "👉 Encaminhe o bloco abaixo ao cliente como comprovante.\n\
         ```\n\
         🧾 RECIBO — INVOICE #{invoice_id}\n\
         ✅ Pago em USDC (Solana)\n\
         Valor: {received_str} USDC (R$ equivalente na fatura)\n\
         Data: {date}\n\
         🔗 https://solscan.io/tx/{sig}\n\
         ```"
    )
}

/// Convert a unix timestamp (seconds, UTC) to `YYYY-MM-DD HH:MM UTC`.
///
/// Pure integer civil-date conversion (Howard Hinnant's algorithm) — no
/// external crate, no system clock. Valid for the full proleptic Gregorian
/// range and negative timestamps.
fn format_unix_utc(ts: i64) -> String {
    let days = ts.div_euclid(86_400);
    let secs = ts.rem_euclid(86_400);
    let hour = secs / 3_600;
    let minute = (secs % 3_600) / 60;

    // civil_from_days: days since 1970-01-01 → (year, month, day).
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097; // [0, 146096]
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let day = doy - (153 * mp + 2) / 5 + 1; // [1, 31]
    let month = if mp < 10 { mp + 3 } else { mp - 9 }; // [1, 12]
    let year = if month <= 2 { y + 1 } else { y };

    format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02} UTC")
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
        let sigs = vec![sig(
            "VeryLongSignature111ABCDEF",
            true,
            Some("PIX|BRL|inv-1|x"),
        )];
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

    /// Receipt from a decimal string, parsed exactly at USDC's 6 decimals.
    fn recv(amount: &str) -> Option<UsdcReceipt> {
        Some(UsdcReceipt {
            received_units: crate::amount::parse_decimal(amount, 6).unwrap().value,
            decimals: 6,
            block_time: Some(1_700_000_000),
        })
    }

    #[test]
    fn verified_paid_exact_with_receipt() {
        let sigs = vec![sig("VeryLongSignaturePaid1", true, None)];
        let s = status_from_signatures_verified(
            "inv-001",
            "RefABC123456",
            &sigs,
            recv("27.27"),
            Some("27.27"),
            false,
        );
        assert!(s.contains("USDC: PAID ✅"), "{s}");
        assert!(s.contains("🧾 RECIBO — INVOICE #inv-001"), "{s}");
        assert!(s.contains("Valor: 27.27 USDC"), "{s}");
        assert!(s.contains("2023-11-14"), "date from block_time: {s}");
        assert!(s.contains("Encaminhe o bloco abaixo"), "{s}");
        assert!(s.contains("USDC PAID (valor conferido)"), "{s}");
    }

    /// FURO C: there is no tolerance band. 99.5% of an invoice is an
    /// underpayment, and so is a shortfall of a single minor unit.
    #[test]
    fn verified_no_tolerance_band() {
        let sigs = [sig("Sig", true, None)];

        // The old rule called this PAID and issued a receipt: 0.5% of a
        // R$ 1.000 invoice walked away free.
        let s = status_from_signatures_verified(
            "inv-1",
            "Ref",
            &sigs,
            recv("99.6"),
            Some("100"),
            false,
        );
        assert!(s.contains("USDC: UNDERPAID ⚠️"), "{s}");
        assert!(s.contains("recebido 99.6 de 100 USDC — faltam 0.4"), "{s}");
        assert!(!s.contains("RECIBO"), "no receipt on a shortfall: {s}");
        assert!(!s.contains("cron_remove"), "watcher must keep running: {s}");

        // One millionth of a USDC short is still short.
        let s = status_from_signatures_verified(
            "inv-1",
            "Ref",
            &sigs,
            recv("99.999999"),
            Some("100"),
            false,
        );
        assert!(s.contains("USDC: UNDERPAID ⚠️"), "{s}");
        assert!(s.contains("faltam 0.000001"), "{s}");

        // Exactly the invoiced amount is PAID.
        let s =
            status_from_signatures_verified("inv-1", "Ref", &sigs, recv("100"), Some("100"), false);
        assert!(s.contains("USDC: PAID ✅"), "{s}");
    }

    /// A decimal expected value that survived the f64 round-trip badly before.
    #[test]
    fn verified_exact_decimal_expected_is_paid() {
        let sigs = [sig("Sig", true, None)];
        for amount in ["0.1", "0.3", "1.1", "27.272727", "0.000001"] {
            let s = status_from_signatures_verified(
                "inv-1",
                "Ref",
                &sigs,
                recv(amount),
                Some(amount),
                false,
            );
            assert!(s.contains("USDC: PAID ✅"), "{amount}: {s}");
        }
    }

    #[test]
    fn verified_underpaid_no_receipt() {
        let sigs = [sig("Sig", true, None)];
        let s =
            status_from_signatures_verified("inv-1", "Ref", &sigs, recv("0.01"), Some("90"), false);
        assert!(s.contains("USDC: UNDERPAID ⚠️"), "{s}");
        assert!(s.contains("faltam"), "{s}");
        assert!(!s.contains("RECIBO"), "no receipt when underpaid: {s}");
        assert!(s.contains("PENDING (USDC não confirmado por valor)"), "{s}");
    }

    /// The exact line the demo video freezes on.
    #[test]
    fn verified_underpaid_matches_video_script_wording() {
        let sigs = [sig("Sig", true, None)];
        let s = status_from_signatures_verified(
            "INV-DEMO-A",
            "Ref",
            &sigs,
            recv("1"),
            Some("10"),
            false,
        );
        assert!(
            s.contains("USDC: UNDERPAID ⚠️ (recebido 1 de 10 USDC — faltam 9)"),
            "{s}"
        );
    }

    #[test]
    fn verified_overpaid_counts_as_paid() {
        let sigs = [sig("Sig", true, None)];
        let s =
            status_from_signatures_verified("inv-1", "Ref", &sigs, recv("120"), Some("100"), false);
        assert!(s.contains("USDC: OVERPAID"), "{s}");
        assert!(s.contains("excedente 20"), "{s}");
        assert!(s.contains("RECIBO"), "receipt on overpaid: {s}");
    }

    #[test]
    fn verified_no_expected_reports_received() {
        let sigs = [sig("Sig", true, None)];
        let s = status_from_signatures_verified("inv-1", "Ref", &sigs, recv("42.5"), None, false);
        assert!(s.contains("USDC: RECEBIDO 42.5"), "{s}");
        assert!(s.contains("sem valor esperado"), "{s}");
        assert!(s.contains("RECIBO"), "{s}");
    }

    /// An `expected_usdc` that was supplied but cannot be used must NOT be
    /// answered as if none had been supplied.
    ///
    /// `RECEBIDO` is a settled verdict: receipt, `usdc_confirmed`, cron
    /// teardown. Reaching it by typing `"27,27"` (the way a Brazilian merchant
    /// writes it) would mean one dust unit on the reference buys a receipt.
    #[test]
    fn verified_unusable_expected_degrades_instead_of_settling() {
        let sigs = [sig("Sig", true, None)];
        for bad in ["abc", "-5", "0", "R$ 27,27", "27,27"] {
            let s = status_from_signatures_verified(
                "inv-1",
                "Ref",
                &sigs,
                recv("0.000001"),
                Some(bad),
                false,
            );
            assert!(s.contains("USDC: SIG OK"), "{bad}: {s}");
            assert!(s.contains("expected_usdc inválido"), "{bad}: {s}");
            assert!(!s.contains("USDC: RECEBIDO"), "{bad}: {s}");
            assert!(!s.contains("PAID ✅"), "{bad}: {s}");
            assert!(!s.contains("RECIBO"), "no receipt: {bad}: {s}");
            assert!(
                !s.contains("cron_remove"),
                "watcher keeps running: {bad}: {s}"
            );
            assert!(
                s.contains("PENDING (USDC não confirmado por valor)"),
                "{bad}: {s}"
            );
        }
    }

    /// A blank / absent expectation is a different thing and keeps reporting
    /// what arrived, as documented.
    #[test]
    fn verified_absent_expected_still_reports_received() {
        let sigs = [sig("Sig", true, None)];
        for none_ish in [None, Some("  "), Some("")] {
            let s = status_from_signatures_verified(
                "inv-1",
                "Ref",
                &sigs,
                recv("42.5"),
                none_ish,
                false,
            );
            assert!(s.contains("USDC: RECEBIDO 42.5"), "{none_ish:?}: {s}");
        }
    }

    /// A mint with different decimals is rendered at its own precision.
    #[test]
    fn verified_respects_mint_decimals() {
        let sigs = [sig("Sig", true, None)];
        let nine = Some(UsdcReceipt {
            received_units: 1_500_000_000,
            decimals: 9,
            block_time: Some(1_700_000_000),
        });
        let s = status_from_signatures_verified("inv-1", "Ref", &sigs, nine, Some("1.5"), false);
        assert!(
            s.contains("USDC: PAID ✅ (recebido 1.5 de 1.5 USDC)"),
            "{s}"
        );
    }

    #[test]
    fn verified_degrades_when_tx_unavailable() {
        let sigs = [sig("Sig", true, None)];
        let s = status_from_signatures_verified("inv-1", "Ref", &sigs, None, Some("90"), false);
        assert!(s.contains("USDC: SIG OK"), "{s}");
        assert!(s.contains("valor não verificado"), "{s}");
        assert!(!s.contains("USDC: PAID"), "never PAID without value: {s}");
        assert!(!s.contains("RECIBO"), "{s}");
    }

    #[test]
    fn verified_zero_received_is_pending() {
        let sigs = [sig("Sig", true, None)];
        let s =
            status_from_signatures_verified("inv-1", "Ref", &sigs, recv("0"), Some("90"), false);
        assert!(s.contains("USDC: PENDING"), "{s}");
        assert!(s.contains("sem transferência de USDC"), "{s}");
    }

    #[test]
    fn verified_empty_sigs_pending() {
        let s = status_from_signatures_verified("inv-1", "Ref", &[], None, Some("90"), false);
        assert!(s.contains("USDC: PENDING (nenhuma assinatura"), "{s}");
    }

    /// Paid-with-confirmed-value cases must end with the cron-teardown line,
    /// after (and outside) the shareable receipt.
    #[test]
    fn settled_cron_hint_on_paid_overpaid_and_recebido() {
        let sigs = vec![sig("SigPaid", true, None)];
        let cases = [
            ("PAID", recv("27.27"), Some("27.27")),
            ("OVERPAID", recv("120"), Some("100")),
            ("RECEBIDO", recv("42.5"), None),
        ];
        for (name, verified, expected) in cases {
            let s =
                status_from_signatures_verified("inv-1", "Ref", &sigs, verified, expected, false);
            let last = s.lines().last().unwrap();
            assert_eq!(last, SETTLED_CRON_HINT, "{name}: {s}");
            assert!(last.starts_with("[sistema]"), "{name}");
            assert!(last.contains("cron_remove"), "{name}");
            assert_eq!(s.matches("cron_remove").count(), 1, "{name}: {s}");
            // The verbatim hint sits above it, so its "as linhas acima" cannot
            // scope over the cron instruction and invite the agent to echo an
            // internal directive at the merchant.
            assert!(
                s.find(VERBATIM_HINT).unwrap() < s.find(SETTLED_CRON_HINT).unwrap(),
                "{name}: {s}"
            );
            // Receipt still intact and *before* the system line.
            let rc = s.find("🧾 RECIBO").unwrap_or_else(|| panic!("{name}: {s}"));
            assert!(rc < s.find(SETTLED_CRON_HINT).unwrap(), "{name}: {s}");
            assert!(s.contains("Encaminhe o bloco abaixo ao cliente"), "{name}");
        }
    }

    /// `REF:` and the Solscan URL are the only parts of a status a reader can
    /// check independently. A live agent answered "a fatura está PENDING"
    /// in prose and dropped `REF:` entirely, which reads exactly as convincing
    /// as the version that kept it. The hint must therefore ride on every
    /// verdict, not just the paid ones, and must name the lines it protects —
    /// a generic "não reescreva" was what the invoice card started with, and
    /// it lost to the model.
    #[test]
    fn every_verdict_fences_its_checkable_identifiers() {
        // The hint now points at the fence rather than listing line names: the
        // fence is what actually holds, and the hint only explains it.
        assert!(
            VERBATIM_HINT.contains("blocos de código"),
            "{VERBATIM_HINT}"
        );
        // It licenses prose around the verdict, so the agent is not pushed into
        // dumping a raw block at a merchant who asked a plain question.
        assert!(
            VERBATIM_HINT.contains("Resuma o veredito"),
            "{VERBATIM_HINT}"
        );

        let sigs = [sig("Sig", true, None)];
        let cases = [
            ("PENDING empty", &[][..], None, Some("90")),
            ("UNDERPAID", &sigs[..], recv("0.01"), Some("90")),
            ("SIG OK", &sigs[..], None, Some("90")),
            ("PAID", &sigs[..], recv("90"), Some("90")),
            ("RECEBIDO", &sigs[..], recv("42.5"), None),
        ];
        for (name, s_in, verified, expected) in cases {
            let s =
                status_from_signatures_verified("inv-1", "Ref", s_in, verified, expected, false);
            assert!(s.contains(VERBATIM_HINT), "{name}: {s}");
            assert_eq!(s.matches(VERBATIM_HINT).count(), 1, "{name}: {s}");

            // REF must sit *inside* a fence, not merely be present. The fence
            // is the whole mechanism: v0.3.3 asked for REF verbatim in prose
            // and the agent dropped it anyway.
            let fenced: Vec<&str> = s.split("```").skip(1).step_by(2).collect();
            assert!(
                fenced
                    .iter()
                    .any(|b| b.contains("REF: https://solscan.io/account/Ref")),
                "{name}: REF must be inside a code fence:\n{s}"
            );
            // Fences must balance, or Telegram renders the rest of the message
            // as one code block and the verdict becomes unreadable.
            assert_eq!(s.matches("```").count() % 2, 0, "{name}: {s}");
            // Two fences on consecutive lines read as one empty code block,
            // which spills whatever followed back into plain text — exactly
            // the protection this is here to provide. A line of prose must
            // always separate them.
            assert!(
                !s.contains("```\n```"),
                "{name}: adjacent fences collapse into an empty block:\n{s}"
            );
            // Any explorer URL is fenced too.
            if s.contains("solscan.io") {
                assert!(
                    fenced.iter().any(|b| b.contains("solscan.io")),
                    "{name}: explorer URL escaped the fence:\n{s}"
                );
            }
            // Never inside the forwardable receipt: the merchant sends that
            // block to a customer, and an internal directive must not travel
            // with it.
            if let Some(rc) = s.find("🧾 RECIBO") {
                assert!(rc < s.find(VERBATIM_HINT).unwrap(), "{name}: {s}");
            }
        }
    }

    /// The third distinct way this status lost its evidence, and the first fix
    /// that addresses the class instead of the case.
    ///
    /// The host redacts high-entropy base58 anywhere in chat *except* inside an
    /// `https://` URL. A 12-character truncation was dodging that by luck of
    /// the prefix: the same code produced `REF: FEXKHAX8CDf…` on one invoice
    /// and `REF: [REDACTED_HIGH_ENTROPY_TOKEN]` on the next. Fencing did not
    /// help — the redactor does not care about code blocks, it is a different
    /// threat from the agent rewriting things.
    ///
    /// So no identifier travels as bare text. Each one is a URL, which is also
    /// how it stops being truncated: `REF:` now carries the whole reference and
    /// points at the account page that proves it.
    #[test]
    fn no_identifier_travels_outside_an_https_url() {
        let reference = "A6vpxfrrsjenGU5hfiir6GuwxojfTMhsWxtBy9WRk8qd";
        let signature = "5xTdLmBQ1qWZ8vPmcVzKjR3nHgYbEwUaSfN2oXpJdCtA";
        let sigs = [sig(signature, true, None)];
        let cases = [
            ("PENDING", &[][..], None, Some("90")),
            ("SIG OK", &sigs[..], None, Some("90")),
            ("UNDERPAID", &sigs[..], recv("0.01"), Some("90")),
            ("PAID", &sigs[..], recv("90"), Some("90")),
        ];
        for (name, s_in, verified, expected) in cases {
            let s = status_from_signatures_verified(
                "inv-1", reference, s_in, verified, expected, false,
            );
            for id in [reference, signature] {
                for (i, _) in s.match_indices(id) {
                    let before = &s[..i];
                    assert!(
                        before.ends_with("solscan.io/account/")
                            || before.ends_with("solscan.io/tx/"),
                        "{name}: {id} appears as bare text; the host redacts that:\n{s}"
                    );
                }
            }
        }
        // And the reference is carried whole. A truncated identifier cannot be
        // pasted into an explorer, so it was never evidence of anything — it
        // only looked like it.
        let s = status_from_signatures_verified("inv-1", reference, &sigs, None, None, false);
        assert!(s.contains(reference), "{s}");
        assert!(!s.contains('…'), "no truncated identifiers remain:\n{s}");
    }

    /// Not settled (or value unconfirmed) → the watcher must keep running.
    #[test]
    fn no_settled_cron_hint_when_not_confirmed() {
        let sigs = [sig("Sig", true, None)];
        let cases = [
            // (label, sigs, verified, expected)
            ("PENDING empty", &[][..], None, Some("90")),
            ("PENDING zero", &sigs[..], recv("0"), Some("90")),
            ("UNDERPAID", &sigs[..], recv("0.01"), Some("90")),
            ("SIG OK", &sigs[..], None, Some("90")),
        ];
        for (name, s_in, verified, expected) in cases {
            let s =
                status_from_signatures_verified("inv-1", "Ref", s_in, verified, expected, false);
            assert!(!s.contains("cron_remove"), "{name}: {s}");
            assert!(!s.contains("Fatura liquidada"), "{name}: {s}");
            // The verbatim hint is unconditional and must not be mistaken for
            // the teardown line: an unpaid invoice still carries a REF the
            // agent must not retype.
            assert!(s.ends_with(VERBATIM_HINT), "{name}: {s}");
        }
    }

    /// PIX marked + USDC confirmed still ends with the teardown line.
    #[test]
    fn settled_cron_hint_with_pix_marked() {
        let sigs = [sig("Sig", true, None)];
        let s =
            status_from_signatures_verified("inv-9", "Ref", &sigs, recv("10"), Some("10"), true);
        assert_eq!(s.lines().last().unwrap(), SETTLED_CRON_HINT, "{s}");
    }

    /// The non-verified legacy shaper never emits the teardown line.
    #[test]
    fn legacy_shaper_has_no_cron_hint() {
        let sigs = [sig("Sig", true, None)];
        let s = status_from_signatures("inv-1", "Ref", &sigs, Some("10"), false);
        assert!(!s.contains("cron_remove"), "{s}");
    }

    #[test]
    fn unix_utc_formatting() {
        assert_eq!(format_unix_utc(0), "1970-01-01 00:00 UTC");
        assert_eq!(format_unix_utc(1_700_000_000), "2023-11-14 22:13 UTC");
        assert_eq!(format_unix_utc(1_609_459_200), "2021-01-01 00:00 UTC");
    }
}
