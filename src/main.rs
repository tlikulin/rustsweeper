#![allow(dead_code)]
mod commands;
mod field;

use commands::{Command, CommandResult};
use field::Field;

use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut field = Field::new(5, 20);
    let mut last_result = CommandResult::None;

    loop {
        println!("{field}");

        match last_result {
            CommandResult::AlreadyFlagged => print!("(flagged - can't dig) "),
            CommandResult::AlreadyOpen => print!("(already open) "),
            CommandResult::BadCommand => print!("[bad command/coords] "),
            CommandResult::OutOfBounds => print!("[out of bounds] "),
            CommandResult::Revealed | CommandResult::None => (),
            CommandResult::Boom => {
                println!("BOOM!");
                return Ok(());
            }
        }

        print!("Your turn: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if let Ok(command) = input.parse::<Command>() {
            match command {
                Command::Exit => {
                    println!();
                    return Ok(());
                }
                Command::Dig(y, x) => last_result = field.dig_tile(y, x),
                Command::Flag(..) => (),
            }
        } else {
            last_result = CommandResult::BadCommand;
        }
    }
}
