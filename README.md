# Textinator
A CLI to convert text into many formats. Yes it's a play on Dr. Doofenschmirtz's inventions

## Usage
```bash
textinator [subcommand] <options> [STRING]
```

### Subcommands
- `mock` : CONVeRts TExT into ThE mocKINg SPONGEBoB caSe
- `reverse` : txet sesreveR
- `morse` : -.-- --- ..- / -.-. .- ..- --. .... - / .- -. / . .- ... - . .-. / . --. --.

### Options
Capable of interacting with the clipboard, or with files
```bash
# converts string and pastes the conversion to the clipboard
textinator mock -p "my mango is to blow up" 

# converts clipboard contents
textinator reverse -c

# converts clipboard contents and copies it back
textinator morse -cp

# converts file.txt contents
textinator morse -f file.txt
```

## What is This?
This is just a fun little project for myself to apply Rust and learn to use some crates. 

Started with `mock` only, when I was joking with a friend over text and wanted something more efficient than hitting SHIFT every other character. Decided to add more commmands to it later on
