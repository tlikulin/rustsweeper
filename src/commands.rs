use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    Exit,
    Check(usize, usize),
    Flag(usize, usize),
}

#[derive(Debug)]
pub enum CommandParseError {
    UnknownCommand,
    InvalidCoords,
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            // Ctrl-D
            return Ok(Command::Exit);
        }

        let lower = s.to_ascii_lowercase();
        let s = lower.trim();

        if let Some((cmd, coord)) = s.split_once(|c: char| c.is_ascii_whitespace()) {
            if let Some((y, x)) = parse_coord(coord) {
                match cmd {
                    "check" | "c" => Ok(Command::Check(y, x)),
                    "flag" | "f" => Ok(Command::Flag(y, x)),
                    _ => Err(CommandParseError::UnknownCommand),
                }
            } else {
                Err(CommandParseError::InvalidCoords)
            }
        } else if s == "exit" || s == "e" || s == "quit" || s == "q" {
            Ok(Command::Exit)
        } else {
            Err(CommandParseError::UnknownCommand)
        }
    }
}

fn parse_coord(s: &str) -> Option<(usize, usize)> {
    if s.len() < 2 {
        return None;
    }

    let row = match s.chars().next()? {
        letter @ 'a'..='z' => ((letter as u8) - b'a') as usize,
        _ => return None,
    };
    let column = match s[1..].parse::<usize>().ok()? {
        0 => return None,
        n => n - 1,
    };

    Some((row, column))
}
