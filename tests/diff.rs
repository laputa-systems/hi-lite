//! Checked-in unified diff fixtures derived from the `fx` semantic diff.

use std::fs;
use std::path::Path;

use hi_lite::{
    DEFAULT_CONTEXT_LINES, DEFAULT_MAX_DISPLAY_LINES, DiffLine, DiffOp, DiffPreviewLine,
    DiffPreviewOp, DiffPreviewText, DiffScratch, diff, diff_preview,
};

const FIXTURES: &[&str] = &["diff-basic", "diff-hunks"];

#[test]
fn unified_diff_fixtures_match_checked_in_goldens() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    for &name in FIXTURES {
        assert_fixture(&root, name);
    }
}

#[test]
fn trailing_newline_changes_use_fx_markers() {
    let removed = diff(b"line\n", b"line");
    assert_eq!(removed[1].op, DiffOp::Remove);
    assert_eq!(removed[1].text, b"(trailing newline removed)");

    let added = diff(b"line", b"line\n");
    assert_eq!(added[1].op, DiffOp::Add);
    assert_eq!(added[1].text, b"(trailing newline added)");
}

#[test]
fn a_source_line_matching_the_omission_text_stays_a_deletion() {
    let preview = diff_preview(b"(diff omitted)\n", b"", 0, 1);
    assert_eq!(
        preview,
        [DiffPreviewLine {
            op: DiffPreviewOp::Deletion,
            old_line: Some(1),
            new_line: None,
            text: DiffPreviewText::Source(b"(diff omitted)"),
        }],
    );
}

#[test]
fn reused_preview_clears_context_and_respects_the_zero_row_limit() {
    let mut scratch = DiffScratch::new();
    let mut lines = Vec::new();
    let mut preview = Vec::new();

    scratch.diff_into(b"head\nold\ntail\n", b"head\nnew\ntail\n", &mut lines);
    scratch.unified_preview_into(&lines, 0, 0, &mut preview);
    assert_eq!(
        preview,
        [
            DiffPreviewLine {
                op: DiffPreviewOp::Deletion,
                old_line: Some(2),
                new_line: None,
                text: DiffPreviewText::Source(b"old"),
            },
            DiffPreviewLine {
                op: DiffPreviewOp::Elision,
                old_line: None,
                new_line: None,
                text: DiffPreviewText::Elision,
            },
        ],
    );

    let unchanged = b"head\nold\nnew\ntail\n";
    scratch.diff_into(unchanged, unchanged, &mut lines);
    scratch.unified_preview_into(&lines, 0, 4, &mut preview);
    assert!(preview.is_empty());
}

#[test]
fn reused_diff_keeps_the_longest_alignment_and_numbered_rows() {
    let cases: &[(&[u8], &[u8], usize)] = &[
        (b"a\nb\na\nc\n", b"b\na\nb\nc\n", 3),
        (b"a\nb\nc\n", b"b\nc\na\n", 2),
        (b"a\nb\nc\n", b"c\na\nb\n", 2),
        (b"", b"a\nb\n", 0),
        (b"a\nb\n", b"", 0),
    ];
    let mut scratch = DiffScratch::new();
    let mut lines = Vec::new();

    for &(old, new, equal_count) in cases {
        scratch.diff_into(old, new, &mut lines);
        let actual_equal_count = lines.iter().filter(|line| line.op == DiffOp::Equal).count();
        assert_eq!(actual_equal_count, equal_count);

        let old_rows = lines.iter().filter_map(|row| row.old_line.map(|number| (number, row.text)));
        let new_rows = lines.iter().filter_map(|row| row.new_line.map(|number| (number, row.text)));
        assert!(old_rows.eq(numbered_rows(old)));
        assert!(new_rows.eq(numbered_rows(new)));
    }
}

#[test]
fn repeated_lines_keep_the_tie_break_across_unequal_inputs() {
    let rows = diff(b"a\na\nb\n", b"a\nb\na\na\n");
    let actual: Vec<_> = rows.iter().map(|row| (row.op, row.old_line, row.new_line, row.text)).collect();
    assert_eq!(
        actual,
        [
            (DiffOp::Remove, Some(1), None, b"a".as_slice()),
            (DiffOp::Equal, Some(2), Some(1), b"a"),
            (DiffOp::Equal, Some(3), Some(2), b"b"),
            (DiffOp::Add, None, Some(3), b"a"),
            (DiffOp::Add, None, Some(4), b"a"),
        ],
    );
}

#[test]
fn changed_final_lines_need_no_trailing_newline() {
    let rows = diff(b"head\nold", b"head\nnew");
    let actual: Vec<_> = rows.iter().map(|row| (row.op, row.text)).collect();
    assert_eq!(actual, [(DiffOp::Equal, b"head".as_slice()), (DiffOp::Remove, b"old"), (DiffOp::Add, b"new")]);
}

fn numbered_rows(source: &[u8]) -> impl Iterator<Item = (u32, &[u8])> {
    source
        .split_inclusive(|&byte| byte == b'\n')
        .enumerate()
        .map(|(index, row)| ((index + 1) as u32, &row[..row.len() - 1]))
}

#[test]
fn maximum_context_includes_every_available_diff_row() {
    let preview = diff_preview(b"old\nkeep\n", b"new\nkeep\n", usize::MAX, usize::MAX);
    assert_eq!(
        preview,
        [
            DiffPreviewLine {
                op: DiffPreviewOp::Deletion,
                old_line: Some(1),
                new_line: None,
                text: DiffPreviewText::Source(b"old"),
            },
            DiffPreviewLine {
                op: DiffPreviewOp::Addition,
                old_line: None,
                new_line: Some(1),
                text: DiffPreviewText::Source(b"new"),
            },
            DiffPreviewLine {
                op: DiffPreviewOp::Context,
                old_line: Some(2),
                new_line: Some(2),
                text: DiffPreviewText::Source(b"keep"),
            },
        ],
    );
}

fn assert_fixture(root: &Path, name: &str) {
    let old = fs::read(root.join(format!("{name}.old"))).expect("read old fixture");
    let new = fs::read(root.join(format!("{name}.new"))).expect("read new fixture");
    let golden = fs::read_to_string(root.join(format!("{name}.golden"))).expect("read golden");
    let (expected_full, expected_preview) = parse_golden(&golden);

    let actual_full = diff(&old, &new);
    assert_rows(name, &actual_full, &expected_full);

    let actual_preview = diff_preview(&old, &new, DEFAULT_CONTEXT_LINES, DEFAULT_MAX_DISPLAY_LINES);
    assert_preview_rows(name, &actual_preview, &expected_preview);
}

#[derive(Debug, PartialEq, Eq)]
struct ExpectedRow {
    op: String,
    old_line: Option<u32>,
    new_line: Option<u32>,
    text: Vec<u8>,
}

fn parse_golden(text: &str) -> (Vec<ExpectedRow>, Vec<ExpectedRow>) {
    let mut full = Vec::new();
    let mut preview = Vec::new();
    for raw in text.lines() {
        let raw = raw.trim_end();
        if raw.is_empty() || raw.starts_with('#') {
            continue;
        }
        let (label, fields) = raw.split_once(": ").expect("golden row separator");
        let row = parse_row(fields);
        if label.starts_with("line ") {
            full.push(row);
        } else if label.starts_with("preview ") {
            preview.push(row);
        } else {
            panic!("unknown golden row {label:?}");
        }
    }
    (full, preview)
}

fn parse_row(fields: &str) -> ExpectedRow {
    let mut parts = fields.splitn(4, ' ');
    let op = parts.next().expect("golden op").to_owned();
    let old_line = parse_line_number(parts.next().expect("golden old line"));
    let new_line = parse_line_number(parts.next().expect("golden new line"));
    let text = parts
        .next()
        .expect("golden text")
        .strip_prefix("text=")
        .expect("golden text field")
        .as_bytes()
        .to_owned();
    ExpectedRow {
        op,
        old_line,
        new_line,
        text,
    }
}

fn parse_line_number(field: &str) -> Option<u32> {
    field
        .strip_prefix("old=")
        .or_else(|| field.strip_prefix("new="))
        .and_then(|number| number.parse().ok())
}

fn assert_rows(name: &str, actual: &[DiffLine<'_>], expected: &[ExpectedRow]) {
    assert_eq!(actual.len(), expected.len(), "{name}: full row count");
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert_eq!(
            format!("{:?}", actual.op).to_lowercase(),
            expected.op,
            "{name}: row {index}"
        );
        assert_eq!(
            actual.old_line, expected.old_line,
            "{name}: row {index} old"
        );
        assert_eq!(
            actual.new_line, expected.new_line,
            "{name}: row {index} new"
        );
        assert_eq!(actual.text, expected.text, "{name}: row {index} text");
    }
}

fn assert_preview_rows(name: &str, actual: &[DiffPreviewLine<'_>], expected: &[ExpectedRow]) {
    assert_eq!(actual.len(), expected.len(), "{name}: preview row count");
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        let actual_op = match actual.op {
            DiffPreviewOp::Context => "context",
            DiffPreviewOp::Addition => "addition",
            DiffPreviewOp::Deletion => "deletion",
            DiffPreviewOp::Elision => "elision",
        };
        assert_eq!(actual_op, expected.op, "{name}: preview row {index}");
        assert_eq!(
            actual.old_line, expected.old_line,
            "{name}: preview row {index} old"
        );
        assert_eq!(
            actual.new_line, expected.new_line,
            "{name}: preview row {index} new"
        );
        match actual.text {
            DiffPreviewText::Source(text) => {
                assert_eq!(text, expected.text, "{name}: preview row {index} text")
            }
            DiffPreviewText::Elision => assert_eq!(
                expected.text,
                "⋯".as_bytes(),
                "{name}: preview row {index} elision"
            ),
        }
    }
}
