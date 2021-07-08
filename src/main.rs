mod base_tools;

use base_tools::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub struct Arguments {
    pub path: String,
    pub width: usize,
    pub group: usize,
    pub start: usize,
    pub end: usize,
    pub base: u8,
    pub offset: bool,
    pub ascii: bool,
}

static mut EXIT: bool = false;

fn main() -> std::io::Result<()> {
    let args: Arguments = match handle_args() {
        Ok(a) => a,
        Err(_) => return Ok(()),
    };

    if unsafe{ EXIT } {
        return Ok(());
    }
    
    let file: File = match File::open(args.path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("\x1b[0;31mError file not found.\x1b[0;0m");
            return Ok(());
        },
    };
    let start: usize = args.start;
    let end: usize = args.end;
    let content: Vec<u8> = file.bytes().enumerate().filter(|&(i, _)| {
        i >= start && i < end
    }).map(|(_, b)| {
        b.unwrap()
    }).collect();

    for (i, row) in content.chunks(args.width).enumerate() {
        // Print Offset
        if args.offset {
            print!("0x{:08x}:  ", i * args.width + start);
        }

        // Print Hex
        for grouping in row.chunks(args.group) {
            for byte in grouping {
                print!("{:02x}", byte);
            }
            for _ in 0..(args.group - grouping.len()) {
                if i != content.chunks(args.width).len() - 1 {
                    print!("~~");
                } else {
                    print!{"  "};
                }
            }
            print!(" ");
        }
        let mut printed_len = row.len();
        while printed_len % args.group != 0 {
            printed_len += 1;
        }
        let mut total_len = args.width;
        while total_len % args.group != 0 {
            total_len += 1;
        }
        if printed_len <= total_len {
            for _ in 0..((total_len - printed_len) / args.group) {
                for _ in 0..args.group {
                    print!("  ");
                }
                print!(" ");
            }
        }

        // Print Ascii
        if args.ascii {
            print!("  |");
            for byte in row {
                let mut c: char = *byte as char;
                match c {
                    '\0' => c = '␀',
                    '\n' => c = '␤',
                    c if (c as u8) >= 32 && (c as u8) < 127 => {},
                    _ => c = '◆',
                }
                print!("{}", c);
            }
            for _ in 0..(args.width - row.len()) {
                print!(" ");
            }
            print!("|");
        }

        println!("");
    }

    Ok(())
}

pub fn handle_args() -> Result<Arguments, ()> {
    let mut args: Vec<String> = env::args().collect();
    let mut ret: Arguments = Arguments {
        path: "".to_string(),
        width: 16,
        group: 1,
        start: 0,
        end: usize::MAX,
        base: 16,
        offset: true,
        ascii: true,
    };

    while args.len() > 1 {
        match args.remove(1) {
            h if h == "-h" || h == "--help" => {
                help();
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
            o if (o == "-o" || o == "--noOffset") && ret.offset == true => ret.offset = false,
            a if (a == "-a" || a == "--noAscii") && ret.ascii == true => ret.ascii = false,
            path if ret.path == "" => {
                ret.path = path;
            }
            a => {
                eprintln!("\x1b[0;31mError argument unknown '{}'\x1b[0;0m", a);
                return Err(());
            }
        }
    }
    Ok(ret)
}

fn help() {
    unsafe{ EXIT = true };

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