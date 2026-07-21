# PixZClaw — Onboarding no Telegram

Use esta skill quando o **dono do agent** (operador allowlisted) disser:

- `/configurar`, `/setup`, `configurar pixzclaw`, `setup pix`, `quero configurar cobrança`

**Não** use com usuários aleatórios do grupo. Só o operador/dono do bot.

## Objetivo

Coletar no chat os dados da loja e **mostrar o resumo + comandos `zeroclaw config set`** para gravar no host.  
Você **não** inventa `__config`. A jail do plugin só recebe o que o operador grava no ZeroClaw.

## Regras de segurança

1. Se o interlocutor **não** for o dono/allowlist do canal → recuse configurar e diga para o dono rodar setup.
2. Nunca peça **seed / private key** Solana. Só **pubkey** de recebimento.
3. Chave PIX é identificador de recebimento (não é senha do banco), mas trate com cuidado no chat.
4. Depois de confirmar, `recipient_locked` deve ficar `true`.
5. Não chame `brl_usdc_invoice` com valores inventados durante o onboarding.

## Fluxo (uma pergunta por vez)

### Passo 1 — Boas-vindas

Explique em 2–3 linhas:

> PixZClaw emite cobrança dual: **PIX (BRL)** + **USDC (Solana Pay)**, mesmo invoice id.  
> O agent **não** segura chave e **não** converte BRL→USDC sozinho.  
> Vamos configurar a **sua** loja (só dono).

### Passo 2 — Coletar

Peça, um de cada vez:

| # | Campo | Exemplo |
|---|---|---|
| 1 | `pix_key` | e-mail, telefone, EVP ou CPF só dígitos |
| 2 | `pix_name` | Nome no QR (curto, sem acento se possível) |
| 3 | `pix_city` | Ex.: SAO PAULO |
| 4 | `merchant_solana` | Pubkey base58 da wallet que **recebe** USDC |
| 5 | `max_amount_brl` | Ex.: 1000 |
| 6 | `brl_per_usdc` | Cotação offline ex.: 5.5 (R$ por 1 USDC) |

Opcional: `max_amount_usdc` (default 200).

### Passo 3 — Resumo

Mostre um card:

```text
╭─ PixZClaw · Setup ─────────────╮
│ PIX key    …                   │
│ Nome/cidade …                  │
│ Solana     7xK…abc             │
│ Max fatura R$ …                │
│ Cotação    1 USDC = R$ …       │
│ Lock       recipient_locked=on │
╰────────────────────────────────╯
Confirma? (sim / não)
```

### Passo 4 — Após “sim”

1. Diga que vai **aplicar** a config (se você tiver tool/shell de config do host **e** permissão do operador).
2. Caso **não** tenha tool de config, envie exatamente estes comandos para o operador colar no Pi (SSH):

```bash
zeroclaw config set plugins.entries.brl-usdc-invoice.config.pix_key "VALOR"
zeroclaw config set plugins.entries.brl-usdc-invoice.config.pix_name "VALOR"
zeroclaw config set plugins.entries.brl-usdc-invoice.config.pix_city "VALOR"
zeroclaw config set plugins.entries.brl-usdc-invoice.config.merchant_solana "VALOR"
zeroclaw config set plugins.entries.brl-usdc-invoice.config.max_amount_brl "VALOR"
zeroclaw config set plugins.entries.brl-usdc-invoice.config.max_amount_usdc "200"
zeroclaw config set plugins.entries.brl-usdc-invoice.config.brl_per_usdc "VALOR"
zeroclaw config set plugins.entries.brl-usdc-invoice.config.recipient_locked "true"

zeroclaw config set plugins.entries.invoice-status.config.merchant_solana "VALOR_SOLANA"
zeroclaw config set plugins.entries.invoice-status.config.rpc_url "https://api.mainnet-beta.solana.com"

zeroclaw config set plugins.entries.pixzclaw-brief.config.merchant_solana "VALOR_SOLANA"
zeroclaw config set plugins.entries.pixzclaw-brief.config.rpc_url "https://api.mainnet-beta.solana.com"
```

Substitua VALOR* pelos dados confirmados.

3. Peça para rodar no Pi e responder “pronto”.
4. Teste sugerido:
   - `Use pixzclaw_brief` → card de caixa  
   - `Cobra R$ 5 invoice teste-onboard. Use brl_usdc_invoice`

### Passo 5 — “não”

Volte ao campo que quiser alterar.

## Comandos do dia a dia (depois do setup)

| User diz | Tool |
|---|---|
| Cobra R$ X… | `brl_usdc_invoice` |
| Invoice Y pagou? | `invoice_status` |
| /caixa, saldo, dashboard | `pixzclaw_brief` |

## O que NÃO dizer

- Que o agent “converte PIX em USDC automaticamente”  
- Que precisa de private key  
- Que qualquer membro do grupo pode reconfigurar a loja  
