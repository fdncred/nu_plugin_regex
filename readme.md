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

## Character Classes

| Class | Description |
|-|-|
| `.` | Any character |
| `\d` | Any digit |
| `\D` | Any non-digit |
| `\w` | Any alphanumeric (Latin letters, digits, underscore) |
| `\W` | Any non-alphanumeric (All but \w) |
| `\s` | White space |
| `\S` | Non-white space |
| `\t` | Horizontal tab |
| `\r` | Carriage return |
| `\n` | Line feed |
| `\v` | Vertical tab |
| `\f` | Form feed |
| `^` | The beginning of text (or start-of-line with  |multi-line mode)
| `$` | The end of text (or end-of-line with multi-line mode) |
| `\A` | Only the beginning of text (even with multi-line mode  |enabled)
| `\z` | Only the end of text (even with multi-line mode  |enabled)
| `\b` | A Unicode word boundary (\w on one side and \W, \A,  |or \z on other)
| `\B` | Not a Unicode word boundary |
| `\pN` | One letter name for Unicode character class |
| `\PN` | Negated one-letter name Unicode character class |
| `[xyz]` | Either x, y or z |
| `[^xyz]` | Any character except x, y or z |
| `[a-z]` | Any character in range a-z |
| `[[:^alpha:]]` | Negated (\[^A-Za-z\]) |
| `[[:alnum:]]` |  alphanumeric (\[0-9A-Za-z\]) |
| `[[:alpha:]]` |  alphabetic (\[A-Za-z\]) |
| `[[:ascii:]]` |  ASCII (\[\x00-\x7F\]) |
| `[[:blank:]]` |  blank (\[\t \]) |
| `[[:cntrl:]]` |  control (\[\x00-\x1F\x7F\]) |
| `[[:digit:]]` |  digits (\[0-9\]) |
| `[[:graph:]]` |  graphical (\[!-~\]) |
| `[[:lower:]]` |  lower case (\[a-z\]) |
| `[[:print:]]` |  printable (\[ -~\]) |
| `[[:punct:]]` |  punctuation (\[!-/:-@\[-`{-~\]) |
| `[[:space:]]` |  whitespace (\[\t\n\v\f\r \]) |
| `[[:upper:]]` |  upper case (\[A-Z\]) |
| `[[:word:]]` |  word characters (\[0-9A-Za-z_\]) |
| `[[:xdigit:]]` |  hex digit (\[0-9A-Fa-f\]) |

## Grouping and flags
```
(exp)          numbered capture group (indexed by opening parenthesis)
(?P<name>exp)  named (also numbered) capture group (allowed chars: [_0-9a-zA-Z.\[\]])
(?:exp)        non-capturing group
(?flags)       set flags within current group
(?flags:exp)   set flags for exp (non-capturing)
```
Flags are each a single character. For example, (?x) sets the flag x and (?-x) clears the flag x. Multiple flags can be set or cleared at the same time: (?xy) sets both the x and y flags and (?x-y) sets the x flag and clears the y flag.

All flags are by default disabled unless stated otherwise. They are:
```
i     case-insensitive: letters match both upper and lower case
m     multi-line mode: ^ and $ match begin/end of line
s     allow . to match \n
U     swap the meaning of x* and x*?
u     Unicode support (enabled by default)
x     ignore whitespace and allow line comments (starting with `#`)
```

## Escape Sequences
```
\*          literal *, works for any punctuation character: \.+*?()|[]{}^$
\a          bell (\x07)
\f          form feed (\x0C)
\t          horizontal tab
\n          new line
\r          carriage return
\v          vertical tab (\x0B)
\123        octal character code (up to three digits) (when enabled)
\x7F        hex character code (exactly two digits)
\x{10FFFF}  any hex character code corresponding to a Unicode code point
\u007F      hex character code (exactly four digits)
\u{7F}      any hex character code corresponding to a Unicode code point
\U0000007F  hex character code (exactly eight digits)
\U{7F}      any hex character code corresponding to a Unicode code point

\h : hex digit ([0-9A-Fa-f])
\H : not hex digit ([^0-9A-Fa-f])
\e : escape control character (\x1B)
\K : keep text matched so far out of the overall match
\G : anchor to where the previous match ended
```

## Backreferences
```
\1 : match the exact string that the first capture group matched
\2 : backref to the second capture group, etc
```

## Named Capture Groups
```
(?<name>exp) : match exp, creating capture group named name
\k<name> : match the exact string that the capture group named name matched
(?P<name>exp) : same as (?<name>exp) for compatibility with Python, etc.
(?P=name) : same as \k<name> for compatibility with Python, etc.

Look-around assertions for matching without changing the current position:

(?=exp) : look-ahead, succeeds if exp matches to the right of the current position
(?!exp) : negative look-ahead, succeeds if exp doesn’t match to the right
(?<=exp) : look-behind, succeeds if exp matches to the left of the current position
(?<!exp) : negative look-behind, succeeds if exp doesn’t match to the left
```
More syntax information can be found at the regex crate docs [here](https://docs.rs/regex/1.7.0/regex/#syntax) and at the fancy-regex crate docs [here](https://docs.rs/fancy-regex/0.10.0/fancy_regex/#syntax)