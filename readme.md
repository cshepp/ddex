# ddex

**ddex** (_pronounced dee-decks_) is a toolset for dealing with `.dex` files. Currently it can provide insight into the contents of `.dex` files, along with basic disassembly functionality.

``` bash
USAGE:
    ddex --input <FILE> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <FILE>

SUBCOMMANDS:
    classes        Prints the names of the classes contained within the dex file
    disassemble    Disassembles the app and prints the results
    header         Prints header information from the dex file
    help           Prints this message or the help of the given subcommand(s)
    strings        Prints the strings contained within the dex file
    types          Prints the names of the types contained within the dex file
```

Example:
```bash
ddex --input classes.dex disassemble
```