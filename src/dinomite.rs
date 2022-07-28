use rand::Rng;
use std::collections::HashSet;
struct Position(usize, usize);

struct Dinomite {
    width: usize,
    heigth: usize,
    seen: HashSet<Position>,
    dinos: HashSet<Position>,
    flags: HashSet<Positioin>,
    game_over: bool,
}

impl Dinomite {
    // Create a width x height field with num_dinos hidden dinos.
    // Upon starting no fields have been opened, no flags are set.
    fn new(&self, width: usize, height: usize, num_dinos: usize) -> Self {
        Dinomite {
            width,
            heigth,
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
