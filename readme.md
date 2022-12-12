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
```
╭───┬───────────┬───────┬─────╮
│ # │   match   │ begin │ end │
├───┼───────────┼───────┼─────┤
│ 0 │ abc123abc │     0 │   9 │
╰───┴───────────┴───────┴─────╯
```

# Notes

- Does not handle capture groups yet