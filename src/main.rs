//! Generates the Zed theme `extension/themes/pustota.json` from the upstream pustota palettes.

use std::path::Path;
use std::{env, fs, io};

use serde::Deserialize;
use serde_json::{Map, Value, json};

const ROOT: &str = env!("CARGO_MANIFEST_DIR");
const SOURCE_PALETTES: [&str; 2] = [
    include_str!("../pustota/src/palettes/dark.json"),
    include_str!("../pustota/src/palettes/light.json"),
];
const TARGET_THEME: &str = "extension/themes/pustota.json";

fn main() -> io::Result<()> {
    let palettes = SOURCE_PALETTES
        .into_iter()
        .map(serde_json::from_str)
        .collect::<Result<Vec<Palette>, _>>()?;
    let theme = format!(
        "{}\n",
        serde_json::to_string_pretty(&with_metadata(&palettes))?
    );
    let target = Path::new(ROOT).join(TARGET_THEME);

    if !env::args().any(|arg| arg == "--check") {
        return fs::write(target, theme);
    }

    if fs::read_to_string(target)? != theme {
        return Err(io::Error::other(format!(
            "{TARGET_THEME} is outdated, run `cargo run`"
        )));
    }

    Ok(())
}

#[derive(Deserialize)]
struct Palette {
    name: String,
    #[serde(rename = "type")]
    appearance: String,
    colors: Colors,
}

#[derive(Deserialize)]
struct Colors {
    base: Base,
    accent: Accent,
    syntax: Syntax,
    selection: Selection,
    vcs: Vcs,
    terminal: Map<String, Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Base {
    #[serde(rename = "bg")]
    background: String,
    line: String,
    #[serde(rename = "fg")]
    foreground: String,
    #[serde(rename = "ui")]
    interface: String,
    ui_muted: String,
}

#[derive(Deserialize)]
struct Accent {
    main: String,
}

#[derive(Deserialize)]
struct Syntax {
    comment: String,
    string: String,
    keyword: String,
    func: String,
    entity: String,
    constant: String,
    operator: String,
}

#[derive(Deserialize)]
struct Selection {
    #[serde(rename = "bg")]
    background: String,
    inactive: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Vcs {
    added: String,
    modified: String,
    removed: String,
    modified_file: String,
    conflict: String,
}

fn with_metadata(palettes: &[Palette]) -> Value {
    let themes: Vec<Value> = palettes
        .iter()
        .map(|palette| {
            json!({
                "name": palette.name,
                "appearance": palette.appearance,
                "style": style(&palette.colors),
            })
        })
        .collect();

    json!({
        "$schema": "https://zed.dev/schema/themes/v0.2.0.json",
        "name": "pustota",
        "author": "Nikita Sobolev",
        "themes": themes,
    })
}

fn style(
    Colors {
        base,
        accent,
        syntax,
        selection,
        vcs,
        terminal,
    }: &Colors,
) -> Value {
    let highlights = merge(&[code_highlights(accent, syntax), markup_highlights(vcs)]);

    merge(&[
        surfaces(base, accent),
        text_and_icons(base, accent),
        borders(base, accent),
        elements(base, accent, selection),
        editor(base, selection, vcs),
        search_and_scrollbar(base, accent),
        bars_and_panels(base),
        version_control(base, syntax, vcs),
        diagnostics(base, accent, syntax, vcs),
        terminal_colors(base, terminal),
        json!({ "syntax": highlights }),
    ])
}

fn merge(sections: &[Value]) -> Value {
    let settings = sections
        .iter()
        .filter_map(Value::as_object)
        .flatten()
        .map(|(key, setting)| (key.clone(), setting.clone()));

    Value::Object(Map::from_iter(settings))
}

fn surfaces(base: &Base, accent: &Accent) -> Value {
    json!({
        "accents": [accent.main],
        "background": base.background,
        "background.appearance": "opaque",
        "surface.background": base.background,
        "elevated_surface.background": base.background,
    })
}

fn text_and_icons(base: &Base, accent: &Accent) -> Value {
    json!({
        "text": base.foreground,
        "text.accent": accent.main,
        "text.muted": base.interface,
        "text.disabled": with_opacity(&base.interface, "99"),
        "text.placeholder": with_opacity(&base.foreground, "80"),
        "link_text.hover": accent.main,
        "icon": base.foreground,
        "icon.accent": accent.main,
        "icon.muted": with_opacity(&base.interface, "99"),
        "icon.disabled": with_opacity(&base.interface, "99"),
        "icon.placeholder": with_opacity(&base.foreground, "80"),
    })
}

fn borders(base: &Base, accent: &Accent) -> Value {
    json!({
        "border": base.background,
        "border.variant": base.background,
        "border.transparent": with_opacity(&base.background, "00"),
        "border.focused": accent.main,
        "border.selected": accent.main,
        "border.disabled": base.background,
        "pane.focused_border": base.background,
        "pane_group.border": base.line,
        "panel.focused_border": base.background,
    })
}

fn elements(base: &Base, accent: &Accent, selection: &Selection) -> Value {
    json!({
        "players": [{ "cursor": accent.main, "selection": selection.background }],
        "element.background": base.background,
        "element.hover": base.line,
        "element.active": base.line,
        "element.selected": base.background,
        "element.disabled": base.background,
        "element.selection_background": selection.background,
        "ghost_element.background": base.background,
        "ghost_element.hover": base.background,
        "ghost_element.active": base.line,
        "ghost_element.selected": selection.inactive,
        "ghost_element.disabled": base.background,
        "drop_target.background": with_opacity(&selection.inactive, "f2"),
    })
}

fn editor(base: &Base, selection: &Selection, vcs: &Vcs) -> Value {
    json!({
        "editor.background": base.background,
        "editor.foreground": base.foreground,
        "editor.gutter.background": base.background,
        "editor.line_number": with_opacity(&base.foreground, "80"),
        "editor.active_line_number": with_opacity(&base.foreground, "cc"),
        "editor.active_line.background": base.line,
        "editor.highlighted_line.background": with_opacity(&selection.inactive, "f2"),
        "editor.indent_guide": with_opacity(&base.interface, "66"),
        "editor.indent_guide_active": with_opacity(&base.interface, "b3"),
        "editor.wrap_guide": with_opacity(&base.interface, "66"),
        "editor.active_wrap_guide": with_opacity(&base.interface, "b3"),
        "editor.invisible": with_opacity(&base.interface, "99"),
        "editor.document_highlight.bracket_background": with_opacity(&base.interface, "4d"),
        "editor.document_highlight.read_background": with_opacity(&vcs.modified, "12"),
        "editor.document_highlight.write_background": with_opacity(&vcs.added, "12"),
        "editor.subheader.background": base.background,
    })
}

fn search_and_scrollbar(base: &Base, accent: &Accent) -> Value {
    json!({
        "search.match_background": with_opacity(&accent.main, "33"),
        "scrollbar.thumb.background": with_opacity(&base.interface, "66"),
        "scrollbar.thumb.hover_background": with_opacity(&base.interface, "99"),
        "scrollbar.thumb.border": base.background,
        "scrollbar.track.background": base.background,
        "scrollbar.track.border": base.background,
    })
}

fn bars_and_panels(base: &Base) -> Value {
    json!({
        "title_bar.background": base.background,
        "title_bar.inactive_background": base.background,
        "toolbar.background": base.background,
        "tab_bar.background": base.background,
        "tab.active_background": base.background,
        "tab.inactive_background": base.background,
        "panel.background": base.background,
        "panel.indent_guide": with_opacity(&base.interface, "b3"),
        "panel.indent_guide_active": with_opacity(&base.interface, "b3"),
        "panel.indent_guide_hover": with_opacity(&base.interface, "99"),
        "status_bar.background": base.background,
    })
}

fn version_control(base: &Base, syntax: &Syntax, vcs: &Vcs) -> Value {
    json!({
        "created": syntax.string,
        "created.background": with_opacity(&syntax.string, "26"),
        "created.border": with_opacity(&vcs.added, "99"),
        "modified": vcs.modified_file,
        "modified.background": with_opacity(&vcs.modified, "26"),
        "modified.border": with_opacity(&vcs.modified, "99"),
        "deleted": with_opacity(&vcs.removed, "b3"),
        "deleted.background": with_opacity(&syntax.operator, "26"),
        "deleted.border": with_opacity(&vcs.removed, "99"),
        "renamed": with_opacity(&syntax.constant, "b3"),
        "conflict": vcs.conflict,
        "ignored": base.ui_muted,
        "ignored.background": base.background,
        "ignored.border": base.background,
        "hidden": base.ui_muted,
    })
}

fn diagnostics(base: &Base, accent: &Accent, syntax: &Syntax, vcs: &Vcs) -> Value {
    json!({
        "success": syntax.string,
        "success.background": with_opacity(&syntax.string, "26"),
        "success.border": with_opacity(&vcs.added, "99"),
        "error": syntax.keyword,
        "error.background": base.background,
        "error.border": syntax.keyword,
        "warning": accent.main,
        "warning.background": base.background,
        "warning.border": accent.main,
        "info": syntax.entity,
        "info.background": base.background,
        "info.border": syntax.entity,
        "hint": syntax.comment,
        "hint.background": base.background,
        "hint.border": syntax.comment,
        "predictive": syntax.comment,
        "unreachable": syntax.comment,
    })
}

fn terminal_colors(base: &Base, terminal: &Map<String, Value>) -> Value {
    let ansi = terminal.iter().map(|(name, color)| {
        let snake_case = name.replace("bright", "bright_").to_lowercase();
        (format!("terminal.ansi.{snake_case}"), color.clone())
    });

    let panel = json!({
        "terminal.background": base.background,
        "terminal.foreground": base.foreground,
        "terminal.ansi.background": base.background,
    });

    merge(&[panel, Value::Object(Map::from_iter(ansi))])
}

fn code_highlights(accent: &Accent, syntax: &Syntax) -> Value {
    let rules = [
        (
            json!({ "color": syntax.comment, "font_style": "italic" }),
            COMMENTS,
        ),
        (json!({ "color": syntax.comment }), HINTS),
        (json!({ "color": syntax.string }), STRINGS),
        (json!({ "color": accent.main }), CONSTANTS),
        (json!({ "color": syntax.keyword }), KEYWORDS),
        (json!({ "color": syntax.func }), FUNCTIONS),
        (json!({ "color": syntax.entity }), ENTITIES),
    ];

    let highlights = rules.iter().flat_map(|(highlight, names)| {
        names
            .iter()
            .map(move |name| ((*name).to_owned(), highlight.clone()))
    });

    Value::Object(Map::from_iter(highlights))
}

fn markup_highlights(vcs: &Vcs) -> Value {
    json!({
        "diff.delta": { "color": with_opacity(&vcs.modified, "99") },
        "diff.plus": { "color": with_opacity(&vcs.added, "99") },
        "diff.minus": { "color": with_opacity(&vcs.removed, "99") },
        "emphasis": { "font_style": "italic" },
        "emphasis.strong": { "font_weight": 700 },
    })
}

const COMMENTS: &[&str] = &["comment", "comment.doc", "tag.doctype"];
const HINTS: &[&str] = &["hint", "predictive"];

const STRINGS: &[&str] = &[
    "string",
    "string.escape",
    "string.regex",
    "string.special",
    "string.special.symbol",
    "text.literal",
    "link_uri",
];

const CONSTANTS: &[&str] = &["boolean", "constant", "constant.builtin", "number"];

const KEYWORDS: &[&str] = &[
    "keyword",
    "operator",
    "preproc",
    "punctuation.list_marker",
    "punctuation.special",
    "tag",
];

const FUNCTIONS: &[&str] = &[
    "attribute",
    "constructor",
    "function.definition",
    "function.method.constructor",
    "function.method.definition",
    "function.macro",
    "label",
];

const ENTITIES: &[&str] = &["link_text", "title", "type.class.definition"];

fn with_opacity(color: &str, opacity: &str) -> String {
    assert!(
        color.len() == 7 && color.starts_with('#'),
        "expected an opaque #rrggbb color, got {color}"
    );
    format!("{color}{opacity}")
}
