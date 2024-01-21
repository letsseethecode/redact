# REDACT

This is a simple program designed to work like `cat` but to redact sensitive
values from lines.

## Building

`cargo build`

## Executing

`cargo run` or `./target/debug/redact`

And content can be passed in via different ways:

1. pipe STDOUT `echo "Foo\nBar\nMeep\n" | redact`
2. from files, by either
   1. load a file directly `redact examples.txt`
   2. redirect a file to STDIN `redact < examples.txt`

## Help

`cargo run -- --help` or `./target/debug/redact --help`

```
Redact the sensitive contents of a file / stdin

Usage: redact [OPTIONS] [INPUT]

Arguments:
  [INPUT]  The file to process

Options:
  -m, --mask <MASK>    The text to appear in the output in place of redacted values [default: <REDACTED>]
  -r, --rules <RULES>  The input file containing patterns to redact [default: .redact]
  -h, --help           Print help
  ```

  ## Example
  
  run with `cargo run < examples.txt` or `./target/debug/redact < examples.txt`

  ```
  Foo
  Bar
  Meep
  password: <REDACTED>
  ```