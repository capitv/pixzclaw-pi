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

12. **Verificação endurecida** (2026-07-22, commit `7b69a73`, ainda sem
    release): três furos que faziam o veredito mentir, achados por
    auditoria adversarial Fable e fechados por time Opus.
    - **Spam mascarava pagamento:** `status_tool.rs` varria só as 5
      assinaturas bem-sucedidas mais recentes. Seis txs de poeira
      tocando a reference empurravam o pagamento real pra fora →
      fatura paga respondia `PENDING`. Custo do ataque: 6 taxas.
      Agora varre **todas** as assinaturas do lookback, com parada
      antecipada quando cobre, teto 64, e as que passam do teto são
      **contadas como não-varridas**.
      → `received_units` é um **limite inferior** explícito quando a
      varredura é parcial. Limite inferior confirma pagamento que já
      cobre, mas **nunca** afirma falta. Varredura incompleta que não
      cobriu degrada pra `SIG OK`, não pra `UNDERPAID`.
    - **Reuso de invoice_id → falso PAID:** `auto_invoice_id` era
      `sha256(amount|description|merchant)`, sem tempo. Duas cobranças
      de "R$ 10" em dias diferentes = mesmo id = mesma reference = o
      pagamento de ontem liquidava a fatura de hoje, com recibo.
      Reproduzível **sem atacante**. Agora salgado com o instante da
      emissão, fornecido pelo shim (core continua puro), e **falha
      fechado**: timestamp implausível recusa emitir id em vez de
      emitir um que colide. Id explícito continua respeitado; READMEs
      dizem que precisa ser único por venda. Derivação da reference
      **não mudou** — o vídeo depende dela.
    - **f64 com tolerância de 0,5%:** pagador podia mandar 0,5% a menos
      e receber recibo de quitação (R$ 5 numa fatura de R$ 1.000), e a
      soma era ponto flutuante justo na parte anunciada como
      "verificação real" — enquanto a emissão já era `u128` exato.
      Agora os dois lados são inteiros (`uiTokenAmount.amount` +
      `decimals`), decimals divergentes entre txs **recusam** produzir
      veredito, tolerância eliminada.
    - Comentário, README e descrição da tool que afirmavam que a
      varredura de 5 já protegia contra mascaramento: corrigidos.
      Protegia contra 1 spam, não contra 6.
    - **135 testes host** (83 core + 20 + 25 + 7), clippy limpo,
      vendor sem drift, rustfmt limpo.

13. **CI upstream rodada na mão + verificador sem Pi** (2026-07-25,
    commits `42d8430` no repo do PR e `1f8d7bf` aqui):
    - Rodei os jobs do `validate.yml` upstream localmente e **achei uma
      falha real**: `git diff --check` contra a merge-base estava
      **falhando**. Duas linhas do README do core vendorado tinham
      espaço à direita (quebra dura markdown) e o `Cargo.toml` vendorado
      terminava em linha em branco. A linha em branco **não era
      escrita à mão**: `strip_workspace()` corta a seção `[workspace]`
      e deixava para trás a linha em branco que a separava — todo
      re-vendor reproduzia. Corrigido na **causa** (os dois
      `vendor-core.sh` agora colapsam newlines finais para exatamente
      uma), não no arquivo gerado.
    - Plugins subiram para **0.3.1** no repo do PR, para o PR nomear os
      bytes que estão de fato instalados e demonstrados, em vez de um
      0.3.0 que não corresponde a artefato nenhum.
    - **`examples/verify-live/`**: qualquer pessoa roda a verificação T0
      contra a chain real, sem ZeroClaw, sem wasmtime, sem Pi. Chama
      `invoice_status::status_tool::fetch_and_status` — a **mesma**
      função que o componente wasm chama — trocando só o `HttpTransport`
      injetado (`waki` sobre `wasi:http` no plugin, `curl` aqui). Sem
      crate de HTTP/TLS de propósito: duas requisições não justificam
      200 dependências, e os bytes ficam inspecionáveis. Imprime o
      endereço que vai ler e o link do Solscan **antes** do veredito.
      `--reference` aceita qualquer endereço que recebeu o mint, então
      dá para exercitar a verificação de valor sem fatura PixZClaw.
      Testado ao vivo contra a mainnet: RPC → assinaturas →
      `getTransaction` → delta de saldo → veredito, tudo real.
    - Comentário de evidência postado no PR #123
      ([#issuecomment-5080143760](https://github.com/zeroclaw-labs/zeroclaw-plugins/pull/123#issuecomment-5080143760)),
      com o que passou, o que não deu para rodar aqui (link wasm
      bloqueado pelo Smart App Control) e por quê — dito explicitamente
      em vez de omitido.
    - Landing e rodapé diziam "101 testes host". O número real, contado,
      é **135**. Corrigido nos dois idiomas.

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

### Resolvido em 2026-07-28 (v0.5.6-plugins / brl-usdc-invoice 0.3.6) ✅

**Card literal no Telegram — confirmado no Pi.** Até 0.3.5 o agente parafraseava
o card e comia as três linhas que sustentam a narrativa do vídeo
(`🧾 … cotação R$/USDC usada`, `🔒 teto R$ … · destino travado=`, `🔔 (só pra você)`).
Duas travas juntas resolveram; nenhuma sozinha foi testada como suficiente:
1. `skills/SOUL.md` → seção “O card da fatura é literal — regra dura”, proibições concretas;
2. linha `[sistema]` no fim da saída da tool: *“Reenvie TUDO acima desta linha na
   íntegra, caractere por caractere…”*. O agente corretamente **não** reenvia essa
   linha (ela diz “acima desta linha”) — a ausência dela na mensagem do Telegram é
   o comportamento certo, não uma falha.

Saída verificada em produção com `Cobra R$ 55 do cliente, pedido INV-DEMO-A`:
merchant `GX11xnCq…`, `amount=10`, `spl-token=EPjFWdd5…`, reference
`A6vpxfrrsje…`, PIX de 157 chars com CRC único `630497E1`.

**Se regredir:** o próximo degrau é mover `🧾`/`🔒` para dentro do bloco de código
cercado do PIX — território onde o agente comprovadamente não mexe. Não foi
preciso; não fazer preventivamente (polui o copia-e-cola do PIX).

**Release 500 no CDN:** o asset de v0.5.6 registrou `uploaded` com 396514 bytes mas
o CDN servia 160949 bytes e depois HTTP 500. `delete-asset` + reupload **não**
resolve. O que resolve: deletar e recriar a release inteira (tag preservada, notas
re-supridas de arquivo), depois esperar propagação em loop `until curl -sfI … `.

### Em andamento / frágil ⚠️
1. **Onboard no chat** coleta dados mas **não grava** config sozinho — precisa `config set` no host  
2. `zeroclaw config set <chave> <valor>` **ignora o argumento** e pergunta no TTY para segredos criptografados → um comando por vez, sem pipe  
3. Persona depende de skills/SOUL carregados + restart  
4. Bot Telegram “aba” separada (agent/canal dedicado) — **documentado, não implementado na config do user**  

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
Cobra R$ 55 do cliente, pedido INV-DEMO-A
Me mostra o caixa
A INV-DEMO-A pagou?
```

**Sem muletas, de propósito.** Versões antigas destes testes mandavam
`Use brl_usdc_invoice. Sem redact.` — as duas partes eram lixo:

- `Use brl_usdc_invoice` — medido: o agente escolhe a tool sozinho a partir da
  frase natural. Dizer o nome da tool no vídeo entrega uma história pior do que
  a verdade, e foi o próprio dono do projeto que apontou isso.
- `Sem redact` — nunca fez nada. O redact do host não é o plugin `redact-text` e
  não obedece instrução no prompt (ver seção de problemas conhecidos).

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
| `cargo build` falha `os error 4551` | Smart App Control (Windows) bloqueia proc-macro DLL | **Só afeta o alvo `wasm32-wasip2`.** `cargo test` e `cargo clippy` no host rodam normal — sempre teste antes de assumir bloqueio (custou 1h em 22/07) |
| `git push` 403 em `origin` | `origin` = repo upstream deles | Push vai pro remote `fork` (`capitv/zeroclaw-plugins`); o PR sai do fork |
| `cargo fmt --check` falha em plugin não tocado | CI só falha por fmt em plugin com `.rs` alterado; ressincronizar vendor arrasta o plugin pro portão | Rodar `cargo fmt` nos 3 workspaces. Reverter vendor pra evitar isso = publicar plugin com core bugado |
| **GitHub Actions não roda nada** (`The job was not started because your account is locked due to a billing issue`) | Conta `capitv` travada por cobrança — apareceu entre 22/07 e 25/07 | **Só o dono resolve**, em github.com/settings/billing. Trava tudo: build-pi, CI no fork, artefatos reproduzíveis. GitHub **Pages continua deployando** (infra separada), então a landing sobe normal |
| `git diff --check` falha no CI upstream | `strip_workspace()` cortava `[workspace]` e deixava a linha em branco que a separava → linha em branco no EOF do `Cargo.toml` vendorado | Corrigido nos dois `vendor-core.sh` (colapsa newlines finais para uma). Rodar `git diff --check $(git merge-base origin/main HEAD) -- plugins/` **antes** de qualquer push pro PR |
| `cargo test` de um plugin quebra com "trait não implementado" apontando pro **outro repo** | Reusar `CARGO_TARGET_DIR` de um plugin para compilar outra crate que depende de um `invoice-status` homônimo: dois pacotes de mesmo nome/versão colidem no fingerprint | `rm -rf target/debug/.fingerprint/<pkg>-* target/debug/deps/*<pkg>*`. Melhor: não compartilhar target dir entre repos |

---

## 7. Decisões de produto (não reabrir sem motivo)

1. T0/T1 only — sem T2 no bounty  
2. Dual-rail ≠ on-ramp  
3. Depth: 3 tools + core, não 10 plugins rasos  
4. Marca **PixZClaw**, emoji **🦞**  
5. QR via URL pública (qrserver), não botões Telegram  
6. Config no host jail, não `.env` do workspace  
7. `crates/` e `tools/vendor-core.sh` **não vão no PR** — o upstream recebe só `plugins/`, autossuficiente pelo vendor. Impor um diretório na raiz da árvore do mantenedor é atrito desnecessário  
8. Verificação **falha fechado**: em qualquer dúvida (RPC incompleto, decimals divergentes, relógio implausível) o sistema degrada e diz que não conferiu — nunca afirma PAID nem afirma falta  
9. **Toda afirmação forte da doc tem que ser executável.** O README afirmava três propriedades da verificação e não dava ao leitor jeito nenhum de conferir sem comprar um Raspberry Pi. `examples/verify-live` existe por isso. Se uma afirmação nova entrar na landing ou no README sem um comando que a exercite, ela é marketing, não prova  
10. `examples/verify-live` **não vai no PR upstream** — depende de `plugins/invoice-status` por path e o snapshot da CI upstream copia um plugin por vez. Fica só em `pixzclaw-pi`, linkado do comentário do PR  
---

## 8. Próximos passos recomendados (prioridade)

1. **User Pi:** install v0.3.0 + config set real + restart + teste cobrança sem redact  
2. **Desligar redact** no agent de cobrança  
3. **Soul/system prompt** colar `skills/SOUL.md`  
4. (Opcional) Agent/canal Telegram dedicado PixZClaw  
5. ~~**PR** para `zeroclaw-labs/zeroclaw-plugins`~~ — **feito**, PR #123 aberto  
6. **Vídeo** ≤3 min — roteiro pronto em `pixzclaw-pi/VIDEO-SCRIPT.md` (11 planos, 2:56); falta **gravar**. Único bloqueio real da submissão  
7. **Submit** Superteam Earn — deadline **06/08/2026 23:59 BRT**, `agentAccess: HUMAN_ONLY` (só o humano submete)  
8. **Destravar a cobrança do GitHub** (github.com/settings/billing). Enquanto travado, nenhum job de Actions roda em nenhum repo. Sem isso não dá pra: sincronizar o workflow defasado, rodar a CI dos 3 plugins, nem republicar os assets da release como artefatos reproduzíveis de CI. Não bloqueia a gravação nem a submissão — os artefatos publicados foram verificados byte-a-byte contra as fontes do PR  
9. Depois de destravar: `gh auth refresh -s workflow,repo`, `cp ci/build-pi.yml .github/workflows/build-pi.yml`, commit. O workflow no GitHub ainda só conhece **2** plugins — `pixzclaw-brief` nunca passou por CI  
10. Habilitar Actions no fork `capitv/zeroclaw-plugins` (um clique em Actions → "I understand my workflows") para rodar o `validate.yml` upstream de verdade e ter check verde no PR  
11. Depois de gravar: congelar as txs reais do vídeo em `examples/verify-live` (`demo::MERCHANT` está vazio de propósito) + README + landing, para o juiz reproduzir o veredito exato com um comando  
12. Roadmap only: on-ramp Transak/MoonPay; botões se host suportar  

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
