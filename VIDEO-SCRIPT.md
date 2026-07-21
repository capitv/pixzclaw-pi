# PixZClaw — Roteiro do vídeo demo (Superteam Earn)

**Duração alvo:** 2:56 (limite do edital: 3:00 — sobram 4 s de margem)
**Formato:** terminal real (SSH no Raspberry Pi 3B+) + tela do celular (Telegram e Phantom). **Sem slides, sem deck, sem tela de título.**
**Regra do edital:** *"A real ZeroClaw agent, a real channel (Telegram/Discord), your plugin doing the thing. No slides. Terminal + phone is perfect."*

**A cena que carrega o vídeo:** fatura de **10 USDC**, pagamento **real de 1 USDC** na mainnet, e o agente respondendo **`USDC: UNDERPAID ⚠️ (recebido 1 de 10 USDC — faltam 9)`** — valor lido do delta `pre/postTokenBalances` da transação, não da existência de uma assinatura. Planos 5, 6 e 7 são intocáveis. Se faltar tempo na edição, corte o plano 10 (cron).

---

## 1. Tabela de planos

Legenda da coluna **Fonte**: `PI` = terminal SSH no Raspberry Pi (desktop) · `TG` = tela do celular no Telegram · `PH` = tela do celular no Phantom · `WEB` = navegador do desktop.

| # | Timecode | Fonte | O que aparece na tela | Narração (bloco EN/PT correspondente) |
|---|---|---|---|---|
| 1 | 0:00–0:10 | PI | Terminal já logado no Pi, prompt limpo. Digitar e rodar: `uname -m && zeroclaw plugin list`. Saída mostra `aarch64` e os 3 plugins: `brl-usdc-invoice`, `invoice-status`, `pixzclaw-brief`. | **N1** |
| 2 | 0:10–0:26 | TG | Chat do bot vazio. O lojista digita e envia: `Cobra R$ 55 do cliente, fatura INV-DEMO-A. Use brl_usdc_invoice. Sem redact.` O card da fatura aparece: `🦞 PixZClaw — Fatura #INV-DEMO-A` · `💰 R$ 55.00 · ₮ 10.000000 USDC`. | **N2** |
| 3 | 0:26–0:44 | TG | Scroll lento pelo card: bloco `🇧🇷 PIX (BRL)` com o QR e o copia-e-cola em code block; bloco `◎ Solana Pay (USDC)` com o QR. Parar com os dois trilhos visíveis na mesma tela. | **N3** |
| 4 | 0:44–0:56 | TG | Segue no card. Destaque (dedo/scroll) no rodapé: `🔒 teto R$ 1000 · destino travado=sim`. Sem edição gráfica — só a rolagem parando na linha. | **N4** |
| 5 | 0:56–1:14 | PH | Phantom abre a requisição de transferência (Solana Pay parcial — ver §4, item "QR de pagamento parcial"): destino = pubkey do lojista, `1 USDC`. Toque em confirmar. Tela de sucesso. **Pagamento real, mainnet.** | **N5** |
| 6 | 1:14–1:26 | TG | De volta ao Telegram. O lojista envia: `A INV-DEMO-A pagou? Use invoice_status com expected_usdc 10.` Aparece o indicador de "digitando". | **N6** |
| 7 | 1:26–1:44 | TG | A resposta chega. Bloco visível **por inteiro** e parado na tela (ver §1.1 para o texto esperado, palavra por palavra). Ficar 4–5 s parado na linha `USDC: UNDERPAID ⚠️ …`. | **N7** |
| 8 | 1:44–1:58 | WEB | Abrir no navegador do desktop o link `EXPLORER:` que o próprio plugin devolveu (Solscan). Mostrar a transação e o valor transferido: `1 USDC`. Zoom na linha de transferência. | **N8** |
| 9 | 1:58–2:14 | PI | Terminal: `cat ~/.zeroclaw/plugins/invoice-status/manifest.toml`. Parar na linha de `permissions` (`config_read`, `http_client`) — ⟨CONFIRMAR: caminho instalado e nome exato do campo no manifest⟩. | **N9** |
| 10 | 2:14–2:34 | TG | Telegram: `avisa quando a INV-DEMO-A pagar` → resposta do agente confirmando o vigia. Em seguida: `quais lembretes tenho?` → o agente lista `pixzclaw-watch-INV-DEMO-A`. | **N10** |
| 11 | 2:34–2:56 | PI | Terminal, comando final já digitado: `zeroclaw plugin list` (volta ao plano de abertura, fecha o círculo). Cursor piscando no fim. Fade de áudio, corte seco no vídeo. | **N11** |

**Soma:** 10 + 16 + 18 + 12 + 18 + 12 + 18 + 14 + 16 + 20 + 22 = **176 s = 2:56** ✅

### 1.1 Texto exato esperado no plano 7

Do `README.md` do `invoice-status` (caso UNDERPAID, saída verbatim de `fetch_and_status`), adaptado aos números desta demo:

```text
INVOICE: INV-DEMO-A
REF: ⟨CONFIRMAR: 11 primeiros chars da reference⟩…
USDC: UNDERPAID ⚠️ (recebido 1 de 10 USDC — faltam 9) latest=⟨CONFIRMAR: 11 chars da assinatura⟩…
EXPLORER: https://solscan.io/tx/⟨CONFIRMAR: assinatura completa⟩
PIX: PENDING (tool não vê SPI do banco; use pix_marked_paid=true se confirmou)
OVERALL: PENDING (USDC não confirmado por valor)
```

Três coisas que **têm que** estar ausentes nesse bloco, e que valem ser notadas na edição (não custa tempo de fala):
- **não** existe bloco `🧾 RECIBO` — recibo só sai em fatura liquidada;
- **não** existe a linha `[sistema] Fatura liquidada …` — logo o vigia do cron continuaria valendo;
- a linha `PIX: PENDING` diz, na própria saída, que a ferramenta não enxerga o SPI do banco.

**Formatação confirmada no código-fonte.** `fmt_amount()` em `crates/solana-wasm-core/src/status.rs:258` formata com `{x:.6}` e depois corta os zeros à direita e o ponto órfão. Logo 1.0 → `1`, 10.0 → `10`, 9.0 → `9`. A saída é literalmente `recebido 1 de 10 USDC — faltam 9`. Não precisa ajustar a narração N7.

---

## 2. Narração em inglês (para ler em voz alta)

Ritmo de referência: **150 palavras/min = 2,5 palavras/s**. Cada bloco traz `palavras → segundos falados` e a duração do plano. Todos os blocos cabem com folga.

> **N1** — 19 palavras → **7,6 s** (plano: 10 s)
> "A Raspberry Pi 3 running a real ZeroClaw agent. Three WebAssembly plugins, wasm32-wasip2. No slides — terminal and phone."

> **N2** — 29 palavras → **11,6 s** (plano: 16 s)
> "The merchant charges in reais, from Telegram. Fifty-five reais, invoice INV-DEMO-A. The agent calls brl_usdc_invoice: one invoice, two payment rails — PIX in reais, and ten USDC on Solana."

> **N3** — 43 palavras → **17,2 s** (plano: 18 s)
> "The PIX string is a real EMV payload with its CRC — the customer pastes it into any Brazilian bank app. The Solana QR carries a full Solana Pay URL, with a reference derived deterministically from the invoice id and the merchant pubkey."

> **N4** — 27 palavras → **10,8 s** (plano: 12 s)
> "One honest note: this plugin cannot see PIX settle. Bank transfers are not on-chain. Only a human marks PIX as paid — the tool never infers it."

> **N5** — 43 palavras → **17,2 s** (plano: 18 s)
> "Now the Solana rail, for real, on mainnet. The invoice asks for ten USDC. I'm going to pay one — a partial payment, same reference. Signing in Phantom, on my phone. The agent never touches this: it holds no key and signs nothing."

> **N6** — 19 palavras → **7,6 s** (plano: 12 s)
> "Back in Telegram: did INV-DEMO-A pay? The agent calls invoice_status. It derives the same reference, and asks the chain."

> **N7** — 43 palavras → **17,2 s** (plano: 18 s)
> "There it is. UNDERPAID. Received one of ten USDC — nine short. That number is not a guess: the plugin pulled the transaction, summed the merchant's USDC balance delta, pre versus post, and compared. A signature alone would never have produced this line."

> **N8** — 30 palavras → **12,0 s** (plano: 14 s)
> "Same transaction on Solscan: one USDC. It also refuses to bluff — if the RPC won't return the transaction, it degrades to signature-only and says the amount was not verified."

> **N9** — 28 palavras → **11,2 s** (plano: 16 s)
> "This is the whole permission surface of invoice_status: config read, and HTTP. Two JSON-RPC reads. No key, no signing path, no write to any chain. Custody tier zero."

> **N10** — 43 palavras → **17,2 s** (plano: 20 s)
> "Tell it to watch the invoice and the agent schedules a job on ZeroClaw's native cron, allowed to call two tools: invoice_status and cron_remove. While the invoice is pending it stays silent. When the value clears it sends the receipt and deletes itself."

> **N11** — 51 palavras → **20,4 s** (plano: 22 s)
> "Three plugins, one shared core, no solana-sdk anywhere — it doesn't build for wasm32-wasip2, so base58, SHA-256 and the JSON-RPC shapes are hand-written and host-tested. PIX and USDC as two rails on one invoice, and an amount the merchant can actually check. Code and the pull request are in the description."

**Total falado:** 375 palavras → **150 s** de fala dentro de 176 s de vídeo. Sobram ~26 s de respiro, distribuídos nas pausas entre planos. Se atrasar, o bloco com mais gordura é **N3** (pode terminar em "…any Brazilian bank app.").

---

## 3. Narração em português (alternativa — grave em PT e legende em EN)

Português rende ~15% mais sílabas por palavra que o inglês. Os tempos abaixo já usam **140 palavras/min = 2,33 palavras/s**, que é o ritmo realista de leitura calma em PT-BR.

> **N1 (PT)** — 20 palavras → **8,6 s** (plano: 10 s)
> "Um Raspberry Pi 3 rodando um agente ZeroClaw de verdade. Três plugins WebAssembly, wasm32-wasip2. Sem slides — terminal e celular."

> **N2 (PT)** — 30 palavras → **12,9 s** (plano: 16 s)
> "O lojista cobra em reais, pelo Telegram. Cinquenta e cinco reais, fatura INV-DEMO-A. O agente chama o brl_usdc_invoice: uma fatura, dois trilhos — PIX em reais, e dez USDC na Solana."

> **N3 (PT)** — 40 palavras → **17,2 s** (plano: 18 s)
> "O código PIX é um payload EMV de verdade, com CRC — o cliente cola em qualquer banco brasileiro. O QR da Solana carrega a URL Solana Pay inteira, com uma reference derivada do id da fatura e da pubkey do lojista."

> **N4 (PT)** — 27 palavras → **11,6 s** (plano: 12 s)
> "Uma ressalva honesta: este plugin não enxerga o PIX cair. Transferência bancária não existe on-chain. Só um humano marca o PIX como pago — a ferramenta nunca deduz."

> **N5 (PT)** — 41 palavras → **17,6 s** (plano: 18 s)
> "Agora o trilho Solana, de verdade, na mainnet. A fatura pede dez USDC. Eu vou pagar um — pagamento parcial, mesma reference. Assinando no Phantom, no celular. O agente não encosta nisso: não tem chave e não assina nada."

> **N6 (PT)** — 19 palavras → **8,2 s** (plano: 12 s)
> "De volta ao Telegram: a INV-DEMO-A pagou? O agente chama o invoice_status. Ele deriva a mesma reference e pergunta pra blockchain."

> **N7 (PT)** — 41 palavras → **17,6 s** (plano: 18 s)
> "Aí está. UNDERPAID. Recebido um de dez USDC — faltam nove. Esse número não é chute: o plugin buscou a transação, somou o delta de saldo USDC do lojista, antes e depois, e comparou. Uma assinatura sozinha nunca produziria essa linha."

> **N8 (PT)** — 30 palavras → **12,9 s** (plano: 14 s)
> "A mesma transação no Solscan: um USDC. E ele também não blefa — se o RPC não devolver a transação, ele degrada para 'assinatura ok' e diz que o valor não foi conferido."

> **N9 (PT)** — 28 palavras → **12,0 s** (plano: 16 s)
> "Esta é a superfície de permissão inteira do invoice_status: leitura de config e HTTP. Duas leituras JSON-RPC. Sem chave, sem caminho de assinatura, sem escrita em chain nenhuma. Custódia tier zero."

> **N10 (PT)** — 45 palavras → **19,3 s** (plano: 20 s)
> "Peça pra ele vigiar a fatura e o agente agenda um job no cron nativo do ZeroClaw, autorizado a chamar duas tools: invoice_status e cron_remove. Enquanto a fatura está pendente, ele fica em silêncio. Quando o valor cai, manda o recibo e se apaga."

> **N11 (PT)** — 51 palavras → **21,9 s** (plano: 22 s)
> "Três plugins, um core compartilhado, zero solana-sdk — ele não compila para wasm32-wasip2, então base58, SHA-256 e as formas do JSON-RPC são escritas à mão e testadas no host. PIX e USDC como dois trilhos de uma fatura só, e um valor que o lojista consegue conferir. Código e pull request na descrição."

**Total falado (PT):** 372 palavras → **160 s** dentro de 176 s. Margem menor que na versão EN: **não improvise frase extra em PT**. Se estourar, corte a segunda metade de N3 e a última frase de N10.

---

## 4. Checklist de preparação (antes do REC)

### 4.1 Pi e agente
- [ ] `zeroclaw service` **rodando** no Pi (o cron do ZeroClaw só dispara no tick de manutenção do daemon — sem serviço, o job do plano 10 fica gravado e nunca roda).
- [ ] `zeroclaw plugin list` mostra os 3 plugins na versão da release v0.5.0-plugins.
- [ ] Config aplicada e conferida: `pix_key`, `pix_name`, `pix_city`, `merchant_solana`, `max_amount_brl`, `brl_per_usdc`, `recipient_locked=true`, `watch_hint=true` no `brl-usdc-invoice`; `merchant_solana`, `rpc_url`, `usdc_mint` no `invoice-status`.
- [ ] `brl_per_usdc` = ⟨CONFIRMAR: valor configurado. Este roteiro assume **5.5**, o que faz R$ 55,00 = exatamente 10.000000 USDC. Se for outro, recalcule o `amount_brl` para cair em 10 USDC redondos e ajuste N2 e o plano 2⟩.
- [ ] `max_amount_brl` ≥ 55 (senão a fatura falha fechado, corretamente, e o plano 2 morre).
- [ ] **Redact desligado** no agente de cobrança. Teste antes: emita uma fatura descartável e confira que o link `api.qrserver.com` **não** vem com `[REDACTED_…]` no meio.
- [ ] Skills `pixzclaw-daily` e `pixzclaw-watch` + `SOUL.md` carregados, serviço reiniciado depois de qualquer `config set`.
- [ ] `rpc_url`: ⟨CONFIRMAR: usar endpoint dedicado se existir. O público `api.mainnet-beta.solana.com` limita taxa e é o maior risco de o plano 7 falhar ao vivo⟩.
- [ ] Bot/agent **dedicado** ao PixZClaw, se possível — evita que a sessão traga histórico de outro uso para dentro do plano.

### 4.2 Dinheiro e cadeia
- [ ] Carteira do pagador (celular, Phantom) com **≥ 2,5 USDC** de mainnet e **≥ 0,01 SOL** para taxa e eventual criação de ATA. ⟨CONFIRMAR: saldo real disponível⟩.
- [ ] Valor exato do teste on-chain: **1,000000 USDC**. Fatura: **10 USDC** (R$ 55,00). Falta esperada: **9**.
- [ ] `invoice_id` da demo: **`INV-DEMO-A`**. O da gravação de segurança: **`INV-DEMO-B`**.
- [ ] Conta de token USDC do lojista (`merchant_solana`) já existente — se não existir, a primeira transferência cria a ATA e custa mais SOL; o plugin lida com isso (não há entrada `pre`), mas leva alguns segundos a mais.

### 4.3 QR de pagamento parcial (a parte que precisa ser montada antes)
O QR da própria fatura carrega `amount=10` e o Phantom **não deixa editar** o valor de um transfer request. Para pagar 1 de 10 mantendo **a mesma reference** (é isso que faz o `invoice_status` encontrar a transação), monte antes:

1. Emita a `INV-DEMO-A` **uma vez, em ensaio**, no mesmo bot e com o mesmo `merchant_solana`. A reference é determinística — `bs58(sha256("zc-inv-v1" ‖ invoice_id ‖ "|" ‖ merchant))` — então ela será **idêntica** na gravação.
2. Copie o link do QR da Solana no Telegram (o `https://api.qrserver.com/...?data=solana%3A...`).
3. Decodifique o percent-encoding num editor de texto e extraia o valor de `reference=`.
4. Monte a URL parcial trocando só o amount:
   `solana:<merchant_solana>?amount=1&spl-token=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v&reference=<a mesma reference>&label=...&message=INV-DEMO-A`
5. Gere o QR dessa URL: `https://api.qrserver.com/v1/create-qr-code/?size=480x480&margin=8&data=<url percent-encoded>` e deixe aberto numa aba do desktop.
6. No plano 5, escaneie esse QR com o scanner do Phantom (a gravação de tela do celular mostra o visor da câmera lendo a tela do desktop — não precisa de segunda câmera).

⟨CONFIRMAR: método preferido para extrair a reference completa. A alternativa é rodar `derive_reference` no host a partir do `solana-wasm-core` — mais confiável, exige toolchain Rust no desktop.⟩

**Nota de honestidade:** isso é literalmente o que acontece quando um cliente paga menos do que a fatura pede. Não é encenação de resultado — a transação é real, o valor é real, e a verificação não sabe de nada disso.

### 4.4 Gravação de segurança (faça isto ANTES do REC)
- [ ] Emita a **`INV-DEMO-B`** de 10 USDC e pague **1 USDC** nela pelo mesmo método, **no mínimo 15 min antes** de gravar. Se o pagamento ao vivo do plano 5 não indexar a tempo, os planos 6–8 são refeitos com `INV-DEMO-B` e ninguém percebe: o resultado on-chain é o mesmo e igualmente real.

### 4.5 Telas, janelas e legibilidade
- [ ] Terminal SSH: janela em **1600×900** dentro de um desktop **1920×1080**, fonte monoespaçada em **22 pt** (mínimo 20 pt), ~100 colunas. Tema escuro de alto contraste, **transparência desligada**, sem imagem de fundo.
- [ ] Prompt curto (`arcanjo@pi:~$`) — prompt gigante come largura útil.
- [ ] `clear` antes de cada plano de terminal.
- [ ] Área de trabalho limpa: sem ícones, sem widgets, barra de tarefas escondida.
- [ ] Navegador (plano 8) em janela nova, sem barra de favoritos, sem abas extras, zoom em **125–150%**.
- [ ] Celular: **Não perturbe** ligado, brilho no máximo, bateria > 50%, rotação travada em retrato.
- [ ] Windows: **Assistente de foco / Não perturbe** ligado; feche Slack, Discord, e-mail; silencie o Telegram Desktop se estiver logado na mesma conta (senão a notificação do desktop vaza no vídeo).
- [ ] Telegram do celular no **tema claro ou escuro consistente** e com **tamanho de fonte acima do padrão** (Ajustes → Aparência → tamanho do texto) — o card da fatura é denso e o vídeo vai ser comprimido.

### 4.6 Limpar o histórico do chat
- [ ] No chat com o bot: menu do chat → **Limpar histórico** (em bot chats o Telegram oferece "Limpar histórico" / "Clear chat"). Faça isso **depois** do ensaio da §4.3 e **antes** do REC.
- [ ] Se o bot for compartilhado com outros testes, prefira criar um chat novo com um bot dedicado — evita que uma mensagem antiga apareça no scroll do plano 3.
- [ ] Apague também as mensagens do ensaio no lado do bot (`/start` novo, se o host permitir) para o "digitando…" do plano 6 não vir depois de um histórico visível.
- [ ] Deixe o teclado do celular **fechado** ao final de cada envio — teclado aberto rouba metade da tela nos planos 3 e 7.

### 4.7 Áudio
- [ ] Grave a narração **depois** do vídeo, num take separado, lendo a §2 (ou §3). Não tente narrar ao vivo: você vai errar comando e narração juntos.
- [ ] Ambiente sem eco, microfone a ~20 cm, sem ar-condicionado ligado.
- [ ] Um take por bloco (N1…N11), arquivo separado por bloco: facilita re-locução se o texto do plano 7 sair diferente do previsto.

---

## 5. Plano B por risco

**Regra geral que salva a gravação: cada plano é um take independente.** Nada é gravado em fluxo contínuo. Se um plano quebrar, você regrava **aquele plano**, não o vídeo. A narração é dublada depois, então nada precisa sincronizar ao vivo.

| Risco | O que acontece na tela | Plano B (sem reiniciar a gravação) |
|---|---|---|
| **A transação Solana demora a confirmar / o RPC ainda não indexou a reference** | Plano 7 volta `USDC: PENDING (nenhuma assinatura no reference)` | 1) Espere e repita a pergunta — o "PENDING" é uma resposta válida e você **corta o intervalo na edição**. 2) Se em ~2 min não indexar, refaça os planos 6–8 com a **`INV-DEMO-B`** já paga na §4.4. 3) Nunca narre um valor que não apareceu: reescreva N7 com o que a tela mostra. |
| **O RPC devolve assinatura mas não a transação** | Sai `USDC: SIG OK (valor não verificado — RPC não retornou a transação)` | **Isso é conteúdo bom, não erro.** Grave esse plano, use como plano 8-bis, e reaproveite a frase de N8 ("it refuses to bluff"). Depois repita a pergunta até sair o UNDERPAID e use os dois planos em sequência. |
| **O bot não responde** | Nenhuma mensagem chega depois do envio | 1) Reenvie **exatamente a mesma mensagem** e corte o tempo morto. 2) Se o LLM respondeu sem chamar a tool, reenvie com a forma imperativa: `Use invoice_status com invoice_id INV-DEMO-A e expected_usdc 10.` 3) Se continuar mudo: `zeroclaw service restart` no SSH — esse plano de restart **não entra no vídeo**, e o `invoice_id` é determinístico, então nada se perde. |
| **Saída sai com `[REDACTED_…]`** | Link do QR ou reference mascarados | Reenvie com `Sem redact.` na mensagem. Se persistir, confira a config do agente (o redact é do host, não do plugin). O plano 3 pode ser gravado mostrando **só o QR** — o QR sobrevive à redação porque está percent-encoded. |
| **O Pi trava / OOM / undervoltage** | SSH congela ou o serviço morre | 1) `zeroclaw service restart`. 2) Se o SSH cair, reconecte e rode `clear` — o plano 1 pode ser regravado em 10 s. 3) Nada de estado se perde: a reference é derivada, não armazenada, e a transação já está na chain. 4) Se o Pi cair **depois** do pagamento, os planos 8 (Solscan) e 9 (manifest) podem ser gravados a qualquer momento, inclusive noutro dia. |
| **A rede cai** | Telegram sem entrega, RPC com timeout | O pagamento, se já foi enviado, **é permanente**. Grave o plano 8 (Solscan) mais tarde, do desktop, com a assinatura anotada. Planos 1 e 9 são locais no Pi e não dependem de internet além do SSH. |
| **O Phantom recusa o transfer request** | Erro no plano 5 | Confira saldo de SOL para taxa. Alternativa: abrir a mesma URL `solana:` como **deep link** direto no celular em vez de escanear o QR. |
| **O texto do UNDERPAID sai diferente do previsto** | Números com casas decimais, ou ordem de linha diferente | **Ajuste a narração, nunca o vídeo.** Regrave só o bloco N7 com o texto que apareceu. É por isso que o áudio é feito por bloco. |
| **Estourou 3:00 na montagem** | — | Corte nesta ordem: (1) plano 10 inteiro (cron), (2) plano 4, (3) segunda metade do plano 3. **Planos 5, 6 e 7 são intocáveis.** |

---

## 6. Instruções de captura e montagem (tudo grátis)

Pasta de trabalho sugerida: `E:\zeroclawbount\video\`.

### 6.1 Capturar o desktop (terminal + navegador)
Opção recomendada: **OBS Studio** (gratuito, open source) — captura de janela, 1920×1080, 30 fps, gravação em MKV.
Alternativa só com ffmpeg (Windows, `gdigrab`):

```powershell
# tela inteira
ffmpeg -f gdigrab -framerate 30 -video_size 1920x1080 -offset_x 0 -offset_y 0 -i desktop `
  -c:v libx264 -preset veryfast -crf 18 -pix_fmt yuv420p E:\zeroclawbount\video\raw\desktop.mkv

# apenas a janela do terminal (substitua pelo título exato da janela)
ffmpeg -f gdigrab -framerate 30 -i title="arcanjo@pi: ~" `
  -c:v libx264 -preset veryfast -crf 18 -pix_fmt yuv420p E:\zeroclawbount\video\raw\terminal.mkv
```

Grave em MKV (sobrevive a queda de energia) e converta depois. Encerre a captura com `q` no terminal do ffmpeg, nunca com Ctrl+C duplo.

### 6.2 Capturar o celular
Use o **gravador de tela nativo**: Android (Gravador de tela nas Configurações rápidas) ou iOS (Central de Controle → Gravação de Tela). Grave em **1080×2340** (ou o que o aparelho oferecer), 30 fps, **com o áudio do sistema desligado**. Transfira por cabo — não por WhatsApp/Telegram, que recomprimem e destroem a legibilidade do card.

⟨CONFIRMAR: resolução nativa do gravador do celular do operador⟩

### 6.3 Normalizar todos os trechos para a mesma grade
O concat do ffmpeg exige codec, resolução e fps idênticos. Normalize **cada** clipe bruto:

```powershell
# clipe de desktop → 1920x1080 30fps
ffmpeg -i raw\terminal.mkv -vf "scale=1920:1080:flags=lanczos,fps=30,format=yuv420p" `
  -c:v libx264 -crf 18 -preset medium -an norm\p01.mp4

# clipe de celular (retrato) → 1920x1080 com fundo desfocado do próprio vídeo
ffmpeg -i raw\phone.mp4 -filter_complex `
  "[0:v]scale=1920:1080:force_original_aspect_ratio=increase,crop=1920:1080,gblur=sigma=28[bg];[0:v]scale=-2:1040[fg];[bg][fg]overlay=(W-w)/2:(H-h)/2,fps=30,format=yuv420p" `
  -c:v libx264 -crf 18 -preset medium -an norm\p02.mp4
```

### 6.4 Cortar cada plano (corte exato, re-encodando)

```powershell
# -ss/-to DEPOIS do -i = corte preciso no frame (re-encoda). Não use -c copy aqui.
ffmpeg -i norm\p02.mp4 -ss 00:00:04.500 -to 00:00:20.500 `
  -c:v libx264 -crf 18 -preset medium -an cut\02.mp4
```

Repita para os 11 planos, nomeando `cut\01.mp4` … `cut\11.mp4` com a duração exata da tabela da §1.

### 6.5 Juntar

```powershell
# lista.txt (uma linha por plano, na ordem)
# file 'cut/01.mp4'
# file 'cut/02.mp4'
# ...
# file 'cut/11.mp4'

ffmpeg -f concat -safe 0 -i lista.txt -c copy montagem.mp4
```

### 6.6 Picture-in-picture (celular sobre o terminal)
Útil no plano 5 (Phantom sobre o terminal parado) e como recurso se você quiser manter o terminal visível durante os planos de celular:

```powershell
ffmpeg -i cut\09_terminal.mp4 -i cut\05_phone.mp4 -filter_complex `
  "[1:v]scale=-2:900,setsar=1,pad=iw+8:ih+8:4:4:color=0x1b1f27[pip];[0:v][pip]overlay=W-w-48:H-h-48:shortest=1,format=yuv420p" `
  -c:v libx264 -crf 18 -preset medium -an pip.mp4
```

`overlay=W-w-48:H-h-48` = canto inferior direito com 48 px de margem. Para canto inferior **esquerdo**, use `overlay=48:H-h-48` (útil quando a saída do terminal é longa e ocupa a direita).

### 6.7 Narração e mixagem

```powershell
# gravar a narração (microfone padrão do Windows)
ffmpeg -f dshow -i audio="⟨CONFIRMAR: nome do microfone em 'ffmpeg -list_devices true -f dshow -i dummy'⟩" `
  -ac 1 -ar 48000 -c:a pcm_s16le vo\n07.wav

# juntar os blocos de narração na ordem (concat de áudio)
ffmpeg -f concat -safe 0 -i vo\lista.txt -c copy vo\narracao.wav

# normalizar loudness para padrão de plataforma (-16 LUFS, stereo)
ffmpeg -i vo\narracao.wav -af "loudnorm=I=-16:TP=-1.5:LRA=11" -ar 48000 vo\narracao_norm.wav

# casar áudio com vídeo
ffmpeg -i montagem.mp4 -i vo\narracao_norm.wav -map 0:v -map 1:a `
  -c:v copy -c:a aac -b:a 192k -shortest FINAL.mp4
```

### 6.8 Legendas (obrigatório se narrar em PT)
Escreva `legendas.srt` com a tradução EN alinhada aos timecodes da §1 e queime no vídeo (garante que apareçam em qualquer player):

```powershell
ffmpeg -i FINAL.mp4 -vf "subtitles=legendas.srt:force_style='FontName=Arial,FontSize=22,OutlineColour=&H90000000,BorderStyle=3,MarginV=48'" `
  -c:v libx264 -crf 20 -preset medium -c:a copy FINAL_legendado.mp4
```

### 6.9 Entrega final
- **1920×1080, 30 fps, H.264 (yuv420p) + AAC 192 kbps, MP4.**
- Confira a duração antes de subir: `ffprobe -v error -show_entries format=duration -of csv=p=0 FINAL.mp4` → tem que dar **< 180**.
- Confira legibilidade num celular: se o texto do plano 7 não é legível numa tela de 6", aumente a fonte e regrave aquele plano. É o único plano que **precisa** ser lido pelo jurado.

---

## 7. Metadados de submissão

**Título (77 caracteres):**
> PixZClaw — a ZeroClaw agent that checks the amount on-chain (PIX + USDC, Telegram)

**Título alternativo, mais direto ao ponto:**
> Invoice 10 USDC, pay 1: a ZeroClaw plugin that answers UNDERPAID with the real number

**Descrição:**

```text
PixZClaw — three WebAssembly (wasm32-wasip2) tool plugins for ZeroClaw, running
in production on a Raspberry Pi 3B+, operated from Telegram.

A Brazilian merchant issues one invoice in BRL and gets paid on either of two
rails: PIX (BRL) or USDC on Solana via Solana Pay. Custody T0/T1 only — the
agent holds no private key and signs nothing.

In this video the invoice asks for 10 USDC and I pay 1 USDC for real on mainnet.
invoice_status finds the transaction through the invoice's deterministic
reference, calls getTransaction, sums the merchant's USDC pre/post token balance
deltas, and answers UNDERPAID with the actual amount received and the exact
shortfall. It never reports PAID without a confirmed amount: when the RPC will
not return the transaction it degrades to "signature ok, value not verified".

The PIX rail is not verifiable by software and the product says so out loud —
bank settlement is not on-chain, so a human operator confirms PIX. The tool
never infers it.

Plugins:
- brl_usdc_invoice (T1) — dual-rail invoice: PIX EMV + CRC16, and a Solana Pay
  USDC URL under one invoice_id
- invoice_status  (T0) — value-verified settlement check, shareable receipt
- pixzclaw_brief  (T0) — merchant cash view: balances, 24h close-out, 7d sparkline

No solana-sdk anywhere: it does not build for wasm32-wasip2, so base58, the
SHA-256 reference derivation, the Solana Pay grammar, PIX EMV/CRC16 and the
JSON-RPC shapes are hand-written in a shared core and tested on the host.
101 host tests, no network.

Repo:      https://github.com/capitv/pixzclaw-pi
Release:   https://github.com/capitv/pixzclaw-pi/releases/tag/v0.5.0-plugins
Landing:   https://capitv.github.io/pixzclaw-pi/
PR:        https://github.com/zeroclaw-labs/zeroclaw-plugins/pull/123
ZeroClaw:  https://docs.zeroclawlabs.ai/master/en/introduction.html
Solana Pay spec: https://docs.solanapay.com/spec
Demo transaction on Solscan: ⟨CONFIRMAR: link da tx real de 1 USDC⟩

Built for the ZeroClaw × Superteam Brasil bounty. MIT OR Apache-2.0.
```

**Onde hospedar:**
1. **YouTube, "Não listado" (unlisted)** — recomendação principal: sem limite de duração, sem recompressão agressiva, link estável, funciona em qualquer país e o jurado não precisa de conta. Marque "Não é conteúdo para crianças" e deixe os comentários como preferir.
2. **Espelho:** suba o mesmo MP4 como *release asset* em `capitv/pixzclaw-pi` (link direto, não some se a conta do YouTube tiver problema). Cite os dois links no formulário se o campo aceitar.
3. **Evite** Google Drive (exige permissão e trava em alguns países), WeTransfer (expira) e Loom no plano grátis (limite de duração e marca d'água).
4. No formulário do Superteam Earn, o campo de duração é obrigatório: informe **2:56** e confira com `ffprobe` antes de enviar.

---

## 8. Resumo operacional (cole no celular durante a gravação)

```
P1  0:00 PI   uname -m && zeroclaw plugin list
P2  0:10 TG   "Cobra R$ 55 do cliente, fatura INV-DEMO-A. Use brl_usdc_invoice. Sem redact."
P3  0:26 TG   scroll: PIX copia-e-cola + QR Solana
P4  0:44 TG   rodapé: teto R$ 1000 · destino travado=sim
P5  0:56 PH   Phantom → QR parcial (1 USDC, mesma reference) → confirmar
P6  1:14 TG   "A INV-DEMO-A pagou? Use invoice_status com expected_usdc 10."
P7  1:26 TG   UNDERPAID ⚠️ (recebido 1 de 10 USDC — faltam 9)     ← plano intocável
P8  1:44 WEB  solscan → 1 USDC
P9  1:58 PI   cat ~/.zeroclaw/plugins/invoice-status/manifest.toml
P10 2:14 TG   "avisa quando a INV-DEMO-A pagar" + "quais lembretes tenho?"
P11 2:34 PI   zeroclaw plugin list  (fecha o círculo)
FIM 2:56
```
