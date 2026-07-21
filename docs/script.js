(function () {
  "use strict";

  /* ============================================================
     Single source of truth for the shipped version.
     Bump it here and nowhere else: it is injected into
       [data-version]      → "v0.5.0"        (brand chip, footer)
       [data-version-tag]  → "v0.5.0-plugins" (inside the install command)
       [data-version-href] → href template, {tag} / {version}
       i18n strings        → the {version} placeholder
     ============================================================ */

  var VERSION = "0.5.0";
  var VERSION_LABEL = "v" + VERSION;
  var PLUGIN_TAG = VERSION_LABEL + "-plugins";

  /* ============================================================
     i18n dictionary
     Keys are dot-paths matching data-i18n / data-i18n-aria in index.html.
     Values may contain safe inline HTML (<code>, <strong>, <em>, <br>)
     because all of this content is static and trusted.
     Elements inside the SVG diagram are written with textContent.
     ============================================================ */

  var translations = {
    pt: {
      /* ---------- a11y / chrome ---------- */
      "a11y.skip": "Pular para o conteúdo",
      "a11y.nav": "Seções da página",
      "a11y.lang": "Escolha de idioma / Language choice",
      "a11y.ghLink": "Ver repositório no GitHub",
      "a11y.copy1": "Copiar comando",
      "a11y.copy2": "Copiar comandos",
      "a11y.footerNav": "Links do projeto",

      "nav.flow": "fluxo",
      "nav.tools": "ferramentas",
      "nav.reminder": "lembrete",
      "nav.where": "onde roda",
      "nav.install": "instalar",

      /* ---------- hero ---------- */
      "hero.eyebrow": "plugin <b class=\"zc\">ZeroClaw</b> · T0/T1 · sem private key",
      "hero.headline": "Cobra em BRL.<br>Recebe em PIX <em>ou</em> USDC.",
      "hero.sub": "Uma fatura, dois trilhos de pagamento. Seu cliente escolhe PIX (BRL) ou USDC via Solana Pay — você opera tudo pelo Telegram, num Raspberry Pi ou numa VPS de 1 GB.",
      "hero.ctaPrimary": "Instalar no meu host",
      "hero.ctaSecondary": "Abrir o repositório",
      "hero.spec.runtime": "runtime",
      "hero.spec.tools": "ferramentas",
      "hero.spec.host": "host de produção",
      "hero.spec.hostVal": "Raspberry Pi 3",
      "hero.spec.keys": "chaves privadas",

      /* ---------- chat mock ---------- */
      "chat.status": "online · Telegram",
      "chat.merchant": "Cobra R$ 150 do João, pedido 412",
      "chat.invoiceTitle": "PixZClaw — Fatura #412",
      "chat.amountBrl": "R$ 150,00",
      "chat.amountUsdc": "₮ 27,27 USDC",
      "chat.railPix": "🇧🇷 PIX (BRL)",
      "chat.railPixMeta": "QR + Copia e Cola",
      "chat.railSol": "◎ Solana Pay (USDC)",
      "chat.railSolMeta": "QR · reference única",
      "chat.forward": "👉 Encaminhe esta mensagem ao cliente. Ele paga por PIX ou por USDC — os dois valem a fatura #412.",
      "chat.lock": "teto R$ 1000 · destino travado",
      "chat.remind": "🔔 avisa quando a 412 pagar",
      "chat.gap": "18 minutos depois — nenhuma mensagem no meio",
      "chat.paid.badge": "PAGA ✅",
      "chat.paid.title": "Fatura #412 — valor conferido",
      "chat.paid.amount": "recebido 27,27 de 27,27 USDC",
      "chat.paid.receipt": "🧾 recibo pronto pra encaminhar",
      "chat.paid.job": "🔕 lembrete cumprido — job removido do cron",

      /* ---------- proof band ---------- */
      "proof.srTitle": "Provas verificáveis",
      "proof.tests.label": "testes host",
      "proof.tests.note": "<code>cargo test</code> sem rede",
      "proof.pr.label": "PR no repo oficial",
      "proof.pr.note": "aberto em <code class=\"zc\">zeroclaw-labs/zeroclaw-plugins</code>",
      "proof.prod.label": "em produção",
      "proof.prod.note": "num Raspberry Pi, operado por Telegram",
      "proof.nosdk.label": "linhas de <code class=\"sol\">solana-sdk</code>",
      "proof.nosdk.note": "core puro + shim fino",
      "proof.bounty": "Construído para o bounty ZeroClaw × Superteam Brasil.",

      /* ---------- 01 fluxo ---------- */
      "flow.kicker": "O caminho do dinheiro",
      "flow.title": "Uma fatura sai.<br>Dois trilhos voltam.",
      "flow.lead": "Não é on-ramp, não é câmbio. São dois trilhos independentes carregando o mesmo <code>invoice_id</code>: o PIX vai pro banco do lojista, o USDC vai pra carteira dele via Solana Pay com uma <em class=\"em-sol\">reference</em> única. Só um dos dois pode ser verificado por software — e a página inteira é honesta sobre qual.",

      "legend.title": "Legenda de cores — cada cor é um ator, nunca enfeite",
      "legend.brand": "<b>PixZClaw</b> — o plugin, a marca, o que você instala",
      "legend.host": "<b>ZeroClaw</b> — o runtime host, o cron, o agente",
      "legend.pix": "<b>trilho PIX</b> — BRL, banco, confirmação humana",
      "legend.sol": "<b>trilho Solana</b> — USDC, on-chain, valor conferido",

      "dg.a11yTitle": "Diagrama: da fatura aos dois trilhos de pagamento até o recibo",
      "dg.a11yDesc": "A fatura emitida no Telegram se divide em dois trilhos. O trilho PIX gera um código Copia e Cola, o dinheiro cai no banco do lojista e só o operador pode confirmar. O trilho Solana gera uma URL Solana Pay com reference, o USDC cai na carteira do lojista e o valor recebido é conferido on-chain. Os dois convergem num recibo.",
      "dg.pixRail": "TRILHO PIX · BRL · banco",
      "dg.solRail": "TRILHO SOLANA · USDC · on-chain",
      "dg.chat": "Telegram · lojista",
      "dg.invoice": "FATURA #412",
      "dg.invoiceAmt": "R$ 150,00",
      "dg.invoiceSub": "teto · destino travado",
      "dg.pixCode": "PIX Copia e Cola",
      "dg.pixBank": "banco do lojista",
      "dg.pixBankSub": "fora do alcance do agente",
      "dg.pixConfirm": "operador confirma",
      "dg.pixConfirmSub": "humano, nunca o bot",
      "dg.solUrl": "URL Solana Pay",
      "dg.solUrlSub": "reference = invoice_id",
      "dg.solWallet": "carteira do lojista",
      "dg.solWalletSub": "USDC · pubkey travada",
      "dg.solCheck": "valor conferido",
      "dg.receipt": "RECIBO 🧾",
      "dg.receiptSub": "com link Solscan",
      "dg.legendSolid": "linha cheia — automático, valor conferido",
      "dg.legendDash": "linha tracejada — depende de confirmação humana",
      "ui.dragX": "← arraste para o lado para ver o diagrama inteiro →",
      "ui.dragTable": "← arraste para o lado para ver as observações →",

      "flow.step1.title": "Cobre no chat.",
      "flow.step1.body": "Você diz o valor em BRL no Telegram e o PixZClaw emite a fatura dual na hora — PIX Copia e Cola real e URL Solana Pay, mesmo <code>invoice_id</code>.",
      "flow.step2.title": "Cliente escolhe o trilho.",
      "flow.step2.body": "Encaminhe a mensagem: ele paga por PIX (BRL) ou por USDC (Solana Pay) — mesma fatura, mesmo valor, dois QRs.",
      "flow.step3.title": "Confira e comprove.",
      "flow.step3.body": "O PixZClaw lê a blockchain, compara o que o lojista <em>realmente</em> recebeu, emite o recibo e fecha o caixa do dia.",

      /* ---------- 02 ferramentas ---------- */
      "tools.kicker": "As três ferramentas",
      "tools.title": "Uma escreve. Duas só leem.",
      "tools.lead": "Três plugins WebAssembly compilados para <code class=\"zc\">wasm32-wasip2</code>. Só um deles produz algo — e mesmo esse não move dinheiro, só monta a cobrança.",
      "tools.outLabel": "saída real da ferramenta",
      "tools.t0tooltip": "read-only",

      "tools.t1.tag": "emite a fatura dual",
      "tools.t1.desc": "Emite uma fatura com dois trilhos: PIX Copia e Cola (EMV+CRC16 de verdade) e URL Solana Pay em USDC, com o mesmo <code>invoice_id</code> nos dois. QR codes por link, teto de valor e destino travado — pronto para encaminhar ao cliente.",
      "tools.t1.f1": "payload EMV montado e assinado com CRC16 — não é imagem, é código válido",
      "tools.t1.f2": "a <em class=\"em-sol\">reference</em> do Solana Pay é o próprio <code>invoice_id</code>",
      "tools.t1.f3": "acima do teto, falha fechado: recusa em vez de emitir",
      "tools.t1.output": "🦞 PixZClaw — Fatura #412\n💰 R$ 150.00 · ₮ 27.272727 USDC\n\n🇧🇷 PIX (BRL)\n📷 QR (toque): …\nOu copie o código (toque para copiar):\n00020126…5204000053039865802BR…6304AB12\n\n◎ Solana Pay (USDC)\n📷 QR (toque): …\nEscaneie com Phantom/Solflare.\n\n👉 Encaminhe esta mensagem ao cliente\n🧾 Fatura 412 · cotação R$/USDC usada: 5.5\n🔒 teto R$ 1000 · destino travado=sim",

      "tools.t2.tag": "o juiz on-chain",
      "tools.t2.desc": "Verifica o pagamento em USDC direto na blockchain: confere o valor que o lojista REALMENTE recebeu, pelo delta de <code class=\"sol\">pre/postTokenBalances</code>. Soma pagamentos parciais, nunca diz PAID sem valor conferido e gera um recibo compartilhável com link do Solscan.",
      "tools.t2.v1": "o valor bate",
      "tools.t2.v2": "diz exatamente quanto falta",
      "tools.t2.v3": "recebeu a mais, e avisa",
      "tools.t2.output": "INVOICE: inv-412\nUSDC: PAID ✅ (recebido 27.27 de 27.27 USDC)\nEXPLORER: https://solscan.io/tx/…\nOVERALL: USDC PAID (valor conferido); PIX não confirmado\n──────────────────────\n🧾 RECIBO — INVOICE #inv-412\n✅ Pago em USDC (Solana)\nValor: 27.27 USDC (R$ equivalente na fatura)\nData: 2026-07-21 22:13 UTC\n🔗 https://solscan.io/tx/…\n──────────────────────",

      "tools.t3.tag": "o caixa",
      "tools.t3.desc": "O caixa do comerciante num card: saldo em USDC e SOL, e o fechamento das últimas 24h com as faturas pagas e seus ids. Ainda traz um sparkline de 7 dias e as últimas movimentações on-chain.",
      "tools.t3.output": "╭─ PixZClaw · Caixa ─────────────────╮\n│ Wallet     8xkR3pQ9mNvT2wLzAb7cD… │\n│ USDC       142.5                  │\n│ SOL (gas)  0.41 SOL               │\n╰───────────────────────────────────╯\n\nHoje (últimas 24h)\n• txs ok:      2\n• faturas PIX: 2\n• pagas: demo-1, inv-412\n\n7 dias\n• 7d ▁▃▅▂▇▁▂  (velho→novo)\n\nT0 read-only · sem chave · PixZClaw",

      /* ---------- 03 lembrete ---------- */
      "rem.kicker": "O lembrete que se apaga",
      "rem.badge": "novo na {version} · lembrete automático",
      "rem.title": "Você não precisa perguntar<br>se o cliente pagou.",
      "rem.lead": "Toda fatura vem com uma oferta. Aceite e o agente cria um job no cron nativo do <b class=\"zc\">ZeroClaw</b> — sem serviço externo, sem webhook, sem fila.",
      "rem.offer": "🔔 avisa quando a 412 pagar",
      "rem.silence": "E o mais importante: enquanto está PENDING, <strong>silêncio</strong>. O bot nunca te manda “ainda não pagou”. Ele só fala quando tem um valor conferido pra mostrar.",
      "rem.t1": "O agente registra um job no cron do <b class=\"zc\">ZeroClaw</b>, amarrado ao <code>invoice_id</code>.",
      "rem.t2k": "a cada 5 min",
      "rem.t2": "O job chama <code>invoice_status</code> e confere o valor recebido on-chain.",
      "rem.t3k": "PENDING",
      "rem.t3": "Nada acontece. Nenhuma mensagem. Nenhum ruído no seu Telegram.",
      "rem.t4k": "PAID ✅",
      "rem.t4": "Aviso no Telegram com o valor conferido, o recibo e o link do Solscan.",
      "rem.t5k": "fim",
      "rem.t5": "O job se apaga sozinho. Nada fica rodando à toa no seu Pi.",

      /* ---------- 04 o que não faz ---------- */
      "not.kicker": "O que ele não faz",
      "not.title": "A lista de “não” é<br>a parte mais importante.",
      "not.lead": "Um agente que cobra dinheiro por você tem que ser explícito sobre o que se recusa a fazer. Nada aqui é limitação pendente de roadmap — é escopo, e é de propósito.",
      "not.i1.title": "Não converte moeda.",
      "not.i1.body": "Não é on-ramp, não é câmbio, não é exchange. O <code>brl_per_usdc</code> é uma cotação offline usada só pra montar o valor do Solana Pay — e ela aparece impressa na fatura.",
      "not.i2.title": "Não vê o PIX do seu banco.",
      "not.i2.body": "PIX bancário não existe on-chain. A ferramenta nunca marca o trilho PIX como pago por conta própria: quem confirma o recebimento é o operador.",
      "not.i3.title": "Não guarda private key.",
      "not.i3.body": "Nenhuma chave privada é pedida, lida ou salva. A única coisa que o plugin conhece é a <em class=\"em-sol\">pubkey</em> do lojista — que é pública por definição.",
      "not.i4.title": "Não assina transação.",
      "not.i4.body": "Quem assina é o cliente, na carteira dele. O plugin monta a cobrança e, depois disso, só lê a blockchain.",
      "not.close": "Cada “não” dessa lista é uma superfície de ataque que simplesmente não existe.",

      /* ---------- 05 confiança ---------- */
      "trust.kicker": "Por que dá pra confiar",
      "trust.title": "Garantias, não promessas.",
      "trust.i1.title": "Sem chaves privadas",
      "trust.i1.body": "O plugin nunca guarda nem assina com private key. Só ferramentas T0/T1 — ele monta a cobrança, quem paga é o cliente na carteira dele.",
      "trust.i2.title": "Valor conferido on-chain",
      "trust.i2.body": "Nunca diz PAID sem checar quanto o lojista recebeu de verdade. Se o cliente pagar menos, marca UNDERPAID ⚠️ e mostra exatamente quanto falta.",
      "trust.i3.title": "Teto de valor + destino travado",
      "trust.i3.body": "Cada fatura respeita um teto (<code>max_amount_brl</code>) e um destino travado (<code>recipient_locked</code>) — o agente não redireciona o dinheiro nem estoura o limite.",
      "trust.i4.title": "Prompt injection falha fechado",
      "trust.i4.body": "Instrução maliciosa não fura o teto: um pedido de “999999999,99” bate no limite e é recusado, em vez de gerar a fatura.",
      "trust.i5.title": "Honestidade no PIX",
      "trust.i5.body": "O PIX bancário não aparece on-chain, então o PixZClaw nunca inventa “pago”. Só marca o PIX como pago quando o operador confirma.",

      /* ---------- tese / respiro ---------- */
      "thesis.title": "Duas pinças.<br>Uma fatura no meio.",
      "thesis.body": "A marca é a tese. A pinça de cima é o trilho PIX; a de baixo é o trilho Solana. Elas se fecham sobre o mesmo losango — o <code>invoice_id</code> que os dois carregam. Nenhuma das duas passa por dentro da outra: são vias independentes, e só uma delas a máquina consegue conferir sozinha.",
      "thesis.foot": "Roda no ZeroClaw, mas o símbolo é nosso. Nome e logo do ZeroClaw são marcas da ZeroClaw Labs.",

      /* ---------- 06 onde roda ---------- */
      "where.kicker": "Onde roda",
      "where.title": "O agente inteiro cabe<br>em 1 GB de RAM.",
      "where.lead": "O custo de infraestrutura aqui é ruído, não barreira. Três caminhos reais, com preço de tabela pesquisado — não com número redondo de marketing.",
      "where.th1": "Opção",
      "where.th2": "Custo",
      "where.th3": "Observação",
      "where.r1.opt": "Raspberry Pi 3 / 3B+ <span>1 GB RAM</span>",
      "where.r1.cost": "~R$ 600 <i>uma vez</i>",
      "where.r1.note": "É onde roda hoje, em produção, com o bot do Telegram no ar.",
      "where.r2.opt": "VPS <span>1 vCPU / 1 GB</span>",
      "where.r2.cost": "~R$ 25–30 <i>/mês</i>",
      "where.r2.note": "Vultr US$ 5 · DigitalOcean US$ 6 · Hetzner € 4–5. Preço de tabela em julho/2026.",
      "where.r3.opt": "Oracle Cloud Always Free <span>ARM Ampere A1</span>",
      "where.r3.cost": "R$ 0",
      "where.r3.note": "Tier gratuito. Atenção: os limites do Always Free foram reduzidos pela metade em junho/2026.",
      "where.fine": "Cotação usada: US$ 1 = R$ 5,07 · 21/07/2026. Preço de hardware e de VPS muda; a data fica aqui à vista para esta página envelhecer com honestidade.",

      /* ---------- 07 instalação ---------- */
      "install.kicker": "Instalação",
      "install.title": "Três comandos<br>e um restart.",
      "install.lead": "Roda em qualquer host <b class=\"zc\">ZeroClaw</b> — inclusive num Raspberry Pi 3. Instalar, configurar, reiniciar.",
      "install.step1.label": "Instale os 3 plugins da release {version} com um comando:",
      "install.step2.label": "Ajuste a chave PIX, o destino Solana e os limites. Use seus próprios valores:",
      "install.step3.label": "Reinicie o serviço para aplicar a config:",
      "install.copy": "copiar",
      "install.warning": "<code>merchant_solana</code> é a sua chave PÚBLICA (pubkey), nunca a private key. O PixZClaw não pede — e não guarda — chave privada.",

      /* ---------- 08 FAQ ---------- */
      "faq.kicker": "Perguntas frequentes",
      "faq.title": "O que perguntam antes de instalar.",
      "faq.q1.q": "Preciso de exchange ou conversão de moeda?",
      "faq.q1.a": "Não. PIX e USDC são dois trilhos de pagamento independentes na mesma fatura — não é on-ramp nem câmbio. O <code>brl_per_usdc</code> é só uma cotação offline pra montar o valor do Solana Pay.",
      "faq.q2.q": "E se o cliente pagar menos que o combinado?",
      "faq.q2.a": "O <code>invoice_status</code> confere o valor recebido on-chain e responde UNDERPAID ⚠️, mostrando exatamente quanto ainda falta. Ele soma pagamentos parciais e só marca PAID quando o valor bate.",
      "faq.q3.q": "O PIX confirma sozinho?",
      "faq.q3.a": "Não. O PIX do banco não é visível on-chain, então a ferramenta nunca diz “pago” por conta própria — o operador confirma o recebimento. É honestidade por design.",
      "faq.q4.q": "Quanto custa o host?",
      "faq.q4.a": "Depende de onde você põe. Hoje ele roda num Raspberry Pi 3B+ (~R$ 600 uma vez, preço de julho/2026), mas uma VPS de 1 vCPU e 1 GB sai por ~R$ 25–30/mês, e o tier Always Free da Oracle Cloud custa R$ 0. Os três números estão na seção <a href=\"#onde-roda\">Onde roda</a>.",
      "faq.q5.q": "É seguro deixar um agente cobrar por mim?",
      "faq.q5.a": "Sim. São ferramentas T0/T1 (sem private key), com teto de valor, destino travado e proteção contra prompt injection. O agente monta a cobrança; quem paga é sempre o cliente na carteira dele.",
      "faq.q6.q": "Como o lembrete automático funciona sem serviço externo?",
      "faq.q6.a": "Ele usa o cron nativo do <b class=\"zc\">ZeroClaw</b>. O agente registra um job amarrado ao <code>invoice_id</code>, que chama <code>invoice_status</code> a cada 5 minutos, te avisa no Telegram quando o valor cair — e então remove a si mesmo. Está disponível desde a {version}.",

      /* ---------- footer ---------- */
      "footer.credit": "Feito para o bounty ZeroClaw × Superteam Brasil. Rust puro + <code class=\"zc\">wasm32-wasip2</code> · 101 testes host · sem <code class=\"sol\">solana-sdk</code>.",
      "footer.repo": "Repositório do PixZClaw",
      "footer.pr": "PR #123 · no repo oficial do ZeroClaw",
      "footer.release": "Release {version}",
      "footer.docs": "Docs ZeroClaw",
      "footer.solanapay": "Solana Pay spec",
      "footer.license": "Licenciado sob MIT / Apache-2.0.",
      "footer.tm": "ZeroClaw e o logo do ZeroClaw são marcas da ZeroClaw Labs. O PixZClaw é um plugin de terceiros, com símbolo e identidade próprios.",
      "footer.fx": "Preços desta página conferidos em 21/07/2026 · US$ 1 = R$ 5,07."
    },

    en: {
      /* ---------- a11y / chrome ---------- */
      "a11y.skip": "Skip to content",
      "a11y.nav": "Page sections",
      "a11y.lang": "Escolha de idioma / Language choice",
      "a11y.ghLink": "View repository on GitHub",
      "a11y.copy1": "Copy command",
      "a11y.copy2": "Copy commands",
      "a11y.footerNav": "Project links",

      "nav.flow": "flow",
      "nav.tools": "tools",
      "nav.reminder": "reminder",
      "nav.where": "where it runs",
      "nav.install": "install",

      /* ---------- hero ---------- */
      "hero.eyebrow": "<b class=\"zc\">ZeroClaw</b> plugin · T0/T1 · no private key",
      "hero.headline": "Charge in BRL.<br>Get paid in PIX <em>or</em> USDC.",
      "hero.sub": "One invoice, two payment rails. Your customer picks PIX (BRL) or USDC via Solana Pay — you run it all from Telegram, on a Raspberry Pi or a 1 GB VPS.",
      "hero.ctaPrimary": "Install on my host",
      "hero.ctaSecondary": "Open the repository",
      "hero.spec.runtime": "runtime",
      "hero.spec.tools": "tools",
      "hero.spec.host": "production host",
      "hero.spec.hostVal": "Raspberry Pi 3",
      "hero.spec.keys": "private keys",

      /* ---------- chat mock ---------- */
      "chat.status": "online · Telegram",
      "chat.merchant": "Charge João R$150, order 412",
      "chat.invoiceTitle": "PixZClaw — Invoice #412",
      "chat.amountBrl": "R$150.00",
      "chat.amountUsdc": "₮ 27.27 USDC",
      "chat.railPix": "🇧🇷 PIX (BRL)",
      "chat.railPixMeta": "QR + Copy & Paste",
      "chat.railSol": "◎ Solana Pay (USDC)",
      "chat.railSolMeta": "QR · unique reference",
      "chat.forward": "👉 Forward this message to your customer. They pay by PIX or by USDC — both settle invoice #412.",
      "chat.lock": "cap R$1000 · recipient locked",
      "chat.remind": "🔔 ping me when 412 gets paid",
      "chat.gap": "18 minutes later — nothing in between",
      "chat.paid.badge": "PAID ✅",
      "chat.paid.title": "Invoice #412 — amount verified",
      "chat.paid.amount": "received 27.27 of 27.27 USDC",
      "chat.paid.receipt": "🧾 receipt ready to forward",
      "chat.paid.job": "🔕 reminder done — job removed from cron",

      /* ---------- proof band ---------- */
      "proof.srTitle": "Verifiable proof",
      "proof.tests.label": "host tests",
      "proof.tests.note": "<code>cargo test</code>, no network",
      "proof.pr.label": "PR on the official repo",
      "proof.pr.note": "opened on <code class=\"zc\">zeroclaw-labs/zeroclaw-plugins</code>",
      "proof.prod.label": "in production",
      "proof.prod.note": "on a Raspberry Pi, operated over Telegram",
      "proof.nosdk.label": "lines of <code class=\"sol\">solana-sdk</code>",
      "proof.nosdk.note": "pure core + thin shim",
      "proof.bounty": "Built for the ZeroClaw × Superteam Brasil bounty.",

      /* ---------- 01 flow ---------- */
      "flow.kicker": "How the money moves",
      "flow.title": "One invoice out.<br>Two rails back.",
      "flow.lead": "Not an on-ramp, not an exchange. Two independent rails carrying the same <code>invoice_id</code>: PIX lands in the merchant's bank, USDC lands in the merchant's wallet via Solana Pay with a unique <em class=\"em-sol\">reference</em>. Only one of them can be verified by software — and this page is honest about which.",

      "legend.title": "Colour legend — every colour is an actor, never decoration",
      "legend.brand": "<b>PixZClaw</b> — the plugin, the brand, what you install",
      "legend.host": "<b>ZeroClaw</b> — the host runtime, the cron, the agent",
      "legend.pix": "<b>PIX rail</b> — BRL, bank, human confirmation",
      "legend.sol": "<b>Solana rail</b> — USDC, on-chain, amount verified",

      "dg.a11yTitle": "Diagram: from the invoice through both payment rails to the receipt",
      "dg.a11yDesc": "The invoice issued on Telegram splits into two rails. The PIX rail produces a Copy & Paste code, the money lands in the merchant's bank and only the operator can confirm it. The Solana rail produces a Solana Pay URL with a reference, USDC lands in the merchant's wallet and the received amount is verified on-chain. Both converge into a receipt.",
      "dg.pixRail": "PIX RAIL · BRL · bank",
      "dg.solRail": "SOLANA RAIL · USDC · on-chain",
      "dg.chat": "Telegram · merchant",
      "dg.invoice": "INVOICE #412",
      "dg.invoiceAmt": "R$150.00",
      "dg.invoiceSub": "cap · recipient locked",
      "dg.pixCode": "PIX Copy & Paste",
      "dg.pixBank": "merchant's bank",
      "dg.pixBankSub": "out of the agent's reach",
      "dg.pixConfirm": "operator confirms",
      "dg.pixConfirmSub": "a human, never the bot",
      "dg.solUrl": "Solana Pay URL",
      "dg.solUrlSub": "reference = invoice_id",
      "dg.solWallet": "merchant's wallet",
      "dg.solWalletSub": "USDC · locked pubkey",
      "dg.solCheck": "amount verified",
      "dg.receipt": "RECEIPT 🧾",
      "dg.receiptSub": "with Solscan link",
      "dg.legendSolid": "solid line — automatic, amount verified",
      "dg.legendDash": "dashed line — needs human confirmation",
      "ui.dragX": "← drag sideways to see the whole diagram →",
      "ui.dragTable": "← drag sideways to see the notes →",

      "flow.step1.title": "Charge in the chat.",
      "flow.step1.body": "Say the BRL amount on Telegram and PixZClaw issues the dual invoice instantly — a real PIX Copy & Paste code and a Solana Pay URL, same <code>invoice_id</code>.",
      "flow.step2.title": "Customer picks a rail.",
      "flow.step2.body": "Forward the message: they pay by PIX (BRL) or by USDC (Solana Pay) — same invoice, same amount, two QRs.",
      "flow.step3.title": "Verify and receipt.",
      "flow.step3.body": "PixZClaw reads the blockchain, compares what the merchant <em>actually</em> received, issues the receipt and closes the day's books.",

      /* ---------- 02 tools ---------- */
      "tools.kicker": "The three tools",
      "tools.title": "One writes. Two only read.",
      "tools.lead": "Three WebAssembly plugins compiled to <code class=\"zc\">wasm32-wasip2</code>. Only one of them produces anything — and even that one moves no money, it just builds the charge.",
      "tools.outLabel": "real tool output",
      "tools.t0tooltip": "read-only",

      "tools.t1.tag": "issues the dual invoice",
      "tools.t1.desc": "Issues one invoice on two rails: PIX Copy & Paste (real EMV+CRC16) plus a Solana Pay USDC URL, sharing the same <code>invoice_id</code>. QR codes by link, amount cap, and locked recipient — ready to forward to the customer.",
      "tools.t1.f1": "EMV payload assembled and signed with CRC16 — not a picture, a valid code",
      "tools.t1.f2": "the Solana Pay <em class=\"em-sol\">reference</em> is the <code>invoice_id</code> itself",
      "tools.t1.f3": "above the cap it fails closed: it refuses instead of issuing",
      "tools.t1.output": "🦞 PixZClaw — Fatura #412\n💰 R$ 150.00 · ₮ 27.272727 USDC\n\n🇧🇷 PIX (BRL)\n📷 QR (toque): …\nOu copie o código (toque para copiar):\n00020126…5204000053039865802BR…6304AB12\n\n◎ Solana Pay (USDC)\n📷 QR (toque): …\nEscaneie com Phantom/Solflare.\n\n👉 Encaminhe esta mensagem ao cliente\n🧾 Fatura 412 · cotação R$/USDC usada: 5.5\n🔒 teto R$ 1000 · destino travado=sim",

      "tools.t2.tag": "the on-chain judge",
      "tools.t2.desc": "Checks the USDC payment straight on-chain: it verifies the amount the merchant ACTUALLY received, from the <code class=\"sol\">pre/postTokenBalances</code> delta. It sums partial payments, never says PAID without a verified amount, and generates a shareable receipt with a Solscan link.",
      "tools.t2.v1": "the amount matches",
      "tools.t2.v2": "says exactly how much is missing",
      "tools.t2.v3": "received more, and says so",
      "tools.t2.output": "INVOICE: inv-412\nUSDC: PAID ✅ (recebido 27.27 de 27.27 USDC)\nEXPLORER: https://solscan.io/tx/…\nOVERALL: USDC PAID (valor conferido); PIX não confirmado\n──────────────────────\n🧾 RECIBO — INVOICE #inv-412\n✅ Pago em USDC (Solana)\nValor: 27.27 USDC (R$ equivalente na fatura)\nData: 2026-07-21 22:13 UTC\n🔗 https://solscan.io/tx/…\n──────────────────────",

      "tools.t3.tag": "the till",
      "tools.t3.desc": "The merchant's till in one card: USDC and SOL balance, plus the last-24h close with paid invoices and their ids. It also shows a 7-day sparkline and the latest on-chain activity.",
      "tools.t3.output": "╭─ PixZClaw · Caixa ─────────────────╮\n│ Wallet     8xkR3pQ9mNvT2wLzAb7cD… │\n│ USDC       142.5                  │\n│ SOL (gas)  0.41 SOL               │\n╰───────────────────────────────────╯\n\nHoje (últimas 24h)\n• txs ok:      2\n• faturas PIX: 2\n• pagas: demo-1, inv-412\n\n7 dias\n• 7d ▁▃▅▂▇▁▂  (velho→novo)\n\nT0 read-only · sem chave · PixZClaw",

      /* ---------- 03 reminder ---------- */
      "rem.kicker": "The reminder that deletes itself",
      "rem.badge": "new in {version} · automatic reminder",
      "rem.title": "You never have to ask<br>whether the customer paid.",
      "rem.lead": "Every invoice comes with an offer. Accept it and the agent creates a job on <b class=\"zc\">ZeroClaw</b>'s native cron — no external service, no webhook, no queue.",
      "rem.offer": "🔔 ping me when 412 gets paid",
      "rem.silence": "And the important part: while it's PENDING, <strong>silence</strong>. The bot never sends you “still not paid”. It only speaks when it has a verified amount to show.",
      "rem.t1": "The agent registers a job on <b class=\"zc\">ZeroClaw</b>'s cron, tied to the <code>invoice_id</code>.",
      "rem.t2k": "every 5 min",
      "rem.t2": "The job calls <code>invoice_status</code> and verifies the amount received on-chain.",
      "rem.t3k": "PENDING",
      "rem.t3": "Nothing happens. No message. No noise in your Telegram.",
      "rem.t4k": "PAID ✅",
      "rem.t4": "A Telegram ping with the verified amount, the receipt and the Solscan link.",
      "rem.t5k": "end",
      "rem.t5": "The job deletes itself. Nothing keeps spinning on your Pi for nothing.",

      /* ---------- 04 what it doesn't do ---------- */
      "not.kicker": "What it doesn't do",
      "not.title": "The list of “no”<br>is the important part.",
      "not.lead": "An agent that charges money on your behalf has to be explicit about what it refuses to do. None of this is a roadmap gap — it's scope, and it's deliberate.",
      "not.i1.title": "It doesn't convert currency.",
      "not.i1.body": "Not an on-ramp, not an exchange, not a broker. <code>brl_per_usdc</code> is an offline quote used only to build the Solana Pay amount — and it's printed right on the invoice.",
      "not.i2.title": "It doesn't see your bank's PIX.",
      "not.i2.body": "Bank PIX doesn't exist on-chain. The tool never marks the PIX rail as paid on its own: the operator is the one who confirms receipt.",
      "not.i3.title": "It doesn't hold a private key.",
      "not.i3.body": "No private key is ever requested, read or stored. The only thing the plugin knows is the merchant's <em class=\"em-sol\">pubkey</em> — which is public by definition.",
      "not.i4.title": "It doesn't sign transactions.",
      "not.i4.body": "The customer signs, from their own wallet. The plugin builds the charge and, after that, only reads the blockchain.",
      "not.close": "Every “no” on this list is an attack surface that simply doesn't exist.",

      /* ---------- 05 trust ---------- */
      "trust.kicker": "Why you can trust it",
      "trust.title": "Guarantees, not promises.",
      "trust.i1.title": "No private keys",
      "trust.i1.body": "The plugin never holds or signs with a private key. T0/T1 tools only — it builds the charge; the customer pays from their own wallet.",
      "trust.i2.title": "Amount verified on-chain",
      "trust.i2.body": "It never says PAID without checking what the merchant actually received. If the customer pays less, it flags UNDERPAID ⚠️ and shows exactly how much is missing.",
      "trust.i3.title": "Amount cap + locked recipient",
      "trust.i3.body": "Every invoice respects a cap (<code>max_amount_brl</code>) and a locked recipient (<code>recipient_locked</code>) — the agent can't reroute the money or blow past the limit.",
      "trust.i4.title": "Prompt injection fails closed",
      "trust.i4.body": "A malicious instruction can't break the cap: a request for “999999999.99” hits the limit and is refused instead of minting the invoice.",
      "trust.i5.title": "Honesty on PIX",
      "trust.i5.body": "Bank PIX isn't visible on-chain, so PixZClaw never fakes a “paid”. It only marks PIX paid once the operator confirms.",

      /* ---------- thesis / breathing band ---------- */
      "thesis.title": "Two pincers.<br>One invoice between them.",
      "thesis.body": "The mark is the thesis. The upper pincer is the PIX rail; the lower one is the Solana rail. They close on the same rhombus — the <code>invoice_id</code> both of them carry. Neither one runs through the other: they are independent routes, and only one of them can be verified by a machine on its own.",
      "thesis.foot": "It runs on ZeroClaw, but the symbol is ours. The ZeroClaw name and logo are trademarks of ZeroClaw Labs.",

      /* ---------- 06 where it runs ---------- */
      "where.kicker": "Where it runs",
      "where.title": "The whole agent fits<br>in 1 GB of RAM.",
      "where.lead": "Infrastructure cost here is noise, not a barrier. Three real paths, with researched list prices — not a round marketing number.",
      "where.th1": "Option",
      "where.th2": "Cost",
      "where.th3": "Notes",
      "where.r1.opt": "Raspberry Pi 3 / 3B+ <span>1 GB RAM</span>",
      "where.r1.cost": "~R$600 <i>once</i>",
      "where.r1.note": "This is where it runs today, in production, with the Telegram bot live.",
      "where.r2.opt": "VPS <span>1 vCPU / 1 GB</span>",
      "where.r2.cost": "~R$25–30 <i>/mo</i>",
      "where.r2.note": "Vultr US$5 · DigitalOcean US$6 · Hetzner €4–5. List prices as of July 2026.",
      "where.r3.opt": "Oracle Cloud Always Free <span>ARM Ampere A1</span>",
      "where.r3.cost": "R$0",
      "where.r3.note": "Free tier. Heads up: Always Free limits were cut in half in June 2026.",
      "where.fine": "FX rate used: US$1 = R$5.07 · 2026-07-21. Hardware and VPS prices move; the date stays visible so this page can age honestly.",

      /* ---------- 07 install ---------- */
      "install.kicker": "Installation",
      "install.title": "Three commands<br>and one restart.",
      "install.lead": "Runs on any <b class=\"zc\">ZeroClaw</b> host — including a Raspberry Pi 3. Install, configure, restart.",
      "install.step1.label": "Install the 3 plugins from the {version} release with one command:",
      "install.step2.label": "Set your PIX key, Solana recipient, and limits. Use your own values:",
      "install.step3.label": "Restart the service to apply the config:",
      "install.copy": "copy",
      "install.warning": "<code>merchant_solana</code> is your PUBLIC key (pubkey), never the private key. PixZClaw doesn't ask for — and never stores — a private key.",

      /* ---------- 08 FAQ ---------- */
      "faq.kicker": "FAQ",
      "faq.title": "What people ask before installing.",
      "faq.q1.q": "Do I need an exchange or currency conversion?",
      "faq.q1.a": "No. PIX and USDC are two independent payment rails on the same invoice — not an on-ramp or currency exchange. The <code>brl_per_usdc</code> is just an offline quote used to build the Solana Pay amount.",
      "faq.q2.q": "What if the customer pays less than agreed?",
      "faq.q2.a": "<code>invoice_status</code> checks the received amount on-chain and answers UNDERPAID ⚠️, showing exactly how much is still missing. It sums partial payments and only marks PAID once the amount matches.",
      "faq.q3.q": "Does PIX confirm itself?",
      "faq.q3.a": "No. Bank PIX isn't visible on-chain, so the tool never claims “paid” on its own — the operator confirms receipt. Honesty by design.",
      "faq.q4.q": "What does the host cost?",
      "faq.q4.a": "Depends where you put it. Today it runs on a Raspberry Pi 3B+ (~R$600 once, July 2026 pricing), but a 1 vCPU / 1 GB VPS runs ~R$25–30/month, and Oracle Cloud's Always Free tier costs R$0. All three numbers are in the <a href=\"#onde-roda\">Where it runs</a> section.",
      "faq.q5.q": "Is it safe to let an agent charge on my behalf?",
      "faq.q5.a": "Yes. These are T0/T1 tools (no private key), with an amount cap, a locked recipient, and prompt-injection protection. The agent builds the charge; the customer always pays from their own wallet.",
      "faq.q6.q": "How does the automatic reminder work with no external service?",
      "faq.q6.a": "It uses <b class=\"zc\">ZeroClaw</b>'s native cron. The agent registers a job tied to the <code>invoice_id</code>, which calls <code>invoice_status</code> every 5 minutes, pings you on Telegram when the money lands — and then removes itself. It has shipped since {version}.",

      /* ---------- footer ---------- */
      "footer.credit": "Built for the ZeroClaw × Superteam Brasil bounty. Pure Rust + <code class=\"zc\">wasm32-wasip2</code> · 101 host tests · no <code class=\"sol\">solana-sdk</code>.",
      "footer.repo": "PixZClaw repository",
      "footer.pr": "PR #123 · on ZeroClaw's official repo",
      "footer.release": "Release {version}",
      "footer.docs": "ZeroClaw Docs",
      "footer.solanapay": "Solana Pay spec",
      "footer.license": "Licensed under MIT / Apache-2.0.",
      "footer.tm": "ZeroClaw and the ZeroClaw logo are trademarks of ZeroClaw Labs. PixZClaw is a third-party plugin, with its own symbol and identity.",
      "footer.fx": "Prices on this page checked on 2026-07-21 · US$1 = R$5.07."
    }
  };

  /* ============================================================
     Language toggle
     ============================================================ */

  var SVG_NS = "http://www.w3.org/2000/svg";
  var STORAGE_KEY = "pixzclaw-lang";
  var htmlEl = document.documentElement;
  var langButtons = document.querySelectorAll("[data-lang-btn]");
  var currentLang = "pt";

  function getStoredLang() {
    try { return localStorage.getItem(STORAGE_KEY); } catch (e) { return null; }
  }

  function storeLang(lang) {
    try { localStorage.setItem(STORAGE_KEY, lang); } catch (e) { /* private mode */ }
  }

  function lookup(dict, key) {
    var value = dict[key] != null ? dict[key] : translations.pt[key];
    if (typeof value !== "string") return value;
    return value.replace(/\{version\}/g, VERSION_LABEL).replace(/\{tag\}/g, PLUGIN_TAG);
  }

  /* Version injection for the plain (non-translated) mounting points. */
  function applyVersion() {
    Array.prototype.forEach.call(document.querySelectorAll("[data-version]"), function (el) {
      el.textContent = VERSION_LABEL;
    });
    Array.prototype.forEach.call(document.querySelectorAll("[data-version-tag]"), function (el) {
      el.textContent = PLUGIN_TAG;
    });
    Array.prototype.forEach.call(document.querySelectorAll("[data-version-href]"), function (el) {
      el.setAttribute(
        "href",
        el.getAttribute("data-version-href")
          .replace(/\{tag\}/g, PLUGIN_TAG)
          .replace(/\{version\}/g, VERSION_LABEL)
      );
    });
  }

  /* Translates a subtree (inclusive of the root itself). */
  function translateWithin(root, dict) {
    var textNodes = root.querySelectorAll("[data-i18n]");
    var ariaNodes = root.querySelectorAll("[data-i18n-aria]");

    function applyText(el) {
      var value = lookup(dict, el.getAttribute("data-i18n"));
      if (value == null) return;
      if (el.namespaceURI === SVG_NS) {
        /* SVG <text>/<title>/<desc>: plain text only, no markup parsing */
        el.textContent = value;
      } else {
        el.innerHTML = value;
      }
    }

    function applyAria(el) {
      var value = lookup(dict, el.getAttribute("data-i18n-aria"));
      if (value != null) el.setAttribute("aria-label", value);
    }

    if (root.hasAttribute && root.hasAttribute("data-i18n")) applyText(root);
    if (root.hasAttribute && root.hasAttribute("data-i18n-aria")) applyAria(root);

    Array.prototype.forEach.call(textNodes, applyText);
    Array.prototype.forEach.call(ariaNodes, applyAria);
  }

  function applyLanguage(lang) {
    var dict = translations[lang] || translations.pt;
    currentLang = translations[lang] ? lang : "pt";

    translateWithin(document.body, dict);

    htmlEl.lang = currentLang === "pt" ? "pt-BR" : "en";

    Array.prototype.forEach.call(langButtons, function (btn) {
      var isActive = btn.getAttribute("data-lang-btn") === currentLang;
      btn.classList.toggle("is-active", isActive);
      btn.setAttribute("aria-pressed", String(isActive));
    });

    storeLang(currentLang);
  }

  Array.prototype.forEach.call(langButtons, function (btn) {
    btn.addEventListener("click", function () {
      applyLanguage(btn.getAttribute("data-lang-btn"));
    });
  });

  applyVersion();

  var initialLang = getStoredLang();
  if (initialLang !== "pt" && initialLang !== "en") initialLang = "pt";
  applyLanguage(initialLang);

  /* ============================================================
     Copy-to-clipboard for command blocks
     ============================================================ */

  function fallbackCopy(text, done) {
    var ta = document.createElement("textarea");
    ta.value = text;
    ta.style.position = "fixed";
    ta.style.opacity = "0";
    document.body.appendChild(ta);
    ta.select();
    try { document.execCommand("copy"); } catch (e) { /* ignore */ }
    document.body.removeChild(ta);
    if (done) done();
  }

  Array.prototype.forEach.call(document.querySelectorAll(".copy-btn"), function (btn) {
    var busy = false;

    btn.addEventListener("click", function () {
      var target = document.getElementById(btn.getAttribute("data-copy-target"));
      if (!target || busy) return;

      var text = target.textContent;

      var markCopied = function () {
        busy = true;
        var originalHTML = btn.innerHTML;
        btn.classList.add("is-copied");
        btn.textContent = "✓";
        setTimeout(function () {
          btn.classList.remove("is-copied");
          btn.innerHTML = originalHTML;
          /* re-translate in case the language changed meanwhile */
          translateWithin(btn, translations[currentLang] || translations.pt);
          busy = false;
        }, 1500);
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
})();
