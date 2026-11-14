//! What the site gives search engines and AI agents besides its pages: each
//! page as Markdown beside its HTML, `llms.txt` and `llms-full.txt` at the
//! root, structured data (JSON-LD) in the pages' heads, and the dates the
//! sitemap gives.

use std::process::Command;

use mightty_json::Value;

use crate::{SITE_URL, text};

/// `markdown` as a page served on its own: site links made absolute, and
/// `<kbd>` keys and the `{{…}}` references replaced with plain Markdown.
pub fn page(markdown: &str, data: &Value) -> String {
    let mut out = String::with_capacity(markdown.len());
    for line in markdown.lines() {
        match line.trim() {
            "{{settings}}" => out += &settings(data),
            "{{actions}}" => out += &actions(data),
            "{{layers}}" => out += LAYERS,
            _ => {
                out += line;
                out.push('\n');
            }
        }
    }
    absolute(&out.replace("<kbd>", "").replace("</kbd>", ""))
}

/// Site links (`](/…`) as full addresses, which mean the same wherever the
/// Markdown is read.
pub fn absolute(markdown: &str) -> String {
    markdown.replace("](/", &format!("]({SITE_URL}/"))
}

/// The layers of the architecture page, which the HTML draws as a diagram.
const LAYERS: &str = "\
| Layer | Crates |
|---|---|
| Interfaces | `mightty-macos` (Mightty.app), `mightty` (`mightty ctl`), `mightty-mcp` (`mightty mcp`), `mightty-ffi` (libmightty) |
| Sessions | `mightty-mux` (multiplexer and control socket) |
| Terminal core | `mightty-term` (terminal state), `mightty-render` (drawing primitives), `mightty-font` (font rasterizer) |
| Foundations | `mightty-vt` (parser), `mightty-pty` (pseudo-terminals), `mightty-json` (JSON), `mightty-image` (PNG and zlib) |

A crate depends only on crates in its own layer or below.
";

/// Every setting, by section, as Markdown.
fn settings(data: &Value) -> String {
    let empty = Vec::new();
    let all = data.get("settings").and_then(Value::as_array).unwrap_or(&empty);
    let sections = data.get("sections").and_then(Value::as_array).unwrap_or(&empty);
    let mut out = String::new();
    for section in sections.iter().filter_map(Value::as_str) {
        out += &format!("### {section}\n\n");
        for s in all.iter().filter(|s| text(s, "section") == section) {
            let choices: Vec<String> = s
                .get("choices")
                .and_then(Value::as_array)
                .unwrap_or(&empty)
                .iter()
                .filter_map(Value::as_str)
                .map(|c| format!("`{c}`"))
                .collect();
            let takes = if choices.is_empty() { text(s, "value").to_owned() } else { choices.join(", ") };
            out += &format!("- `{}` ({takes}", text(s, "name"));
            if let Some(d) = s.get("default").and_then(Value::as_str).filter(|d| !d.is_empty()) {
                out += &format!("; default `{d}`");
            }
            if s.get("many").and_then(Value::as_bool) == Some(true) {
                out += "; can be given more than once";
            }
            out += &format!("): {}\n", text(s, "help"));
        }
        out.push('\n');
    }
    out
}

/// Every action, as a Markdown table.
fn actions(data: &Value) -> String {
    let empty = Vec::new();
    let all = data.get("actions").and_then(Value::as_array).unwrap_or(&empty);
    let mut out = String::from("| Action | Default key | What it does |\n|---|---|---|\n");
    for a in all {
        let key = a.get("key").and_then(Value::as_str).unwrap_or("");
        out += &format!("| `{}` | {key} | {} |\n", text(a, "name"), text(a, "help").replace('|', "\\|"));
    }
    out + "\n"
}

/// A docs page in the lists `llms.txt` gives.
pub struct Entry {
    pub title: String,
    pub summary: String,
    pub path: String,
}

/// `llms.txt`: what Mightty is, and where each page is as Markdown.
pub fn llms(version: &str, docs: &[Entry]) -> String {
    let mut out = format!(
        "# Mightty\n\n\
         > The fast, lightweight terminal for macOS. GPU-rendered with Metal, written in Rust with no third-party dependencies, and built to work with coding agents.\n\n\
         Mightty is a terminal emulator for macOS 12 or later, on Apple Silicon and Intel. It is free and open source under the MIT License. The current version is {version}.\n\n\
         Coding agents can control Mightty: `mightty mcp` is an MCP server that opens panes, runs commands in them and reads their output and exit codes, and `mightty ctl` does the same from the shell over a JSON-RPC control socket. Every link below is a Markdown version of a page on {SITE_URL}.\n\n\
         ## Download\n\n\
         - [Download]({SITE_URL}/download/index.md): the latest version and every earlier one, with checksums\n\n\
         ## Documentation\n\n"
    );
    for d in docs {
        out += &format!("- [{}]({SITE_URL}/{}/index.md): {}\n", d.title, d.path, d.summary);
    }
    out += &format!(
        "\n## Optional\n\n\
         - [Changelog]({SITE_URL}/changelog/index.md): what changed in each version\n\
         - [All documentation in one file]({SITE_URL}/llms-full.txt)\n\
         - [Source code](https://github.com/mightty-org/mightty): the Rust workspace, with a README and AGENTS.md for coding agents\n"
    );
    out
}

/// `llms-full.txt`: every docs page in one file, in order.
pub fn llms_full(version: &str, pages: &[(Entry, String)]) -> String {
    let mut out = format!(
        "# Mightty documentation\n\n\
         > The fast, lightweight terminal for macOS. Version {version}. Source: {SITE_URL}/docs/\n"
    );
    for (entry, markdown) in pages {
        out += &format!("\n---\n\nSource: {SITE_URL}/{}/\n\n{}", entry.path, markdown.trim_start());
    }
    out
}

/// `value` as a JSON-LD block for a page's head.
pub fn json_ld(value: &Value) -> String {
    // `<` escaped, so no text in it can close the script element.
    format!(
        "<script type=\"application/ld+json\">{}</script>\n",
        value.to_string().replace('<', "\\u003c")
    )
}

/// Mightty as a `SoftwareApplication`.
pub fn application(version: &str, description: &str) -> Value {
    Value::object([
        ("@context", "https://schema.org".into()),
        ("@type", "SoftwareApplication".into()),
        ("name", "Mightty".into()),
        ("description", description.into()),
        ("url", format!("{SITE_URL}/").into()),
        ("image", format!("{SITE_URL}/og.png").into()),
        ("applicationCategory", "DeveloperApplication".into()),
        ("operatingSystem", "macOS 12 or later".into()),
        ("softwareVersion", version.into()),
        (
            "downloadUrl",
            "https://github.com/mightty-org/mightty/releases/latest/download/Mightty.dmg".into(),
        ),
        ("license", "https://opensource.org/license/mit".into()),
        ("codeRepository", "https://github.com/mightty-org/mightty".into()),
        (
            "offers",
            Value::object([
                ("@type", "Offer".into()),
                ("price", "0".into()),
                ("priceCurrency", "USD".into()),
            ]),
        ),
    ])
}

/// A docs page as a `TechArticle`, with where it sits in the site.
pub fn article(title: &str, description: &str, path: &str, modified: Option<&str>) -> Value {
    let crumb = |n: u32, name: &str, item: String| {
        Value::object([
            ("@type", "ListItem".into()),
            ("position", n.into()),
            ("name", name.into()),
            ("item", item.into()),
        ])
    };
    let mut page = Value::object([
        ("@type", "TechArticle".into()),
        ("headline", title.into()),
        ("description", description.into()),
        ("url", format!("{SITE_URL}/{path}/").into()),
        ("inLanguage", "en".into()),
    ]);
    if let Some(date) = modified {
        page.insert("dateModified", date);
    }
    Value::object([
        ("@context", "https://schema.org".into()),
        (
            "@graph",
            vec![
                page,
                Value::object([
                    ("@type", "BreadcrumbList".into()),
                    (
                        "itemListElement",
                        vec![
                            crumb(1, "Documentation", format!("{SITE_URL}/docs/")),
                            crumb(2, title, format!("{SITE_URL}/{path}/")),
                        ]
                        .into(),
                    ),
                ]),
            ]
            .into(),
        ),
    ])
}

/// The date the newest commit touching `sources` was made, as `2026-11-13`,
/// if git knows: when the page they make last changed.
pub fn modified(sources: &[&str]) -> Option<String> {
    let output = Command::new("git").args(["log", "-1", "--format=%cs", "--"]).args(sources).output().ok()?;
    let date = String::from_utf8(output.stdout).ok()?.trim().to_owned();
    (output.status.success() && date.len() == 10).then_some(date)
}

