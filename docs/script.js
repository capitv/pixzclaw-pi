(function () {
  "use strict";

  /* ============================================================
     i18n dictionary — built from landing-copy.md
     Keys use dot-paths matching data-i18n attributes in index.html.
     Values may contain safe inline HTML (e.g. <code>) since this
     content is fully static/trusted.
     ============================================================ */

  var translations = {
    pt: {
      "a11y.skip": "Pular para o conteúdo",
      "a11y.ghLink": "Ver repositório no GitHub",
      "a11y.copy1": "Copiar comando",
      "a11y.copy2": "Copiar comandos",

      "hero.eyebrow": "Plugin ZeroClaw · T0/T1 · sem private key",
      "hero.headline": "Cobra em BRL, receba em PIX ou USDC.",
      "hero.sub": "Uma fatura, dois trilhos de pagamento. Seu cliente escolhe PIX (BRL) ou USDC (Solana Pay) — você opera tudo pelo Telegram, num Raspberry Pi de R$ 200.",
      "hero.ctaPrimary": "Instalar no meu Pi",
      "hero.ctaSecondary": "Ver no GitHub",
      "hero.chat.merchant": "Cobra R$ 150 do João, pedido 412",
      "hero.chat.invoiceTitle": "PixZClaw — Fatura #412",
      "hero.chat.amount": "R$ 150.00 · ₮ 27.27 USDC",
      "hero.chat.forward": "👉 Encaminhe esta mensagem ao cliente",
      "hero.chat.lock": "teto R$ 1000 · destino travado",

      "how.title": "Como funciona",
      "how.step1.title": "Cobre no chat.",
      "how.step1.body": "Você diz o valor em BRL no Telegram e o PixZClaw emite a fatura dual na hora.",
      "how.step2.title": "Cliente escolhe o trilho.",
      "how.step2.body": "Encaminhe a mensagem: ele paga por PIX (BRL) ou por USDC (Solana Pay) — mesma fatura, mesmo valor.",
      "how.step3.title": "Confira e comprove.",
      "how.step3.body": "O PixZClaw verifica o USDC on-chain, gera recibo e fecha seu caixa do dia.",

      "tools.title": "As 3 ferramentas",
      "tools.t0tooltip": "read-only",
      "tools.t1.tier": "T1 · emite fatura",
      "tools.t1.desc": "Emite uma fatura com dois trilhos: PIX Copia e Cola (EMV+CRC16 de verdade) e URL Solana Pay em USDC, com o mesmo invoice_id nos dois. QR codes por link, teto de valor e destino travado — pronto para encaminhar ao cliente.",
      "tools.t1.output": "🦞 PixZClaw — Fatura #412\n💰 R$ 150.00 · ₮ 27.27 USDC\n\n🇧🇷 PIX (BRL)\n📷 QR (toque): …\nOu copie o código (toque para copiar):\n00020126…5204000053039865802BR…6304AB12\n\n◎ Solana Pay (USDC)\n📷 QR (toque): …\nEscaneie com Phantom/Solflare.\n\n👉 Encaminhe esta mensagem ao cliente\n🧾 Fatura 412 · cotação R$/USDC usada: 5.5\n🔒 teto R$ 1000 · destino travado=sim",
      "tools.t2.tier": "T0 · só leitura",
      "tools.t2.desc": "Verifica o pagamento em USDC direto na blockchain: confere o valor que o lojista REALMENTE recebeu e responde PAID ✅, UNDERPAID ⚠️ (mostra quanto falta) ou OVERPAID. Soma pagamentos parciais, nunca diz PAID sem valor conferido e gera um recibo PT-BR com link do Solscan.",
      "tools.t2.output": "INVOICE: inv-412\nUSDC: PAID ✅ (recebido 27.27 de 27.27 USDC)\nEXPLORER: https://solscan.io/tx/…\nOVERALL: USDC PAID (valor conferido); PIX não confirmado\n──────────────────────\n🧾 RECIBO — INVOICE #inv-412\n✅ Pago em USDC (Solana)\nValor: 27.27 USDC (R$ equivalente na fatura)\nData: 2026-07-21 22:13 UTC\n🔗 https://solscan.io/tx/…\n──────────────────────",
      "tools.t3.tier": "T0 · o caixa",
      "tools.t3.desc": "O caixa do comerciante num card: saldo em USDC e SOL, e o fechamento das últimas 24h com as faturas pagas e seus ids. Ainda traz um sparkline de 7 dias e as últimas movimentações on-chain.",
      "tools.t3.output": "╭─ PixZClaw · Caixa ─────────────────╮\n│ Wallet     8xkR3pQ9mNvT2wLzAb7cD… │\n│ USDC       142.5                  │\n│ SOL (gas)  0.41 SOL               │\n╰───────────────────────────────────╯\n\nHoje (últimas 24h)\n• txs ok:      2\n• faturas PIX: 2\n• pagas: demo-1, inv-412\n\n7 dias\n• 7d ▁▃▅▂▇▁▂  (velho→novo)\n\nT0 read-only · sem chave · PixZClaw",

      "security.title": "Por que dá pra confiar",
      "security.item1.title": "Sem chaves privadas",
      "security.item1.body": "O plugin nunca guarda nem assina com private key. Só ferramentas T0/T1 — ele monta a cobrança, quem paga é o cliente na carteira dele.",
      "security.item2.title": "Valor conferido on-chain",
      "security.item2.body": "Nunca diz PAID sem checar quanto o lojista recebeu de verdade. Se o cliente pagar menos, marca UNDERPAID ⚠️ e mostra exatamente quanto falta.",
      "security.item3.title": "Teto de valor + destino travado",
      "security.item3.body": "Cada fatura respeita um teto (max_amount_brl) e um destino travado (recipient_locked) — o agente não redireciona o dinheiro nem estoura o limite.",
      "security.item4.title": "Prompt injection falha fechado",
      "security.item4.body": "Instrução maliciosa não fura o teto: um pedido de \"999999999,99\" bate no limite e é recusado, em vez de gerar a fatura.",
      "security.item5.title": "Honestidade no PIX",
      "security.item5.body": "O PIX bancário não aparece on-chain, então o PixZClaw nunca inventa \"pago\". Só marca o PIX como pago quando o operador confirma.",

      "install.title": "Instalação",
      "install.intro": "Roda em qualquer host ZeroClaw — inclusive num Raspberry Pi 3 — em três passos: instalar, configurar e reiniciar.",
      "install.step1.label": "Instale os 3 plugins da release v0.4.0 com um comando:",
      "install.step2.label": "Ajuste a chave PIX, o destino Solana e os limites. Use seus próprios valores:",
      "install.step3.label": "Reinicie o serviço para aplicar a config:",
      "install.copy": "copiar",
      "install.warning": "⚠️ <code>merchant_solana</code> é a sua chave PÚBLICA (pubkey), nunca a private key. O PixZClaw não pede — e não guarda — chave privada.",

      "faq.title": "Perguntas frequentes",
      "faq.q1.q": "Preciso de exchange ou conversão de moeda?",
      "faq.q1.a": "Não. PIX e USDC são dois trilhos de pagamento independentes na mesma fatura — não é on-ramp nem câmbio. O <code>brl_per_usdc</code> é só uma cotação offline pra montar o valor do Solana Pay.",
      "faq.q2.q": "E se o cliente pagar menos que o combinado?",
      "faq.q2.a": "O <code>invoice_status</code> confere o valor recebido on-chain e responde UNDERPAID ⚠️, mostrando exatamente quanto ainda falta. Ele soma pagamentos parciais e só marca PAID quando o valor bate.",
      "faq.q3.q": "O PIX confirma sozinho?",
      "faq.q3.a": "Não. O PIX do banco não é visível on-chain, então a ferramenta nunca diz \"pago\" por conta própria — o operador confirma o recebimento. É honestidade por design.",
      "faq.q4.q": "Onde isso roda?",
      "faq.q4.a": "Num Raspberry Pi 3 (ou superior) ou em qualquer host que rode o agente ZeroClaw. O hardware sai por uns R$ 200.",
      "faq.q5.q": "É seguro deixar um agente cobrar por mim?",
      "faq.q5.a": "Sim. São ferramentas T0/T1 (sem private key), com teto de valor, destino travado e proteção contra prompt injection. O agente monta a cobrança; quem paga é sempre o cliente na carteira dele.",

      "footer.credit": "Feito para o bounty ZeroClaw × Superteam Brasil. Rust puro + wasm32-wasip2 · 92 testes host · sem solana-sdk.",
      "footer.release": "Release v0.4.0",
      "footer.docs": "Docs ZeroClaw",
      "footer.solanapay": "Solana Pay spec",
      "footer.license": "Licenciado sob MIT / Apache-2.0."
    },

    en: {
      "a11y.skip": "Skip to content",
      "a11y.ghLink": "View repository on GitHub",
      "a11y.copy1": "Copy command",
      "a11y.copy2": "Copy commands",

      "hero.eyebrow": "ZeroClaw plugin · T0/T1 · no private key",
      "hero.headline": "Charge in BRL, get paid in PIX or USDC.",
      "hero.sub": "One invoice, two payment rails. Your customer picks PIX (BRL) or USDC (Solana Pay) — you run it all from Telegram, on a R$200 Raspberry Pi.",
      "hero.ctaPrimary": "Install on my Pi",
      "hero.ctaSecondary": "View on GitHub",
      "hero.chat.merchant": "Charge João R$150, order 412",
      "hero.chat.invoiceTitle": "PixZClaw — Invoice #412",
      "hero.chat.amount": "R$150.00 · ₮ 27.27 USDC",
      "hero.chat.forward": "👉 Forward this message to your customer",
      "hero.chat.lock": "cap R$1000 · recipient locked",

      "how.title": "How it works",
      "how.step1.title": "Charge in the chat.",
      "how.step1.body": "Say the BRL amount on Telegram and PixZClaw issues the dual invoice instantly.",
      "how.step2.title": "Customer picks a rail.",
      "how.step2.body": "Forward the message: they pay by PIX (BRL) or by USDC (Solana Pay) — same invoice, same amount.",
      "how.step3.title": "Verify and receipt.",
      "how.step3.body": "PixZClaw checks the USDC on-chain, generates a receipt, and closes your daily books.",

      "tools.title": "The 3 tools",
      "tools.t0tooltip": "read-only",
      "tools.t1.tier": "T1 · issues invoice",
      "tools.t1.desc": "Issues one invoice on two rails: PIX Copy & Paste (real EMV+CRC16) plus a Solana Pay USDC URL, sharing the same invoice_id. QR codes by link, amount cap, and locked recipient — ready to forward to the customer.",
      "tools.t1.output": "🦞 PixZClaw — Fatura #412\n💰 R$ 150.00 · ₮ 27.27 USDC\n\n🇧🇷 PIX (BRL)\n📷 QR (toque): …\nOu copie o código (toque para copiar):\n00020126…5204000053039865802BR…6304AB12\n\n◎ Solana Pay (USDC)\n📷 QR (toque): …\nEscaneie com Phantom/Solflare.\n\n👉 Encaminhe esta mensagem ao cliente\n🧾 Fatura 412 · cotação R$/USDC usada: 5.5\n🔒 teto R$ 1000 · destino travado=sim",
      "tools.t2.tier": "T0 · read-only",
      "tools.t2.desc": "Checks the USDC payment straight on-chain: it verifies the amount the merchant ACTUALLY received and answers PAID ✅, UNDERPAID ⚠️ (shows how much is missing), or OVERPAID. It sums partial payments, never says PAID without a verified amount, and generates a receipt with a Solscan link.",
      "tools.t2.output": "INVOICE: inv-412\nUSDC: PAID ✅ (recebido 27.27 de 27.27 USDC)\nEXPLORER: https://solscan.io/tx/…\nOVERALL: USDC PAID (valor conferido); PIX não confirmado\n──────────────────────\n🧾 RECIBO — INVOICE #inv-412\n✅ Pago em USDC (Solana)\nValor: 27.27 USDC (R$ equivalente na fatura)\nData: 2026-07-21 22:13 UTC\n🔗 https://solscan.io/tx/…\n──────────────────────",
      "tools.t3.tier": "T0 · the till",
      "tools.t3.desc": "The merchant's till in one card: USDC and SOL balance, plus the last-24h close with paid invoices and their ids. It also shows a 7-day sparkline and the latest on-chain activity.",
      "tools.t3.output": "╭─ PixZClaw · Caixa ─────────────────╮\n│ Wallet     8xkR3pQ9mNvT2wLzAb7cD… │\n│ USDC       142.5                  │\n│ SOL (gas)  0.41 SOL               │\n╰───────────────────────────────────╯\n\nHoje (últimas 24h)\n• txs ok:      2\n• faturas PIX: 2\n• pagas: demo-1, inv-412\n\n7 dias\n• 7d ▁▃▅▂▇▁▂  (velho→novo)\n\nT0 read-only · sem chave · PixZClaw",

      "security.title": "Why you can trust it",
      "security.item1.title": "No private keys",
      "security.item1.body": "The plugin never holds or signs with a private key. T0/T1 tools only — it builds the charge; the customer pays from their own wallet.",
      "security.item2.title": "Amount verified on-chain",
      "security.item2.body": "It never says PAID without checking what the merchant actually received. If the customer pays less, it flags UNDERPAID ⚠️ and shows exactly how much is missing.",
      "security.item3.title": "Amount cap + locked recipient",
      "security.item3.body": "Every invoice respects a cap (max_amount_brl) and a locked recipient (recipient_locked) — the agent can't reroute the money or blow past the limit.",
      "security.item4.title": "Prompt injection fails closed",
      "security.item4.body": "A malicious instruction can't break the cap: a request for \"999999999.99\" hits the limit and is refused instead of minting the invoice.",
      "security.item5.title": "Honesty on PIX",
      "security.item5.body": "Bank PIX isn't visible on-chain, so PixZClaw never fakes a \"paid\". It only marks PIX paid once the operator confirms.",

      "install.title": "Installation",
      "install.intro": "Runs on any ZeroClaw host — including a Raspberry Pi 3 — in three steps: install, configure, restart.",
      "install.step1.label": "Install the 3 plugins from the v0.4.0 release with one command:",
      "install.step2.label": "Set your PIX key, Solana recipient, and limits. Use your own values:",
      "install.step3.label": "Restart the service to apply the config:",
      "install.copy": "copy",
      "install.warning": "⚠️ <code>merchant_solana</code> is your PUBLIC key (pubkey), never the private key. PixZClaw doesn't ask for — and never stores — a private key.",

      "faq.title": "FAQ",
      "faq.q1.q": "Do I need an exchange or currency conversion?",
      "faq.q1.a": "No. PIX and USDC are two independent payment rails on the same invoice — not an on-ramp or currency exchange. The <code>brl_per_usdc</code> is just an offline quote used to build the Solana Pay amount.",
      "faq.q2.q": "What if the customer pays less than agreed?",
      "faq.q2.a": "<code>invoice_status</code> checks the received amount on-chain and answers UNDERPAID ⚠️, showing exactly how much is still missing. It sums partial payments and only marks PAID once the amount matches.",
      "faq.q3.q": "Does PIX confirm itself?",
      "faq.q3.a": "No. Bank PIX isn't visible on-chain, so the tool never claims \"paid\" on its own — the operator confirms receipt. Honesty by design.",
      "faq.q4.q": "Where does this run?",
      "faq.q4.a": "On a Raspberry Pi 3 (or newer) or any host running the ZeroClaw agent. The hardware runs about R$200.",
      "faq.q5.q": "Is it safe to let an agent charge on my behalf?",
      "faq.q5.a": "Yes. These are T0/T1 tools (no private key), with an amount cap, a locked recipient, and prompt-injection protection. The agent builds the charge; the customer always pays from their own wallet.",

      "footer.credit": "Built for the ZeroClaw × Superteam Brasil bounty. Pure Rust + wasm32-wasip2 · 92 host tests · no solana-sdk.",
      "footer.release": "Release v0.4.0",
      "footer.docs": "ZeroClaw Docs",
      "footer.solanapay": "Solana Pay spec",
      "footer.license": "Licensed under MIT / Apache-2.0."
    }
  };

  /* ============================================================
     Language toggle
     ============================================================ */

  var STORAGE_KEY = "pixzclaw-lang";
  var htmlEl = document.documentElement;
  var langButtons = document.querySelectorAll("[data-lang-btn]");

  function getStoredLang() {
    try {
      return localStorage.getItem(STORAGE_KEY);
    } catch (e) {
      return null;
    }
  }

  function storeLang(lang) {
    try {
      localStorage.setItem(STORAGE_KEY, lang);
    } catch (e) {
      /* ignore (private mode, etc.) */
    }
  }

  function applyLanguage(lang) {
    var dict = translations[lang] || translations.pt;

    document.querySelectorAll("[data-i18n]").forEach(function (el) {
      var key = el.getAttribute("data-i18n");
      var value = dict[key] != null ? dict[key] : translations.pt[key];
      if (value != null) {
        el.innerHTML = value;
      }
    });

    document.querySelectorAll("[data-i18n-aria]").forEach(function (el) {
      var key = el.getAttribute("data-i18n-aria");
      var value = dict[key] != null ? dict[key] : translations.pt[key];
      if (value != null) {
        el.setAttribute("aria-label", value);
      }
    });

    htmlEl.lang = lang === "pt" ? "pt-BR" : "en";

    langButtons.forEach(function (btn) {
      var isActive = btn.getAttribute("data-lang-btn") === lang;
      btn.classList.toggle("is-active", isActive);
      btn.setAttribute("aria-pressed", String(isActive));
    });

    storeLang(lang);
  }

  langButtons.forEach(function (btn) {
    btn.addEventListener("click", function () {
      applyLanguage(btn.getAttribute("data-lang-btn"));
    });
  });

  var initialLang = getStoredLang();
  if (initialLang !== "pt" && initialLang !== "en") {
    initialLang = "pt";
  }
  applyLanguage(initialLang);

  /* ============================================================
     Copy-to-clipboard for code blocks
     ============================================================ */

  document.querySelectorAll(".copy-btn").forEach(function (btn) {
    btn.addEventListener("click", function () {
      var targetId = btn.getAttribute("data-copy-target");
      var target = document.getElementById(targetId);
      if (!target) return;

      var text = target.textContent;

      var markCopied = function () {
        btn.classList.add("is-copied");
        var originalHTML = btn.innerHTML;
        btn.textContent = "✓";
        setTimeout(function () {
          btn.classList.remove("is-copied");
          btn.innerHTML = originalHTML;
        }, 1600);
      };

      if (navigator.clipboard && window.isSecureContext) {
        navigator.clipboard.writeText(text).then(markCopied, function () {
          fallbackCopy(text, markCopied);
        });
      } else {
        fallbackCopy(text, markCopied);
      }
    });
  });

  function fallbackCopy(text, done) {
    var ta = document.createElement("textarea");
    ta.value = text;
    ta.style.position = "fixed";
    ta.style.opacity = "0";
    document.body.appendChild(ta);
    ta.select();
    try {
      document.execCommand("copy");
    } catch (e) {
      /* ignore */
    }
    document.body.removeChild(ta);
    if (done) done();
  }

  /* ============================================================
     Scroll reveal (IntersectionObserver)
     ============================================================ */

  var prefersReducedMotion = window.matchMedia(
    "(prefers-reduced-motion: reduce)"
  ).matches;

  if (!prefersReducedMotion && "IntersectionObserver" in window) {
    var revealObserver = new IntersectionObserver(
      function (entries) {
        entries.forEach(function (entry) {
          if (entry.isIntersecting) {
            entry.target.classList.add("is-visible");
            revealObserver.unobserve(entry.target);
          }
        });
      },
      { threshold: 0.15, rootMargin: "0px 0px -40px 0px" }
    );

    document.querySelectorAll(".reveal").forEach(function (el) {
      revealObserver.observe(el);
    });
  } else {
    document.querySelectorAll(".reveal").forEach(function (el) {
      el.classList.add("is-visible");
    });
  }
})();
