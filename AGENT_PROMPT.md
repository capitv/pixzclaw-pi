# Prompt pronto — cole no próximo agente

```
# Continuar PixZClaw / ZeroClaw Superteam Solana bounty

## Leia primeiro
Arquivo de handoff completo (contexto, links, etapa, problemas, comandos):

  E:\zeroclawbount\HANDOFF.md

Se não tiver o workspace, use:
  https://github.com/capitv/pixzclaw-pi
  e procure HANDOFF.md se estiver no repo; senão use o HANDOFF do workspace do user.

## Resumo
Produto **PixZClaw**: plugins ZeroClaw (wasm32-wasip2) para cobrança dual PIX (BRL) + Solana Pay (USDC), status USDC, e dashboard de caixa no Telegram.
- brl_usdc_invoice (T1)
- invoice_status (T0)
- pixzclaw_brief (T0)
Core: solana-wasm-core (sem solana-sdk).
Release atual plugins: https://github.com/capitv/pixzclaw-pi/releases/tag/v0.3.0-plugins
Skills/persona 🦞: https://github.com/capitv/pixzclaw-pi/tree/main/skills
Bounty: https://superteam.fun/earn/listing/zeroclaw/
Docs plugin: https://docs.zeroclawlabs.ai/master/en/plugins/writing-a-tool-plugin.html
Ref layout: https://github.com/zeroclaw-labs/zeroclaw-plugins/tree/main/plugins/redact-text
Solana Pay: https://docs.solanapay.com/spec

User: Raspberry Pi 3 (arcanjo), ZeroClaw COM subcomando plugin já instalado; 3 plugins listados. Não compilar ZeroClaw no Pi. Config via `zeroclaw config set plugins.entries.<plugin>.config.<key>`. merchant_solana = pubkey pública. Problema UX: redact quebrava solana:; v0.3.0 adiciona QR via api.qrserver.com e proíbe redact.

## Feito vs pendente
Feito: código, wasm, releases, Pi plugin host, skills onboard/daily/soul, QR na fatura.
Pendente: config estável, desligar redact, validar cobrança no Telegram, PR upstream, vídeo ≤3min, submit Superteam; opcional bot/agent Telegram isolado.

## Regras
Pure core + thin wasm shim; host tests; T0/T1 only; sem private keys; sem trading bot; dual-rail ≠ on-ramp automático.
PT-BR com o user.

## Missão
Siga o que o user pedir agora, com base no HANDOFF. Não reinvente o monorepo. Prefira iterar releases em capitv/pixzclaw-pi e ajudar o deploy no Pi.
```
