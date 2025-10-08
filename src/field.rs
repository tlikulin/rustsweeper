use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
struct Tile {
    num_mines: u8,
    status: TileStatus,
}

#[derive(Clone, Copy, Debug)]
enum TileStatus {
    Unknown,
    Open,
    Flagged,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.num_mines)
    }
}

#[derive(Debug)]
pub struct Field<const N: usize> {
    tiles: Vec<Tile>,
}

impl<const N: usize> Field<N> {
    pub fn new() -> Self {
        assert!(N > 1, "Size too small");

        Field {
            tiles: vec![
                Tile {
                    num_mines: 1,
                    status: TileStatus::Open,
                };
                N * N
            ],
        }
    }

    fn get_tile(&self, y: usize, x: usize) -> Option<&Tile> {
        if y >= N || x >= N {
            None
        } else {
            Some(&self.tiles[y * N + x])
        }
    }
}

impl<const N: usize> Display for Field<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let horizontal_line = format!("-{}", "+-".repeat(N - 1));

        for y in 0..N {
            for x in 0..N - 1 {
                write!(f, "{}|", self.get_tile(y, x).unwrap())?;
            }
            write!(f, "{}", self.get_tile(y, N - 1).unwrap())?;
            if y != N - 1 {
                writeln!(f, "\n{}", horizontal_line)?;
            }
        }

        write!(f, "")
    }
}
