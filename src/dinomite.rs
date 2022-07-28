use crate::dinomite::PositionResult::{Clear, Dino, DinosInSurrounding};
use itertools::Itertools;
use rand::Rng;
use std::cmp::min;
use std::collections::HashSet;
use std::fmt::Write as _;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum PositionResult {
    Clear,                     // 0 dinos nearby
    DinosInSurrounding(usize), //
    Dino,
    Flagged,
}
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Position(usize, usize);

pub struct Dinomite {
    width: usize,
    height: usize,
    seen: HashSet<Position>,
    dinos: HashSet<Position>,
    flags: HashSet<Position>,
    game_over: bool,
}

impl Dinomite {
    // Create a width x height field with num_dinos hidden dinos.
    // Upon starting no fields have been opened, no flags are set.
    pub fn new(width: usize, height: usize, num_dinos: usize) -> Self {
        Dinomite {
            width,
            height,
            seen: HashSet::new(),
            dinos: {
                let mut rng = rand::thread_rng();
                let mut d = HashSet::new();
                while d.len() < num_dinos {
                    d.insert(Position(rng.gen_range(0..width), rng.gen_range(0..height)));
                }
                d
            },
            flags: HashSet::new(),
            game_over: false,
        }
    }
    pub fn reconfigure(&mut self, height: usize, width: usize, num_dinos: usize) {
        let tmp = Dinomite::new(height, width, num_dinos);
        self.dinos = tmp.dinos.clone();
        self.flags = tmp.flags.clone();
        self.seen = tmp.seen.clone();
        self.width = tmp.width;
        self.height = tmp.height;
    }

    fn check_position(&self, pos: &Position) -> PositionResult {
        let mut surrounding = 0usize;
        if self.dinos.contains(pos) {
            return Dino;
        }
        for n in self.get_neighbors(pos) {
            if self.dinos.contains(&n) {
                surrounding += 1;
            }
        }
        match surrounding {
            0 => Clear,
            _ => DinosInSurrounding(surrounding),
        }
    }

    fn get_neighbors(&self, pos: &Position) -> impl Iterator<Item = Position> {
        let neighbors = [
            (pos.0.saturating_sub(1), pos.1),                          //left
            (pos.0.saturating_sub(1), pos.1.saturating_sub(1)),        // top left
            (pos.0.saturating_sub(1), min(pos.1 + 1, self.height)),    // bottom left
            (pos.0, pos.1.saturating_sub(1)),                          // top
            (pos.0, min(pos.1 + 1, self.height)),                      // bottom
            (min(pos.0 + 1, self.width), pos.1),                       // right
            (min(pos.0 + 1, self.width), pos.1.saturating_sub(1)),     // top right
            (min(pos.0 + 1, self.width), min(pos.1 + 1, self.height)), // bottom right
        ];
        neighbors.into_iter().unique().map(|x| Position(x.0, x.1))
    }
    fn get_neighboring_dino_count(&self, pos: &Position) -> usize {
        todo!()
    }
}

impl Display for Dinomite {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.dinos.contains(&Position(x, y)) {
                    write!(board, "🦖")?;
                } else if self.flags.contains(&Position(x, y)) {
                    write!(board, " F")?;
                } else {
                    write!(board, " _")?;
                }
            }
            write!(board, "\n")?;
        }

        write!(f, "{}", board)
    }
}

#[cfg(test)]
pub mod test {
    use crate::dinomite::PositionResult::DinosInSurrounding;
    use crate::dinomite::{Dinomite, Position, PositionResult};
    use std::collections::HashSet;

    #[test]
    fn test_repr() {
        let expected = 5;
        let dinomite = Dinomite::new(10, 10, expected);
        print!("{}", dinomite);
        assert_eq!(dinomite.dinos.len(), expected);
    }
    #[test]
    fn test_reset() {
        let expected = 10;
        let mut dinomite = Dinomite::new(10, 10, 5);
        println!("{}", dinomite);
        dinomite.reconfigure(20, 20, expected);
        println!("{}", dinomite);
        assert_eq!(dinomite.dinos.len(), expected);
    }
    #[test]
    fn test_neighbors() {
        let expected: HashSet<Position> = HashSet::from([
            Position(0, 0),
            Position(0, 1),
            Position(1, 0),
            Position(1, 1),
        ]);
        let mut dinomite = Dinomite::new(10, 10, 5);
        println!("{}", dinomite);
        println!("{}", dinomite);
        assert_eq!(
            dinomite
                .get_neighbors(&Position(0, 0))
                .collect::<HashSet<Position>>(),
            expected
        );
    }

    #[test]
    fn test_surrounding() {
        let expected: PositionResult = DinosInSurrounding(3);
        let mut dinomite = Dinomite::new(10, 10, 0);
        dinomite.dinos.insert(Position(0, 0));
        dinomite.dinos.insert(Position(1, 0));
        dinomite.dinos.insert(Position(1, 1));
        let pos = Position(0, 1);
        println!("{}", dinomite);
        assert_eq!(dinomite.check_position(&pos), expected);
    }
}
