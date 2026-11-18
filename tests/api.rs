use hi_lite::{
    Highlighter, Kind, Language, Run, State, TextPosition, byte_kinds_to_char_kinds_into,
    find_bracket_match, highlight_line_into, runs,
};

#[test]
fn rust_raw_string_delimiters_keep_exact_hash_count_across_lines() {
    let mut highlighter = Highlighter::new(Language::Rust);
    let mut scratch = Vec::new();
    let hashes = "#".repeat(255);
    let opening = format!("r{hashes}\"inside");
    let kinds = highlighter.highlight_into(opening.as_bytes(), &mut scratch);
    assert_eq!(kinds, vec![Kind::String; opening.len()]);

    let almost = format!("\"{} let", &hashes[..254]);
    let kinds = highlighter.highlight_into(almost.as_bytes(), &mut scratch);
    assert_eq!(kinds, vec![Kind::String; almost.len()]);

    let closing = format!("\"{hashes} let");
    let kinds = highlighter.highlight_into(closing.as_bytes(), &mut scratch);
    let split = 1 + hashes.len();
    assert!(kinds[..split].iter().all(|&kind| kind == Kind::String));
    assert_eq!(
        &kinds[split..],
        &[Kind::Normal, Kind::Keyword, Kind::Keyword, Kind::Keyword]
    );

    let too_many = format!("r{hashes}#\"inside");
    let kinds = highlighter.highlight_into(too_many.as_bytes(), &mut scratch);
    assert_eq!(kinds[0], Kind::Normal);
}

#[test]
fn rust_byte_and_raw_prefixes_require_adjacent_letters() {
    let mut highlighter = Highlighter::new(Language::Rust);
    let mut scratch = Vec::new();
    let kinds = highlighter.highlight_into(b"b \"hello\"", &mut scratch);
    assert_eq!(&kinds[..2], &[Kind::Normal, Kind::Normal]);
    assert!(kinds[2..].iter().all(|&kind| kind == Kind::String));

    let line = b"br#\"hello\"#";
    let kinds = highlighter.highlight_into(line, &mut scratch);
    assert_eq!(kinds, vec![Kind::String; line.len()]);
}

#[test]
fn rust_nested_comment_resumes_code_after_adjacent_closers() {
    let mut highlighter = Highlighter::new(Language::Rust);
    let mut scratch = Vec::new();
    let kinds = highlighter.highlight_into(b"/* /**/*/ let", &mut scratch);
    assert!(kinds[..9].iter().all(|&kind| kind == Kind::Comment));
    assert_eq!(kinds[9], Kind::Normal);
    assert_eq!(&kinds[10..], &[Kind::Keyword; 3]);
}

#[test]
fn rust_underscore_lifetime_and_character_have_distinct_kinds() {
    let mut highlighter = Highlighter::new(Language::Rust);
    let mut scratch = Vec::new();
    let kinds = highlighter.highlight_into(b"'_ '_'", &mut scratch);
    assert_eq!(
        kinds,
        &[Kind::Type, Kind::Type, Kind::Normal, Kind::String, Kind::String, Kind::String]
    );
}

#[test]
fn public_bracket_query_only_loads_lines_in_range() {
    let document: &[&[u8]] = &[b"[]"];
    let multibyte: &[&[u8]] = &["é[".as_bytes()];
    let mut scratch = Vec::new();
    for (lines, start, expected) in [
        (&document[..0], TextPosition::new(0, 0), None),
        (document, TextPosition::new(1, 0), None),
        (multibyte, TextPosition::new(0, 3), None),
        (
            document,
            TextPosition::new(0, 0),
            Some(TextPosition::new(0, 1)),
        ),
    ] {
        let mut load = |index: usize, buffer: &mut Vec<u8>| {
            buffer.clear();
            buffer.extend_from_slice(lines[index]);
        };
        assert_eq!(
            find_bracket_match(start, &mut load, &mut scratch, lines.len()),
            expected
        );
    }
}

#[test]
fn public_bracket_query_honors_the_scan_limit_in_both_directions() {
    for (closing_line, found) in [(999, true), (1000, false)] {
        let opening = TextPosition::new(0, 2);
        let closing = TextPosition::new(closing_line, 1);
        let mut scratch = Vec::new();
        let mut load = |line: usize, buffer: &mut Vec<u8>| {
            buffer.clear();
            if line == 0 {
                buffer.extend_from_slice("éλ[".as_bytes());
            } else if line == closing_line {
                buffer.extend_from_slice("λ]".as_bytes());
            }
        };
        for (start, other) in [(opening, closing), (closing, opening)] {
            assert_eq!(
                find_bracket_match(start, &mut load, &mut scratch, closing_line + 1),
                found.then_some(other),
                "query from {start:?} with closing line {closing_line}"
            );
        }
    }
}

#[test]
fn public_api_streams_opaque_state_and_reuses_output() {
    let mut highlighter = Highlighter::new(Language::Rust);
    let mut output = Vec::new();

    let first = highlighter.highlight_into(b"/* start", &mut output);
    assert!(first.iter().all(|&kind| kind == Kind::Comment));
    assert!(!highlighter.state().is_normal());

    let second = highlighter.highlight_into(b"end */ let", &mut output);
    assert_eq!(second[8], Kind::Keyword);
    assert!(highlighter.state().is_normal());

    highlighter.reset();
    assert_eq!(highlighter.state(), State::default());
}

#[test]
fn public_api_clears_reused_slots_between_equal_length_lines() {
    let mut highlighter = Highlighter::new(Language::Rust);
    let mut output = Vec::new();

    let first = highlighter.highlight_into(b"//x", &mut output);
    assert!(first.iter().all(|&kind| kind == Kind::Comment));

    let second = highlighter.highlight_into(b"let", &mut output);
    assert!(second.iter().all(|&kind| kind == Kind::Keyword));
}

#[test]
fn public_api_supports_stateless_lines_and_runs() {
    let line = b"let x = 1";
    let mut kinds = vec![Kind::Normal; line.len()];
    assert!(highlight_line_into(Language::Rust, State::default(), line, &mut kinds).is_normal());
    assert_eq!(kinds[0], Kind::Keyword);

    let grouped: Vec<_> = runs(&[Kind::Keyword, Kind::Keyword, Kind::Normal]).collect();
    assert_eq!(
        grouped,
        vec![
            Run {
                start: 0,
                end: 2,
                kind: Kind::Keyword,
            },
            Run {
                start: 2,
                end: 3,
                kind: Kind::Normal,
            },
        ]
    );
}

#[test]
fn public_display_mapping_preserves_kinds_across_tabs_and_utf8() {
    let mut display = vec![Kind::Normal; 8];
    byte_kinds_to_char_kinds_into(
        "a\té".as_bytes(),
        &[Kind::Keyword, Kind::Comment, Kind::String, Kind::Normal],
        &mut display,
    );
    assert_eq!(
        display,
        [Kind::Keyword, Kind::Comment, Kind::Comment, Kind::String]
    );

    byte_kinds_to_char_kinds_into(b"\t1", &[Kind::String, Kind::Number], &mut display);
    assert_eq!(display, [Kind::String, Kind::String, Kind::Number]);

    byte_kinds_to_char_kinds_into(b"1", &[Kind::Number], &mut display);
    assert_eq!(display, [Kind::Number]);
}

#[test]
fn language_aliases_are_small_and_explicit() {
    assert_eq!(Language::from_name("rs"), Some(Language::Rust));
    assert_eq!(Language::from_name("shell"), Some(Language::Bash));
    assert_eq!(Language::from_file_ext(".md"), Some(Language::Markdown));
    assert_eq!(Language::from_name("unknown"), None);

    for name in [
        "actionscript",
        "applescript",
        "d",
        "elm",
        "graphviz",
        "less",
        "matlab",
        "pascal",
        "tcl",
    ] {
        assert_eq!(Language::from_name(name), None, "removed language: {name}");
    }
}

#[test]
fn language_detection_owns_file_extension_and_shebang_mapping() {
    assert_eq!(Language::from_file_ext("rs"), Some(Language::Rust));
    assert_eq!(Language::from_file_ext("css.erb"), Some(Language::Css));
    assert_eq!(
        Language::from_file_ext("Dockerfile"),
        Some(Language::Dockerfile)
    );
    assert_eq!(
        Language::from_file_ext("GNUmakefile"),
        Some(Language::Makefile)
    );
    assert_eq!(Language::from_file_ext("xsh"), Some(Language::Xsh));
    assert_eq!(Language::from_file_ext("txt"), None);

    assert_eq!(
        Language::from_shebang(b"#!/usr/bin/python3.11"),
        Some(Language::Python)
    );
    assert_eq!(
        Language::from_shebang(b"#!/usr/bin/env -S python3 -O"),
        Some(Language::Python)
    );
    assert_eq!(
        Language::from_shebang(b"#!/usr/bin/node"),
        Some(Language::JavaScript)
    );
    assert_eq!(
        Language::from_shebang(b"#!/usr/bin/env xshi"),
        Some(Language::Xsh)
    );
    assert_eq!(Language::from_shebang(b"not a shebang"), None);
    assert_eq!(
        Language::detect(Some("unknown"), b"#!/bin/bash"),
        Some(Language::Bash)
    );
    assert_eq!(
        Language::detect(Some("rs"), b"#!/bin/bash"),
        Some(Language::Rust)
    );
}

#[test]
fn shebang_uses_the_interpreter_basename_and_ascii_separators() {
    for (line, expected) in [
        (b"#! /bin/sh\r\n".as_slice(), Some(Language::Bash)),
        (b"#!/usr/bin/env\t-S\tpython3.15.1 -B", Some(Language::Python)),
        (b"#!/usr/bin/env -S /opt/bin/node --version", Some(Language::JavaScript)),
        (b"#! /opt/\xff/bin/xsht", Some(Language::Xsh)),
        (b"#!/usr/bin/env -S", None),
        (b"#!/opt/bin/", None),
        (b"#!/opt/bin/launcher python3", None),
    ] {
        assert_eq!(Language::detect(None, line), expected, "{line:?}");
    }
}

#[test]
fn language_detection_preserves_the_case_sensitive_objective_c_extensions() {
    assert_eq!(Language::from_file_ext(".m"), Some(Language::ObjectiveC));
    assert_eq!(Language::from_file_ext(".M"), Some(Language::ObjectiveCpp));
    assert_eq!(Language::from_file_ext("MM"), Some(Language::ObjectiveCpp));
}

#[test]
fn state_checkpoints_do_not_cross_languages() {
    let mut python = Highlighter::new(Language::Python);
    let mut scratch = Vec::new();
    python.highlight_into(b"\"\"\"open", &mut scratch);
    assert!(!python.state().is_normal());
    let checkpoint = python.state();

    let mut latex = Highlighter::new(Language::LaTeX);
    latex.set_state(checkpoint);
    assert!(
        latex.state().is_normal(),
        "a Python checkpoint must not resume a LaTeX stream"
    );
    let kinds = latex.highlight_into(b"begin{document}", &mut scratch);
    assert_eq!(kinds[0], Kind::Keyword);

    let mut python_resume = Highlighter::new(Language::Python);
    python_resume.set_state(checkpoint);
    assert!(!python_resume.state().is_normal());
    let continued = python_resume.highlight_into(b"still open", &mut scratch);
    assert!(continued.iter().all(|&kind| kind == Kind::String));
}

#[test]
fn language_comment_delimiters_are_canonical() {
    assert_eq!(Language::Rust.comment(), "//");
    assert_eq!(Language::Python.comment(), "#");
    assert_eq!(Language::Html.comment(), "<!--");
    assert_eq!(Language::Markdown.comment(), "<!--");
    assert_eq!(Language::Css.comment(), "/*");
    assert_eq!(Language::Regex.comment(), "");
}

#[test]
fn css_and_scss_block_comments_cross_lines() {
    for language in [Language::Css, Language::Scss] {
        let mut highlighter = Highlighter::new(language);
        let mut output = Vec::new();

        let open = highlighter.highlight_into(b"/* open", &mut output);
        assert!(
            open.iter().all(|&kind| kind == Kind::Comment),
            "{language:?} open"
        );
        assert!(!highlighter.state().is_normal(), "{language:?} open state");

        let continued = highlighter.highlight_into(b"color: red;", &mut output);
        assert!(
            continued.iter().all(|&kind| kind == Kind::Comment),
            "{language:?} continuation"
        );
        assert!(
            !highlighter.state().is_normal(),
            "{language:?} continuation state"
        );

        let close = highlighter.highlight_into(b"still */", &mut output);
        assert!(
            close.iter().all(|&kind| kind == Kind::Comment),
            "{language:?} close"
        );
        assert!(highlighter.state().is_normal(), "{language:?} close state");
    }
}

#[test]
fn comment_delimiter_matches_the_lexer() {
    assert_comment_line(Language::Scss, b"// line comment");
    assert_eq!(Language::Ocaml.comment(), "--");
    assert_comment_line(Language::Ocaml, b"-- dash");
    assert_eq!(Language::Batch.comment(), "//");
    assert_comment_line(Language::Batch, b"// slash");
    assert_eq!(Language::Regex.comment(), "");
    let mut highlighter = Highlighter::new(Language::Regex);
    let mut output = Vec::new();
    let kinds = highlighter.highlight_into(b"// slash", &mut output);
    assert!(
        kinds.iter().all(|&kind| kind != Kind::Comment),
        "regex highlighted a C line comment"
    );
}

fn assert_comment_line(language: Language, line: &[u8]) {
    assert!(
        line.starts_with(language.comment().as_bytes()),
        "{language:?} comment delimiter"
    );
    let mut highlighter = Highlighter::new(language);
    let mut output = Vec::new();
    let kinds = highlighter.highlight_into(line, &mut output);
    assert!(
        kinds.iter().all(|&kind| kind == Kind::Comment),
        "{language:?} lexer comment"
    );
}

#[test]
fn syntect_programming_syntax_names_have_dependency_free_lexers() {
    // Keep this list aligned with the programming grammars in syntect's
    // default SyntaxSet. Embedded/template syntaxes intentionally resolve to
    // their reusable host lexer (for example JSP to HTML and SQL (Rails) to
    // SQL).
    let names = [
        "ASP",
        "HTML (ASP)",
        "Batch File",
        "C#",
        "C++",
        "C",
        "Clojure",
        "Erlang",
        "HTML (Erlang)",
        "Go",
        "Groovy",
        "HTML",
        "Haskell",
        "Literate Haskell",
        "Java Server Page (JSP)",
        "Java",
        "JavaDoc",
        "JSON",
        "Regular Expressions (Javascript)",
        "BibTeX",
        "LaTeX Log",
        "LaTeX",
        "TeX",
        "Lisp",
        "Lua",
        "Makefile",
        "Markdown",
        "MultiMarkdown",
        "OCaml",
        "OCamllex",
        "OCamlyacc",
        "camlp4",
        "Objective-C++",
        "Objective-C",
        "PHP Source",
        "PHP",
        "Perl",
        "Python",
        "Regular Expressions (Python)",
        "R Console",
        "R",
        "Rd (R Documentation)",
        "HTML (Rails)",
        "JavaScript (Rails)",
        "Ruby Haml",
        "Ruby on Rails",
        "SQL (Rails)",
        "Regular Expression",
        "reStructuredText",
        "Ruby",
        "Rust",
        "SQL",
        "Scala",
        "Bourne Again Shell (bash)",
        "Shell-Unix-Generic",
        "commands-builtin-shell-bash",
        "HTML (Tcl)",
        "Textile",
        "XML",
        "YAML",
    ];
    for name in names {
        assert!(
            Language::from_name(name).is_some(),
            "missing syntax: {name}"
        );
    }
}

#[test]
fn generic_language_families_share_the_common_scanner() {
    let cases = [
        (
            Language::Java,
            b"public int answer() { return 42; }".as_slice(),
            0,
            Kind::Keyword,
        ),
        (
            Language::Ruby,
            b"value = 42 # comment".as_slice(),
            8,
            Kind::Number,
        ),
        (
            Language::Haskell,
            b"value = 42 -- comment".as_slice(),
            8,
            Kind::Number,
        ),
        (
            Language::Sql,
            b"select count from users where id = 42".as_slice(),
            0,
            Kind::Keyword,
        ),
    ];
    for (language, line, offset, expected) in cases {
        let mut highlighter = Highlighter::new(language);
        let mut scratch = Vec::new();
        let kinds = highlighter.highlight_into(line, &mut scratch);
        assert_eq!(kinds[offset], expected, "{}: {line:?}", language.name());
    }
}

#[test]
fn every_public_language_round_trips_its_canonical_name() {
    let languages = [
        Language::Rust,
        Language::Python,
        Language::Go,
        Language::JavaScript,
        Language::TypeScript,
        Language::Bash,
        Language::C,
        Language::Cpp,
        Language::CSharp,
        Language::Json,
        Language::Yaml,
        Language::Toml,
        Language::Ini,
        Language::Makefile,
        Language::Html,
        Language::Css,
        Language::Scss,
        Language::Dockerfile,
        Language::Markdown,
        Language::Xml,
        Language::Batch,
        Language::Clojure,
        Language::Erlang,
        Language::Groovy,
        Language::Haskell,
        Language::Java,
        Language::LaTeX,
        Language::Lisp,
        Language::Lua,
        Language::Ocaml,
        Language::ObjectiveC,
        Language::ObjectiveCpp,
        Language::Perl,
        Language::Php,
        Language::R,
        Language::Ruby,
        Language::Scala,
        Language::Sql,
        Language::Swift,
        Language::Kotlin,
        Language::Regex,
        Language::Xsh,
    ];
    for language in languages {
        assert_eq!(Language::from_name(language.name()), Some(language));
    }
}

#[test]
fn syntect_extensions_resolve_to_host_or_generic_lexers() {
    let cases = [
        (".cpp", Language::Cpp),
        (".csx", Language::CSharp),
        ("GNUmakefile", Language::Makefile),
        (".asa", Language::Html),
        (".gradle", Language::Groovy),
        (".lhs", Language::Haskell),
        (".jsp", Language::Html),
        (".mli", Language::Ocaml),
        (".mm", Language::ObjectiveCpp),
        (".phtml", Language::Php),
        (".rst", Language::Markdown),
        (".sql.erb", Language::Sql),
        (".svg", Language::Xml),
        (".textile", Language::Markdown),
    ];
    for (extension, language) in cases {
        assert_eq!(
            Language::from_file_ext(extension),
            Some(language),
            "{extension}"
        );
    }
    for extension in [".txt", ".diff", ".patch", ".build"] {
        assert_eq!(
            Language::from_file_ext(extension),
            None,
            "removed plain text mode: {extension}"
        );
    }
}
