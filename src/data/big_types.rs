use crate::data::id_types::{LID, PID};
use crate::data::locations::{Exit, Location};
use rand::Rng;

pub struct Game {
    pub env: Env,
    pub pc: PC,
    pub persons: Vec<Person>,
    pub locations: Vec<Location>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            env: Env::new(),
            pc: PC::new(),
            persons: Vec::with_capacity(200),
            locations: Vec::with_capacity(100),
        }
    }

    pub fn add_location(&mut self, mut l: Location) -> LID {
        let ln = self.locations.len();
        l.id = LID(ln);
        self.locations.push(l);
        self.locations[ln].id
    }
    pub fn add_person(&mut self, mut p: Person) -> PID {
        let ln = self.persons.len();
        p.id = PID(ln);
        self.persons.push(p);
        self.persons[ln].id
    }
    pub fn add_exit(&mut self, from: LID, to: LID, name: impl Into<String>, dur: usize) {
        let dir = from.get(&self.locations).dir_to(to.get(&self.locations));
        let e = Exit::new(from, to, dur, dir, name.into());
        from.get_mut(&mut self.locations).add_exit(e);
    }
}

impl Game {
    pub fn move_out(&mut self, p: PID, l: LID) {
        l.get_mut(&mut self.locations).drop_p(p);
        p.get_mut(&mut self.persons).loc = None;
    }
    pub fn set_at(&mut self, p: PID, l: LID) {
        l.get_mut(&mut self.locations).present.push(p);
        p.get_mut(&mut self.persons).loc = Some(l);
    }
    pub fn set_pc_at(&mut self, l: LID) {
        self.pc.loc = Some(l);
    }
}

pub struct Person {
    pub id: PID,
    pub name: String,
    pub loc: Option<LID>,
}

impl Person {
    pub fn new_rand() -> Self {
        use crate::data::names_list::PER_NAMES;

        let i = rand::thread_rng().gen_range(1, PER_NAMES.len());
        Person {
            id: PID(0),
            name: PER_NAMES[i].to_string(),
            loc: None,
        }
    }
    pub fn is_at(&self, l: LID) -> bool {
        self.loc == Some(l)
    }
}

pub struct Env {
    pub u_time: usize,
}

impl Env {
    pub fn new() -> Self {
        Env { u_time: 0 }
    }
}

pub struct PC {
    pub loc: Option<LID>,
}

impl PC {
    pub fn new() -> Self {
        PC { loc: None }
    }
    pub fn is_at(&self, l: LID) -> bool {
        return self.loc == Some(l);
    }
}
