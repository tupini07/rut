# rut

The _cut_ clone with regex capabilities.

## Why not just use `sed` and `cut`?

I like Rust's regex implementation and I've always wanted a `cut` implementation that accepts a multi-character _delimiter_.

## Installing

### Directly build and install repository

Clone repo and install with cargo

```bash
cargo install --path .
```

### Regex for text replacement

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