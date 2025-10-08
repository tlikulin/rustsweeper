use rand::Rng;
use std::fmt::Display;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
];

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
            TileStatus::Open(num) => write!(f, "{num}"),
            TileStatus::Flagged => write!(f, "⚑"),
        }
    }
}

#[derive(Debug)]
pub struct Field<const N: usize> {
    tiles: [[Tile; N]; N],
}

impl<const N: usize> Field<N> {
    pub fn new() -> Self {
        assert!(N > 1, "Size too small");

        let mut rng = rand::rng();
        let mut mines_left = N * N / 5;
        let mut field = Field {
            tiles: [[Tile {
                has_mine: false,
                status: TileStatus::Unknown,
            }; N]; N],
        };

        while mines_left > 0 {
            let pos = rng.random_range(0..N * N);
            let (y, x) = (pos / N, pos % N);

            if !field.tiles[y][x].has_mine {
                // println!("Mine at ({y},{x})");
                field.tiles[y][x].has_mine = true;
                mines_left -= 1;
            }
        }

        for y in 0..N {
            for x in 0..N {
                if field.tiles[y][x].has_mine {
                    field.tiles[y][x].status = TileStatus::Flagged;
                } else {
                    field.tiles[y][x].status = TileStatus::Open(field.count_neigbours(y, x));
                }
            }
        }

        field
    }

    fn count_neigbours(&self, y: usize, x: usize) -> u8 {
        DIRECTIONS
            .iter()
            .filter_map(|&(dy, dx)| {
                if let Some(new_y) = y.checked_add_signed(dy)
                    && let Some(new_x) = x.checked_add_signed(dx)
                {
                    self.get_tile(new_y, new_x)
                        .map(|tile| tile.has_mine)
                        .and_then(|is_mine| if is_mine { Some(()) } else { None })
                } else {
                    None
                }
            })
            .count() as u8
    }

    fn get_tile(&self, y: usize, x: usize) -> Option<&Tile> {
        if y >= N || x >= N {
            None
        } else {
            Some(&self.tiles[y][x])
        }
    }
}

impl<const N: usize> Display for Field<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let horizontal_line = format!("-{}", "+-".repeat(N - 1));

        for y in 0..N {
            for x in 0..N - 1 {
                write!(f, "{}|", self.tiles[y][x])?;
            }
            write!(f, "{}", self.tiles[y][N - 1])?;
            if y != N - 1 {
                writeln!(f, "\n{}", horizontal_line)?;
            }
        }

        write!(f, "")
    }
}
