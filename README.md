# typez

![Build Badge](https://img.shields.io/github/check-runs/Pistonite/typez/main)
![Version Badge](https://img.shields.io/crates/v/typez)
![License Badge](https://img.shields.io/github/license/Pistonite/typez)
![Issue Badge](https://img.shields.io/github/issues/Pistonite/typez)

CLI for generating ASCII art with Sub-Zero font

```bash
typez typez -SS
```
```
 ______ __  __ ______ ______ ______  
/\__  _\\ \_\ \\  == \\  ___\\___  \ 
\/_/\ \/ \____ \\  _-/ \  __\/_/  /__
   \ \_\\/\_____\\_\  \ \_____\\_____\
    \/_/ \/_____//_/   \/_____//_____/
```

## Credit
```
-> Conversion to FigLet font by MEPH. (Part of ASCII Editor Service Pack I)
(http://studenten.freepage.de/meph/ascii/ascii/editor/_index.htm)
-> Defined: ASCII code alphabet
-> Uppercase characters only.

ScarecrowsASCIIArtArchive1.0.txt
From: "Sub-Zero" <bodom@papaya.ucs.indiana.edu>
"Here's a font I've been working on lately. Can someone make the V, Q, and X
look better? Also, the B, P, and R could use an improvement too.
Oh, here it is."
```

## Installation
```bash
cargo install typez
```

## Usage
Type `typez -h` for help.

The program is very simple. It does not come with line break logic.
If the output is longer than the terminal width, it will display incorrectly.
You should line-break the input yourself by inserting `\n` in your input string.

Inputs are parsed as UTF-8 string:
- `0x10` (Line Feed) is treated as a line break.
- Other invisible characters (`0x00-0x20, 0x7F, 0xFF`) are skipped
- Lower case `a-z` are converted to upper case.
- Upper case `A-Z` will be printed
- Other characters are replaced with space

To generate from command line input:
```bash
typez "Hello"
```
```
 __  __     ______     __         __         ______  
/\ \_\ \   /\  ___\   /\ \       /\ \       /\  __ \ 
\ \  __ \  \ \  __\   \ \ \____  \ \ \____  \ \ \/\ \
 \ \_\ \_\  \ \_____\  \ \_____\  \ \_____\  \ \_____\
  \/_/\/_/   \/_____/   \/_____/   \/_____/   \/_____/
```

To generate from a file:
```bash
typez -i /path/to/file.txt
# use - to use stdin
typez -i - < /path/to/file.txt
```

Other CLI options:
```
-s/--space   N   Number of spaces for the space character (default: 5)
-b/--between N   Number of spaces between characters (default: 2)
-S/--squash      Squash the characters
```

`--squash` will squash the characters together, `--squash --squash` or `-SS` will
squash them even further :)
