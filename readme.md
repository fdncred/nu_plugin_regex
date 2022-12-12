# nu_plugin_regex

This is a nushell plugin to parse regular expressions.

# Examples

```bash
'abc123abc' | regex '\d+'
```
```console
╭───┬───────┬───────┬─────╮
│ # │ match │ begin │ end │
├───┼───────┼───────┼─────┤
│ 0 │ 123   │     3 │   6 │
╰───┴───────┴───────┴─────╯
```

```bash
'abc123abc' | regex '[a-c]'
```
```console
╭───┬───────┬───────┬─────╮
│ # │ match │ begin │ end │
├───┼───────┼───────┼─────┤
│ 0 │ a     │     0 │   1 │
│ 1 │ b     │     1 │   2 │
│ 2 │ c     │     2 │   3 │
│ 3 │ a     │     6 │   7 │
│ 4 │ b     │     7 │   8 │
│ 5 │ c     │     8 │   9 │
╰───┴───────┴───────┴─────╯
```
```bash
'abc123abc' | regex '\w+'
```
```console
╭───┬───────────┬───────┬─────╮
│ # │   match   │ begin │ end │
├───┼───────────┼───────┼─────┤
│ 0 │ abc123abc │     0 │   9 │
╰───┴───────────┴───────┴─────╯
```

```base
ls | each {|e| $e.name | regex '[Cc]'} | flatten
```
```console
╭────┬─────────────────────────────┬───────┬───────┬─────╮
│  # │            input            │ match │ begin │ end │
├────┼─────────────────────────────┼───────┼───────┼─────┤
│  0 │ .cargo                      │ c     │     1 │   2 │
│  1 │ .vscode                     │ c     │     3 │   4 │
│  2 │ CODE_OF_CONDUCT.md          │ C     │     0 │   1 │
│  3 │ CODE_OF_CONDUCT.md          │ C     │     8 │   9 │
│  4 │ CODE_OF_CONDUCT.md          │ C     │    13 │  14 │
│  5 │ CONTRIBUTING.md             │ C     │     0 │   1 │
│  6 │ Cargo.lock                  │ C     │     0 │   1 │
│  7 │ Cargo.lock                  │ c     │     8 │   9 │
│  8 │ Cargo.toml                  │ C     │     0 │   1 │
│  9 │ LICENSE                     │ C     │     2 │   3 │
│ 10 │ VSWorkspaceSettings.json    │ c     │     9 │  10 │
│ 11 │ build-all-maclin.sh         │ c     │    12 │  13 │
│ 12 │ build-all-windows.cmd       │ c     │    18 │  19 │
│ 13 │ crates                      │ c     │     0 │   1 │
│ 14 │ docker                      │ c     │     2 │   3 │
│ 15 │ docs                        │ c     │     2 │   3 │
│ 16 │ rust-toolchain.toml         │ c     │     9 │  10 │
│ 17 │ src                         │ c     │     2 │   3 │
│ 18 │ unstable_cargo_features.txt │ c     │     9 │  10 │
╰────┴─────────────────────────────┴───────┴───────┴─────╯
```

# Notes

- Does not handle capture groups yet