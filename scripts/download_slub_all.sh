#!/usr/bin/env bash
# v0.9.5 N16 — SLUB Dresden full-codex download
# Downloads pages 1..74 from the SLUB IIIF endpoint, skipping pages already on disk.
# Each page: ~5 MB JPEG, 3874×7649 RGB at 300 DPI. License: PDM 1.0.

set -u
DEST="/c/Users/hackf/Agents/imports/slub_dresden"
mkdir -p "$DEST"

BASE='https://digital.slub-dresden.de/data/kitodo/codedrm_280742827/codedrm_280742827_tif/jpegs'
UA='dccms-research/0.9.5 (Skyelabz210)'

success=0
skip=0
fail=0
for n in $(seq 1 74); do
    nn=$(printf '%08d' "$n")
    out="$DEST/page_${nn}.jpg"
    if [ -f "$out" ] && [ "$(stat -c %s "$out")" -gt 100000 ]; then
        skip=$((skip+1))
        continue
    fi
    url="$BASE/${nn}.tif.original.jpg"
    code=$(curl -s -o "$out" -w "%{http_code}" -A "$UA" --max-time 90 "$url")
    size=$(stat -c %s "$out" 2>/dev/null || echo 0)
    if [ "$code" = "200" ] && [ "$size" -gt 100000 ]; then
        success=$((success+1))
        printf "OK  page %02d: %d bytes\n" "$n" "$size"
    else
        fail=$((fail+1))
        rm -f "$out"
        printf "FAIL page %02d: status=%s size=%s\n" "$n" "$code" "$size"
    fi
done

echo "----"
echo "summary: success=$success skip=$skip fail=$fail total=$((success+skip+fail))"
