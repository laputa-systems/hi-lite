# HIL-3: One comment delimiter per language

Status: draft

## Summary

`Language::comment` and `Language::rules` name different comment delimiters for SCSS, OCaml, Batch, and Regex. Editor integrations that insert `comment()` and the highlighter that reads `rules()` cannot agree. Pick one owner for the delimiter, and decide which of these variants are real grammars.

## Motivation

`comment()` is documented as the line-comment delimiter editor integrations use. The highlighter never reads it. It reads `RuleSet::line_comment` and `RuleSet::block_comment`.

A probe against the public API shows the split:

- SCSS `comment()` is `//`. The lexer is `CSS_RULES`, whose line comment is empty, so `// line comment` stays `Normal`.
- OCaml `comment()` is `//` only because of the default arm. The lexer is `DASH_RULES`, so `-- dash` is a comment and `(*` is not.
- Batch `comment()` is `#`. The lexer is `C_LIKE_RULES`, so `//` is a comment and `#` is not. `REM` is an uppercase constant, not a comment.
- Regex `comment()` is empty. The lexer is `C_LIKE_RULES`, so `//` is a comment.

HIL-1 is separate. Even after delimiters match, CSS still drops an unclosed `/*` at the end of the line.

## Design

Make `RuleSet` the only delimiter table. `comment()` returns that table's line-comment string, or the block opener when the language has no line comment (HTML, XML, Markdown, CSS).

Give SCSS its own rule set copied from CSS with `line_comment: "//"`. SCSS exists as a variant because `//` comments are the difference a caller can see. Sharing `CSS_RULES` deletes that difference.

Leave OCaml, Batch, and Regex on their family tables until you choose otherwise. After `comment()` reads the table, OCaml reports `--`, Batch reports `//`, and Regex reports `//`. That is still the wrong human syntax for those languages. It is at least one syntax.

## Alternatives considered

Return the current `comment()` strings and change the lexers to match them. SCSS would gain `//`, which is the right SCSS fix, but OCaml would highlight `//` and Batch would highlight `#`. Neither is the language's real comment, and the rest of those lexers would stay on the family keyword list.

Delete `comment()` and make callers read the rule table. The table is crate-private, and editors already call `comment()`.

## Open questions

- Should OCaml leave `DASH_RULES` and use `(*` `*)`? Recommendation: not until a snippet needs it. Sync `comment()` to the family table now.
- Should Batch leave `C_LIKE_RULES`? Recommendation: same. `REM` and `::` are a different lexer, not a one-line delimiter change.
- Should Regex stop using C line comments? Recommendation: yes if Regex stays a public language. An empty `comment()` and a lexer with no `//` match the name better than C comments.

## Decision

Unset.
