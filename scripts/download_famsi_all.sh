#!/usr/bin/env bash
# v0.9.5 N17 — Download all 6 FAMSI Förstemann-Schele PDFs covering pp01-74.
# Source: famsi.org public-domain by age (Förstemann 1880 + Schele color edition).

set -u
DEST="/c/Users/hackf/Agents/imports/famsi_dresden"
mkdir -p "$DEST"
UA='dccms-research/0.9.5 (Skyelabz210)'

declare -a PDFS=(
    "1_dresden_fors_schele_pp01-12.pdf"
    "2_dresden_fors_schele_pp13-24.pdf"
    "3_dresden_fors_schele_pp25-35.pdf"
    "4_dresden_fors_schele_pp36-45.pdf"
    "5_dresden_fors_schele_pp46-59.pdf"
    "6_dresden_fors_schele_pp60-74.pdf"
)

success=0; skip=0; fail=0
for pdf in "${PDFS[@]}"; do
    out="$DEST/$pdf"
    if [ -f "$out" ] && [ "$(stat -c %s "$out")" -gt 1000000 ]; then
        skip=$((skip+1))
        printf "SKIP %s (already %d bytes)\n" "$pdf" "$(stat -c %s "$out")"
        continue
    fi
    url="https://www.famsi.org/mayawriting/codices/pdf/$pdf"
    code=$(curl -s -o "$out" -w "%{http_code}" -A "$UA" --max-time 180 "$url")
    size=$(stat -c %s "$out" 2>/dev/null || echo 0)
    if [ "$code" = "200" ] && [ "$size" -gt 1000000 ]; then
        success=$((success+1))
        printf "OK   %s: %d bytes\n" "$pdf" "$size"
    else
        fail=$((fail+1))
        rm -f "$out"
        printf "FAIL %s: status=%s size=%s\n" "$pdf" "$code" "$size"
    fi
done

# Also rename existing famsi_pp13-24.pdf to canonical name if present
if [ -f "$DEST/famsi_pp13-24.pdf" ] && [ ! -f "$DEST/2_dresden_fors_schele_pp13-24.pdf" ]; then
    cp "$DEST/famsi_pp13-24.pdf" "$DEST/2_dresden_fors_schele_pp13-24.pdf"
    printf "renamed: famsi_pp13-24.pdf -> 2_dresden_fors_schele_pp13-24.pdf\n"
fi

echo "----"
echo "summary: success=$success skip=$skip fail=$fail total=$((success+skip+fail))"
