use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Arguments {
    path: String,
    width: Option<String>,
    group: Option<String>,
    offset: bool,
    ascii: bool,
}

static mut EXIT: bool = false;

fn main() -> std::io::Result<()> {
    let args: Arguments = handle_args();

    if unsafe{ EXIT } {
        return Ok(());
    }
    
    let file: File = File::open(args.path)?;
    let content: Vec<u8> = file.bytes().map(|b| {
        b.unwrap()
    }).collect();

    let width: usize;
    match args.width {
        Some(w) => width = w.parse::<usize>().unwrap(),
        None => width = 16,
    }
    let group: usize;
    match args.group {
        Some(g) => group = g.parse::<usize>().unwrap(),
        None => group = 1,
    }

    for (i, row) in content.chunks(width).enumerate() {
        // Print Offset
        if args.offset {
            print!("0x{:08x}:  ", i * width);
        }

        // Print Hex
        for grouping in row.chunks(group) {
            for byte in grouping {
                print!("{:02x}", byte);
            }
            for _ in 0..(group - grouping.len()) {
                if i != content.chunks(width).len() - 1 {
                    print!("~~");
                } else {
                    print!{"  "};
                }
            }
            print!(" ");
        }
        let mut printed_len = row.len();
        while printed_len % group != 0 {
            printed_len += 1;
        }
        let mut total_len = width;
        while total_len % group != 0 {
            total_len += 1;
        }
        if printed_len <= total_len {
            for _ in 0..((total_len - printed_len) / group) {
                for _ in 0..group {
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
            for _ in 0..(width - row.len()) {
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
        width: None,
        group: None,
        offset: true,
        ascii: true,
    };

    while args.len() > 1 {
        match args.remove(1) {
            h if h == "-h" => help(),
            w if args.len() > 1 && w == "-w" && ret.width == None => ret.width = Some(args.remove(1)),
            g if args.len() > 1 && g == "-g" && ret.group == None => ret.group = Some(args.remove(1)),
            o if o == "-o" && ret.offset == true => ret.offset = false,
            a if a == "-a" && ret.ascii == true => ret.ascii = false,
            path if ret.path == "" => {
                ret.path = path;
            }
            _ => {
                panic!("Arguments Incorrect");
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
    \t\t\x1b[0;32m-g <grouping>\x1b[0;0m\tSet the number of bytes to group together (default = 1)\n\
    \t\t\x1b[0;32m-o\x1b[0;0m\t\t\tHide the address offset\n\
    \t\t\x1b[0;32m-a\x1b[0;0m\t\t\tHide the asciee representation");
}