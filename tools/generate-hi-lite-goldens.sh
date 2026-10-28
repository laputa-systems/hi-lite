#!/bin/sh
set -eu

# Syntect is intentionally a generator-only dependency. This script creates a
# checked-in tool project, and leaves only golden text files in the repository.
# The normal test suite never invokes Cargo with syntect and therefore remains
# dependency-free.

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
manifest="$root/tools/hi-lite-syntect/Cargo.toml"
fixtures="$root/tests/fixtures"
# Entries may use name:token:source so several languages can share one
# family fixture while retaining independent golden files. Each family
# snippet exercises the keyword and comment delimiters its shared rule
# table claims. Clojure delimiters map to brackets; the C token covers
# the brace-family languages, SCSS (no syntect SCSS grammar), and Regex
# (whose syntect grammar has no keyword scopes). XML uses the HTML token
# because it shares the HTML rule table. LaTeX is hand-maintained: its
# `.golden` is an explicit semantic fixture, since syntect scopes the
# command name (`begin`) while hi-lite marks the braced word
# (`document`).
for entry in \
  "bash:bash" "c:c" "css:css" "go:go" \
  "html:html" "javascript:js" "json:json" "makefile:makefile" \
  "markdown:md" "python:py" "rust:rs" "typescript:js" "yaml:yml" \
  "cpp:c:family_c_like" "csharp:c:family_c_like" \
  "batch:c:family_c_like" "groovy:c:family_c_like" \
  "java:c:family_c_like" \
  "objective-c:c:family_c_like" "objective-cpp:c:family_c_like" \
  "php:c:family_c_like" "scala:c:family_c_like" \
  "swift:c:family_c_like" "kotlin:c:family_c_like" \
  "clojure:clojure:family_lisp" "lisp:clojure:family_lisp" \
  "haskell:haskell:family_dash" "lua:haskell:family_dash" \
  "ocaml:haskell:family_dash" \
  "perl:ruby:family_hash" "r:ruby:family_hash" \
  "ruby:ruby:family_hash" \
  "erlang:erlang:erlang" "sql:sql:sql" \
  "regex:c:regex" "xml:html:xml" "scss:c:scss"
do
  name=${entry%%:*}
  remainder=${entry#*:}
  token=${remainder%%:*}
  source=${remainder#*:}
  if [ "$source" = "$remainder" ]; then
    source=$name
  fi
  cargo run --quiet --manifest-path "$manifest" -- \
    "$token" "$fixtures/$source.snippet" "$fixtures/$name.golden"
done
