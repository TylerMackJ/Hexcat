pub enum Parsing { WIDTH, GROUP, START, END }

pub fn parse_with_base(s: String, p: Parsing) -> Result<usize, ()> {
    let name: String = match p {
        Parsing::WIDTH => "width",
        Parsing::GROUP => "group size",
        Parsing::START => "starting position",
        Parsing::END => "ending position",
    }.to_string();
    let usage: String = match p {
        Parsing::WIDTH => "--width (-w) <width>",
        Parsing::GROUP => "--group (-g) <group size>",
        Parsing::START => "--start (-s) <starting position>",
        Parsing::END => "--end (-e) <ending position>",
    }.to_string();

    let base: u32;
    let number: &str;
    if s.starts_with("0b") {
        base = 2;
        number = &s[2..];
    }
    else if s.ends_with("b") {
        base = 2;
        number = &s[..(s.len() - 1)];
    }
    else if s.starts_with("0o") {
        base = 8;
        number = &s[2..];
    }
    else if s.ends_with("o") {
        base = 8;
        number = &s[..(s.len() - 1)];
    }
    else if s.starts_with("0x") {
        base = 16;
        number = &s[2..];
    }
    else if s.ends_with("x") {
        base = 16;
        number = &s[..(s.len() - 1)];
    }
    else {
        base = 10;
        number = &s;
    }

    return match isize::from_str_radix(number, base) {
        Ok(i) if i <= 0 => {
            eprintln!("\x1b[0;31mError {} must be positive.\n\x1b[0;33mUSAGE: {}\x1b[0;0m", name, usage);
            Err(())
        },
        Ok(i) => Ok(i as usize),
        Err(_) => {
            eprintln!("\x1b[0;31mError {} undefined.\n\x1b[0;33mUSAGE: {}\x1b[0;0m", name, usage);
            Err(())
        },
    }
}

