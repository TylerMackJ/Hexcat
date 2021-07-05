use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Arguments {
    path: String,
    width: usize,
    group: usize,
    start: usize,
    end: usize,
    offset: bool,
    ascii: bool,
}

static mut EXIT: bool = false;

fn main() -> std::io::Result<()> {
    let args: Arguments = handle_args();

    if unsafe{ EXIT } {
        return Ok(());
    }
    
    let file: File = match File::open(args.path) {
        Ok(f) => f,
        Err(_) => panic!("\x1b[0;31mError file not found.\x1b[0;0m"),
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

fn handle_args() -> Arguments {
    let mut args: Vec<String> = env::args().collect();
    let mut ret: Arguments = Arguments {
        path: "".to_string(),
        width: 16,
        group: 1,
        start: 0,
        end: usize::MAX,
        offset: true,
        ascii: true,
    };

    while args.len() > 1 {
        match args.remove(1) {
            h if h == "-h" => help(),
            w if args.len() > 1 && w == "-w" && ret.width == 16 => {
                match args.remove(1).parse::<isize>() {
                    Ok(w) if w <= 0 => panic!("\x1b[0;31mError width must be positive.\n\x1b[0;33mUSAGE: -w <width>\x1b[0;0m"),
                    Ok(w) => ret.width = w as usize,
                    Err(_) => panic!("\x1b[0;31mError width undefined.\n\x1b[0;33mUSAGE: -w <width>\x1b[0;0m"),
                }
            },
            g if args.len() > 1 && g == "-g" && ret.group == 1 => {
                match args.remove(1).parse::<isize>() {
                    Ok(g) if g <= 0 => panic!("\x1b[0;31mError group size must be positive.\n\x1b[0;33mUSAGE: -g <group>\x1b[0;0m"),
                    Ok(g) => ret.group = g as usize,
                    Err(_) => panic!("\x1b[0;31mError group size undefined.\n\x1b[0;33mUSAGE: -g <group>\x1b[0;0m"),
                }
            },
            s if args.len() > 1 && s == "-s" && ret.start == 0 => {
                match args.remove(1).parse::<isize>() {
                    Ok(s) if s <= 0 => panic!("\x1b[0;31mError starting position must be positive.\n\x1b[0;33mUSAGE: -s <start>\x1b[0;0m"),
                    Ok(s) => ret.start = s as usize,
                    Err(_) => panic!("\x1b[0;31mError starting position undefined.\n\x1b[0;33mUSAGE: -w <start>\x1b[0;0m"),
                }
            },
            e if args.len() > 1 && e == "-e" && ret.end == usize::MAX => {
                match args.remove(1).parse::<isize>() {
                    Ok(e) if e <= 0 => panic!("\x1b[0;31mError ending position must be positive.\n\x1b[0;33mUSAGE: -e <end>\x1b[0;0m"),
                    Ok(e) => ret.end = e as usize,
                    Err(_) => panic!("\x1b[0;31mError ending position undefined.\n\x1b[0;33mUSAGE: -e <end>\x1b[0;0m"),
                }
            },
            o if o == "-o" && ret.offset == true => ret.offset = false,
            a if a == "-a" && ret.ascii == true => ret.ascii = false,
            path if ret.path == "" => {
                ret.path = path;
            }
            a => {
                panic!("\x1b[0;31mError argument unknown '{}'\x1b[0;0m", a);
            }
        }
    }

    ret
}

fn help() {
    unsafe{ EXIT = true };

    println!("\x1b[0;32mhexcat\x1b[0;0m\n\
    A hex display with Unicode symbols for specials.\n\
    \n\
    \x1b[0;33mUSAGE:\x1b[0;0m\n\
    \t\thexcat [OPTIONS] [FILE]\n\
    \n\
    \x1b[0;33mOPTIONS:\x1b[0;0m\n\
    \t\t\x1b[0;32m-w <width>\x1b[0;0m\t\tSet the number of bytes to show per row (default = 16)\n\
    \t\t\x1b[0;32m-g <grouping>\x1b[0;0m\t\tSet the number of bytes to group together (default = 1)\n\
    \t\t\x1b[0;32m-s <start>\x1b[0;0m\t\tSet the starting byte (default = 0)\n\
    \t\t\x1b[0;32m-e <end>\x1b[0;0m\t\tSet the ending byte (default = end)\n\
    \t\t\x1b[0;32m-o\x1b[0;0m\t\t\tHide the address offset\n\
    \t\t\x1b[0;32m-a\x1b[0;0m\t\t\tHide the asciee representation\n\
    \t\t\x1b[0;32m-h\x1b[0;0m\t\t\tDisplay this menu");
    
}