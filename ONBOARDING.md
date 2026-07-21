# PixZClaw — Onboarding (dono do agent)

## Dashboard: status

| Item | Status |
|---|---|
| Plugin `pixzclaw-brief` | ✅ implementado |
| Tool `pixzclaw_brief` | ✅ T0, card + sparkline |
| WASM + release v0.2.0 | ✅ [release](https://github.com/capitv/pixzclaw-pi/releases/tag/v0.2.0-plugins) |
| Testes host | ✅ |

## Suite completa

1. **brl-usdc-invoice** — cobra PIX + USDC  
2. **invoice-status** — confere USDC  
3. **pixzclaw-brief** — dashboard /caixa  

## A) Install plugins no Pi (se ainda não tiver o brief)

```bash
cd ~
wget https://github.com/capitv/pixzclaw-pi/releases/download/v0.2.0-plugins/pixzclaw-plugins-v0.2.0.tar.gz
tar -xzf pixzclaw-plugins-v0.2.0.tar.gz

zeroclaw plugin install ~/plugins/brl-usdc-invoice
zeroclaw plugin install ~/plugins/invoice-status
zeroclaw plugin install ~/plugins/pixzclaw-brief
zeroclaw plugin list
```

## B) Config rápida (SSH) — sem onboarding de chat

```bash
zeroclaw config set plugins.entries.brl-usdc-invoice.config.pix_key "SUA_CHAVE"
zeroclaw config set plugins.entries.brl-usdc-invoice.config.pix_name "SEU NOME"
zeroclaw config set plugins.entries.brl-usdc-invoice.config.pix_city "SAO PAULO"
zeroclaw config set plugins.entries.brl-usdc-invoice.config.merchant_solana "SUA_PUBKEY"
zeroclaw config set plugins.entries.brl-usdc-invoice.config.max_amount_brl "1000"
zeroclaw config set plugins.entries.brl-usdc-invoice.config.max_amount_usdc "200"
zeroclaw config set plugins.entries.brl-usdc-invoice.config.brl_per_usdc "5.5"
zeroclaw config set plugins.entries.brl-usdc-invoice.config.recipient_locked "true"

zeroclaw config set plugins.entries.invoice-status.config.merchant_solana "SUA_PUBKEY"
zeroclaw config set plugins.entries.invoice-status.config.rpc_url "https://api.mainnet-beta.solana.com"

zeroclaw config set plugins.entries.pixzclaw-brief.config.merchant_solana "SUA_PUBKEY"
zeroclaw config set plugins.entries.pixzclaw-brief.config.rpc_url "https://api.mainnet-beta.solana.com"
```

## C) Onboarding no Telegram (UX)

Skills em `skills/`:

| Skill | Uso |
|---|---|
| `pixzclaw-onboard` | `/configurar` — coleta dados e gera `config set` |
| `pixzclaw-daily` | cobrança / status / caixa no dia a dia |
| `pixzclaw-watch` | “avisa quando a X pagar” — lembrete automático via cron |

### Instalar skills no ZeroClaw

ZeroClaw carrega skills do workspace / plugins dir (varia por versão). Opções:

**1) Copiar para o workspace do agent**

```bash
mkdir -p ~/.zeroclaw/workspace/skills
cp -r ~/skills/pixzclaw-onboard ~/skills/pixzclaw-daily ~/skills/pixzclaw-watch ~/.zeroclaw/workspace/skills/ 2>/dev/null || true
# se você clonou o repo:
# cp -r ~/pixzclaw-pi/skills/* ~/.zeroclaw/workspace/skills/
```

**2) Ou colar o conteúdo de `SKILL.md` no system prompt / soul do agent**

O importante: o modelo **segue o fluxo** e no fim entrega os `config set` para o dono rodar no Pi (a menos que o host permita config tool só para o dono).

### Fluxo no chat

```text
Você: /configurar
Bot:  pergunta PIX, nome, cidade, pubkey, teto…
Você: (responde)
Bot:  resumo + "confirma?"
Você: sim
Bot:  cola os comandos config set (ou aplica se tiver tool)
Você: (cola no SSH se preciso)
Você: me mostra o caixa
Bot:  pixzclaw_brief → card
```

## D) Testes no Telegram

```text
/caixa
→ tool pixzclaw_brief

Cobra R$ 25 invoice demo-1 café
→ brl_usdc_invoice

demo-1 pagou?
→ invoice_status

avisa quando a demo-1 pagar
→ cron_add (ver seção abaixo)
```

## Lembrete automático de pagamento

O lojista não precisa ficar perguntando “pagou?”. O agente agenda um vigia no **cron nativo do ZeroClaw** (skill `pixzclaw-watch`).

### Fluxo em 3 passos

1. **Pedir** — no Telegram: `avisa quando a INV-0E9175E9 pagar` (ou tocar no CTA que a fatura sugere).
2. **Agendar** — o agente chama `cron_add` sozinho: job `agent`, sessão `isolated`, `{"kind":"every","every_ms":300000}` (5 min), allowlist `["invoice_status","cron_remove"]`, entrega `announce` no Telegram. Nome do job: `pixzclaw-watch-<invoice_id>`.
3. **Avisar e sumir** — a cada tick o job chama `invoice_status` **em silêncio**; só fala quando o USDC cai (manda o recibo) e então se apaga sozinho com `cron_remove`. PENDING nunca gera mensagem.

### Como cancelar / gerenciar

```text
quais lembretes eu tenho?   → cron_list
para de vigiar a 412        → cron_list + cron_remove
muda pra 10 em 10 minutos   → cron_update
```

Ou direto no Pi:

```bash
zeroclaw cron list
zeroclaw cron remove pixzclaw-watch-INV-0E9175E9
```

### Requisitos

- O **serviço/daemon precisa estar rodando** (`zeroclaw service status`). Em host só-gateway o job fica gravado mas não dispara.
- Intervalo mínimo: **60000 ms** (1 min). Padrão recomendado: 5 min.
- Sugestão de teto: até **5 lembretes ativos** por lojista.

## Segurança do onboarding

- Só o **dono** (allowlist Telegram) deve reconfigurar.  
- Nunca pedir private key.  
- `recipient_locked=true` após setup.  
