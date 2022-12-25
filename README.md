# Mock
A CLI to convert text into **"MOckinG SPonGebOb CASE"**

## Usage
Default usage
```bash
$ mock "string of text you want to convert"
STRiNg of TExT YOU waNt TO COnVeRT
```

Extra capabilities such as interacting with the clipboard can be found with
```bash
$ mock --help
```

## Building
This command can be exported using
```bash
$ cargo build --release
$ cp target/release/mock /usr/local/bin # on Unix systems
```

## What is This?
This is just a little fun project for myself to apply Rust and learn to use some crates. No one really needs a command to `mock`

It just happened after I was joking with a friend over text and wanted something more efficient than hitting SHIFT every other character