# PixZClaw / ZeroClaw Solana Bounty — Handoff completo

**Última atualização:** 2026-07-21 (v0.4.0 publicada)  
**Objetivo do handoff:** outro agente (ou humano) continuar o trabalho sem redescobrir contexto.

---

## 1. O que é este projeto

### Bounty
- **Superteam Earn — Build Solana-native plugins for ZeroClaw**  
- Listing: https://superteam.fun/earn/listing/zeroclaw/  
- Sponsor: Superteam Brasil  
- Stack exigida: **tool plugins** `wasm32-wasip2`, world `tool-plugin` (WIT v0)  
- Custody: preferir **T0/T1** (sem private key no plugin)  
- Layout canônico: `plugins/redact-text`  
- Repo de plugins oficial: https://github.com/zeroclaw-labs/zeroclaw-plugins  
- Runtime: https://github.com/zeroclaw-labs/zeroclaw  
- Docs ZeroClaw: https://docs.zeroclawlabs.ai/master/en/introduction.html  
- Plugin authoring: https://docs.zeroclawlabs.ai/master/en/plugins/writing-a-tool-plugin.html  
- Plugins overview: https://docs.zeroclawlabs.ai/master/en/plugins/index.html  
- WIT: https://github.com/zeroclaw-labs/zeroclaw-plugins/tree/main/wit  
- Solana Pay spec: https://docs.solanapay.com/spec  

### Produto: **PixZClaw**
Marca comercial do pacote de plugins:

| Plugin (manifest) | Tool (LLM) | Tier | Função |
|---|---|---|---|
| `brl-usdc-invoice` | `brl_usdc_invoice` | **T1** | Emite fatura dual: PIX Copia e Cola (BRL) + Solana Pay (USDC) |
| `invoice-status` | `invoice_status` | **T0** | Checa se USDC da fatura caiu (reference/memo) |
| `pixzclaw-brief` | `pixzclaw_brief` | **T0** | Dashboard/caixa: saldo USDC/SOL + sparkline 7d + últimas txs |

**Não faz (de propósito):**
- Converter BRL→USDC automaticamente (não é on-ramp)
- Guardar private key / assinar tx (T2)
- Botões inline nativos Telegram (host limitado) — QR via **link de imagem**

**Conversão de moeda:** só cotação offline `brl_per_usdc` no config para montar o amount do Solana Pay. PIX e USDC são **dois trilhos de pagamento**, não câmbio.

---

## 2. Links essenciais (bookmark)

### Código / deploy (nosso)
| Recurso | URL |
|---|---|
| Repo deploy + skills | https://github.com/capitv/pixzclaw-pi |
| Skills | https://github.com/capitv/pixzclaw-pi/tree/main/skills |
| SOUL (persona 🦞) | https://github.com/capitv/pixzclaw-pi/blob/main/skills/SOUL.md |
| Onboarding skill | https://github.com/capitv/pixzclaw-pi/blob/main/skills/pixzclaw-onboard/SKILL.md |
| Daily skill | https://github.com/capitv/pixzclaw-pi/blob/main/skills/pixzclaw-daily/SKILL.md |
| ONBOARDING.md | https://github.com/capitv/pixzclaw-pi/blob/main/ONBOARDING.md |
| PI_INSTALL.md | https://github.com/capitv/pixzclaw-pi/blob/main/PI_INSTALL.md |
| CI workflow (fonte) | https://github.com/capitv/pixzclaw-pi/blob/main/ci/build-pi.yml |
| Release **v0.5.0** (lembrete cron) | https://github.com/capitv/pixzclaw-pi/releases/tag/v0.5.0-plugins |
| Landing page (GitHub Pages) | https://capitv.github.io/pixzclaw-pi/ |
| Release v0.4.0 (valor verificado + recibo + caixa) | https://github.com/capitv/pixzclaw-pi/releases/tag/v0.4.0-plugins |
| Release v0.3.0 (fatura + QR) | https://github.com/capitv/pixzclaw-pi/releases/tag/v0.3.0-plugins |
| Release v0.2.1 (brief + skills) | https://github.com/capitv/pixzclaw-pi/releases/tag/v0.2.1-plugins |
| Artifact Actions (binário aarch64) | via Actions run do repo (login); não é URL pública estável |

### Workspace local (Windows dev)
```
E:\zeroclawbount\
├── HANDOFF.md                 ← este arquivo
├── PLANNING.md                ← plano bounty / dual-rail
├── WORKFLOW.md                ← multi-agente
├── QUICK_WINS.md              ← checklist polish
├── BOUNTY_STATUS.md
├── DEMO_SCRIPT.md
├── examples\demo-chat.md
├── scripts\build-all.ps1
├── zeroclaw-plugins\          ← clone de trabalho + código plugins
│   ├── crates\solana-wasm-core\
│   └── plugins\
│       ├── brl-usdc-invoice\
│       ├── invoice-status\
│       └── pixzclaw-brief\
├── pixzclaw-pi\               ← repo GitHub capitv/pixzclaw-pi
└── dist-pixzclaw\             ← staging de release tarballs
```

### Upstream
| Recurso | URL |
|---|---|
| ZeroClaw | https://github.com/zeroclaw-labs/zeroclaw |
| zeroclaw-plugins | https://github.com/zeroclaw-labs/zeroclaw-plugins |
| redact-text (referência) | https://github.com/zeroclaw-labs/zeroclaw-plugins/tree/main/plugins/redact-text |
| telegram plugin (padrão waki HTTP) | https://github.com/zeroclaw-labs/zeroclaw-plugins/tree/main/plugins/telegram |

### Hardware do usuário
- **Raspberry Pi 3** aarch64, ~905 MiB RAM, user `arcanjo`
- **Não compilar** ZeroClaw no Pi (SIGSEGV / undervoltage / OOM)
- Binário com plugins: build via **GitHub Actions** cross aarch64 ou artifact baixado
- Install path binário típico: `~/.cargo/bin/zeroclaw`
- Plugins instalados: `~/.zeroclaw/plugins/`
- Skills: `~/.zeroclaw/workspace/skills/`
- Config: `zeroclaw config set plugins.entries.<name>.config.<key>`

---

## 3. Arquitetura técnica

### Pure core / thin shim (obrigatório bounty)
- Lógica em Rust puro (`rlib`), testável com `cargo test` no host
- Shim `#[cfg(target_family = "wasm")]` + `wit-bindgen` 0.46 + world `tool-plugin`
- HTTP: `waki` só em wasm; trait `HttpTransport` no core
- **Sem** `solana-sdk` / `solana-client`

### Crate `solana-wasm-core`
Módulos: `amount`, `pix` (EMV+CRC16), `solana_pay`, `reference` (sha256→bs58), `rpc`, `invoice`, `status`, `dashboard`, `shape`

### Fatura (v0.3.0 UX)
- Output formatado para Telegram com:
  - Links QR: `https://api.qrserver.com/v1/create-qr-code/?size=320x320&data=...`
  - PIX copia e cola + link `solana:` completo
- Instrução embutida: agent **não** deve usar **redact** (quebra pubkey/mint)

### Reference determinística
```
bs58(sha256("zc-inv-v1" || invoice_id || "|" || merchant)[0..32])
```
Memo: `PIX|BRL|<invoice_id>|<short>`

---

## 4. Etapa atual (status)

### Feito ✅
1. Planejamento dual-rail PIX↔USDC (T0/T1)  
2. Core + 3 plugins implementados e testados no host  
3. Build `wasm32-wasip2`  
4. Repo `capitv/pixzclaw-pi` + releases de plugins  
5. Pi: ZeroClaw **com** subcomando `plugin` (binário aarch64 de Actions)  
6. Pi: 3 plugins **instalados** (`plugin list` ok)  
7. Skills onboarding + daily + SOUL (🦞 PixZClaw)  
8. UX fatura v0.3.0 (QR links) release criada  
9. Docs locais: PLANNING, WORKFLOW, QUICK_WINS, DEMO_SCRIPT, ONBOARDING  
10. **v0.4.0** (2026-07-21, release `v0.4.0-plugins`, plugins @ 0.2.0):
    - invoice-status: **valor verificado on-chain** via `getTransaction`
      (delta pre/postTokenBalances) → PAID ✅ / UNDERPAID ⚠️ / OVERPAID;
      soma pagamentos parciais (até 5 sigs); spam não mascara pagamento;
      nunca PAID sem valor conferido (degrada p/ `SIG OK`).
      Recibo PT-BR compartilhável quando pago.
    - pixzclaw-brief: fechamento de caixa 24h (txs, faturas PIX, ids),
      legenda sparkline, horários relativos.
    - brl-usdc-invoice: card novo — PIX em code block (tap-to-copy),
      bloco "Encaminhe ao cliente", cotação no rodapé; USDC **QR-only**.
    - 92 testes host. Feito por times Opus A/B + validação Fable.

11. **v0.5.0** (2026-07-21, release `v0.5.0-plugins`, plugins @ 0.3.0):
    lembrete automático via **cron nativo do ZeroClaw** (`cron_add`).
    Fatura oferece CTA "avisa quando a X pagar" (config `watch_hint`);
    `invoice_status` emite `[sistema] Fatura liquidada … cron_remove`
    quando confirma valor; skill `pixzclaw-watch` monta o job
    (`job_type: agent`, `every_ms`, `allowed_tools: [invoice_status,
    cron_remove]`, `delivery` telegram). Silêncio em PENDING;
    UNDERPAID avisa e encerra (sessão cron é isolada, sem memória).
    101 testes host.

**✅ PR upstream ABERTO (2026-07-21): [#123](https://github.com/zeroclaw-labs/zeroclaw-plugins/pull/123)**
`feat(plugins): PixZClaw — dual-rail BRL PIX + Solana Pay USDC invoicing (T0/T1)`
branch `feat/pixzclaw-dual-rail-brl-usdc`, state OPEN, não-draft.
CI em `action_required` (gate de first-time contributor — atinge os 125 PRs
do bounty, não é defeito nosso). Nenhum maintainer revisou nenhum PR do
bounty ainda, o que ativa a cláusula do edital: fork limpo é aceito para
julgamento.

**Histórico — bloqueio que foi resolvido antes do PR:**
`tools/ci/validate_components.sh` do repo upstream copia **apenas**
`plugins/<nome>/` + `wit/v0` para um snapshot temporário e builda lá.
Nossa path dep `solana-wasm-core = { path = "../../crates/..." }`
**não existe** nesse snapshot → build falha no CI.
Fix obrigatório antes do PR: vendorizar o core em
`plugins/<nome>/vendor/solana-wasm-core/` nos 3 plugins e apontar a path
dep para lá (manter `crates/` como fonte única + script de sync).
Evidência de que é o padrão aceito: PR #116 (concorrente) usa
`plugins/<nome>/vendor/zeroclaw-solana-core/`.

**⚠️ Lição v0.3.1/0.3.2 (não regredir):** o host ZeroClaw redacta base58 de
alta entropia no chat SEMPRE (não é o plugin redact-text; instrução não
desliga). Política: linha `solana:` crua NUNCA aparece; trilho USDC é
QR-only (o QR codifica a URL completa e sobrevive). PIX copia-e-cola
sobrevive (não é base58).

### Em andamento / frágil ⚠️
1. **Config real no Pi** (`pix_key`, `merchant_solana`) — usuário em processo de configurar  
2. **Redact** no agent ainda pode mascarar `solana:` se não desligado  
3. **Onboard no chat** coleta dados mas **não grava** config sozinho — precisa `config set` no host  
4. Persona depende de skills/SOUL carregados + restart  
5. Bot Telegram “aba” separada (agent/canal dedicado) — **documentado, não implementado na config do user**  

### Não feito ❌
1. Vídeo demo ≤3 min Superteam — roteiro pronto em `pixzclaw-pi/VIDEO-SCRIPT.md` (11 planos, 2:56), falta gravar  
2. Submit Superteam Earn (deadline 2026-08-06 23:59 BRT, `agentAccess: HUMAN_ONLY`)  
4. On-ramp (MoonPay/Transak/exchange) — descartado como prioridade  
5. Botões inline Telegram nativos  
6. Conversão BRL→USDC real  
7. Publicar binário aarch64 ZeroClaw em Release pública estável (hoje via Actions artifact)  

---

## 5. Comandos de referência

### Build plugins (Windows)
```powershell
$env:Path = "$HOME\.cargo\bin;" + $env:Path
cd E:\zeroclawbount\zeroclaw-plugins\plugins\brl-usdc-invoice
cargo test
cargo build --target wasm32-wasip2 --release
# idem invoice-status, pixzclaw-brief
```

### Pi — atualizar plugins v0.3.0
```bash
cd ~
wget https://github.com/capitv/pixzclaw-pi/releases/download/v0.3.0-plugins/pixzclaw-plugins-v0.3.0.tar.gz
tar -xzf pixzclaw-plugins-v0.3.0.tar.gz
zeroclaw plugin install ~/plugins/brl-usdc-invoice
zeroclaw plugin install ~/plugins/invoice-status
zeroclaw plugin install ~/plugins/pixzclaw-brief
cp -r ~/skills/* ~/.zeroclaw/workspace/skills/
zeroclaw service restart
```

### Pi — config (valores reais do dono)
```bash
zeroclaw config set plugins.entries.brl-usdc-invoice.config.pix_key "..."
zeroclaw config set plugins.entries.brl-usdc-invoice.config.pix_name "..."
zeroclaw config set plugins.entries.brl-usdc-invoice.config.pix_city "..."
zeroclaw config set plugins.entries.brl-usdc-invoice.config.merchant_solana "PUBKEY_PUBLICA"
zeroclaw config set plugins.entries.brl-usdc-invoice.config.max_amount_brl "1000"
zeroclaw config set plugins.entries.brl-usdc-invoice.config.brl_per_usdc "5.5"
zeroclaw config set plugins.entries.brl-usdc-invoice.config.recipient_locked "true"

zeroclaw config set plugins.entries.invoice-status.config.merchant_solana "PUBKEY_PUBLICA"
zeroclaw config set plugins.entries.invoice-status.config.rpc_url "https://api.mainnet-beta.solana.com"

zeroclaw config set plugins.entries.pixzclaw-brief.config.merchant_solana "PUBKEY_PUBLICA"
zeroclaw config set plugins.entries.pixzclaw-brief.config.rpc_url "https://api.mainnet-beta.solana.com"

zeroclaw service restart
```

**Nunca** private key em config.

### Telegram — testes
```text
configurar pixzclaw
Cobra R$ 10 invoice demo-1. Use brl_usdc_invoice. Sem redact.
Me mostra o caixa. Use pixzclaw_brief.
demo-1 pagou? Use invoice_status.
```

---

## 6. Problemas conhecidos e fixes

| Sintoma | Causa | Fix |
|---|---|---|
| `unrecognized subcommand plugin` | Binário sem `plugins-wasm` | Usar binário aarch64 de Actions |
| `cargo` SIGSEGV no Pi | RAM / undervoltage | Não compilar no Pi 3 |
| `pix_key is required` | Config não setada | `config set` + restart |
| `solana:[REDACTED_…]` | redact-text | Desligar redact; skills v0.3 proíbem |
| PIX feio / inventado | LLM não usou tool | Forçar “use brl_usdc_invoice”; atualizar v0.3.0 |
| Onboard não grava sozinho | Design | config set no Pi após chat |
| Sessão mistura com outro uso | Um bot/agent só | Bot+agent `pixzclaw` separado |

---

## 7. Decisões de produto (não reabrir sem motivo)

1. T0/T1 only — sem T2 no bounty  
2. Dual-rail ≠ on-ramp  
3. Depth: 3 tools + core, não 10 plugins rasos  
4. Marca **PixZClaw**, emoji **🦞**  
5. QR via URL pública (qrserver), não botões Telegram  
6. Config no host jail, não `.env` do workspace  

---

## 8. Próximos passos recomendados (prioridade)

1. **User Pi:** install v0.3.0 + config set real + restart + teste cobrança sem redact  
2. **Desligar redact** no agent de cobrança  
3. **Soul/system prompt** colar `skills/SOUL.md`  
4. (Opcional) Agent/canal Telegram dedicado PixZClaw  
5. **PR** para `zeroclaw-labs/zeroclaw-plugins` (3 plugins + core ou path dep documentado)  
6. **Vídeo** ≤3 min (DEMO_SCRIPT.md)  
7. **Submit** Superteam Earn  
8. Roadmap only: on-ramp Transak/MoonPay; botões se host suportar  

---

## 9. Contas / identidade git usadas

- GitHub user que publicou: **capitv** (`gh` autenticado no PC Windows do dev)  
- Repo: `capitv/pixzclaw-pi`  
- Commits locais no repo com `user.name=capitv`  

---

## 10. Prompt para o próximo agente

Copie o bloco abaixo integralmente para o próximo agente.

```text
# Prompt — continuar PixZClaw / ZeroClaw Superteam bounty

Você está assumindo um projeto em andamento. Leia PRIMEIRO o arquivo:

  E:\zeroclawbount\HANDOFF.md

(ou no GitHub: https://github.com/capitv/pixzclaw-pi e o HANDOFF se copiado para o repo)

## Contexto em 30 segundos
- Bounty Superteam: plugins Solana para ZeroClaw (wasm32-wasip2, tool-plugin WIT).
- Produto: **PixZClaw** — 3 plugins: brl_usdc_invoice (T1), invoice_status (T0), pixzclaw_brief (T0 dashboard).
- NÃO é on-ramp; dual-rail PIX BRL + Solana Pay USDC; sem private keys.
- Código: E:\zeroclawbount\zeroclaw-plugins e E:\zeroclawbount\pixzclaw-pi
- Release plugins: https://github.com/capitv/pixzclaw-pi/releases/tag/v0.3.0-plugins
- User roda ZeroClaw num Raspberry Pi 3 (arcanjo); NÃO compilar ZeroClaw no Pi.
- Plugins JÁ instalados no Pi; UX de fatura foi feia por redact quebrando solana: — v0.3.0 adiciona QR links e proíbe redact.

## Links obrigatórios
- Bounty: https://superteam.fun/earn/listing/zeroclaw/
- ZeroClaw docs plugins: https://docs.zeroclawlabs.ai/master/en/plugins/writing-a-tool-plugin.html
- zeroclaw-plugins: https://github.com/zeroclaw-labs/zeroclaw-plugins
- redact-text ref: https://github.com/zeroclaw-labs/zeroclaw-plugins/tree/main/plugins/redact-text
- Solana Pay: https://docs.solanapay.com/spec
- Nosso repo: https://github.com/capitv/pixzclaw-pi
- Skills/SOUL: https://github.com/capitv/pixzclaw-pi/tree/main/skills

## Estado atual
FEITO: core, 3 plugins, wasm, releases, Pi com `zeroclaw plugin`, skills onboarding/daily/soul, fatura v0.3 com QR.
PENDENTE: config real estável no Pi, desligar redact no agent, persona 🦞 carregada, possível bot/agent Telegram isolado, gravar vídeo demo, submit Superteam.

## Regras
- Manter pure core / thin shim; crate-type cdylib+rlib; cargo test sem rede; wasm32-wasip2.
- permissions só as usadas (config_read, http_client).
- merchant_solana = PUBKEY, nunca private key.
- Não reintroduzir T2, trading bot, ou god-tool.
- Responder/configurar em PT-BR com o user quando for o caso.

## Sua missão agora (escolha conforme o user pedir)
1. Ajudar o user a validar v0.3.0 no Pi + Telegram (config, restart, teste sem REDACTED).
2. Melhorar UX/skills/SOUL se ainda robótico.
3. Isolar agent/canal Telegram “PixZClaw”.
4. Abrir PR em zeroclaw-labs/zeroclaw-plugins.
5. Roteiro/gravação demo + checklist Superteam.
6. Só se pedido: on-ramp de terceiros (não prioritário).

Antes de codar em massa, leia HANDOFF.md seções 3–6 e o código do plugin relevante. Não reconstrua do zero o que já está em release.
```

---

*Fim do handoff. Atualize a seção 4 quando o status mudar.*
