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
| 2 | 0:10–0:26 | TG | Chat do bot vazio. O lojista digita e envia: `Cobra R$ 5,50 do cliente, pedido INV-DEMO-A`. O ZeroClaw pede aprovação da tool — **deixe aparecer**, é a superfície de custódia visível na tela. Aprove. O card aparece: `🦞 PixZClaw — Fatura #INV-DEMO-A` · `💰 R$ 5.50 · ₮ 1 USDC`. | **N2** |
| 3 | 0:26–0:42 | TG | Scroll lento pelo card: bloco `🇧🇷 PIX (BRL)` com o link do QR e o copia-e-cola em code block; bloco `◎ Solana Pay (USDC)` com o link do QR. Parar com os dois trilhos visíveis na mesma tela. | **N3** |
| 4 | 0:42–0:52 | TG | Segue no card. Rolagem parando no rodapé: `🔒 teto R$ 1000 · destino travado=sim`. Sem edição gráfica. | **N4** |
| 5 | 0:52–1:10 | PH | Tocar no link `📷 Toque para abrir o QR do Solana Pay`, ler o QR com a câmera do **segundo** aparelho. A Phantom abre já com destino e valor preenchidos: `1 USDC`. Confirmar. Tela de sucesso. **Pagamento real, mainnet.** | **N5** |
| 6 | 1:10–1:20 | TG | De volta ao Telegram. O lojista envia só: `A INV-DEMO-A pagou?`. Indicador de "digitando". | **N6** |
| 7 | 1:20–1:42 | TG | **Plano principal.** A resposta chega: `USDC: PROVÁVEL ⚠️` com a ressalva inteira e a linha de fecho. Bloco parado na tela 5–6 s (texto esperado em §1.1). | **N7** |
| 8 | 1:42–1:58 | WEB | Abrir o link `EXPLORER:` que o próprio plugin devolveu. Mostrar a transferência de `1 USDC` e então **rolar até a lista de contas da transação** — a reference não está lá. A ressalva do plano 7 fica verificada na tela. | **N8** |
| 9 | 1:58–2:14 | TG | Telegram: `confirmo o pagamento da INV-DEMO-A` → resposta com `USDC: PAID ✅` e o bloco `🧾 RECIBO` encaminhável (texto em §1.2). Parar no recibo. | **N9** |
| 10 | 2:14–2:28 | PI | Terminal: `cat ~/plugins/invoice-status/manifest.toml`. Parar na linha `permissions = ["http_client", "config_read"]`. | **N10** |
| 11 | 2:28–2:44 | TG | Telegram: `avisa quando a INV-DEMO-B pagar` → o agente confirma o vigia. Em seguida `quais lembretes tenho?` → lista `pixzclaw-watch-INV-DEMO-B`. | **N11** |
| 12 | 2:44–2:56 | PI | Terminal, comando final já digitado: `zeroclaw plugin list` (volta à abertura, fecha o círculo). Cursor piscando. Fade de áudio, corte seco. | **N12** |

**Soma:** 10 + 16 + 16 + 10 + 18 + 10 + 22 + 16 + 16 + 14 + 16 + 12 = **176 s = 2:56** ✅

### 1.0 Por que este roteiro mudou depois de um pagamento real

A versão anterior encenava um pagamento **parcial** de 1 de 10 USDC e mostrava
`UNDERPAID ⚠️` detectado pela reference. Ela não sobreviveu ao contato com a
mainnet.

Pagando de verdade pela Phantom, medido na transação `3UQpJTip…`:

```text
REFERENCE presente na transação?  NÃO
Memo presente?                    NÃO
delta +181818 para o lojista      SIM
```

A Phantom lê a URI Solana Pay — mostrou o valor exato na tela, logo interpretou
destino e valor — e depois monta uma transferência SPL comum, **descartando a
conta de reference**. O trecho encenado do roteiro antigo não aconteceria com a
carteira que a maioria dos clientes usa.

O roteiro novo mostra o que o sistema realmente faz, e é material melhor: um
produto lidando com o mundo real vale mais que um produto encenado no caso
feliz. Também é a única versão que alguém consegue reproduzir.

### 1.1 Texto exato esperado no plano 7

Saída verbatim de `status_unreferenced_match`, com os números desta demo:

```text
INVOICE: INV-DEMO-A
USDC: PROVÁVEL ⚠️ (recebido 1 em ⟨data⟩ — valor, moeda e destino batem com esta fatura, mas a transação NÃO carrega a reference: a carteira do pagador não a incluiu. Isso é indício, não prova. Se você emitiu outra fatura do mesmo valor, este pagamento pode ser da outra.)
PIX: PENDING (tool não vê SPI do banco; use pix_marked_paid=true se confirmou)
OVERALL: PENDING (indício de pagamento, sem prova de vínculo com esta fatura)
```
```text
REF: https://solscan.io/account/⟨reference completa⟩
EXPLORER: https://solscan.io/tx/⟨assinatura completa⟩
```
```text
👉 Só você pode fechar isso: se confere que este pagamento é da fatura INV-DEMO-A, responda "confirmo o pagamento da INV-DEMO-A" e eu registro com comprovante.
```

Três ausências que valem ser notadas na edição (não custam tempo de fala):
- **não** existe bloco `🧾 RECIBO` — indício não gera comprovante;
- **não** existe a linha `[sistema] Fatura liquidada …` — o vigia continuaria valendo;
- a linha `PIX: PENDING` diz, na própria saída, que a ferramenta não enxerga o SPI.

**Os identificadores vão dentro de bloco de código de propósito.** O host redacta
base58 de alta entropia solto no chat e ignora conteúdo dentro de URLs `https://`
— por isso a reference viaja como URL do Solscan, completa, em vez de truncada.
Custo assumido: URL dentro de cerca não é clicável no Telegram. No plano 8, copie
e cole no navegador do desktop; não tente tocar no link.

### 1.2 Texto exato esperado no plano 9

Saída verbatim de `status_from_declared_tx` após a confirmação do lojista:

```text
INVOICE: INV-DEMO-A
USDC: PAID ✅ (valor conferido na transação informada: 1 de 1 USDC — vínculo com a fatura informado por você, não pela chain: esta transação não carrega a reference)
PIX: PENDING (tool não vê SPI do banco; use pix_marked_paid=true se confirmou)
OVERALL: USDC conferido na transação informada; vínculo com a fatura afirmado pelo operador
```
```text
REF: https://solscan.io/account/⟨reference⟩
EXPLORER: https://solscan.io/tx/⟨assinatura⟩
```
```text
👉 Encaminhe o bloco abaixo ao cliente como comprovante.
```
```text
🧾 RECIBO — INVOICE #INV-DEMO-A
✅ Pago em USDC (Solana)
Valor: 1 USDC (R$ equivalente na fatura)
Data: ⟨data UTC⟩
🔗 https://solscan.io/tx/⟨assinatura⟩
```

O lojista **não digita o hash**: o agente o toma da linha `EXPLORER:` da resposta
anterior. Se ele pedir o hash na tela, o SOUL não foi carregado — pare a gravação
e rode o reinstalador antes de continuar.

### 1.3 Cotação e custo

`brl_per_usdc = 5.5`, então **R$ 5,50 = exatamente 1,000000 USDC**. Número redondo
na tela, e o custo real da gravação é 1 USDC — o mesmo que o roteiro antigo
gastava no pagamento parcial.

### 1.2 Sem muleta na frase

Uma versão anterior deste roteiro mandava `Use brl_usdc_invoice. Sem redact.`
junto com a cobrança. Testado no Pi: o agente escolhe a ferramenta sozinho a
partir de `Cobra R$ 55 do cliente, pedido INV-DEMO-A`, e `Sem redact` nunca fez
nada — a redação de base58 é do host e instrução no chat não a desliga, que é a
razão de o trilho USDC ser QR-only.

As duas frases contavam uma história pior que a verdade: que o agente não
escolhe a ferramenta, e que o operador está contornando algo quebrado. Um
lojista real não digita nome de função.

---

## 2. Narração em inglês (para ler em voz alta)

Ritmo de referência: **150 palavras/min = 2,5 palavras/s**. Cada bloco traz `palavras → segundos falados` e a duração do plano. Todos os blocos cabem com folga.

> **N1** — 19 palavras → **7,6 s** (plano: 10 s)
> "A Raspberry Pi 3 running a real ZeroClaw agent. Three WebAssembly plugins, wasm32-wasip2. No slides — terminal and phone."

> **N2** — 32 palavras → **12,8 s** (plano: 16 s)
> "The merchant charges in reais, from Telegram. Five reais fifty, invoice INV-DEMO-A. The agent picks the tool itself and asks permission first. One invoice, two payment rails — PIX in reais, one USDC on Solana."

> **N3** — 38 palavras → **15,2 s** (plano: 16 s)
> "The PIX string is a real EMV payload with its CRC — it pastes into any Brazilian bank app. The Solana QR carries a full Solana Pay URL, with a reference derived from the invoice id and the merchant key."

> **N4** — 24 palavras → **9,6 s** (plano: 10 s)
> "A ceiling the agent cannot exceed, and a destination it cannot change. Prompt injection can ask for a different wallet. The plugin refuses."

> **N5** — 43 palavras → **17,2 s** (plano: 18 s)
> "The customer scans, on a second phone. Phantom opens with the destination and the amount already filled in — one USDC, real money, mainnet. The agent never touches this. It holds no key and signs nothing."

> **N6** — 20 palavras → **8,0 s** (plano: 10 s)
> "Back in Telegram, plain language: did INV-DEMO-A pay? Nothing else — no hash, no arguments, no tool name."

> **N7** — 52 palavras → **20,8 s** (plano: 22 s)
> "And here is the interesting part. Phantom dropped the reference. It built a plain transfer, so there is nothing on the reference to find. The plugin found the payment anyway, in the merchant's own account — and refuses to call it paid. Right amount, right wallet. Evidence, not proof."

> **N8** — 38 palavras → **15,2 s** (plano: 16 s)
> "Check it. The money is there — one USDC. Now the accounts in that transaction: the reference is not among them. The explorer shows exactly the gap the plugin just described. It was not being cautious. It was being accurate."

> **N9** — 40 palavras → **16,0 s** (plano: 16 s)
> "The merchant is the one who knows. They confirm, and only then does it settle — with a receipt to forward, and a line saying the link came from a human, not from the chain. Same standing as marking PIX paid."

> **N10** — 27 palavras → **10,8 s** (plano: 14 s)
> "This is the whole permission surface: HTTP, and config read. No key, no signing path, no write to any chain. Custody tier zero."

> **N11** — 38 palavras → **15,2 s** (plano: 16 s)
> "Ask it to watch an invoice and it schedules a job on ZeroClaw's native cron. While the invoice is pending it stays silent. When the value clears it sends the receipt and deletes itself."

> **N12** — 45 palavras → **18,0 s** (plano: 12 s → **corte a última frase se estourar**)
> "Three plugins, one shared core, no solana-sdk — it doesn't build for wasm32-wasip2, so base58, SHA-256 and the JSON-RPC shapes are hand-written and host-tested. Two rails on one invoice, and a number the merchant can check. Code in the description."

**Total falado:** 417 palavras → **167 s** de fala dentro de 176 s. A margem é menor
que na versão anterior porque N7 e N8 carregam o argumento central. Se atrasar,
corte nesta ordem: última frase de N12, depois a última de N8 ("It was not being
cautious…" — dói, mas é a que menos informa), depois a segunda de N3.

---

## 3. Narração em português (alternativa — grave em PT e legende em EN)

Português rende ~15% mais sílabas por palavra que o inglês. Os tempos abaixo já usam **140 palavras/min = 2,33 palavras/s**, que é o ritmo realista de leitura calma em PT-BR.

> **N1 (PT)** — 20 palavras → **8,6 s** (plano: 10 s)
> "Um Raspberry Pi 3 rodando um agente ZeroClaw de verdade. Três plugins WebAssembly, wasm32-wasip2. Sem slides — terminal e celular."

> **N2 (PT)** — 31 palavras → **13,3 s** (plano: 16 s)
> "O lojista cobra em reais, pelo Telegram. Cinco e cinquenta, fatura INV-DEMO-A. O agente escolhe a ferramenta sozinho e pede permissão antes. Uma fatura, dois trilhos — PIX em reais, um USDC na Solana."

> **N3 (PT)** — 36 palavras → **15,5 s** (plano: 16 s)
> "O código PIX é um payload EMV de verdade, com CRC — cola em qualquer banco brasileiro. O QR da Solana carrega a URL Solana Pay inteira, com a reference derivada do id da fatura e da chave do lojista."

> **N4 (PT)** — 23 palavras → **9,9 s** (plano: 10 s)
> "Um teto que o agente não passa e um destino que ele não troca. Injeção de prompt pode pedir outra carteira. O plugin recusa."

> **N5 (PT)** — 40 palavras → **17,2 s** (plano: 18 s)
> "O cliente escaneia, num segundo celular. A Phantom abre com destino e valor já preenchidos — um USDC, dinheiro real, mainnet. O agente não encosta nisso. Não tem chave e não assina nada."

> **N6 (PT)** — 19 palavras → **8,2 s** (plano: 10 s)
> "De volta ao Telegram, em português comum: a INV-DEMO-A pagou? Só isso — sem hash, sem argumento, sem nome de função."

> **N7 (PT)** — 48 palavras → **20,6 s** (plano: 22 s)
> "E aqui está a parte interessante. A Phantom descartou a reference. Montou uma transferência comum, então não há nada na reference pra achar. O plugin achou o pagamento assim mesmo, na conta do próprio lojista — e se recusa a chamar de pago. Indício, não prova."

> **N8 (PT)** — 37 palavras → **15,9 s** (plano: 16 s)
> "Confere. O dinheiro está lá — um USDC. Agora as contas dessa transação: a reference não está entre elas. O explorer mostra exatamente a lacuna que o plugin acabou de descrever. Ele não estava sendo cauteloso. Estava sendo exato."

> **N9 (PT)** — 37 palavras → **15,9 s** (plano: 16 s)
> "Quem sabe é o lojista. Ele confirma, e só então liquida — com recibo pra encaminhar, e uma linha dizendo que o vínculo veio de um humano, não da chain. Mesmo status de marcar o PIX como pago."

> **N10 (PT)** — 25 palavras → **10,7 s** (plano: 14 s)
> "Esta é a superfície de permissão inteira: HTTP e leitura de config. Sem chave, sem caminho de assinatura, sem escrita em chain nenhuma. Custódia tier zero."

> **N11 (PT)** — 36 palavras → **15,5 s** (plano: 16 s)
> "Peça pra ele vigiar uma fatura e o agente agenda um job no cron nativo do ZeroClaw. Enquanto está pendente, ele fica em silêncio. Quando o valor cai, manda o recibo e se apaga."

> **N12 (PT)** — 42 palavras → **18,0 s** (plano: 12 s → **corte a última frase se estourar**)
> "Três plugins, um core compartilhado, zero solana-sdk — não compila para wasm32-wasip2, então base58, SHA-256 e as formas do JSON-RPC são escritas à mão e testadas no host. Dois trilhos numa fatura só, e um número que o lojista confere. Código na descrição."

**Total falado (PT):** 374 palavras → **161 s** dentro de 176 s. **Não improvise frase
extra em PT.** Se estourar, corte na ordem: última frase de N12, última de N8,
segunda de N3.

---

## 4. Checklist de preparação (antes do REC)

### 4.1 Pi e agente
- [ ] `zeroclaw service` **rodando** no Pi (o cron do ZeroClaw só dispara no tick de manutenção do daemon — sem serviço, o job do plano 10 fica gravado e nunca roda).
- [ ] `zeroclaw plugin list` mostra `brl-usdc-invoice v0.3.6`, `invoice-status v0.3.2`, `pixzclaw-brief v0.3.2` (release **v0.5.6-plugins** — a primeira em que o card chega ao Telegram literal).
- [ ] Config aplicada e conferida: `pix_key`, `pix_name`, `pix_city`, `merchant_solana`, `max_amount_brl`, `brl_per_usdc`, `recipient_locked=true`, `watch_hint=true` no `brl-usdc-invoice`; `merchant_solana`, `rpc_url`, `usdc_mint` no `invoice-status`.
- [x] `brl_per_usdc` = **5.5** — confirmado em 2026-07-28 pelo próprio card, que imprime a cotação usada (`cotação R$/USDC usada: 5.5`). R$ 55,00 = exatamente 10.000000 USDC.
- [ ] `max_amount_brl` ≥ 55 (senão a fatura falha fechado, corretamente, e o plano 2 morre).
- [x] **Redact não precisa ser desligado — e não pode.** Medido: o host redacta base58 de alta entropia no chat sempre, mas **pula o conteúdo dentro de URLs `https://`**. Por isso o trilho USDC é QR-only: o link `api.qrserver.com` carrega a URI Solana Pay inteira e passa intacto. Uma linha `solana:` crua vira `solana:[REDACTED_HIGH_ENTROPY_TOKEN]?…` — a v0.5.4 tentou e foi revertida na v0.5.5. Teste de fumaça antes de gravar: emita uma fatura descartável e confira que não há `[REDACTED_` em lugar nenhum do card.
- [ ] Skills `pixzclaw-daily` e `pixzclaw-watch` + `SOUL.md` carregados, serviço reiniciado depois de qualquer `config set`.
- [ ] `rpc_url`: ⟨CONFIRMAR: usar endpoint dedicado se existir. O público `api.mainnet-beta.solana.com` limita taxa e é o maior risco de o plano 7 falhar ao vivo⟩.
- [ ] Bot/agent **dedicado** ao PixZClaw, se possível — evita que a sessão traga histórico de outro uso para dentro do plano.

### 4.2 Dinheiro e cadeia
- [ ] Carteira do pagador (celular, Phantom) com **≥ 2,5 USDC** de mainnet e **≥ 0,01 SOL** para taxa e eventual criação de ATA. ⟨CONFIRMAR: saldo real disponível⟩.
- [ ] Valor da demo: fatura de **R$ 5,50 = 1,000000 USDC** (cotação 5.5). Custo real da gravação: **1 USDC**.
- [ ] `invoice_id` da demo: **`INV-DEMO-A`**. Segurança: **`INV-DEMO-B`**. Vigia (plano 11): **`INV-DEMO-B`**.
- [ ] Conta de token USDC do lojista (`merchant_solana`) já existente — se não existir, a primeira transferência cria a ATA e custa mais SOL; o plugin lida com isso (não há entrada `pre`), mas leva alguns segundos a mais.
- [ ] **Só uma fatura aberta com valor de 1 USDC.** Se houver outra, a varredura por valor encontra as duas e o plano 7 sai com `ATENÇÃO: 2 transferências deste mesmo valor` — que é o comportamento correto, e uma cena diferente da roteirizada. Use ids diferentes **e valores diferentes** nos ensaios (R$ 5,50 na gravação, R$ 6,05 nos ensaios).

### 4.3 Nada para montar antes — o QR da fatura é o QR do pagamento

A versão anterior deste roteiro montava um QR de pagamento parcial à mão, porque
a Phantom não deixa editar o valor de um transfer request. Isso saiu junto com o
`UNDERPAID` encenado (ver §1.0). Agora o plano 5 usa **o QR que o próprio card
emitiu**, sem preparação:

1. No Telegram, toque em `📷 Toque para abrir o QR do Solana Pay` — abre a imagem.
2. Leia essa imagem com a câmera do **segundo** aparelho. Não dá para escanear a
   própria tela, e o card diz isso ao cliente.
3. A Phantom abre com destino e valor já preenchidos. Confirmar.

Medido: a Phantom interpreta a URI (mostra o valor exato) e depois monta uma
transferência SPL comum, **sem a conta de reference**. É essa lacuna que os
planos 7 e 8 mostram.

**Ensaio a seco, sem gastar nada.** O verificador imprime a reference derivada
antes de qualquer veredito e confirma que a conta está zerada:

```bash
cd examples/verify-live
cargo run -- --merchant <merchant_solana> --invoice INV-DEMO-A --expected 1
```

Usa a mesma `derive_reference` do plugin, então a reference impressa é exatamente
a que o `invoice_status` vai procurar. Serve também para conferir o RPC antes de
gravar — se ele falhar aqui, falharia ao vivo.

### 4.4 Gravação de segurança (faça isto ANTES do REC)
- [ ] Emita a **`INV-DEMO-B`** de R$ 5,50 e pague pelo mesmo método, **no mínimo 15 min antes** de gravar. Se o pagamento ao vivo do plano 5 não indexar a tempo, os planos 6–9 são refeitos com `INV-DEMO-B` e ninguém percebe: o resultado on-chain é o mesmo e igualmente real.
- [ ] **Atenção ao conflito com §4.2:** duas faturas de 1 USDC pagas no mesmo dia fazem a varredura por valor achar as duas. Para a gravação de segurança, use **R$ 6,05 (1,1 USDC)** na `INV-DEMO-B`, e o valor volta a ser único.

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
- [ ] Um take por bloco (N1…N12), arquivo separado por bloco: facilita re-locução se o texto do plano 7 sair diferente do previsto.

---

## 5. Plano B por risco

**Regra geral que salva a gravação: cada plano é um take independente.** Nada é gravado em fluxo contínuo. Se um plano quebrar, você regrava **aquele plano**, não o vídeo. A narração é dublada depois, então nada precisa sincronizar ao vivo.

| Risco | O que acontece na tela | Plano B (sem reiniciar a gravação) |
|---|---|---|
| **A transação demora a indexar** | Plano 7 volta `USDC: PENDING (nenhuma assinatura no reference)` e **nada** de PROVÁVEL | 1) Espere e repita a pergunta — corte o intervalo na edição. 2) Se em ~2 min não sair, refaça os planos 6–9 com a **`INV-DEMO-B`** já paga na §4.4. 3) Nunca narre o que não apareceu: reescreva N7 com o texto da tela. |
| **A carteira do pagador INCLUIU a reference** | Plano 7 sai `USDC: PAID ✅` direto, sem a ressalva | **Isso não é erro, é o caminho bom** — e derruba os planos 7, 8 e 9 como roteirizados. Grave assim mesmo: vira uma demo mais curta e mais forte (prova criptográfica, sem intervenção humana). Reescreva N7 para "the reference is in the transaction, so the chain answers by itself", corte o plano 9 inteiro e ganhe 16 s. Só a Phantom foi medida descartando a reference; outra carteira pode não descartar. |
| **Sai `SIG OK (valor não verificado)`** | O RPC não devolveu a transação | **Conteúdo bom.** Grave, use como 8-bis com a frase "it refuses to bluff", e repita a pergunta até sair o veredito previsto. |
| **O bot não responde** | Nenhuma mensagem chega | 1) Reenvie a **mesma** mensagem e corte o tempo morto. 2) Se ele respondeu sem chamar a tool, reenvie na forma imperativa: `Use invoice_status com invoice_id INV-DEMO-A e expected_usdc 1.` 3) Se continuar mudo: `zeroclaw service restart` — esse plano **não entra no vídeo**, e o `invoice_id` é determinístico, nada se perde. |
| **Sai `[REDACTED_…]`** | Reference ou link mascarados | **Não existe `Sem redact` — não tente.** A redação é do host e ignora instrução no chat; ela pula conteúdo dentro de URLs `https://`, e é por isso que a reference viaja como link do Solscan. Se aparecer redação mesmo assim, os plugins estão desatualizados: pare e reinstale a release corrente. |
| **`ATENÇÃO: N transferências deste mesmo valor`** | Plano 7 traz o aviso de ambiguidade | Havia outra fatura do mesmo valor paga na janela (ver §4.2). Emita a fatura de novo com **outro valor** e refaça os planos 2 e 5–9. Alternativa honesta: **mantenha** e narre — mostra o sistema recusando um palpite. Custa uma frase nova em N7. |
| **O agente pede o hash na tela** | Plano 9 não fecha sozinho | O SOUL não foi carregado. Pare, rode o reinstalador (ele instala SOUL e skills), `zeroclaw service restart`, e refaça o plano 9. |
| **O Pi trava** | SSH congela ou o serviço morre | 1) `zeroclaw service restart`. 2) Plano 1 se regrava em 10 s. 3) Nada de estado se perde: a reference é derivada, não armazenada, e a transação já está na chain. 4) Planos 8 e 10 podem ser gravados noutro dia. |
| **A rede cai** | Telegram sem entrega, RPC com timeout | O pagamento, se enviado, **é permanente**. Grave o plano 8 depois, com a assinatura anotada. Planos 1 e 10 são locais no Pi. |
| **O Phantom recusa** | Erro no plano 5 | Confira saldo de SOL para a taxa. Alternativa: abrir a URI `solana:` como deep link no celular em vez de escanear. |
| **O texto sai diferente do previsto** | Ordem de linha ou números diferentes | **Ajuste a narração, nunca o vídeo.** Regrave só o bloco afetado — é para isso que o áudio é por bloco. |
| **Estourou 3:00** | — | Corte nesta ordem: (1) plano 11 inteiro (cron), (2) plano 4, (3) segunda metade do plano 3. **Planos 5, 7, 8 e 9 são intocáveis** — são o argumento inteiro. |

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

**Título (79 caracteres):**
> PixZClaw — a ZeroClaw agent that knows what it proved and what it only observed

**Título alternativo, mais direto ao ponto:**
> Phantom dropped the Solana Pay reference. The plugin found the payment and still refused to call it paid.

**Descrição:**

```text
PixZClaw — three WebAssembly (wasm32-wasip2) tool plugins for ZeroClaw, running
in production on a Raspberry Pi 3B+, operated from Telegram.

A Brazilian merchant issues one invoice in BRL and gets paid on either of two
rails: PIX (BRL) or USDC on Solana via Solana Pay. Custody T0/T1 only — the
agent holds no private key and signs nothing.

The payment in this video is real, on mainnet, and so is the complication.
Phantom reads the Solana Pay URI — it fills in the exact amount — and then
builds a plain SPL transfer, dropping the reference account. So there is nothing
on the invoice's reference to find, and the first version of this demo would
have shown a paid invoice reported as pending.

invoice_status looks in the merchant's own token account instead, finds a
transfer of the exact invoiced amount, and reports it as PROVÁVEL — evidence,
not proof. Right amount, right mint, right wallet; no cryptographic link to this
invoice, because the wallet did not put one there. Two invoices for the same
amount would be indistinguishable that way, and the verdict says so rather than
rounding up to "paid". The merchant — the one party who does know — confirms,
and only then does it settle, with a receipt that records that the link was
asserted by a human.

Everything it can prove, it proves by arithmetic: getTransaction, the merchant's
USDC pre/post token balance deltas, exact integer comparison, no tolerance band.
When the RPC will not return a transaction it degrades to "signature ok, value
not verified" rather than guessing.

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
152 host tests, no network.

Repo:      https://github.com/capitv/pixzclaw-pi
Release:   https://github.com/capitv/pixzclaw-pi/releases/tag/v0.5.13-plugins
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
P2  0:10 TG   "Cobra R$ 5,50 do cliente, pedido INV-DEMO-A"     (aprovar a tool)
P3  0:26 TG   scroll: PIX copia-e-cola + os dois links de QR
P4  0:42 TG   rodapé: teto R$ 1000 · destino travado=sim
P5  0:52 PH   2o celular lê o QR do card → Phantom c/ 1 USDC → confirmar
P6  1:10 TG   "A INV-DEMO-A pagou?"                    (só isso, sem argumento)
P7  1:20 TG   PROVÁVEL ⚠️ + ressalva + linha de fecho   ← INTOCÁVEL, 5-6 s parado
P8  1:42 WEB  solscan: 1 USDC entrou / reference NÃO está nas contas
P9  1:58 TG   "confirmo o pagamento da INV-DEMO-A" → PAID ✅ + 🧾 RECIBO
P10 2:14 PI   cat ~/plugins/invoice-status/manifest.toml
P11 2:28 TG   "avisa quando a INV-DEMO-B pagar" + "quais lembretes tenho?"
P12 2:44 PI   zeroclaw plugin list  (fecha o círculo)
FIM 2:56
```

**Se der errado no P7 e sair `PAID ✅` direto:** a carteira incluiu a reference.
Não é erro — grave, corte o P9, e reescreva N7. Ver §5.
