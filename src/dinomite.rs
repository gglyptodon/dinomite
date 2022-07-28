use rand::Rng;
use std::collections::HashSet;
use std::fmt::Write as _;
use std::fmt::{Display, Formatter};

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

        //for d in &self.dinos {
        //    write!(board, "🦖 at {:>1?} ", d)?;
        //}
        write!(f, "{}", board)
    }
}

#[cfg(test)]
pub mod test {
    use crate::dinomite::Dinomite;

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
}
