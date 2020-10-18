use crate::data::big_types::{Game, Person};
use crate::data::locations::Location;

pub fn gen_mockup() -> Game {
    let mut g = Game::new();
    let l1 = g.add_location(Location::new("Place One", 1));
    let l2 = g.add_location(Location::new("Place Two", 1));
    g.add_exit(l1, l2, "The High Road", 2);
    g.add_exit(l1, l2, "The Low Road", 3);
    g.add_exit(l2, l1, "Highway to Place One", 1);
    g.set_pc_at(l1);
    for i in 0..4 {
        let p = g.add_person(Person::new_rand());
        if i % 2 == 0 {
            g.set_at(p, l1);
        } else {
            g.set_at(p, l2);
        }
    }
    g
}
