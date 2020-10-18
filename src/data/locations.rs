use crate::data::id_types::{LID, PID};

pub struct Location {
    pub id: LID,
    pub tier: usize,
    pub name: String,
    pub exits: Vec<Exit>,
    pub present: Vec<PID>,
}

impl Location {
    pub fn new(n: impl Into<String>, tier: usize) -> Self {
        Location {
            id: LID(0),
            tier: tier,
            name: n.into(),
            exits: Vec::with_capacity(3),
            present: Vec::with_capacity(10),
        }
    }
    pub fn find_paths_to(&mut self, target: LID) -> Vec<&Exit> {
        self.exits.iter().filter(|&x| x.to == target).collect()
    }
    pub fn has_p(&self, p: PID) -> bool {
        self.present.iter().position(|&x| x == p).is_some()
    }
    pub fn drop_p(&mut self, p: PID) {
        if let Some(i) = self.present.iter().position(|&x| x == p) {
            self.present.remove(i);
        }
    }
    pub fn dir_to(&self, other: &Self) -> LocDir {
        use std::cmp::Ordering;
        match self.tier.cmp(&other.tier) {
            Ordering::Less => LocDir::Down,
            Ordering::Greater => LocDir::Up,
            Ordering::Equal => LocDir::Side,
        }
    }
    pub fn add_exit(&mut self, e: Exit) {
        self.exits.push(e);
    }
}

pub struct Exit {
    pub from: LID,
    pub to: LID,
    pub duration: usize,
    pub name: String,
    pub dir: LocDir,
}

#[derive(Debug, Clone, Copy)]
pub enum LocDir {
    Up,
    Down,
    Side,
}

impl Exit {
    pub fn new(f: LID, t: LID, d: usize, dir: LocDir, n: String) -> Self {
        Exit {
            from: f,
            to: t,
            duration: d,
            name: n,
            dir: dir,
        }
    }
}
