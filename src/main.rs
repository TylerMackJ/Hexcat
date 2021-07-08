mod arguments;
mod base_tools;

use crate::arguments::*;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut exit: bool = false;
    let args: Arguments = match handle_args(&mut exit) {
        Ok(a) => a,
        Err(_) => return Ok(()),
    };

    if exit {
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
                match args.base {
                    BaseOption::BIN => print!("{:08b}", byte),
                    BaseOption::OCT => print!("{:03o}", byte),
                    BaseOption::HEX => print!("{:02x}", byte),
                }
            }
            for _ in 0..(args.group - grouping.len()) {
                if i != content.chunks(args.width).len() - 1 {
                    match args.base {
                        BaseOption::BIN => print!("~~~~~~~~"),
                        BaseOption::OCT => print!("~~~"),
                        BaseOption::HEX => print!("~~"),
                    }
                } else {
                    match args.base {
                        BaseOption::BIN => print!("        "),
                        BaseOption::OCT => print!("   "),
                        BaseOption::HEX => print!("  "),
                    }
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
                    match args.base {
                        BaseOption::BIN => print!("        "),
                        BaseOption::OCT => print!("   "),
                        BaseOption::HEX => print!("  "),
                    }
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