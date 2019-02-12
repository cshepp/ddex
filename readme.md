# ddex

**ddex** (_pronounced dee-decks_) is a toolset for dealing with `.dex` files. Currently it can provide basic insight into the contents of `.dex` files, with the future goal of disassembly and decompliation functionality (and maybe a GUI).

``` bash
USAGE:
    ddex <FILE> [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>

SUBCOMMANDS:
    classes    Prints the names of the classes contained within the dex file
    header     Prints header information from the dex file
    help       Prints this message or the help of the given subcommand(s)
    strings    Prints the strings contained within the dex file
    types      Prints the names of the types contained within the dex file
```