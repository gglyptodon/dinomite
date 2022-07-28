use rand::Rng;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fmt::Write as _;

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
}

impl Display for Dinomite {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for d in &self.dinos{
            write!(board, "🦖 at {:>1?} ", d)?;
        }
        write!(f,"{}", board)
    }
}


#[cfg(test)]
pub mod test{
    use crate::dinomite::Dinomite;

    #[test]
    fn test_repr(){
        let expected =5;
        let dinomite = Dinomite::new(10, 10, expected);
        print!("{}", dinomite);
        assert_eq!(dinomite.dinos.len(), expected);
    }
}