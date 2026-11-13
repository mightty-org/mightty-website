//! Builds the Mightty website into `dist/`: the home page, the docs (Markdown
//! in `content/docs`, the settings and actions reference filled in from
//! `data/mightty.json`, which `scripts/sync.sh` exports from the app), the
//! changelog, and every bundled theme for the page to wear.
//!
//! usage: cargo run [-- path to the mightty repository, default ../mightty]
//!
//! The site is published at mightty.org (`static/CNAME`). `SITE_BASE` is the
//! path it is served under when it isn't at a domain's root (`/mightty-website` on
//! GitHub Pages without a domain of its own); links are made under it.

mod markdown;

use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use mightty_json::Value;
use markdown::{escape, front_matter, inline, render};

/// A docs page: where it's served, its title, a line about it, and its HTML.
struct Doc {
    slug: String,
    title: String,
    summary: String,
    order: u32,
    html: String,
}

fn main() -> io::Result<()> {
    let app = PathBuf::from(std::env::args().nth(1).unwrap_or_else(|| "../mightty".into()));
    let out = Path::new("dist");
    if out.exists() {
        fs::remove_dir_all(out)?;
    }
    fs::create_dir_all(out)?;

    let data = fs::read_to_string("data/mightty.json")?;
    let data = mightty_json::parse(&data).map_err(|e| io::Error::other(format!("data/mightty.json: {e}")))?;
    let version = data.get("version").and_then(Value::as_str).unwrap_or("").to_owned();

    // The docs, in their order.
    let mut docs = Vec::new();
    for entry in fs::read_dir("content/docs")? {
        let path = entry?.path();
        if path.extension().is_none_or(|e| e != "md") {
            continue;
        }
        let text = fs::read_to_string(&path)?;
        let (fields, body) = front_matter(&text);
        let field = |k: &str| fields.iter().find(|f| f.0 == k).map(|f| f.1.clone()).unwrap_or_default();
        let mut html = render(body).0;
        html = html.replace("<!--settings-->", &settings_reference(&data));
        html = html.replace("<!--actions-->", &actions_reference(&data));
        if html.contains("<!--layers-->") {
            html = html.replace("<!--layers-->", &fs::read_to_string("templates/layers.html")?);
        }
        let slug = path.file_stem().unwrap().to_string_lossy().into_owned();
        docs.push(Doc { slug, title: field("title"), summary: field("summary"), order: field("order").parse().unwrap_or(99), html });
    }
    docs.sort_by_key(|d| d.order);

    // Home.
    let home = fs::read_to_string("templates/home.html")?.replace("{{version}}", &escape(&version));
    write(out, "", &layout("Mightty, a terminal emulator for macOS", "Mightty is the fast, lightweight terminal for macOS, written in Rust and drawn with Metal.", "home", &home))?;

    // Download: the latest version, then every earlier one.
    let releases = fs::read_to_string("data/releases.json")?;
    let releases = mightty_json::parse(&releases).map_err(|e| io::Error::other(format!("data/releases.json: {e}")))?;
    write(
        out,
        "download",
        &layout(
            "Download · Mightty",
            "Download Mightty for macOS: the latest version and every earlier one, with checksums.",
            "download",
            &download_page(&releases),
        ),
    )?;

    // Docs: an index of them, then each.
    let mut index = String::from("<div class=\"doc-index\">\n<h1>Documentation</h1>\n<p class=\"lede\">The settings and key binding reference is generated from Mightty's source, so it matches the current version.</p>\n<ul class=\"doc-list\">\n");
    for d in &docs {
        index += &format!("<li><a href=\"/docs/{}/\"><span class=\"doc-title\">{}</span><span class=\"doc-summary\">{}</span></a></li>\n", d.slug, escape(&d.title), inline(&d.summary));
    }
    index += "</ul>\n</div>\n";
    write(out, "docs", &layout("Documentation · Mightty", "Installing, setting up and using Mightty.", "docs", &docs_page(&docs, None, &index)))?;
    for (n, d) in docs.iter().enumerate() {
        let mut body = d.html.clone();
        // Where to go next, at the end of each page.
        let prev = n.checked_sub(1).and_then(|p| docs.get(p));
        let next = docs.get(n + 1);
        body += "<nav class=\"pager\">";
        if let Some(p) = prev {
            body += &format!("<a class=\"prev\" href=\"/docs/{}/\"><span>Previous</span>{}</a>", p.slug, escape(&p.title));
        }
        if let Some(p) = next {
            body += &format!("<a class=\"next\" href=\"/docs/{}/\"><span>Next</span>{}</a>", p.slug, escape(&p.title));
        }
        body += "</nav>";
        write(out, &format!("docs/{}", d.slug), &layout(&format!("{} · Mightty", d.title), &d.summary, "docs", &docs_page(&docs, Some(&d.slug), &body)))?;
    }

    // Changelog.
    let text = fs::read_to_string("content/changelog.md")?;
    let (_, body) = front_matter(&text);
    let changelog = format!("<article class=\"prose changelog\">\n{}</article>\n", render(body).0);
    write(out, "changelog", &layout("Changelog · Mightty", "What's new in Mightty.", "changelog", &changelog))?;

    // Static files, but for what every page holds already; and the themes
    // for the page to wear.
    for entry in fs::read_dir("static")? {
        let path = entry?.path();
        let name = path.file_name().unwrap();
        if name != "style.css" && name != "site.js" {
            fs::copy(&path, out.join(name))?;
        }
    }
    let themes = themes(&app.join("assets/themes"))?;
    fs::write(out.join("themes.json"), themes)?;

    // For addresses that lead nowhere, and for search engines.
    let lost = "<article class=\"prose\">\n<h1>Page not found</h1>\n<p>There is no page at this address. The <a href=\"/docs/\">documentation</a> lists every page, and the <a href=\"/\">overview</a> is the place to start.</p>\n</article>\n";
    // It has no address of its own: it's shown at whichever one was asked for.
    let lost: String = layout("Page not found · Mightty", "There is no page at this address.", "", lost)
        .lines()
        .filter(|line| !line.contains(URL_HERE))
        .map(|line| format!("{line}\n"))
        .collect();
    fs::write(out.join("404.html"), based(&lost))?;
    let mut pages = Vec::new();
    sitemap(out, "", &mut pages)?;
    fs::write(
        out.join("sitemap.xml"),
        format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n{}\n</urlset>\n", pages.join("\n")),
    )?;
    println!("built dist/ ({} docs)", docs.len());
    Ok(())
}

/// Writes `html` as `dir/index.html` under `out`, its site links under `SITE_BASE`.
fn write(out: &Path, dir: &str, html: &str) -> io::Result<()> {
    let html = html.replace(URL_HERE, &url_of(dir));
    let dir = out.join(dir);
    fs::create_dir_all(&dir)?;
    fs::write(dir.join("index.html"), based(&html))
}

/// Where the site is published.
const SITE_URL: &str = "https://mightty.org";

/// Where a page's own address goes in it, until it's written.
const URL_HERE: &str = "/*url*/";

/// The address of the page in `dir`.
fn url_of(dir: &str) -> String {
    if dir.is_empty() { format!("{SITE_URL}/") } else { format!("{SITE_URL}/{dir}/") }
}

/// `html` with its site links under `SITE_BASE`.
fn based(html: &str) -> String {
    let base = std::env::var("SITE_BASE").unwrap_or_default();
    let base = base.trim_end_matches('/');
    html.replace("href=\"/", &format!("href=\"{base}/")).replace("src=\"/", &format!("src=\"{base}/")).replace("<html lang=\"en\">", &format!("<html lang=\"en\" data-base=\"{base}\">"))
}

/// Every page under `dir` (itself at `path` in the site), as sitemap entries.
fn sitemap(dir: &Path, path: &str, out: &mut Vec<String>) -> io::Result<()> {
    if dir.join("index.html").exists() {
        out.push(format!("<url><loc>{}</loc></url>", url_of(path)));
    }
    let mut dirs: Vec<PathBuf> = fs::read_dir(dir)?.filter_map(|e| Some(e.ok()?.path())).filter(|p| p.is_dir()).collect();
    dirs.sort();
    for sub in dirs {
        let name = sub.file_name().unwrap().to_string_lossy().into_owned();
        let path = if path.is_empty() { name } else { format!("{path}/{name}") };
        sitemap(&sub, &path, out)?;
    }
    Ok(())
}

/// The stylesheet and script, put in every page rather than linked: one
/// request a page, and nothing to wait for before it's drawn. Together
/// they're smaller than the requests' own overhead would be.
fn assets() -> &'static (String, String) {
    static ASSETS: OnceLock<(String, String)> = OnceLock::new();
    ASSETS.get_or_init(|| {
        let read = |path| fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
        (minify_css(&read("static/style.css")), minify_js(&read("static/site.js")))
    })
}

/// `css` without comments and the whitespace it doesn't need. Spaces
/// before `:` stay, as in a selector they mean a descendant.
fn minify_css(css: &str) -> String {
    let mut text = String::new();
    let mut rest = css;
    while let Some(start) = rest.find("/*") {
        text.push_str(&rest[..start]);
        rest = rest[start..].find("*/").map_or("", |end| &rest[start + end + 2..]);
    }
    text.push_str(rest);
    let mut out = String::with_capacity(text.len());
    for word in text.split_whitespace() {
        let joins = out.ends_with(['{', '}', ';', ',', ':', '>'])
            || word.starts_with(['{', '}', ';', ',', '>']);
        if !out.is_empty() && !joins {
            out.push(' ');
        }
        out.push_str(word);
    }
    out.replace(";}", "}")
}

/// Where a page's stylesheet goes, until the page says which rules it uses.
const CSS_HERE: &str = "/*css*/";

/// Minified `css` without the rules that can't apply to `html`: a selector
/// naming a class the page doesn't have is dropped, and a rule with it when
/// none of its selectors are left. Classes the script adds count as there.
fn used_css(css: &str, html: &str) -> String {
    let mut classes: HashSet<&str> = html
        .split("class=\"")
        .skip(1)
        .flat_map(|rest| rest.split('"').next().unwrap_or_default().split_whitespace())
        .collect();
    classes.insert("switching");
    let mut out = String::with_capacity(css.len());
    let mut rest = css;
    while let Some(open) = rest.find('{') {
        // The block's end: the brace that closes this one.
        let mut depth = 0;
        let close = rest[open..]
            .char_indices()
            .find(|&(_, c)| {
                depth += match c {
                    '{' => 1,
                    '}' => -1,
                    _ => 0,
                };
                depth == 0
            })
            .map_or(rest.len(), |(i, _)| open + i);
        let (prelude, body) = (&rest[..open], &rest[open + 1..close]);
        rest = rest.get(close + 1..).unwrap_or_default();
        if prelude.starts_with("@media") || prelude.starts_with("@supports") {
            let inner = used_css(body, html);
            if !inner.is_empty() {
                out.push_str(&format!("{prelude}{{{inner}}}"));
            }
        } else if prelude.starts_with('@') {
            out.push_str(&format!("{prelude}{{{body}}}"));
        } else {
            let kept: Vec<&str> = selectors(prelude).into_iter().filter(|s| applies(s, &classes)).collect();
            if !kept.is_empty() {
                out.push_str(&format!("{}{{{body}}}", kept.join(",")));
            }
        }
    }
    out
}

/// A selector list split at its commas, but for those inside brackets.
fn selectors(list: &str) -> Vec<&str> {
    let (mut parts, mut depth, mut start) = (Vec::new(), 0, 0);
    for (i, c) in list.char_indices() {
        match c {
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(&list[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(&list[start..]);
    parts
}

/// Whether every class `selector` asks for is among `classes`. What's
/// inside `:not()` is asked not to be there, so it doesn't count.
fn applies(selector: &str, classes: &HashSet<&str>) -> bool {
    let mut text = selector;
    let mut wanted = String::new();
    while let Some(at) = text.find(":not(") {
        wanted.push_str(&text[..at]);
        let mut depth = 0;
        let end = text[at..]
            .char_indices()
            .find(|&(_, c)| {
                depth += match c {
                    '(' => 1,
                    ')' => -1,
                    _ => 0,
                };
                depth == 0
            })
            .map_or(text.len(), |(i, _)| at + i + 1);
        text = &text[end..];
    }
    wanted.push_str(text);
    wanted.split('.').skip(1).all(|after| {
        let name = after
            .split(|c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '_'))
            .next()
            .unwrap_or_default();
        classes.contains(name)
    })
}

/// `js` without its comment lines and indentation.
fn minify_js(js: &str) -> String {
    js.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The page around `body`: header, navigation, footer.
fn layout(title: &str, description: &str, current: &str, body: &str) -> String {
    let link = |href: &str, name: &str, key: &str| {
        let here = if key == current { " aria-current=\"page\"" } else { "" };
        format!("<a href=\"{href}\"{here}>{name}</a>")
    };
    let page = format!(
        r##"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'unsafe-inline'; style-src 'unsafe-inline'; img-src 'self'; connect-src 'self'; base-uri 'none'; form-action 'none'">
<title>{title}</title>
<meta name="description" content="{description}">
<link rel="canonical" href="/*url*/">
<meta property="og:type" content="website">
<meta property="og:site_name" content="Mightty">
<meta property="og:title" content="{title}">
<meta property="og:description" content="{description}">
<meta property="og:url" content="/*url*/">
<meta property="og:image" content="https://mightty.org/apple-touch-icon.png">
<link rel="icon" type="image/png" sizes="32x32" href="/wizard-32.png">
<link rel="apple-touch-icon" href="/apple-touch-icon.png">
<style>{css}</style>
<script>
// The theme picked on an earlier visit, before the first paint; site.js does the rest.
try {{
  const t = JSON.parse(localStorage.getItem("mightty-theme"));
  if (t && Array.isArray(t.c) && t.c.length === 20 && t.c.every((c) => /^#[0-9a-f]{{6}}$/i.test(c))) {{
    const s = document.documentElement.style;
    ["--bg", "--fg", "--cursor", "--sel"].forEach((n, i) => s.setProperty(n, t.c[i]));
    for (let i = 0; i < 16; i++) s.setProperty(`--c${{i}}`, t.c[i + 4]);
  }}
}} catch {{}}
</script>
<script type="speculationrules">
{{"prerender": [{{"where": {{"href_matches": "/*"}}, "eagerness": "moderate"}}]}}
</script>
</head>
<body class="page-{current}">
<a class="skip" href="#main">Skip to content</a>
<header class="site">
<a class="brand" href="/"><img src="/wizard-64.webp" alt="" width="32" height="32">Mightty</a>
<nav aria-label="Main">{home}{download}{docs}{changelog}<a href="https://github.com/mightty-org/mightty">Source</a></nav>
</header>
<main id="main">
{body}
</main>
<footer class="site">
<nav aria-label="Footer">
<a href="/docs/">Documentation</a>
<a href="/changelog/">Changelog</a>
<a href="https://github.com/mightty-org/mightty">Source code</a>
<a href="https://github.com/mightty-org/mightty/issues">Report an issue</a>
<a href="https://github.com/mightty-org/mightty/security/policy">Security</a>
</nav>
<p>Released under the <a href="https://github.com/mightty-org/mightty/blob/main/LICENSE">MIT License</a>.</p>
<p class="wearing">Theme: <span data-theme-name>Mightty</span>. <button type="button" data-shuffle>Change</button>. Themes from <a href="https://github.com/mbadolato/iTerm2-Color-Schemes">iTerm2-Color-Schemes</a>.</p>
</footer>
<script>{js}</script>
</body>
</html>
"##,
        title = escape(title),
        description = escape(description),
        home = link("/", "Overview", "home"),
        download = link("/download/", "Download", "download"),
        docs = link("/docs/", "Docs", "docs"),
        changelog = link("/changelog/", "Changelog", "changelog"),
        css = CSS_HERE,
        js = assets().1,
    );
    // Only the rules this page can use, found from the page itself.
    let css = used_css(&assets().0, &page);
    page.replace(CSS_HERE, &css)
}

/// A docs page: the list of docs beside `body`.
fn docs_page(docs: &[Doc], current: Option<&str>, body: &str) -> String {
    let mut nav = String::from("<nav class=\"doc-nav\" aria-label=\"Documentation\"><ul>\n");
    for d in docs {
        let here = if Some(d.slug.as_str()) == current { " aria-current=\"page\"" } else { "" };
        nav += &format!("<li><a href=\"/docs/{}/\"{here}>{}</a></li>\n", d.slug, escape(&d.title));
    }
    nav += "</ul></nav>\n";
    format!("<div class=\"docs\">\n{nav}<article class=\"prose\">\n{body}</article>\n</div>\n")
}

/// The download page: the latest release first, then a table of them all.
/// Each file's address is the permanent one of its version.
fn download_page(releases: &Value) -> String {
    let empty = Vec::new();
    let releases = releases.as_array().unwrap_or(&empty);
    let file = |version: &str, name: &str| format!("https://github.com/mightty-org/mightty/releases/download/v{version}/{name}");
    let notes = |version: &str| format!("https://github.com/mightty-org/mightty/releases/tag/v{version}");
    let megabytes = |r: &Value, key: &str| format!("{:.1} MB", r[key].as_u64().unwrap_or(0) as f64 / 1e6);
    let mut out = String::from("<article class=\"prose download\">\n<h1>Download</h1>\n");
    let Some(latest) = releases.first() else {
        out += "<p>There are no releases yet.</p>\n</article>\n";
        return out;
    };
    let version = text(latest, "version");
    out += &format!(
        "<p class=\"lede\">Mightty {version} is the latest version, released on {}. It requires macOS 12 or later and runs on Apple Silicon and Intel.</p>\n\
         <p class=\"cta\"><a class=\"button\" href=\"{}\">Download Mightty {version}</a><a class=\"button quiet\" href=\"{}\">Release notes</a></p>\n\
         <p class=\"version\">Disk image, {}. Signed and notarized by Apple.</p>\n\
         <p>Open the disk image and drag Mightty to Applications. The <a href=\"/docs/install/\">install guide</a> has the details, including how Mightty updates itself.</p>\n",
        long_date(text(latest, "date")),
        file(version, "Mightty.dmg"),
        notes(version),
        megabytes(latest, "dmg"),
    );
    out += "<h2 id=\"all-versions\">All versions</h2>\n\
            <p>Every release stays available at a permanent address. The disk image is the usual download. The zip archive holds the same app, for scripts.</p>\n\
            <div class=\"table\"><table class=\"files\">\n<tr><th>Version</th><th>Released</th><th>Download</th><th>Notes</th></tr>\n";
    for r in releases {
        let version = text(r, "version");
        out += &format!(
            "<tr><td>{version}</td><td>{}</td><td><a href=\"{}\">Disk image</a> ({}), <a href=\"{}\">zip</a> ({})</td><td><a href=\"{}\">Release notes</a></td></tr>\n",
            long_date(text(r, "date")),
            file(version, "Mightty.dmg"),
            megabytes(r, "dmg"),
            file(version, "Mightty.zip"),
            megabytes(r, "zip"),
            notes(version),
        );
    }
    out += "</table></div>\n\
            <p>The <a href=\"/changelog/\">changelog</a> describes what changed in each version.</p>\n\
            <h2 id=\"verifying\">Verifying a download</h2>\n\
            <p>Each release includes a <code>SHA256SUMS</code> file. To check a disk image, compare its checksum with the one listed here:</p>\n\
            <pre><code>shasum -a 256 Mightty.dmg</code></pre>\n\
            <div class=\"table\"><table>\n<tr><th>Version</th><th>SHA-256 of Mightty.dmg</th></tr>\n";
    for r in releases {
        out += &format!("<tr><td>{}</td><td><code>{}</code></td></tr>\n", escape(text(r, "version")), escape(text(r, "sha256")));
    }
    out += "</table></div>\n\
            <p>macOS also checks every download itself: Mightty is signed with a Developer ID and notarized by Apple, and it opens without a warning only if that check passes.</p>\n\
            </article>\n";
    out
}

/// `2026-11-12` as `November 12, 2026`.
fn long_date(iso: &str) -> String {
    const MONTHS: [&str; 12] =
        ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let mut parts = iso.split('-').map(|p| p.parse::<usize>().unwrap_or(0));
    let (year, month, day) = (parts.next().unwrap_or(0), parts.next().unwrap_or(0), parts.next().unwrap_or(0));
    match MONTHS.get(month.wrapping_sub(1)) {
        Some(name) => format!("{name} {day}, {year}"),
        None => iso.to_owned(),
    }
}

fn text<'a>(v: &'a Value, key: &str) -> &'a str {
    v.get(key).and_then(Value::as_str).unwrap_or("")
}

/// Every setting, by section: its name, what it takes, its default and its help.
fn settings_reference(data: &Value) -> String {
    let empty = Vec::new();
    let settings = data.get("settings").and_then(Value::as_array).unwrap_or(&empty);
    let sections = data.get("sections").and_then(Value::as_array).unwrap_or(&empty);
    let mut out = String::new();
    for section in sections.iter().filter_map(Value::as_str) {
        let id = markdown::slug(section);
        out += &format!("<h3 id=\"{id}\"><a class=\"anchor\" href=\"#{id}\">{}</a></h3>\n<dl class=\"settings\">\n", escape(section));
        for s in settings.iter().filter(|s| text(s, "section") == section) {
            let name = text(s, "name");
            let choices: Vec<String> = s.get("choices").and_then(Value::as_array).unwrap_or(&empty).iter().filter_map(Value::as_str).map(|c| format!("<code>{}</code>", escape(c))).collect();
            let takes = if choices.is_empty() { format!("<span class=\"kind\">{}</span>", escape(text(s, "value"))) } else { choices.join(" ") };
            let default = match s.get("default").and_then(Value::as_str) {
                Some(d) if !d.is_empty() => format!("<span class=\"default\">Default <code>{}</code></span>", escape(d)),
                _ => String::new(),
            };
            let many = if s.get("many").and_then(Value::as_bool) == Some(true) { "<span class=\"many\">One a line; repeat it for more</span>" } else { "" };
            // Capitalized, unless it starts with a value the setting takes, as written.
            let help = escape(text(s, "help"));
            let first: String = help.chars().take_while(|c| c.is_alphanumeric() || *c == '-' || *c == '_').collect();
            let literal = s.get("choices").and_then(Value::as_array).unwrap_or(&empty).iter().any(|c| c.as_str() == Some(first.as_str()));
            let help = if literal { help } else { capitalize(&help) };
            out += &format!(
                "<dt id=\"{name}\"><a href=\"#{name}\"><code>{name}</code></a></dt>\n<dd><p>{help}</p><p class=\"meta\">{takes} {default} {many}</p></dd>\n",
                name = escape(name),
            );
        }
        out += "</dl>\n";
    }
    out
}

/// Every action: how a `keybind` names it, its default key, what it does.
fn actions_reference(data: &Value) -> String {
    let empty = Vec::new();
    let actions = data.get("actions").and_then(Value::as_array).unwrap_or(&empty);
    let mut out = String::from("<div class=\"table\"><table class=\"actions\">\n<tr><th>Action</th><th>Key</th><th>What it does</th></tr>\n");
    for a in actions {
        let key = a.get("key").and_then(Value::as_str).map(|k| format!("<kbd>{}</kbd>", escape(k))).unwrap_or_default();
        out += &format!("<tr><td><code>{}</code></td><td>{key}</td><td>{}</td></tr>\n", escape(text(a, "name")), escape(text(a, "help")));
    }
    out + "</table></div>\n"
}

fn capitalize(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

/// The bundled themes, as JSON for the page: each a name and its colours
/// (background, foreground, cursor, selection, then palette 0–15).
fn themes(dir: &Path) -> io::Result<String> {
    let mut names: Vec<String> = fs::read_dir(dir)?.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().into_owned()).filter(|n| !n.starts_with('.')).collect();
    names.sort_by_key(|n| n.to_lowercase());
    let mut out = Vec::new();
    for name in names {
        let body = fs::read_to_string(dir.join(&name))?;
        let mut colors = vec![String::new(); 20];
        for line in body.lines() {
            let Some((key, value)) = line.split_once('=') else { continue };
            let value = value.trim();
            match key.trim() {
                "background" => colors[0] = value.to_owned(),
                "foreground" => colors[1] = value.to_owned(),
                "cursor-color" => colors[2] = value.to_owned(),
                "selection-background" => colors[3] = value.to_owned(),
                "palette" => {
                    if let Some((n, c)) = value.split_once('=') {
                        if let Ok(n) = n.trim().parse::<usize>() {
                            if n < 16 {
                                colors[4 + n] = c.trim().to_owned();
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        // Only complete ones: a page in half a theme isn't one.
        if colors.iter().all(|c| c.starts_with('#') && c.len() == 7) {
            out.push(Value::object([("n", Value::from(name)), ("c", Value::from(colors.into_iter().map(Value::from).collect::<Vec<_>>()))]));
        }
    }
    Ok(Value::from(out).to_string())
}
