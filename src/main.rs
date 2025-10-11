#![allow(dead_code)]
mod commands;
mod field;

use commands::Command;
use field::Field;

use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut field = Field::new(5, 20);
    let mut extra_message = "";

    loop {
        println!("{field}");
        print!("{extra_message}Your turn: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if let Ok(command) = input.parse::<Command>() {
            match command {
                Command::Exit => {
                    println!();
                    return Ok(());
                }
                Command::Check(y, x) => extra_message = field.check_tile(y, x),
                Command::Flag(..) => extra_message = "",
            }
        } else {
            extra_message = "[Unkown command] ";
        }
    }
}
