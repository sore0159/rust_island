mod tests {
    #[test]
    fn pile_o_stuff() {
        use crate::data::big_types::{Game, Location, Person};
        use crate::data::id_types::{get_multi, get_multi_mut, LID, PID};
        use crate::data::locations::Location;
        let mut game = Game::new();
        for _ in 0..10 {
            let p = Person::new_rand();
            game.add_person(p);
        }
        let l1 = Location::new("World", 0);
        let l2 = Location::new("Island", 1);
        game.add_location(l1);
        game.add_location(l2);
        let mut v = vec![0, 1, 2, 3, 4, 5, 6];

        println!("multi get test:");
        for x in get_multi(&v, &mut [1, 6, 5]) {
            println!("{}", x);
        }
        println!("multi_mut get test:");
        for x in get_multi_mut(&mut v, &mut [1, 6, 5]) {
            *x += 1;
            println!("{}", x);
        }
        println!("get two lid test:");
        let gotten = LID::get_two_mut(&mut game.locations, LID(1), LID(0));
        println!("{:?}, {:?}", gotten.0.id, gotten.1.id);

        let pl1: Vec<PID> = vec![PID(7), PID(3)];
        let persons = PID::get_ordered_mut(&mut game.persons, &pl1);
        for p in persons {
            println!("{}({:?}) says: Hello, {}!", p.name, p.id, gotten.0.name);
        }
        let (l1, l2) = gotten;
        println!("generating path");
        l1.gen_path_to(&l2, 2);
        let mut p = PID(1).get_mut(&mut game.persons);
        let mut l = LID(1).get_mut(&mut game.locations);
        p.name = "TEST".to_string();
        l.name = "TEST2".to_string();
    }
}
