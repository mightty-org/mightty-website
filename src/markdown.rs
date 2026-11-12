//! Markdown, as far as these pages use it: front matter, headings (with
//! anchors), paragraphs, lists, fenced code, tables, block quotes, and
//! inline code, bold, italic and links. Lines holding just `{{name}}` are
//! left for the caller to fill, as `<!--name-->`.

/// A page's front matter (`key: value` lines between `---`) and the rest.
pub fn front_matter(text: &str) -> (Vec<(String, String)>, &str) {
    let Some(rest) = text.strip_prefix("---\n") else { return (Vec::new(), text) };
    let Some(end) = rest.find("\n---\n") else { return (Vec::new(), text) };
    let fields = rest[..end].lines().filter_map(|l| l.split_once(':')).map(|(k, v)| (k.trim().to_owned(), v.trim().to_owned())).collect();
    (fields, &rest[end + 5..])
}

pub fn escape(text: &str) -> String {
    text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// An anchor for a heading: lowercase words joined by hyphens.
pub fn slug(text: &str) -> String {
    let mut out = String::new();
    for c in text.chars() {
        if c.is_alphanumeric() {
            out.extend(c.to_lowercase());
        } else if !out.ends_with('-') && !out.is_empty() {
            out.push('-');
        }
    }
    out.trim_end_matches('-').to_owned()
}

/// Inline markup in `text`: `code`, **bold**, *italic*, [links](url), and
/// `<kbd>` for keys, the one tag let through.
pub fn inline(text: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    let take_until = |from: usize, end: &str| -> Option<usize> {
        let rest: String = chars[from..].iter().collect();
        rest.find(end).map(|at| from + rest[..at].chars().count())
    };
    while i < chars.len() {
        let c = chars[i];
        let rest: String = chars[i..].iter().collect();
        if rest.starts_with("<kbd>") || rest.starts_with("</kbd>") {
            let tag = if rest.starts_with("<kbd>") { "<kbd>" } else { "</kbd>" };
            out += tag;
            i += tag.len();
            continue;
        }
        if c == '`' {
            if let Some(end) = take_until(i + 1, "`") {
                let code: String = chars[i + 1..end].iter().collect();
                out += &format!("<code>{}</code>", escape(&code));
                i = end + 1;
                continue;
            }
        } else if rest.starts_with("**") {
            if let Some(end) = take_until(i + 2, "**") {
                let inner: String = chars[i + 2..end].iter().collect();
                out += &format!("<strong>{}</strong>", inline(&inner));
                i = end + 2;
                continue;
            }
        } else if c == '*' && chars.get(i + 1).is_some_and(|n| !n.is_whitespace()) {
            if let Some(end) = take_until(i + 1, "*") {
                let inner: String = chars[i + 1..end].iter().collect();
                out += &format!("<em>{}</em>", inline(&inner));
                i = end + 1;
                continue;
            }
        } else if c == '[' {
            if let Some(close) = take_until(i + 1, "](") {
                if let Some(end) = take_until(close + 2, ")") {
                    let label: String = chars[i + 1..close].iter().collect();
                    let url: String = chars[close + 2..end].iter().collect();
                    out += &format!("<a href=\"{}\">{}</a>", escape(&url), inline(&label));
                    i = end + 1;
                    continue;
                }
            }
        }
        out += &escape(&c.to_string());
        i += 1;
    }
    out
}

/// The HTML for `text`, and its headings (level, text, anchor) for a contents list.
pub fn render(text: &str) -> (String, Vec<(usize, String, String)>) {
    let lines: Vec<&str> = text.lines().collect();
    let mut out = String::new();
    let mut headings = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();
        if trimmed.is_empty() {
            i += 1;
        } else if let Some(lang) = trimmed.strip_prefix("```") {
            let mut code = Vec::new();
            i += 1;
            while i < lines.len() && !lines[i].trim_start().starts_with("```") {
                code.push(lines[i]);
                i += 1;
            }
            i += 1;
            let class = if lang.is_empty() { String::new() } else { format!(" class=\"lang-{}\"", escape(lang)) };
            out += &format!("<pre><code{class}>{}</code></pre>\n", escape(&code.join("\n")));
        } else if trimmed.starts_with("{{") && trimmed.ends_with("}}") {
            out += &format!("<!--{}-->\n", &trimmed[2..trimmed.len() - 2]);
            i += 1;
        } else if let Some(level) = (1..=4).find(|&n| trimmed.starts_with(&format!("{} ", "#".repeat(n)))) {
            let text = trimmed[level + 1..].trim();
            let id = slug(text);
            out += &format!("<h{level} id=\"{id}\"><a class=\"anchor\" href=\"#{id}\">{}</a></h{level}>\n", inline(text));
            headings.push((level, text.to_owned(), id));
            i += 1;
        } else if trimmed.starts_with('|') {
            let mut rows = Vec::new();
            while i < lines.len() && lines[i].trim().starts_with('|') {
                rows.push(lines[i].trim());
                i += 1;
            }
            let cells = |row: &str| -> Vec<String> { row.trim_matches('|').split('|').map(|c| c.trim().to_owned()).collect() };
            out += "<div class=\"table\"><table>\n";
            for (n, row) in rows.iter().enumerate() {
                if n == 1 && row.chars().all(|c| matches!(c, '|' | '-' | ':' | ' ')) {
                    continue;
                }
                let tag = if n == 0 { "th" } else { "td" };
                out += "<tr>";
                for cell in cells(row) {
                    out += &format!("<{tag}>{}</{tag}>", inline(&cell));
                }
                out += "</tr>\n";
            }
            out += "</table></div>\n";
        } else if trimmed.starts_with("> ") || trimmed == ">" {
            let mut quote = Vec::new();
            while i < lines.len() && lines[i].trim_start().starts_with('>') {
                quote.push(lines[i].trim_start().trim_start_matches('>').trim_start());
                i += 1;
            }
            out += &format!("<blockquote>{}</blockquote>\n", render(&quote.join("\n")).0);
        } else if let Some(ordered) = list_item(trimmed) {
            let tag = if ordered { "ol" } else { "ul" };
            out += &format!("<{tag}>\n");
            while i < lines.len() && list_item(lines[i].trim()) == Some(ordered) {
                let mut item = item_text(lines[i].trim()).to_owned();
                i += 1;
                // Lines indented under an item continue it.
                while i < lines.len() && lines[i].starts_with("  ") && list_item(lines[i].trim()).is_none() && !lines[i].trim().is_empty() {
                    item.push(' ');
                    item.push_str(lines[i].trim());
                    i += 1;
                }
                out += &format!("<li>{}</li>\n", inline(&item));
            }
            out += &format!("</{tag}>\n");
        } else {
            let mut para = Vec::new();
            while i < lines.len() {
                let t = lines[i].trim();
                if t.is_empty() || t.starts_with("```") || t.starts_with('#') || t.starts_with('|') || t.starts_with("> ") || list_item(t).is_some() {
                    break;
                }
                para.push(t);
                i += 1;
            }
            out += &format!("<p>{}</p>\n", inline(&para.join(" ")));
        }
    }
    (out, headings)
}

/// Whether `line` starts a list item: `Some(true)` numbered, `Some(false)` bulleted.
fn list_item(line: &str) -> Option<bool> {
    if line.starts_with("- ") || line.starts_with("* ") {
        return Some(false);
    }
    let digits = line.chars().take_while(char::is_ascii_digit).count();
    (digits > 0 && line[digits..].starts_with(". ")).then_some(true)
}

fn item_text(line: &str) -> &str {
    match line.find(' ') {
        Some(at) => line[at + 1..].trim_start(),
        None => line,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_and_inline() {
        let (html, headings) = render("# Hello *world*\n\nSome `code` and [a link](/x).\n\n- one\n- two\n  continued\n\n```sh\na < b\n```\n\n| a | b |\n|---|---|\n| 1 | **2** |\n\n{{settings}}\n");
        assert!(html.contains("<h1 id=\"hello-world\">"));
        assert!(html.contains("<code>code</code>") && html.contains("<a href=\"/x\">a link</a>"));
        assert!(html.contains("<li>two continued</li>"));
        assert!(html.contains("<pre><code class=\"lang-sh\">a &lt; b</code></pre>"));
        assert!(html.contains("<td><strong>2</strong></td>") && !html.contains("---"));
        assert!(html.contains("<!--settings-->"));
        assert_eq!(inline("Press <kbd>⌘T</kbd>, not <b>"), "Press <kbd>⌘T</kbd>, not &lt;b&gt;");
        assert_eq!(headings[0].2, "hello-world");
        let (fields, rest) = front_matter("---\ntitle: Hi\norder: 2\n---\nbody");
        assert_eq!((fields[0].1.as_str(), rest), ("Hi", "body"));
    }
}
