# rut

The _cut_ clone with regex capabilities.

### Regex for text replacement

```bash
cat Cargo.toml | grep = | rut -r "(.?*) = (.?*)" -t "key: '{{1}}' - value: '{{2}}'"
```

### Using delimiters and fields with an optional join string

```bash
cat Cargo.toml | grep = | rut -f "1-2,1" -d " -- " -j">>"
```