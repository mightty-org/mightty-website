#!/bin/sh
# Refreshes data/mightty.json from the app: every setting and action, with
# defaults and help, as `mightty config --export` prints them. Also redraws
# the wizard from the app icon for the favicon and the pages.
# usage: scripts/sync.sh [path to the mightty repository, default ../mightty]
set -eu
app=${1:-../mightty}
(cd "$app" && cargo build -q --release -p mightty)
"$app/target/release/mightty" config --export > data/mightty.json
echo "data/mightty.json: $(wc -c < data/mightty.json) bytes"
# WebP at twice the size each is shown (needs ImageMagick and cwebp): half
# the bytes of the PNGs. The favicon stays PNG, for browsers' sake.
tmp=$(mktemp -d)
python3 -I "$app/scripts/make-icon.py" --wizard "$tmp/wizard.png"
for size in 256 64; do
  magick "$tmp/wizard.png" -filter Lanczos -resize "${size}x${size}" "$tmp/$size.png"
done
cwebp -quiet -q 90 -alpha_q 100 -m 6 "$tmp/256.png" -o static/wizard.webp
cwebp -quiet -q 90 -alpha_q 100 -m 6 "$tmp/64.png" -o static/wizard-64.webp
sips -z 32 32 "$tmp/wizard.png" --out static/wizard-32.png >/dev/null
rm -r "$tmp"
echo "static/wizard.webp, wizard-64.webp, wizard-32.png"
