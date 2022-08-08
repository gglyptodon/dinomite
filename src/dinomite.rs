use crate::dinomite::PositionResult::{Clear, Dino, DinosInSurrounding, Flagged, Over};
use itertools::Itertools;
use rand::Rng;
use std::cmp::min;
use std::collections::HashSet;
use std::fmt::Write as _;
use std::fmt::{Display, Formatter};
use crate::wasm_interface::{check_neighbors, check_position};

#[derive(Debug, PartialEq)]
pub enum PositionResult {
    Over,
    Clear,                     // 0 dinos nearby
    DinosInSurrounding(usize), //
    Dino,
    Flagged,
}
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub(crate) struct Position(pub(crate) usize, pub(crate) usize);

pub struct Dinomite {
    width: usize,
    height: usize,
    seen: HashSet<Position>,
    dinos: HashSet<Position>,
    flags: HashSet<Position>,
    game_over: bool,
    won: bool,
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
            won: false,
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

    pub(crate) fn check_position(&mut self, pos: &Position) -> PositionResult {
        if self.won || self.game_over {
            return Over;
        }
        let mut surrounding = 0usize;

        if self.dinos.contains(pos) {
            self.game_over = true;

            return Dino;
        }
        if self.flags.contains(pos) {
            return Flagged;
        }
        if self.seen.contains(pos) {
            return Clear;
        }
        if self.seen.len() == self.width * self.height - self.dinos.len() - 1
            && !self.dinos.contains(pos)
        {

            self.won = true;
            self.game_over = true;
        }
        for n in self.get_neighbors(pos) {
            if self.dinos.contains(&n) {
                surrounding += 1;
            }
        }
        match surrounding {
            0 => {
                self.seen.insert(pos.clone());
                for n in self.get_neighbors(pos) {
                    self.check_position(&n);
                }
                Clear
            }
            _ => {
                self.seen.insert(pos.clone());
                DinosInSurrounding(surrounding)
            }
        }
    }

        pub(crate) fn check_neighbors(&mut self, pos: &Position) {
            let neighbors = self.get_neighbors(pos).collect::<Vec<Position>>();
            let mut num_flags_set = 0;
            for n in &neighbors{
                if self.flags.contains(&n){
                    num_flags_set +=1;
                }
            }
            if self.get_neighboring_dino_count(pos) == num_flags_set{
                for n in &neighbors{
                    if !self.flags.contains(&n){
                    self.check_position(&n);}
                }
            }

        }


    fn get_neighbors(&self, pos: &Position) -> impl Iterator<Item = Position> {
        let neighbors = [
            (pos.0.saturating_sub(1), pos.1),                           //left
            (pos.0.saturating_sub(1), pos.1.saturating_sub(1)),         // top left
            (pos.0.saturating_sub(1), min(pos.1 + 1, self.height - 1)), // bottom left
            (pos.0, pos.1.saturating_sub(1)),                           // top
            (pos.0, min(pos.1 + 1, self.height - 1)),                   // bottom
            (min(pos.0 + 1, self.width - 1), pos.1),                    // right
            (min(pos.0 + 1, self.width - 1), pos.1.saturating_sub(1)),  // top right
            (
                min(pos.0 + 1, self.width - 1),
                min(pos.1 + 1, self.height - 1),
            ), // bottom right
        ];
        neighbors.into_iter().unique().map(|x| Position(x.0, x.1))
    }
    fn get_neighboring_dino_count(&self, pos: &Position) -> usize {
        let mut result = 0;
        for n in self.get_neighbors(pos) {
            if self.dinos.contains(&n) {
                result += 1;
            }
        }
        result
    }
    pub(crate) fn toggle_flag(&mut self, pos: &Position) {
        if self.game_over || self.won {
            return;
        }
        if self.seen.contains(pos) {
            return;
        }
        if self.flags.contains(pos) {
            self.flags.remove(pos);
        } else {
            if self.flags.len() == self.dinos.len() {
                return;
            }

            self.flags.insert(pos.clone());
        }
    }

    pub(crate) fn is_game_over(&self)-> bool{
        self.game_over
    }
      pub(crate) fn is_won(&self)->bool{
        self.won
    }

}
impl Display for Dinomite {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position(x, y);
                match (self.game_over, self.won) {
                // won
                    (true, true) => {
                        if self.flags.contains(&pos) {
                            if self.dinos.contains(&pos) {
                                write!(board, " ✅️ ")?;
                            } else {
                                write!(board, " ❌️ ")?; //should never happen
                            }
                        } else if self.dinos.contains(&pos) {
                            write!(board, " 🦖 ")?;
                        } else if self.seen.contains(&pos) {
                            let count = self.get_neighboring_dino_count(&Position(x, y));
                            match count {
                                0 => {
                                    write!(board, " ☄ ")?;
                                }
                                _ => {
                                    write!(board, " {} ", count)?;
                                }
                            }
                        } else {
                            write!(board, " 🌿 ")?;
                        }
                    }
                    // lost
                    (true, false) => {
                        if self.flags.contains(&Position(x, y)) {
                            if self.game_over {
                                if !self.dinos.contains(&Position(x, y)) {
                                    write!(board, " ❌️ ")?;
                                } else {
                                    write!(board, " ✅️ ")?;
                                }
                            }
                        } else if self.dinos.contains(&pos) {
                            write!(board, " 🦖 ")?;
                        } else if self.seen.contains(&pos) {
                            let count = self.get_neighboring_dino_count(&Position(x, y));
                            match count {
                                0 => {
                                    write!(board, " ⬜ ")?;
                                }
                                _ => {
                                    write!(board, " {} ", count)?;
                                }
                            }
                        } else {
                            write!(board, " 🌿 ")?;
                        }
                    }
                    // game is still running
                    (false, _) => {
                        if self.flags.contains(&pos) {
                            write!(board, " ❗ ")?;
                        } else if self.seen.contains(&Position(x, y)) {
                            let count = self.get_neighboring_dino_count(&Position(x, y));
                            match count {
                                0 => {
                                    write!(board, " ⬜ ")?;
                                }
                                _ => {
                                    write!(board, " {} ", count)?;
                                }
                            }
                        } else {
                            write!(board, " 🌿 ")?;
                        }
                    }
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

    #[test]
    fn test_toggle_flag() {
        let expected = 2;
        let mut dinomite = Dinomite::new(10, 10, 0);
        dinomite.dinos.insert(Position(9, 9));
        dinomite.dinos.insert(Position(8, 8));

        dinomite.toggle_flag(&Position(0, 0));
        dinomite.check_position(&Position(0, 0));

        dinomite.toggle_flag(&Position(0, 0));
        dinomite.toggle_flag(&Position(1, 0));
        dinomite.toggle_flag(&Position(1, 1));
        println!("{}", dinomite);
        println!("{:?}", dinomite.flags);

        assert_eq!(dinomite.flags.len(), expected);
    }

    #[test]
    fn test_check_pos_clear() {
        let expected = 24;
        let mut dinomite = Dinomite::new(5, 5, 0);
        println!("{}", dinomite);
        println!("{:?}", dinomite.seen);

        dinomite.dinos.insert(Position(0, 0));

        println!("{}", dinomite);
        println!("{:?}", dinomite.seen);

        dinomite.check_position(&Position(4, 4));
        for s in &dinomite.seen {
            println!("{:?}", s);
        }
        println!("{}", dinomite);

        assert_eq!(dinomite.seen.len(), expected);
    }

    #[test]
    fn test_check_get_dino_count() {
        let pos = Position(1, 1);
        let expected = 2;
        let mut dinomite = Dinomite::new(5, 5, 0);
        dinomite.dinos.insert(Position(0, 0));
        dinomite.dinos.insert(Position(0, 1));

        println!("{}", dinomite);

        dinomite.check_position(&pos);
        println!("{}", dinomite);
        println!("{:?}", dinomite.seen);

        assert_eq!(dinomite.get_neighboring_dino_count(&pos), expected);
    }
    #[test]
    fn test_check_loose() {
        let pos = Position(1, 1);
        let mut dinomite = Dinomite::new(3, 3, 0);
        dinomite.dinos.insert(pos.clone());
        println!("{}", dinomite);
        dinomite.check_position(&pos);
        println!("{}", dinomite);
        println!("{:?}", dinomite.seen);

        assert_eq!(dinomite.won, false);
        assert_eq!(dinomite.game_over, true);
    }
    #[test]
    fn test_check_win() {
        let pos = Position(1, 1);
        let mut dinomite = Dinomite::new(3, 3, 0);
        dinomite.dinos.insert(pos.clone());
        dinomite.flags.insert(pos.clone());
        println!("{}", dinomite);
        dinomite.check_position(&Position(0, 0));
        dinomite.check_position(&Position(0, 1));
        dinomite.check_position(&Position(0, 2));

        dinomite.check_position(&Position(1, 0));
        dinomite.check_position(&Position(1, 2));

        dinomite.check_position(&Position(2, 0));
        dinomite.check_position(&Position(2, 1));
        dinomite.check_position(&Position(2, 2));


        println!("{}", dinomite);
        println!("{:?}", dinomite.seen);

        assert_eq!(dinomite.won, true);
        assert_eq!(dinomite.game_over, true);
    }
}
