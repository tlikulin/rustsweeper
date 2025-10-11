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
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum TileStatus {
    Unknown,
    Open(u8),
    Flagged,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status {
            TileStatus::Unknown => write!(f, "?"),
            TileStatus::Open(0) => write!(f, " "),
            TileStatus::Open(num) => write!(f, "{num}"),
            TileStatus::Flagged => write!(f, "X"),
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

    fn count_neigbours(&self, y: usize, x: usize) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if let Some(new_y) = y.checked_add_signed(dy)
                    && let Some(new_x) = x.checked_add_signed(dx)
                    && new_y < self.rows
                    && new_x < self.columns
                    && self.tiles[new_y][new_x].has_mine
                {
                    count += 1;
                }
            }
        }

        count
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
                    self.tiles[y][x].status = TileStatus::Open(self.count_neigbours(y, x));
                }
            }
        }
    }

    pub fn check_tile(&mut self, y: usize, x: usize) -> &'static str {
        if !self.is_within_bounds(y, x) {
            "(Out of bounds) "
        } else if let TileStatus::Open(..) = self.tiles[y][x].status {
            "(Already open) "
        } else {
            self.tiles[y][x].status = TileStatus::Open(self.count_neigbours(y, x));
            ""
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
