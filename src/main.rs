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

/// Print formatted text on stderr with an "error: " prefix.
macro_rules! error {
    ($exitcode:expr, $($arg:tt)*) => {{
        // "error: " displayed in red and bold font.
        const ERROR_STR: &str = "\x1b[1;31merror:\x1b[0m";
        eprintln!("{ERROR_STR} {}", format!($($arg)*));
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

    let mut byte_iter = rom.into_iter().enumerate();
    while let Some((i, byte)) = byte_iter.next() {
        // Print the address.
        print!("{i:04x}  ");

        // Find how many bytes are part of a single instruction.
        let instruction_length = match (byte >> 4, byte & 0x0F) {
            (0x0..=0x3, 0x1)
            | (0x2..=0x3 | 0xC..=0xF, 0x2 | 0xA)
            | (0xC, 0x3 | 0xB)
            | (0xC..=0xF, 0x4 | 0xC | 0xD) => 3,
            (0xD, 0x3 | 0xB) | (0x0..=0x3 | 0xC..=0xF, 0x6 | 0xE) => 2,
            _ => 1,
        };

        let mut instruction: [Option<u8>; 3] = [Some(byte), None, None];

        // Eventually print the other bytes of the instruction.
        for partial in instruction.iter_mut().take(instruction_length).skip(1) {
            if let Some((_, next_byte)) = byte_iter.next() {
                *partial = Some(next_byte);
            } else {
                // If the instruction required more bytes but the file doesn't
                // contain any more of them, then the rom is incomplete.

                // Needed to flush stdout.
                println!();

                error!(
                    exitcode::DATAERR,
                    "{:?}",
                    anyhow!("incomplete instruction").context(format!(
                        "reading additional bytes for instruction \"{byte:02x}\""
                    ))
                )
            }
        }

        for byte in instruction {
            if let Some(byte) = byte {
                print!("{byte:02x} ");
            } else {
                // Print padding for shorter instructions.
                print!("   ");
            }
        }

        // Padding
        print!(" ");

        // Translate the instruction to assembly.
        match (byte >> 4, byte & 0x0F) {
            (0x0..=0x3, 0x0 | 0x8) => print!("NOP"),
            (0xC, 0x0) => print!("RNZ"),
            (0xD, 0x0) => print!("RNC"),
            (0xE, 0x0) => print!("RPO"),
            (0xF, 0x0) => print!("RP"),
            (0xE, 0x3) => print!("XTHL"),
            (0xF, 0x3) => print!("DI"),
            (0x7, 0x6) => print!("HLT"),
            (0x0, 0x7) => print!("RLC"),
            (0x1, 0x7) => print!("RAL"),
            (0x2, 0x7) => print!("DAA"),
            (0x3, 0x7) => print!("STC"),
            (0xC, 0x8) => print!("RZ"),
            (0xD, 0x8) => print!("RC"),
            (0xE, 0x8) => print!("RPE"),
            (0xF, 0x8) => print!("RM"),
            (0xC | 0xD, 0x9) => print!("RET"),
            (0xE, 0x9) => print!("PCHL"),
            (0xF, 0x9) => print!("SPHL"),
            (0xE, 0xB) => print!("XCHG"),
            (0xF, 0xB) => print!("EI"),
            (0x0, 0xF) => print!("RRC"),
            (0x1, 0xF) => print!("RAR"),
            (0x2, 0xF) => print!("CMA"),
            (0x3, 0xF) => print!("CMC"),
            _ => (),
        }

        println!();
    }
}
