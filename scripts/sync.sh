#!/bin/sh
# Refreshes data/mightty.json from the app: every setting and action, with
# defaults and help, as `mightty config --export` prints them; and
# data/releases.json from the releases on GitHub, for the download page. Also
# redraws the wizard from the app icon for the favicon and the pages.
# usage: scripts/sync.sh [path to the mightty repository, default ../mightty]
set -eu
app=${1:-../mightty}
(cd "$app" && cargo build -q --release -p mightty)
"$app/target/release/mightty" config --export > data/mightty.json
echo "data/mightty.json: $(wc -c < data/mightty.json) bytes"

# The releases, for the download page: each version with its date, the size
# of its downloads and the checksum of its disk image (needs `gh`).
python3 -I - > data/releases.json <<'PY'
import datetime, json, subprocess, urllib.request
repo = "mightty-org/mightty"
releases = json.loads(subprocess.run(
    ["gh", "api", f"repos/{repo}/releases?per_page=100"], check=True, capture_output=True).stdout)
out = []
for r in releases:
    if r["draft"] or r["prerelease"]:
        continue
    sizes = {a["name"]: a["size"] for a in r["assets"]}
    if "Mightty.dmg" not in sizes:
        continue
    sums = urllib.request.urlopen(
        f"https://github.com/{repo}/releases/download/{r['tag_name']}/SHA256SUMS").read().decode()
    sha = next(line.split()[0] for line in sums.splitlines() if line.endswith("  Mightty.dmg"))
    # The day it was published where this is run, as the changelog dates it.
    day = datetime.datetime.fromisoformat(r["published_at"].replace("Z", "+00:00")).astimezone().date()
    out.append({"version": r["tag_name"].lstrip("v"), "date": day.isoformat(),
                "dmg": sizes["Mightty.dmg"], "zip": sizes.get("Mightty.zip", 0), "sha256": sha})
print(json.dumps(out, indent=2))
PY
echo "data/releases.json: $(grep -c '"version"' data/releases.json) releases"
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
