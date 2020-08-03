# rut

![Build and test](https://github.com/tupini07/rut/workflows/Build%20and%20test/badge.svg)

The _cut_ clone with regex capabilities. This is a work in progress. It is
currently in a _barely usable_ state.

## Why not just use `sed` and `cut`?

I like Rust's regex implementation and I've always wanted a `cut` implementation that accepts a multi-character _delimiter_.

## TODOs

- refactor
    - refactor reusable code
- examples and explanation in readme
- implement debug mode
- documentation
- better error messages
- write tests?
- publish to crates.io
    - section in readme to install from crates.io
    - version badge

### New features in another version

- ability to read file instead of only being able to read from stdin
- Ability to optionally "grep" incoming lines directly within rut
    - ex: `cat Cargo.toml | rut --grep " = " -r "(.?*) = (.?*)" -t "key: '{{1}}' - value: '{{2}}'"` 
- potentially allow to specify color of output in template?

## Installing

### Directly build and install repository

Clone repo and install with cargo

```bash
cargo install --path .
```

## Regex for text replacement

### Named capture groups

Use the [regex crate's named capture group](https://docs.rs/regex/1.3.9/regex/#example-replacement-with-named-capture-groups) syntax: `(?P<group name>group regex)`.

```bash
cat Cargo.toml | grep = | rut -r "(?P<key>.?*) = (?P<value>.?*)" -t "key: '{{key}}' - value: '{{value}}'"
```

### Positional capture groups

Capture groups can also be referenced positionally (starting from _1_).

```bash
cat Cargo.toml | grep = | rut -r "(.?*) = (.?*)" -t "key: '{{1}}' - value: '{{2}}'"
```

### Using delimiters and fields with an optional join string

```bash
cat Cargo.toml | grep = | rut -f "1-2,1" -d " -- " -j">>"
```
### If no delimiter is provided then `fields` will act on the individual characters

```bash
cat Cargo.toml | grep = | rut -f "-5"
```
