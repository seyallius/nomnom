#!/usr/bin/env bash
# gen_icons.sh — Generate all required icon sizes from assets/icons/icon.svg
#
# Requirements:
#   - ImageMagick (magick / convert)
#   - Inkscape (for high-quality SVG → PNG rasterisation)
#     Install: sudo apt install inkscape  /  brew install inkscape
#
# Usage:
#   chmod +x gen_icons.sh
#   ./gen_icons.sh

set -euo pipefail

SRC="assets/icons/icon.svg"
OUT="assets/icons"

if [ ! -f "$SRC" ]; then
  echo "ERROR: Source icon not found at $SRC"
  echo "Place your SVG logo at $SRC and re-run."
  exit 1
fi

mkdir -p "$OUT"

echo "→ Rasterising PNG sizes from $SRC…"

for SIZE in 16 32 64 128 256 512; do
  inkscape --export-type=png \
           --export-width="$SIZE" \
           --export-height="$SIZE" \
           --export-filename="$OUT/${SIZE}x${SIZE}.png" \
           "$SRC"
  echo "  ✔ ${SIZE}x${SIZE}.png"
done

# @2x (Retina)
inkscape --export-type=png \
         --export-width=256 \
         --export-height=256 \
         --export-filename="$OUT/128x128@2x.png" \
         "$SRC"
echo "  ✔ 128x128@2x.png"

echo "→ Building .ico (Windows — multi-size)…"
magick convert \
  "$OUT/16x16.png" \
  "$OUT/32x32.png" \
  "$OUT/64x64.png" \
  "$OUT/128x128.png" \
  "$OUT/256x256.png" \
  "$OUT/icon.ico"
echo "  ✔ icon.ico"

echo "→ Building .icns (macOS)…"
ICONSET="$OUT/nomnom.iconset"
mkdir -p "$ICONSET"

cp "$OUT/16x16.png"    "$ICONSET/icon_16x16.png"
cp "$OUT/32x32.png"    "$ICONSET/icon_16x16@2x.png"
cp "$OUT/32x32.png"    "$ICONSET/icon_32x32.png"
cp "$OUT/64x64.png"    "$ICONSET/icon_32x32@2x.png"
cp "$OUT/128x128.png"  "$ICONSET/icon_128x128.png"
cp "$OUT/128x128@2x.png" "$ICONSET/icon_128x128@2x.png"
cp "$OUT/256x256.png"  "$ICONSET/icon_256x256.png"
cp "$OUT/512x512.png"  "$ICONSET/icon_256x256@2x.png"
cp "$OUT/512x512.png"  "$ICONSET/icon_512x512.png"
cp "$OUT/512x512.png"  "$ICONSET/icon_512x512@2x.png"

if command -v iconutil &>/dev/null; then
  iconutil -c icns "$ICONSET" -o "$OUT/icon.icns"
  echo "  ✔ icon.icns (via iconutil)"
else
  # Fallback for Linux: use png2icns or skip
  if command -v png2icns &>/dev/null; then
    png2icns "$OUT/icon.icns" \
      "$ICONSET/icon_16x16.png" \
      "$ICONSET/icon_32x32.png" \
      "$ICONSET/icon_128x128.png" \
      "$ICONSET/icon_256x256.png" \
      "$ICONSET/icon_512x512.png"
    echo "  ✔ icon.icns (via png2icns)"
  else
    echo "  ⚠ Skipping .icns — run on macOS or install png2icns"
  fi
fi

rm -rf "$ICONSET"

echo ""
echo "✔ All icons generated in $OUT/"
ls -lh "$OUT/"