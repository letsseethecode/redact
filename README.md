# REDACT

This is a simple program designed to work like `cat` but to redact sensitive
values from lines.

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

  ## Instructions

  build with `cargo build`
  run with `cargo run < examples.txt`
  