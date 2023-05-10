# Rust cat

A Rust implementation of the cat command line tool.

## Introduciton

The `cater` command writes the contents of a file, or files, to the standard output in command-line order. `cater` takes the relative path of a file, or files, as an argument(s). This implementation includes four flags:

`-h, --help`  Prints help information.

`-n, --number`  Numbers each line of standard output begining at 1.

`-b, --number-nonblank`  Numbers all nonblank lines written to the standard output.

`-v, --version`  Prints version information.

## Example

Here we have a local document stored in the same directory as `cater` named `test.txt`. To run `cater` with `test.txt` as an argument:

```bash
cargo run cater -- ./test.txt
```

To number all lines of the output:

```bash
cargo run cater -- ./test.txt -n
```

To number only nonblank lines of the output:

```bash
cargo run cater -- ./test.txt -b
```