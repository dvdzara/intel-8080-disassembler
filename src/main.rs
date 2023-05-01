// reference: https://pastraiser.com/cpu/i8080/i8080_opcodes.html

#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious
)]

use anyhow::anyhow;

// Color escape sequences to print colors on the terminal.
const COLOR_RESET: &str = "\x1b[0m";
const COLOR_BOLD: &str = "\x1b[1m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_BLUE: &str = "\x1b[34m";
const COLOR_PURPLE: &str = "\x1b[35m";
const COLOR_GRAY: &str = "\x1b[37m";

/// Print formatted text on stderr with an "error: " prefix.
macro_rules! error {
    ($exitcode:expr, $($arg:tt)*) => {{
        // "error: " displayed in red and bold font.
        eprintln!("{COLOR_RED}{COLOR_BOLD}error:{COLOR_RESET} {}", format!($($arg)*));
        std::process::exit($exitcode);
    }}
}

fn main() {
    let mut args = std::env::args();

    // Read only the first argument, ignore the others.
    // Error if the first argument is missing.
    let rom_file_path;
    if let Some(arg) = args.nth(1) {
        rom_file_path = arg;
    } else {
        println!("usage: {} <FILE>", env!("CARGO_PKG_NAME"));
        std::process::exit(exitcode::USAGE);
    }

    // Read the file into a vector.
    let rom = match std::fs::read(rom_file_path).map_err(|e| anyhow!(e)) {
        Ok(r) => r,
        Err(e) => {
            error!(exitcode::IOERR, "{:?}", e.context("opening rom file"));
        }
    };

    let mut rom_iter = rom.iter().enumerate();
    while let Some((address, first_byte)) = rom_iter.next() {
        print!("{address:04x}  {first_byte:02x} ");

        // Translate the instruction to assembly.
        let (instruction_length, text, additional_text) = match first_byte {
            0x00 | 0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 => (1, "NOP", ""),
            0x01 => (3, "LXI", "B"),
            0x02 => (1, "STAX", "B"),
            0x03 => (1, "INX", "B"),
            0x04 => (1, "INR", "B"),
            0x05 => (1, "DCR", "B"),
            0x06 => (2, "MVI", "B"),
            0x07 => (1, "RLC", ""),
            0x09 => (1, "DAB", "D"),
            0x0A => (1, "LDAX", "B"),
            0x0B => (1, "DCX", "B"),
            0x0C => (1, "INR", "C"),
            0x0D => (1, "DCR", "C"),
            0x0E => (2, "MVI", "C"),
            0x0F => (1, "RRC", ""),
            0x11 => (3, "LXI", "D"),
            0x12 => (1, "STAX", "D"),
            0x13 => (1, "INX", "D"),
            0x14 => (1, "INR", "D"),
            0x15 => (1, "DCR", "D"),
            0x16 => (2, "MVI", "D"),
            0x17 => (1, "RAL", ""),
            0x19 => (1, "DAD", "D"),
            0x1A => (1, "LDAX", "D"),
            0x1B => (1, "DCX", "D"),
            0x1C => (1, "INR", "E"),
            0x1D => (1, "DCR", "E"),
            0x1E => (2, "MVI", "E"),
            0x1F => (1, "RAR", ""),
            0x21 => (3, "LXI", "H"),
            0x22 => (3, "SHLD", ""),
            0x23 => (1, "INX", "H"),
            0x24 => (1, "INR", "H"),
            0x25 => (1, "DCR", "H"),
            0x26 => (2, "MVI", "H"),
            0x27 => (1, "DAA", ""),
            0x29 => (1, "DAD", "H"),
            0x2A => (3, "LHLD", ""),
            0x2B => (1, "DCX", "H"),
            0x2C => (1, "INR", "L"),
            0x2D => (1, "DCR", "L"),
            0x2E => (2, "MVI", "L"),
            0x2F => (1, "CMA", ""),
            0x31 => (3, "LXI", "SP"),
            0x32 => (3, "STA", ""),
            0x33 => (1, "INX", "SP"),
            0x34 => (1, "INR", "M"),
            0x35 => (1, "DCR", "M"),
            0x36 => (2, "MVI", "M"),
            0x37 => (1, "STC", ""),
            0x39 => (1, "DAD", "SP"),
            0x3A => (3, "LDA", ""),
            0x3B => (1, "DCX", "SP"),
            0x3C => (1, "INR", "A"),
            0x3D => (1, "DCR", "A"),
            0x3E => (2, "MVI", "A"),
            0x3F => (1, "CMC", ""),
            0x40 => (1, "MOV", "B,B"),
            0x41 => (1, "MOV", "B,C"),
            0x42 => (1, "MOV", "B,D"),
            0x43 => (1, "MOV", "B,E"),
            0x44 => (1, "MOV", "B,H"),
            0x45 => (1, "MOV", "B,L"),
            0x46 => (1, "MOV", "B,M"),
            0x47 => (1, "MOV", "B,A"),
            0x48 => (1, "MOV", "C,B"),
            0x49 => (1, "MOV", "C,C"),
            0x4A => (1, "MOV", "C,D"),
            0x4B => (1, "MOV", "C,E"),
            0x4C => (1, "MOV", "C,H"),
            0x4D => (1, "MOV", "C,L"),
            0x4E => (1, "MOV", "C,M"),
            0x4F => (1, "MOV", "C,A"),
            0x50 => (1, "MOV", "D,B"),
            0x51 => (1, "MOV", "D,C"),
            0x52 => (1, "MOV", "D,D"),
            0x53 => (1, "MOV", "D,E"),
            0x54 => (1, "MOV", "D,H"),
            0x55 => (1, "MOV", "D,L"),
            0x56 => (1, "MOV", "D,M"),
            0x57 => (1, "MOV", "D,A"),
            0x58 => (1, "MOV", "E,B"),
            0x59 => (1, "MOV", "E,C"),
            0x5A => (1, "MOV", "E,D"),
            0x5B => (1, "MOV", "E,E"),
            0x5C => (1, "MOV", "E,H"),
            0x5D => (1, "MOV", "E,L"),
            0x5E => (1, "MOV", "E,M"),
            0x5F => (1, "MOV", "E,A"),
            0x60 => (1, "MOV", "H,B"),
            0x61 => (1, "MOV", "H,C"),
            0x62 => (1, "MOV", "H,D"),
            0x63 => (1, "MOV", "H,E"),
            0x64 => (1, "MOV", "H,H"),
            0x65 => (1, "MOV", "H,L"),
            0x66 => (1, "MOV", "H,M"),
            0x67 => (1, "MOV", "H,A"),
            0x68 => (1, "MOV", "L,B"),
            0x69 => (1, "MOV", "L,C"),
            0x6A => (1, "MOV", "L,D"),
            0x6B => (1, "MOV", "L,E"),
            0x6C => (1, "MOV", "L,H"),
            0x6D => (1, "MOV", "L,L"),
            0x6E => (1, "MOV", "L,M"),
            0x6F => (1, "MOV", "L,A"),
            0x70 => (1, "MOV", "M,B"),
            0x71 => (1, "MOV", "M,C"),
            0x72 => (1, "MOV", "M,D"),
            0x73 => (1, "MOV", "M,E"),
            0x74 => (1, "MOV", "M,H"),
            0x75 => (1, "MOV", "M,L"),
            0x76 => (1, "HLT", ""),
            0x77 => (1, "MOV", "M,A"),
            0x78 => (1, "MOV", "A,B"),
            0x79 => (1, "MOV", "A,C"),
            0x7A => (1, "MOV", "A,D"),
            0x7B => (1, "MOV", "A,E"),
            0x7C => (1, "MOV", "A,H"),
            0x7D => (1, "MOV", "A,L"),
            0x7E => (1, "MOV", "A,M"),
            0x7F => (1, "MOV", "A,A"),
            0x80 => (1, "ADD", "B"),
            0x81 => (1, "ADD", "C"),
            0x82 => (1, "ADD", "D"),
            0x83 => (1, "ADD", "E"),
            0x84 => (1, "ADD", "H"),
            0x85 => (1, "ADD", "L"),
            0x86 => (1, "ADD", "M"),
            0x87 => (1, "ADD", "A"),
            0x88 => (1, "ADC", "B"),
            0x89 => (1, "ADC", "C"),
            0x8A => (1, "ADC", "D"),
            0x8B => (1, "ADC", "E"),
            0x8C => (1, "ADC", "H"),
            0x8D => (1, "ADC", "L"),
            0x8E => (1, "ADC", "M"),
            0x8F => (1, "ADC", "A"),
            0x90 => (1, "SUB", "B"),
            0x91 => (1, "SUB", "C"),
            0x92 => (1, "SUB", "D"),
            0x93 => (1, "SUB", "E"),
            0x94 => (1, "SUB", "H"),
            0x95 => (1, "SUB", "L"),
            0x96 => (1, "SUB", "M"),
            0x97 => (1, "SUB", "A"),
            0x98 => (1, "SBB", "B"),
            0x99 => (1, "SBB", "C"),
            0x9A => (1, "SBB", "D"),
            0x9B => (1, "SBB", "E"),
            0x9C => (1, "SBB", "H"),
            0x9D => (1, "SBB", "L"),
            0x9E => (1, "SBB", "M"),
            0x9F => (1, "SBB", "A"),
            0xA0 => (1, "ANA", "B"),
            0xA1 => (1, "ANA", "C"),
            0xA2 => (1, "ANA", "D"),
            0xA3 => (1, "ANA", "E"),
            0xA4 => (1, "ANA", "H"),
            0xA5 => (1, "ANA", "L"),
            0xA6 => (1, "ANA", "M"),
            0xA7 => (1, "ANA", "A"),
            0xA8 => (1, "XRA", "B"),
            0xA9 => (1, "XRA", "C"),
            0xAA => (1, "XRA", "D"),
            0xAB => (1, "XRA", "E"),
            0xAC => (1, "XRA", "H"),
            0xAD => (1, "XRA", "L"),
            0xAE => (1, "XRA", "M"),
            0xAF => (1, "XRA", "A"),
            0xB0 => (1, "ORA", "B"),
            0xB1 => (1, "ORA", "C"),
            0xB2 => (1, "ORA", "D"),
            0xB3 => (1, "ORA", "E"),
            0xB4 => (1, "ORA", "H"),
            0xB5 => (1, "ORA", "L"),
            0xB6 => (1, "ORA", "M"),
            0xB7 => (1, "ORA", "A"),
            0xB8 => (1, "CMP", "B"),
            0xB9 => (1, "CMP", "C"),
            0xBA => (1, "CMP", "D"),
            0xBB => (1, "CMP", "E"),
            0xBC => (1, "CMP", "H"),
            0xBD => (1, "CMP", "L"),
            0xBE => (1, "CMP", "M"),
            0xBF => (1, "CMP", "A"),
            0xC0 => (1, "RNZ", ""),
            0xC1 => (1, "POP", "B"),
            0xC2 => (3, "JNZ", ""),
            0xC3 | 0xCB => (3, "JMP", ""),
            0xC4 => (3, "CNZ", ""),
            0xC5 => (1, "PUSH", "B"),
            0xC6 => (2, "ADI", ""),
            0xC7 => (1, "RST", "0"),
            0xC8 => (1, "RZ", ""),
            0xC9 | 0xD9 => (1, "RET", ""),
            0xCA => (3, "JZ", ""),
            0xCC => (3, "CZ", ""),
            0xCD | 0xDD | 0xED | 0xFD => (3, "CALL", ""),
            0xCE => (2, "ACI", ""),
            0xCF => (1, "RST", "1"),
            0xD0 => (1, "RNC", ""),
            0xD1 => (1, "POP", "D"),
            0xD2 => (3, "JNC", ""),
            0xD3 => (2, "OUT", ""),
            0xD4 => (3, "CNC", ""),
            0xD5 => (1, "PUSH", "D"),
            0xD6 => (2, "SUI", ""),
            0xD7 => (1, "RST", "2"),
            0xD8 => (1, "RC", ""),
            0xDA => (3, "JC", ""),
            0xDB => (2, "IN", ""),
            0xDC => (3, "CC", ""),
            0xDE => (2, "SBI", ""),
            0xDF => (1, "RST", "3"),
            0xE0 => (1, "RPO", ""),
            0xE1 => (1, "POP", "H"),
            0xE2 => (3, "JPO", ""),
            0xE3 => (1, "XTHL", ""),
            0xE4 => (3, "CPO", ""),
            0xE5 => (1, "PUSH", "H"),
            0xE6 => (2, "ANI", ""),
            0xE7 => (1, "RST", "4"),
            0xE8 => (1, "RPE", ""),
            0xE9 => (1, "PCHL", ""),
            0xEA => (3, "JPE", ""),
            0xEB => (1, "XCHG", ""),
            0xEC => (3, "CPE", ""),
            0xEE => (2, "XRI", ""),
            0xEF => (1, "RST", "5"),
            0xF0 => (1, "RP", ""),
            0xF1 => (1, "POP", "PSW"),
            0xF2 => (3, "JP", ""),
            0xF3 => (1, "DI", ""),
            0xF4 => (3, "CP", ""),
            0xF5 => (1, "PUSH", "PSW"),
            0xF6 => (2, "ORI", ""),
            0xF7 => (1, "RST", "6"),
            0xF8 => (1, "RM", ""),
            0xF9 => (1, "SPHL", ""),
            0xFA => (3, "JM", ""),
            0xFB => (1, "EI", ""),
            0xFC => (3, "CM", ""),
            0xFE => (2, "CPI", ""),
            0xFF => (1, "RST", "7"),
        };

        let mut second_byte = None;
        if instruction_length > 1 {
            if let Some((_, byte)) = rom_iter.next() {
                print!("{byte:02x} ");
                second_byte = Some(byte);
            } else {
                println!();
                error!(
                    exitcode::DATAERR,
                    "{:?}",
                    anyhow!("instruction incomplete")
                        .context("reading second byte of instruction \"{first_byte:02x}\"")
                )
            }
        }

        let mut third_byte = None;
        if instruction_length > 2 {
            if let Some((_, byte)) = rom_iter.next() {
                print!("{byte:02x} ");
                third_byte = Some(byte);
            } else {
                println!();
                error!(
                    exitcode::DATAERR,
                    "{:?}",
                    anyhow!("instruction incomplete")
                        .context("reading third byte of instruction \"{first_byte:02x}\"")
                )
            }
        }

        // Print padding for shorter instructions.
        for _ in 0..3 - instruction_length {
            print!("   ");
        }

        let additional_bytes_text = {
            match instruction_length {
                1 => String::new(),
                2 => format!("{COLOR_PURPLE}#0x{:02x}{COLOR_RESET}", second_byte.unwrap()),
                3 => format!(
                    "{COLOR_BLUE}${:02x}{:02x}{COLOR_RESET}",
                    third_byte.unwrap(),
                    second_byte.unwrap()
                ),
                _ => unreachable!(),
            }
        };

        let comma = if !additional_text.is_empty() && !additional_bytes_text.is_empty() {
            ","
        } else {
            ""
        };

        let color = match text {
            "NOP" => COLOR_GRAY,
            _ => COLOR_RED,
        };

        print!("   {color}{text}{COLOR_RESET}");
        println!("\t{additional_text}{comma}{additional_bytes_text}");
    }
}
