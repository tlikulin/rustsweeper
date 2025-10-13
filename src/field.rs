use crate::commands::CommandResult;

use rand::Rng;

use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
struct Tile {
    has_mine: bool,
    status: TileStatus,
}

impl Tile {
    fn new() -> Tile {
        Tile {
            has_mine: false,
            status: TileStatus::Unknown,
        }
    }

    fn is_open(&self) -> bool {
        matches!(self.status, TileStatus::Open(..))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum TileStatus {
    Unknown,
    Open(u8),
    Flagged,
    Exploded,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status {
            TileStatus::Unknown => write!(f, "?"),
            TileStatus::Open(0) => write!(f, " "),
            TileStatus::Open(num) => write!(f, "{num}"),
            TileStatus::Flagged => write!(f, "X"),
            TileStatus::Exploded => write!(f, "!"),
        }
    }
}

#[derive(Debug)]
pub struct Field {
    tiles: Vec<Vec<Tile>>,
    rows: usize,
    columns: usize,
}

impl Field {
    pub fn new(rows: usize, columns: usize) -> Self {
        let mut rng = rand::rng();
        let mut mines_left = rng.random_range(rows * columns / 6..rows * columns / 4);
        let mut field = Field {
            tiles: vec![vec![Tile::new(); columns]; rows],
            rows,
            columns,
        };

        while mines_left > 0 {
            let (y, x) = (rng.random_range(0..rows), rng.random_range(0..columns));
            if field.tiles[y][x].has_mine {
                continue;
            }

            field.tiles[y][x].has_mine = true;
            mines_left -= 1;
        }

        field
    }

    fn get_neigbours_coords(&self, y: usize, x: usize) -> Vec<(usize, usize)> {
        let mut neigbours = Vec::new();
        for (dy, dx) in [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
        ] {
            if let Some(new_y) = y.checked_add_signed(dy)
                && let Some(new_x) = x.checked_add_signed(dx)
                && new_y < self.rows
                && new_x < self.columns
            {
                neigbours.push((new_y, new_x));
            }
        }
        neigbours
    }

    fn count_neigbouring_mines(&self, y: usize, x: usize) -> u8 {
        self.get_neigbours_coords(y, x)
            .into_iter()
            .filter(|&(y, x)| self.tiles[y][x].has_mine)
            .count() as u8
    }

    fn is_within_bounds(&self, y: usize, x: usize) -> bool {
        y < self.rows && x < self.columns
    }

    pub fn reveal_all(&mut self) {
        for y in 0..self.rows {
            for x in 0..self.columns {
                if self.tiles[y][x].has_mine {
                    self.tiles[y][x].status = TileStatus::Flagged;
                } else {
                    self.tiles[y][x].status = TileStatus::Open(self.count_neigbouring_mines(y, x));
                }
            }
        }
    }

    pub fn dig_tile(&mut self, y: usize, x: usize) -> CommandResult {
        if !self.is_within_bounds(y, x) {
            CommandResult::OutOfBounds
        } else {
            match self.tiles[y][x].status {
                TileStatus::Open(_) => CommandResult::AlreadyOpen,
                TileStatus::Flagged => CommandResult::AlreadyFlagged,
                TileStatus::Unknown if self.tiles[y][x].has_mine => {
                    self.tiles[y][x].status = TileStatus::Exploded;
                    CommandResult::Boom
                }
                TileStatus::Exploded => CommandResult::Boom,
                TileStatus::Unknown => {
                    self.chain_reveal(y, x);
                    CommandResult::Revealed
                }
            }
        }
    }

    fn chain_reveal(&mut self, y: usize, x: usize) {
        if self.tiles[y][x].is_open() {
            return;
        }

        let neighbours = self.count_neigbouring_mines(y, x);
        self.tiles[y][x].status = TileStatus::Open(neighbours);

        if neighbours == 0 {
            for (ny, nx) in self.get_neigbours_coords(y, x) {
                self.chain_reveal(ny, nx);
            }
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.columns > 9 {
            let mut columns_tens = "  ".to_string();
            for i in 1..=self.columns as u32 {
                columns_tens.push(char::from_digit(i / 10, 10).unwrap());
                columns_tens.push(' ');
            }
            writeln!(f, "{columns_tens}")?;
        }

        let mut columns_units = "  ".to_string();
        for i in 1..=self.columns as u32 {
            columns_units.push(char::from_digit(i % 10, 10).unwrap());
            columns_units.push(' ');
        }
        writeln!(f, "{columns_units}")?;

        let horizontal_line = format!("-{}", "+-".repeat(self.columns - 1));

        for y in 0..self.rows {
            write!(f, "{} ", (b'a' + y as u8) as char)?;
            for x in 0..self.columns - 1 {
                write!(f, "{}|", self.tiles[y][x])?;
            }
            write!(f, "{}", self.tiles[y][self.columns - 1])?;
            if y != self.rows - 1 {
                writeln!(f, "\n  {}", horizontal_line)?;
            }
        }

        write!(f, "")
    }
}
