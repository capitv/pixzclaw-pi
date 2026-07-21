#!/usr/bin/env bash
# No Pi: força atualizar plugins (remove + install). Resolve "already loaded".
set -euo pipefail
cd ~
VER="${1:-v0.3.1-plugins}"
TGZ="pixzclaw-plugins-${VER#v}.tar.gz"
# tag is v0.3.1-plugins, file pixzclaw-plugins-v0.3.1.tar.gz
if [[ "$VER" == v* ]]; then
  FILE="pixzclaw-plugins-${VER}.tar.gz"
  # v0.3.1-plugins -> need pixzclaw-plugins-v0.3.1.tar.gz style
  FILE="pixzclaw-plugins-${VER}.tar.gz"
fi
# Prefer explicit latest known
URL="https://github.com/capitv/pixzclaw-pi/releases/download/${VER}/pixzclaw-plugins-${VER}.tar.gz"
# normalize: tag v0.3.1-plugins asset pixzclaw-plugins-v0.3.1.tar.gz
if [[ "$VER" =~ ^v([0-9.]+)-plugins$ ]]; then
  URL="https://github.com/capitv/pixzclaw-pi/releases/download/${VER}/pixzclaw-plugins-v${BASH_REMATCH[1]}.tar.gz"
fi

echo "==> Download $URL"
wget -q -O /tmp/pixzclaw-plugins.tgz "$URL" || curl -fsSL "$URL" -o /tmp/pixzclaw-plugins.tgz
rm -rf /tmp/pixzclaw-extract
mkdir -p /tmp/pixzclaw-extract
tar -xzf /tmp/pixzclaw-plugins.tgz -C /tmp/pixzclaw-extract
# plugins may be at root or nested
if [[ -d /tmp/pixzclaw-extract/plugins ]]; then
  PDIR=/tmp/pixzclaw-extract/plugins
else
  PDIR=/tmp/pixzclaw-extract
fi
cp -a "$PDIR"/* ~/plugins/ 2>/dev/null || mkdir -p ~/plugins && cp -a "$PDIR"/* ~/plugins/

for name in brl-usdc-invoice invoice-status pixzclaw-brief; do
  if [[ -d "$HOME/plugins/$name" ]]; then
    echo "==> remove $name (ignore errors)"
    zeroclaw plugin remove "$name" 2>/dev/null || true
    echo "==> install $name"
    zeroclaw plugin install "$HOME/plugins/$name"
  fi
done

# soul
curl -fsSL https://raw.githubusercontent.com/capitv/pixzclaw-pi/main/scripts/install-soul-on-pi.sh | bash

echo "==> list"
zeroclaw plugin list
echo "OK. Teste no Telegram: Cobra R$ 10 pro Cig"
echo "A resposta DEVE ter links api.qrserver.com (QR). Se não tiver, o wasm antigo ainda está ativo."
