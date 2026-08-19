#!/bin/sh
set -eu

# Syntect is intentionally a generator-only dependency. This script creates a
# checked-in tool project, and leaves only golden text files in the repository.
# The normal test suite never invokes Cargo with syntect and therefore remains
# dependency-free.

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
manifest="$root/tools/hi-lite-syntect/Cargo.toml"
fixtures="$root/tests/fixtures"
# Entries may use name:token:source so several canonical languages can share
# the syntax-neutral registry probe while retaining independent golden files.
# The probe uses a C token for generic-family variants: its numeric/string
# spans are stable across the dependency-free rule families, while XML and a
# few grammar-specific cases retain their own syntect token.
for entry in \
  "bash:bash" "c:c" "css:css" "go:go" \
  "html:html" "javascript:js" "json:json" "makefile:makefile" \
  "markdown:md" "python:py" "rust:rs" "typescript:js" "yaml:yml" \
  "cpp:c:language_probe" "csharp:c:language_probe" \
  "scss:c:language_probe" "xml:xml:language_probe" \
  "batch:c:language_probe" "clojure:c:language_probe" \
  "erlang:c:language_probe" "groovy:c:language_probe" \
  "haskell:c:language_probe" "java:c:language_probe" \
  "latex:latex:language_probe" "lisp:c:language_probe" \
  "lua:c:language_probe" \
  "ocaml:c:language_probe" "objective-c:c:language_probe" \
  "objective-cpp:c:language_probe" \
  "perl:c:language_probe" "php:c:language_probe" \
  "r:c:language_probe" "ruby:c:language_probe" \
  "scala:c:language_probe" "sql:c:language_probe" \
  "swift:c:language_probe" "kotlin:c:language_probe" \
  "regex:c:language_probe"
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
