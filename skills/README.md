# PixZClaw skills

Markdown skills for ZeroClaw agents (Telegram UX).

| Folder | Trigger |
|---|---|
| [pixzclaw-onboard](./pixzclaw-onboard/SKILL.md) | `/configurar`, setup da loja (dono only) |
| [pixzclaw-daily](./pixzclaw-daily/SKILL.md) | cobrar, status, /caixa |
| [pixzclaw-watch](./pixzclaw-watch/SKILL.md) | “avisa quando a X pagar” — lembrete via cron, se auto-remove |

Copy into your agent workspace skills directory, or paste into the agent system instructions.

These are **not** WASM plugins — they teach the LLM when to call:

- `brl_usdc_invoice`
- `invoice_status`
- `pixzclaw_brief`
- `cron_add` / `cron_list` / `cron_remove` / `cron_update` (tools nativas do host ZeroClaw)
