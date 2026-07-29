#!/usr/bin/env bash
# Rode NO Raspberry Pi. Baixa o SOUL + skills do GitHub e coloca no workspace do ZeroClaw.
# Não precisa colar manualmente no chat.

set -euo pipefail

REPO_RAW="https://raw.githubusercontent.com/capitv/pixzclaw-pi/main"
ZC_HOME="${ZEROCLAW_HOME:-$HOME/.zeroclaw}"
# layouts comuns de workspace ZeroClaw
CANDIDATES=(
  "$ZC_HOME/workspace"
  "$ZC_HOME/agents/pixzclaw/workspace"
  "$ZC_HOME/agents/default/workspace"
  "$HOME/.zeroclaw/workspace"
)

echo "==> Baixando SOUL e skills do GitHub..."
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

# Pega o repositório inteiro em vez de uma lista de arquivos escrita à mão.
#
# A lista à mão já falhou: nomeava pixzclaw-onboard e pixzclaw-daily, foi
# escrita antes de pixzclaw-watch existir, e nunca foi atualizada. A skill do
# vigia de pagamento ficou meses no repositório sem nunca chegar ao Pi, e nada
# no processo apontava isso — o script terminava com "OK".
#
# Com o tarball, uma skill nova passa a ser instalada por existir.
curl -fsSL "https://codeload.github.com/capitv/pixzclaw-pi/tar.gz/refs/heads/main" \
  -o "$TMP/repo.tgz"
tar -xzf "$TMP/repo.tgz" -C "$TMP"
SRC=$(find "$TMP" -maxdepth 1 -type d -name 'pixzclaw-pi-*' | head -1)
if [[ -z "$SRC" || ! -d "$SRC/skills" ]]; then
  echo "ERRO: não achei skills/ no tarball do repositório." >&2
  exit 1
fi

cp -f "$SRC/skills/SOUL.md" "$TMP/SOUL.md"
mkdir -p "$TMP/skills"
# Só diretórios de skill (cada um com seu SKILL.md); SOUL.md e README.md ficam
# de fora — não são skills e o host não deve carregá-los como tal.
for d in "$SRC"/skills/*/; do
  [[ -f "$d/SKILL.md" ]] && cp -r "$d" "$TMP/skills/"
done
echo "    skills encontradas: $(ls "$TMP/skills" | tr '\n' ' ')"

INSTALLED=0
for WS in "${CANDIDATES[@]}"; do
  if [[ -d "$(dirname "$WS")" ]] || [[ -d "$ZC_HOME" ]]; then
    mkdir -p "$WS/skills"
    cp -f "$TMP/SOUL.md" "$WS/SOUL.md"
    # alguns hosts leem IDENTITY / personality
    cp -f "$TMP/SOUL.md" "$WS/IDENTITY.md" 2>/dev/null || true
    cp -f "$TMP/SOUL.md" "$WS/personality.md" 2>/dev/null || true
    cp -r "$TMP/skills/"* "$WS/skills/"
    echo "    instalado em: $WS"
    INSTALLED=1
  fi
done

# sempre grava no path default também
mkdir -p "$ZC_HOME/workspace/skills"
cp -f "$TMP/SOUL.md" "$ZC_HOME/workspace/SOUL.md"
cp -f "$TMP/SOUL.md" "$ZC_HOME/workspace/IDENTITY.md"
cp -r "$TMP/skills/"* "$ZC_HOME/workspace/skills/"
echo "    instalado em: $ZC_HOME/workspace"
INSTALLED=1

# cópia legível na home
mkdir -p "$HOME/pixzclaw-soul"
cp -f "$TMP/SOUL.md" "$HOME/pixzclaw-soul/SOUL.md"
echo "    backup em: $HOME/pixzclaw-soul/SOUL.md"

if [[ "$INSTALLED" -eq 1 ]]; then
  echo "==> OK. Reiniciando serviço (se existir)..."
  if command -v zeroclaw >/dev/null 2>&1; then
    zeroclaw service restart 2>/dev/null || true
  fi
  echo ""
  echo "Pronto. No Telegram teste só:"
  echo "  Cobra R$ 10 pro Cig"
  echo ""
  echo "Se ainda não usar o SOUL: no dashboard/zerocode do ZeroClaw,"
  echo "cole o conteúdo de: $HOME/pixzclaw-soul/SOUL.md  no campo Soul/System."
  echo "Ou abra: $ZC_HOME/workspace/SOUL.md"
else
  echo "Não achei pasta .zeroclaw — SOUL ficou em $HOME/pixzclaw-soul/SOUL.md"
fi
