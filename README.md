# Hexcat
[![aur](https://img.shields.io/aur/version/hexcat-bin)](https://aur.archlinux.org/packages/hexcat-bin/)
[![crates.io](https://img.shields.io/crates/v/hexcat.svg)](https://crates.io/crates/hexcat)
[![GitHub](https://img.shields.io/badge/GitHub-Source-green.svg)](https://github.com/tylermackj/hexcat/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

A hex display with Unicode symbols for specials.

## USAGE:
    hexcat [OPTIONS] [FILE]
  
## OPTIONS:
    --width <width>
    -w <width>          Set the number of bytes to show per row (default = 16)

    --group <group size>
    -g <group size>       Set the number of bytes to group together (default = 1)

    --start
    -s <start>          Set the starting byte (default = 0)

    --end
    -e <end>            Set the ending byte (default = end)

    --base
    -b <base>           Set the base to output in (options = 2 | 8) (default = 16)

    --noOffset
    -o                  Hide the address offset

    --noAscii
    -a                  Hide the ascii representation

    --help
    -h              Display the help menu

## NOTES:
### Prefixes and Suffixes
All digit based inputs can be prefixed or suffixed for base notation.

            Binary  Octal   Hex
    Prefix  0b      0o      0x
    Suffix  b       o       x

## RUN/INSTALL:
### Nix
    nix run github:TylerMackj/Hexcat -- <args>
### Arch
    pacman -S hexcat-bin
### Cargo
    cargo install hexcat
