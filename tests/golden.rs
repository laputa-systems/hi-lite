//! Fixture comparison against checked-in semantic highlight goldens.
//!
//! The `.snippet` files are syntax-complete examples. Their `.golden` siblings
//! contain one `start..end=kind` run list per source line. Goldens are generated
//! from syntect by the standalone tool documented in `tools/hi-lite-goldens.md`;
//! normal tests intentionally need only this crate and the checked-in files.

use std::fs;
use std::path::{Path, PathBuf};

use hi_lite::{Highlighter, Kind, Language, Run, runs};

const FIXTURES: &[(&str, &str, Language)] = &[
    ("bash", "bash", Language::Bash),
    ("c", "c", Language::C),
    ("css", "css", Language::Css),
    ("dockerfile", "dockerfile", Language::Dockerfile),
    ("go", "go", Language::Go),
    ("html", "html", Language::Html),
    ("ini", "ini", Language::Ini),
    ("javascript", "javascript", Language::JavaScript),
    ("json", "json", Language::Json),
    ("makefile", "makefile", Language::Makefile),
    ("markdown", "markdown", Language::Markdown),
    ("python", "python", Language::Python),
    ("rust", "rust", Language::Rust),
    ("toml", "toml", Language::Toml),
    ("typescript", "typescript", Language::TypeScript),
    ("yaml", "yaml", Language::Yaml),
    ("cpp", "language_probe", Language::Cpp),
    ("csharp", "language_probe", Language::CSharp),
    ("scss", "language_probe", Language::Scss),
    ("xml", "language_probe", Language::Xml),
    ("batch", "language_probe", Language::Batch),
    ("clojure", "language_probe", Language::Clojure),
    ("erlang", "language_probe", Language::Erlang),
    ("groovy", "language_probe", Language::Groovy),
    ("haskell", "language_probe", Language::Haskell),
    ("java", "language_probe", Language::Java),
    ("latex", "language_probe", Language::LaTeX),
    ("lisp", "language_probe", Language::Lisp),
    ("lua", "language_probe", Language::Lua),
    ("ocaml", "language_probe", Language::Ocaml),
    ("objective-c", "language_probe", Language::ObjectiveC),
    ("objective-cpp", "language_probe", Language::ObjectiveCpp),
    ("perl", "language_probe", Language::Perl),
    ("php", "language_probe", Language::Php),
    ("r", "language_probe", Language::R),
    ("ruby", "language_probe", Language::Ruby),
    ("scala", "language_probe", Language::Scala),
    ("sql", "language_probe", Language::Sql),
    ("swift", "language_probe", Language::Swift),
    ("kotlin", "language_probe", Language::Kotlin),
    ("regex", "language_probe", Language::Regex),
    ("xsh", "xsh", Language::Xsh),
];

#[test]
fn syntax_complete_fixtures_match_syntect_goldens() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    for &(golden_name, source_name, language) in FIXTURES {
        assert_fixture(&root, golden_name, source_name, language);
    }
}

fn assert_fixture(root: &Path, golden_name: &str, source_name: &str, language: Language) {
    let source_path = root.join(format!("{source_name}.snippet"));
    let golden_path = root.join(format!("{golden_name}.golden"));
    let source = fs::read_to_string(&source_path)
        .unwrap_or_else(|error| panic!("{}: {error}", source_path.display()));
    let golden = fs::read_to_string(&golden_path)
        .unwrap_or_else(|error| panic!("{}: {error}", golden_path.display()));
    let expected = parse_golden(&golden, &golden_path);

    let source = source.strip_suffix('\n').unwrap_or(&source);
    let mut highlighter = Highlighter::new(language);
    let mut scratch = Vec::new();
    for (line_index, line) in source.split('\n').enumerate() {
        let actual: Vec<_> =
            runs(highlighter.highlight_into(line.as_bytes(), &mut scratch)).collect();
        let expected_line = expected.get(line_index).cloned().unwrap_or_default();
        assert_eq!(
            actual,
            expected_line,
            "{golden_name} line {}: {line:?}",
            line_index + 1
        );
    }
    assert_eq!(
        expected.len(),
        source.split('\n').count(),
        "{golden_name}: golden line count"
    );
}

fn parse_golden(text: &str, path: &Path) -> Vec<Vec<Run>> {
    let mut lines = Vec::new();
    for (line_number, raw) in text.lines().enumerate() {
        let raw = raw.trim();
        if raw.is_empty() || raw.starts_with('#') {
            continue;
        }
        let (line_label, runs_text) = raw.split_once(':').unwrap_or_else(|| {
            panic!("{}:{}: expected `line N:`", path.display(), line_number + 1)
        });
        let source_line = line_label
            .strip_prefix("line ")
            .and_then(|number| number.parse::<usize>().ok())
            .unwrap_or_else(|| {
                panic!("{}:{}: invalid line label", path.display(), line_number + 1)
            });
        while lines.len() < source_line {
            lines.push(Vec::new());
        }
        let mut parsed: Vec<Run> = Vec::new();
        for item in runs_text.split_whitespace() {
            let (range, kind) = item.split_once('=').unwrap_or_else(|| {
                panic!(
                    "{}:{}: invalid run {item:?}",
                    path.display(),
                    line_number + 1
                )
            });
            let (start, end) = range
                .split_once("..")
                .and_then(|(start, end)| Some((start.parse().ok()?, end.parse().ok()?)))
                .unwrap_or_else(|| {
                    panic!(
                        "{}:{}: invalid range {range:?}",
                        path.display(),
                        line_number + 1
                    )
                });
            let kind = parse_kind(kind, path, line_number + 1);
            if let Some(previous) = parsed.last_mut()
                && previous.end == start
                && previous.kind == kind
            {
                previous.end = end;
            } else {
                parsed.push(Run { start, end, kind });
            }
        }
        lines[source_line - 1] = parsed;
    }
    lines
}

fn parse_kind(kind: &str, path: &Path, line: usize) -> Kind {
    match kind {
        "normal" => Kind::Normal,
        "keyword" => Kind::Keyword,
        "type" => Kind::Type,
        "string" => Kind::String,
        "comment" => Kind::Comment,
        "number" => Kind::Number,
        "bracket" => Kind::Bracket,
        "operator" => Kind::Operator,
        "function" => Kind::Function,
        "constant" => Kind::Constant,
        "macro" => Kind::Macro,
        _ => panic!("{}:{}: unknown kind {kind:?}", path.display(), line),
    }
}

#[allow(dead_code)]
fn _fixture_paths(root: &Path) -> Vec<PathBuf> {
    FIXTURES
        .iter()
        .map(|(_, source_name, _)| root.join(format!("{source_name}.snippet")))
        .collect()
}
