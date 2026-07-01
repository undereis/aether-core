#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Aether core runtime validation binary.

use std::io;
use std::process;

fn main() {
    let mut stdout = io::stdout();
    let code = match aether_cli::run(std::env::args(), &mut stdout) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("{error}");
            aether_cli::EXIT_FAILURE
        }
    };
    process::exit(code);
}
