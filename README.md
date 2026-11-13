# Mightty website

The site for [Mightty](../mightty): the home page, the docs and the changelog,
built by a small Rust program with no third-party crates into `dist/`, a
folder of static files to host anywhere.

```sh
scripts/sync.sh        # refresh data/mightty.json from the app (every setting and action)
cargo run              # build dist/ (reads the themes from ../mightty/assets/themes)
cargo test
python3 -m http.server -d dist 8000   # look at it on http://localhost:8000
```

- `templates/home.html`: the home page.
- `content/docs/*.md`: the docs, in the order their front matter gives. A line
  `{{settings}}` or `{{actions}}` is filled with the reference generated from
  `data/mightty.json`, so it always matches the app.
- `content/changelog.md`: the changelog, written by hand from the commits.
- `data/releases.json`: the releases on GitHub, with their dates, sizes and
  checksums, which the download page is built from. `scripts/sync.sh`
  refreshes it; run it after each release.
- `static/`: the stylesheet and the script that lets the page wear any of
  Mightty's themes, both put inline in every page (the stylesheet without
  the rules that page doesn't use), and the wizard drawn from the app icon,
  as WebP. The pages use the system's fonts, so there are none to download.

The repository beside this one must be the app's: the generator uses its JSON
crate and its themes.

## Publishing

`.github/workflows/pages.yml` builds the site on every push to `main` and
publishes it with GitHub Pages: in the repository's settings, set Pages'
source to GitHub Actions. It checks out `mightty-org/mightty` beside this
repository and builds with `SITE_BASE` set to the path Pages serves the site
under, which is nothing at a domain of its own. Run `scripts/sync.sh` and
commit `data/mightty.json` when the app's settings change.

The site is served at [mightty.org](https://mightty.org): `static/CNAME`
names the domain for Pages, and the generator writes each page's canonical
address, `sitemap.xml` and `robots.txt` for it. The domain's DNS has four
`A` records for the apex pointing at GitHub Pages (185.199.108.153 to
185.199.111.153) and a `CNAME` for `www` pointing at `mightty-org.github.io`.

