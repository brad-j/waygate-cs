//! A small Markdown renderer for Claude's replies, plus a span-aware word wrapper
//! so every wrapped row is known (which keeps mouse hit-testing exact).

use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};
use unicode_width::UnicodeWidthStr;

use crate::theme;

/// A logical line: wrapped later, continuation rows indented by `indent`.
#[derive(Clone, Default)]
pub struct Block {
    pub spans: Vec<Span<'static>>,
    pub indent: u16,
    pub rule: bool,
}

impl Block {
    pub fn new(spans: Vec<Span<'static>>) -> Self {
        Self { spans, indent: 0, rule: false }
    }
    pub fn indent(mut self, n: u16) -> Self {
        self.indent = n;
        self
    }
    pub fn blank() -> Self {
        Self::default()
    }
}

pub fn render(md: &str) -> Vec<Block> {
    let mut out: Vec<Block> = Vec::new();
    let mut in_code = false;
    for raw in md.lines() {
        let trimmed = raw.trim();
        if trimmed.starts_with("```") {
            in_code = !in_code;
            continue;
        }
        if in_code {
            out.push(
                Block::new(vec![
                    Span::styled("│ ", theme::faint()),
                    Span::styled(raw.replace('\t', "    "), theme::code()),
                ])
                .indent(2),
            );
            continue;
        }
        if trimmed.is_empty() {
            if out.last().is_some_and(|b| !b.spans.is_empty() || b.rule) {
                out.push(Block::blank());
            }
            continue;
        }
        if trimmed.len() >= 3 && trimmed.chars().all(|c| c == '-' || c == '*' || c == '_') {
            out.push(Block { rule: true, ..Default::default() });
            continue;
        }
        if let Some(h) = heading(trimmed) {
            out.push(Block::new(inline(h, theme::label())));
            continue;
        }
        if trimmed.starts_with('|') {
            if trimmed.chars().all(|c| matches!(c, '|' | '-' | ':' | ' ')) {
                continue;
            }
            let mut spans = Vec::new();
            let cells: Vec<&str> = trimmed.trim_matches('|').split('|').map(str::trim).collect();
            for (i, cell) in cells.iter().enumerate() {
                if i > 0 {
                    spans.push(Span::styled("  │  ", theme::faint()));
                }
                spans.extend(inline(cell, theme::text()));
            }
            out.push(Block::new(spans).indent(2));
            continue;
        }
        if let Some(q) = trimmed.strip_prefix('>') {
            let mut spans = vec![Span::styled("▎ ", theme::accent())];
            spans.extend(inline(q.trim_start(), theme::dim().add_modifier(Modifier::ITALIC)));
            out.push(Block::new(spans).indent(2));
            continue;
        }
        let lead = (raw.len() - raw.trim_start().len()) as u16;
        let pad = " ".repeat(lead as usize);
        if let Some(rest) = trimmed
            .strip_prefix("- ")
            .or_else(|| trimmed.strip_prefix("* "))
            .or_else(|| trimmed.strip_prefix("+ "))
        {
            let mut spans = vec![Span::raw(pad), Span::styled("• ", theme::accent())];
            spans.extend(inline(rest, theme::text()));
            out.push(Block::new(spans).indent(lead + 2));
            continue;
        }
        if let Some((num, rest)) = numbered(trimmed) {
            let prefix = format!("{num} ");
            let w = prefix.width() as u16;
            let mut spans = vec![Span::raw(pad), Span::styled(prefix, theme::accent())];
            spans.extend(inline(rest, theme::text()));
            out.push(Block::new(spans).indent(lead + w));
            continue;
        }
        out.push(Block::new(inline(trimmed, theme::text())));
    }
    while out.last().is_some_and(|b| b.spans.is_empty() && !b.rule) {
        out.pop();
    }
    out
}

fn heading(s: &str) -> Option<&str> {
    let hashes = s.chars().take_while(|&c| c == '#').count();
    (1..=6)
        .contains(&hashes)
        .then(|| s[hashes..].strip_prefix(' '))
        .flatten()
}

fn numbered(s: &str) -> Option<(&str, &str)> {
    let digits = s.chars().take_while(char::is_ascii_digit).count();
    if digits == 0 || digits > 3 {
        return None;
    }
    let after = &s[digits..];
    if after.starts_with(". ") || after.starts_with(") ") {
        Some((&s[..digits + 1], &after[2..]))
    } else {
        None
    }
}

/// Inline Markdown: **bold**, *italic*, `code`, [links](url).
pub fn inline(s: &str, base: Style) -> Vec<Span<'static>> {
    let mut out = Vec::new();
    let mut buf = String::new();
    let (mut bold, mut italic) = (false, false);
    let style_for = |bold: bool, italic: bool| {
        let mut st = base;
        if bold {
            st = st.fg(theme::BRIGHT).add_modifier(Modifier::BOLD);
        }
        if italic {
            st = st.add_modifier(Modifier::ITALIC);
        }
        st
    };
    let mut i = 0;
    while i < s.len() {
        let rest = &s[i..];
        let flush = |buf: &mut String, out: &mut Vec<Span<'static>>, b: bool, it: bool| {
            if !buf.is_empty() {
                out.push(Span::styled(std::mem::take(buf), style_for(b, it)));
            }
        };
        if let Some(r) = rest.strip_prefix('`')
            && let Some(end) = r.find('`') {
                flush(&mut buf, &mut out, bold, italic);
                out.push(Span::styled(r[..end].to_string(), theme::code()));
                i += end + 2;
                continue;
            }
        if let Some(r) = rest.strip_prefix("**")
            && (bold || r.contains("**")) {
                flush(&mut buf, &mut out, bold, italic);
                bold = !bold;
                i += 2;
                continue;
            }
        if let Some(r) = rest.strip_prefix('*') {
            let opens = r.chars().next().is_some_and(|c| !c.is_whitespace()) && r.contains('*');
            if italic || opens {
                flush(&mut buf, &mut out, bold, italic);
                italic = !italic;
                i += 1;
                continue;
            }
        }
        if rest.starts_with('[')
            && let Some(close) = rest.find(']')
                && rest[close..].starts_with("](")
                    && let Some(paren) = rest[close..].find(')') {
                        flush(&mut buf, &mut out, bold, italic);
                        out.push(Span::styled(
                            rest[1..close].to_string(),
                            style_for(bold, italic).add_modifier(Modifier::UNDERLINED),
                        ));
                        i += close + paren + 1;
                        continue;
                    }
        let ch = rest.chars().next().unwrap_or(' ');
        buf.push(ch);
        i += ch.len_utf8();
    }
    if !buf.is_empty() {
        out.push(Span::styled(buf, style_for(bold, italic)));
    }
    out
}

/// Word-wraps one logical block to `width` columns.
pub fn wrap(block: &Block, width: u16) -> Vec<Line<'static>> {
    let width = width.max(4) as usize;
    if block.rule {
        return vec![Line::from(Span::styled("─".repeat(width), theme::faint()))];
    }
    if block.spans.is_empty() {
        return vec![Line::default()];
    }
    let indent = (block.indent as usize).min(width / 2);
    let mut lines: Vec<Line<'static>> = Vec::new();
    let mut cur: Vec<Span<'static>> = Vec::new();
    let mut cur_w = 0usize;
    let mut fresh = false;

    let newline = |lines: &mut Vec<Line<'static>>, cur: &mut Vec<Span<'static>>, cur_w: &mut usize| {
        lines.push(Line::from(std::mem::take(cur)));
        cur.push(Span::raw(" ".repeat(indent)));
        *cur_w = indent;
    };

    for span in &block.spans {
        let style = span.style;
        for tok in tokens(&span.content) {
            let w = tok.width();
            let space = tok.chars().all(char::is_whitespace);
            if space {
                if fresh {
                    continue;
                }
                if cur_w + w <= width {
                    cur.push(Span::styled(tok.to_string(), style));
                    cur_w += w;
                } else {
                    newline(&mut lines, &mut cur, &mut cur_w);
                    fresh = true;
                }
                continue;
            }
            fresh = false;
            if cur_w + w > width && cur_w > indent {
                newline(&mut lines, &mut cur, &mut cur_w);
            }
            if w <= width - cur_w {
                cur.push(Span::styled(tok.to_string(), style));
                cur_w += w;
                continue;
            }
            // A single token wider than the line: hard-split it.
            let mut piece = String::new();
            for ch in tok.chars() {
                let cw = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
                if cur_w + cw > width {
                    cur.push(Span::styled(std::mem::take(&mut piece), style));
                    newline(&mut lines, &mut cur, &mut cur_w);
                }
                piece.push(ch);
                cur_w += cw;
            }
            if !piece.is_empty() {
                cur.push(Span::styled(piece, style));
            }
        }
    }
    lines.push(Line::from(cur));
    lines
}

fn tokens(s: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut prev_space: Option<bool> = None;
    for (i, ch) in s.char_indices() {
        let sp = ch.is_whitespace();
        if prev_space.is_some_and(|p| p != sp) {
            out.push(&s[start..i]);
            start = i;
        }
        prev_space = Some(sp);
    }
    if start < s.len() {
        out.push(&s[start..]);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plain(l: &Line) -> String {
        l.spans.iter().map(|s| s.content.as_ref()).collect()
    }

    #[test]
    fn wraps_with_hanging_indent() {
        let b = &render("- one two three four five six")[0];
        let rows: Vec<String> = wrap(b, 12).iter().map(plain).collect();
        assert_eq!(rows[0], "• one two ");
        assert!(rows[1].starts_with("  three"));
    }

    #[test]
    fn inline_styles() {
        let spans = inline("a **b** `c` [d](e)", theme::text());
        let text: String = spans.iter().map(|s| s.content.as_ref()).collect();
        assert_eq!(text, "a b c d");
    }
}
