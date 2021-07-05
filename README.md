# Hexcat
A hex display with Unicode symbols for specials.

![crates.io](https://img.shields.io/crates/v/hexcat.svg)
[![GitHub](https://img.shields.io/badge/GitHub-Source-green.svg)](https://github.com/tylermackj/hexcat/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)



#### USAGE:
    hexcat [OPTIONS] [FILE]
  
#### OPTIONS:
    -w <width>      Set the number of bytes to show per row (default = 16)
    -g <grouping>   Set the number of bytes to group together (default = 1)
    -s <start>      Set the starting byte (default = 0)
    -e <end>        Set the ending byte (default = end)
    -o              Hide the address offset
    -a              Hide the ascii representation
    -h              Display the help menu

#### INSTALL:
    cargo install hexcat