#![allow(dead_code)]
use rand::Rng;
use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
struct Tile {
    has_mine: bool,
    status: TileStatus,
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
pub struct Field<const NY: usize, const NX: usize> {
    tiles: [[Tile; NX]; NY],
}

impl<const NY: usize, const NX: usize> Field<NY, NX> {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut mines_left = rng.random_range(NY * NX / 6..NY * NX / 4);
        let mut field = Field {
            tiles: [[Tile {
                has_mine: false,
                status: TileStatus::Unknown,
            }; NX]; NY],
        };

        while mines_left > 0 {
            let (y, x) = (rng.random_range(0..NY), rng.random_range(0..NX));
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
                    && new_y < NY
                    && new_x < NX
                    && self.tiles[new_y][new_x].has_mine
                {
                    count += 1;
                }
            }
        }

        count
    }

    fn get_tile(&self, y: usize, x: usize) -> Option<&Tile> {
        if y >= NY || x >= NX {
            None
        } else {
            Some(&self.tiles[y][x])
        }
    }

    pub fn reveal(&mut self) {
        for y in 0..NY {
            for x in 0..NX {
                if self.tiles[y][x].has_mine {
                    self.tiles[y][x].status = TileStatus::Flagged;
                } else {
                    self.tiles[y][x].status = TileStatus::Open(self.count_neigbours(y, x));
                }
            }
        }
    }
}

impl<const NY: usize, const NX: usize> Display for Field<NY, NX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if NX > 9 {
            let mut columns_tens = "  ".to_string();
            for i in 1..=NX as u32 {
                columns_tens.push(char::from_digit(i / 10, 10).unwrap());
                columns_tens.push(' ');
            }
            writeln!(f, "{columns_tens}")?;
        }

        let mut columns_units = "  ".to_string();
        for i in 1..=NX as u32 {
            columns_units.push(char::from_digit(i % 10, 10).unwrap());
            columns_units.push(' ');
        }
        writeln!(f, "{columns_units}")?;

        let horizontal_line = format!("-{}", "+-".repeat(NX - 1));

        for y in 0..NY {
            write!(f, "{} ", (b'a' + y as u8) as char)?;
            for x in 0..NX - 1 {
                write!(f, "{}|", self.tiles[y][x])?;
            }
            write!(f, "{}", self.tiles[y][NX - 1])?;
            if y != NY - 1 {
                writeln!(f, "\n  {}", horizontal_line)?;
            }
        }

        write!(f, "")
    }
}
