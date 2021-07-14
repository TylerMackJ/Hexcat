use crate::base_tools::*;
use std::env;

#[derive(std::cmp::PartialEq)]
pub enum BaseOption { BIN, OCT, HEX }

pub struct Arguments {
    pub path: String,
    pub width: usize,
    pub group: usize,
    pub start: usize,
    pub end: usize,
    pub base: BaseOption,
    pub offset: bool,
    pub ascii: bool,
}

pub fn handle_args(exit: &mut bool) -> Result<Arguments, ()> {
    let mut args: Vec<String> = env::args().collect();
    let mut ret: Arguments = Arguments {
        path: "".to_string(),
        width: 16,
        group: 1,
        start: 0,
        end: usize::MAX,
        base: BaseOption::HEX,
        offset: true,
        ascii: true,
    };

    if args.len() == 1 {
        eprintln!("\x1b[0;31mERROR: No file provided\n\x1b[0;33mUSAGE: hexcat [OPTIONS] [FILE]\nHINT: hexcat --help (-h) for additional help\x1b[0;0m");
        return Err(());
    }

    while args.len() > 1 {
        match args.remove(1) {
            h if h == "-h" || h == "--help" => {
                help();
                *exit = true;
                return Err(());
            },
            w if args.len() > 1 && (w == "-w" || w == "--width") && ret.width == 16 => {
                match parse_with_base(args.remove(1), Parsing::WIDTH) {
                    Ok(w) => ret.width = w,
                    Err(_) => return Err(()),
                }
            },
            g if args.len() > 1 && (g == "-g" || g == "--group") && ret.group == 1 => {
                match parse_with_base(args.remove(1), Parsing::GROUP) {
                    Ok(g) => ret.group = g,
                    Err(_) => return Err(()),
                }
            },
            s if args.len() > 1 && (s == "-s" || s == "--start") && ret.start == 0 => {
                match parse_with_base(args.remove(1), Parsing::START) {
                    Ok(s) => ret.start = s,
                    Err(_) => return Err(()),
                }
            },
            e if args.len() > 1 && (e == "-e" || e == "--end") && ret.end == usize::MAX => {
                match parse_with_base(args.remove(1), Parsing::END) {
                    Ok(e) => ret.end = e,
                    Err(_) => return Err(()),
                }
            },
            b if args.len() > 1 && (b == "-b" || b == "--base") && ret.base == BaseOption::HEX => {
                match parse_with_base(args.remove(1), Parsing::BASE) {
                    Ok(b) => {
                        match b {
                            8 => ret.base = BaseOption::OCT,
                            2 => ret.base = BaseOption::BIN,
                            _ => {
                                eprintln!("\x1b[0;31mERROR: Base must be 2 or 8 (default = 16).\n\x1b[0;33mUSAGE: --base (-b) <base>\x1b[0;0m");
                                return Err(());
                            },
                        }
                    },
                    Err(_) => return Err(()),
                }
            }
            o if (o == "-o" || o == "--noOffset") && ret.offset == true => ret.offset = false,
            a if (a == "-a" || a == "--noAscii") && ret.ascii == true => ret.ascii = false,
            path if ret.path == "" => {
                ret.path = path;
            }
            a => {
                eprintln!("\x1b[0;31mERROR: Argument unknown '{}'\x1b[0;0m", a);
                return Err(());
            }
        }
    }
    Ok(ret)
}

fn help() {
    println!("\x1b[0;32mhexcat\x1b[0;0m\n\
    A hex display with Unicode symbols for specials.\n\
    \n\
    \x1b[0;33mUSAGE:\x1b[0;0m\n\
    \thexcat [OPTIONS] [FILE]\n\
    \n\
    \x1b[0;33mOPTIONS:\x1b[0;0m\n\
    \t\x1b[0;32m--width <width>\n\
    \t-w <width>\x1b[0;0m\t\tSet the number of bytes to show per row (default = 16)\n\
    \n\
    \t\x1b[0;32m--group <grouping>\n\
    \t-g <grouping>\x1b[0;0m\t\tSet the number of bytes to group together (default = 1)\n\
    \n\
    \t\x1b[0;32m--start <start>\n\
    \t-s <start>\x1b[0;0m\t\tSet the starting byte (default = 0)\n\
    \n\
    \t\x1b[0;32m--end <end>\n\
    \t-e <end>\x1b[0;0m\t\tSet the ending byte (default = end)\n\
    \n\
    \t\x1b[0;32m--base <base>\n\
    \t-b <base>\x1b[0;0m\t\tSet base to output in (options = 2 | 8) (default = 16)\n\
    \n\
    \t\x1b[0;32m--noOffset\n\
    \t-o\x1b[0;0m\t\t\tHide the address offset\n\
    \n\
    \t\x1b[0;32m--noAscii\n\
    \t-a\x1b[0;0m\t\t\tHide the ascii representation\n\
    \n\
    \t\x1b[0;32m--help\n\
    \t-h\x1b[0;0m\t\t\tDisplay this menu\n\
    \n\
    \x1b[0;33mNOTES:\x1b[0;0m\n\
    \tAll digit based inputs can be prefixed or suffixed for base notation.\n\tSupported prefixes and suffixes:\n\
    \t\t\x1b[0;33m\tBinary\tOctal\tHex\n\
    \t\t\x1b[0;33mPrefix\x1b[0;32m\t0b\t0o\t0x\n\
    \t\t\x1b[0;33mSuffix\x1b[0;32m\tb\to\tx\x1b[0;0m");
}