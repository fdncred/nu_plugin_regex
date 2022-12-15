# nu_plugin_regex

This is a nushell plugin to parse regular expressions.

# Examples

## Example 1 - match digits one or more times
```bash
'abc123abc' | regex '\d+'
```
```console
╭───┬───────────┬───────┬───────┬─────╮
│ # │   input   │ match │ begin │ end │
├───┼───────────┼───────┼───────┼─────┤
│ 0 │ abc123abc │ 123   │     3 │   6 │
╰───┴───────────┴───────┴───────┴─────╯
```
## Example 2 - match single character in a list
```bash
'abc123abc' | regex '[a-c]'
```
```console
╭───┬───────────┬───────┬───────┬─────╮
│ # │   input   │ match │ begin │ end │
├───┼───────────┼───────┼───────┼─────┤
│ 0 │ abc123abc │ a     │     0 │   1 │
│ 1 │ abc123abc │ b     │     1 │   2 │
│ 2 │ abc123abc │ c     │     2 │   3 │
│ 3 │ abc123abc │ a     │     6 │   7 │
│ 4 │ abc123abc │ b     │     7 │   8 │
│ 5 │ abc123abc │ c     │     8 │   9 │
╰───┴───────────┴───────┴───────┴─────╯
```
## Example 3 - match any word character one or more times
```bash
'abc123abc' | regex '\w+'
```
```console
╭───┬───────────┬───────────┬───────┬─────╮
│ # │   input   │   match   │ begin │ end │
├───┼───────────┼───────────┼───────┼─────┤
│ 0 │ abc123abc │ abc123abc │     0 │   9 │
╰───┴───────────┴───────────┴───────┴─────╯
```
## Example 4 - match the output of `ls` with a list of characters
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
## Example 5 - match with 2 unnamed capture groups
```bash
'abc123abc' | regex '(\w{3})(\d{3})'
```
```console
╭───┬───────────┬──────────────┬────────┬───────┬─────╮
│ # │   input   │ capture_name │ match  │ begin │ end │
├───┼───────────┼──────────────┼────────┼───────┼─────┤
│ 0 │ abc123abc │ capgrp0      │ abc123 │     0 │   6 │
│ 1 │ abc123abc │ capgrp1      │ abc    │     0 │   3 │
│ 2 │ abc123abc │ capgrp2      │ 123    │     3 │   6 │
╰───┴───────────┴──────────────┴────────┴───────┴─────╯
```
## Example 6 - match with 2 named capture groups
```bash
'abc123abc' | regex '(?<word>\w{3})(?<num>\d{3})'
```
```console
╭───┬───────────┬──────────────┬────────┬───────┬─────╮
│ # │   input   │ capture_name │ match  │ begin │ end │
├───┼───────────┼──────────────┼────────┼───────┼─────┤
│ 0 │ abc123abc │ capgrp0      │ abc123 │     0 │   6 │
│ 1 │ abc123abc │ word         │ abc    │     0 │   3 │
│ 2 │ abc123abc │ num          │ 123    │     3 │   6 │
╰───┴───────────┴──────────────┴────────┴───────┴─────╯
```
## Example 7 - match with 2 named capture groups and 1 unnamed capture group
```bash
'abc123abc' | regex '(?<word>\w{3})(?<num>\d{3})\w'
```
```console
╭───┬───────────┬──────────────┬─────────┬───────┬─────╮
│ # │   input   │ capture_name │  match  │ begin │ end │
├───┼───────────┼──────────────┼─────────┼───────┼─────┤
│ 0 │ abc123abc │ capgrp0      │ abc123a │     0 │   7 │
│ 1 │ abc123abc │ word         │ abc     │     0 │   3 │
│ 2 │ abc123abc │ num          │ 123     │     3 │   6 │
╰───┴───────────┴──────────────┴─────────┴───────┴─────╯
```
## Example 8 - match non-digits, digits, non-digits with unnamed capture groups
```bash
'abc123abc' | regex '(\D+)(\d+)(\D+)'
```
```console
╭───┬───────────┬──────────────┬───────────┬───────┬─────╮
│ # │   input   │ capture_name │   match   │ begin │ end │
├───┼───────────┼──────────────┼───────────┼───────┼─────┤
│ 0 │ abc123abc │ capgrp0      │ abc123abc │     0 │   9 │
│ 1 │ abc123abc │ capgrp1      │ abc       │     0 │   3 │
│ 2 │ abc123abc │ capgrp2      │ 123       │     3 │   6 │
│ 3 │ abc123abc │ capgrp3      │ abc       │     6 │   9 │
╰───┴───────────┴──────────────┴───────────┴───────┴─────╯
```
# Notes

- capture group 0 is always reported when groups are detected. not sure if this is right. this was mainly done for example 7
- results should match https://regex101.com/