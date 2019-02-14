# ddex

**ddex** (_pronounced dee-decks_) is a toolkit for dealing with `.dex` files. Currently it can provide insight into the contents of `.dex` files, along with basic disassembly functionality.

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

### Examples:

**header** subcommand:
```bash
ddex --input classes.dex header

dex version        035
checksum           1677034034
sha1               54fe1d43e9d9423c0b4c3a4f8f763052987cf717
file size          6779840 bytes
header size        112 bytes
endianness         little endian
link size          0 bytes
link offset        0x0
map offset         0x12d700
string IDs size    54783 bytes
string IDs offset  0x70
type IDs size      7278 bytes
type IDs offset    0x3586c
proto IDs size     10965 bytes
proto IDs offset   0x3ca24
field IDs size     34332 bytes
field IDs offset   0x5cc20
method IDs size    49440 bytes
method IDs offset  0x9fd00
class defs size    5768 bytes
class defs offset  0x100600
data size          5545152 bytes
data offset        0x12d700
```
**disassemble** subcommand:
```bash
ddex --input classes.dex disassemble

0x1ae0f8 701000ae0000 invoke-direct {v0} 0xae00
0x1ae0fe 5b012000     iput-object v1 v0 0x20
0x1ae102 5b022300     iput-object v2 v0 0x23
0x1ae106 0e00         return-void
0x1ae208 701000ae0100 invoke-direct {v1} 0xae00
0x1ae20e 2200d118     new-instance v0 0x18d1
0x1ae212 7010b1b10000 invoke-direct {v0} 0xb1b1
0x1ae218 5b102a00     iput-object v0 v1 0x2a
...
```