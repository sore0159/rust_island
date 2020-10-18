#[derive(PartialEq, Debug, Clone, Copy)]
pub struct LID(pub usize);

use crate::data::big_types::Person;
use crate::data::locations::Location;

impl LID {
    pub fn new(i: usize) -> Self {
        LID(i)
    }
    pub fn get<'a>(&self, full_list: &'a [Location]) -> &'a Location {
        &full_list[self.0]
    }
    pub fn get_mut<'a>(&self, list: &'a mut [Location]) -> &'a mut Location {
        &mut list[self.0]
    }
    pub fn int_list(list: &[LID]) -> Vec<usize> {
        let mut v = Vec::with_capacity(list.len());
        list.iter().for_each(|x| v.push(x.0));
        v
    }
    pub fn get_two(list: &[Location], l1: LID, l2: LID) -> (&Location, &Location) {
        get_two(list, l1.into(), l2.into())
    }
    pub fn get_two_mut(list: &mut [Location], l1: LID, l2: LID) -> (&mut Location, &mut Location) {
        get_two_mut(list, l1.into(), l2.into())
    }

    pub fn get_multi<'a>(list: &'a [Location], lids: &'a [LID]) -> Vec<&'a Location> {
        let mut ids = LID::int_list(lids);
        get_multi(list, &mut ids)
    }
    pub fn get_multi_mut<'a>(list: &'a mut [Location], lids: &'a [LID]) -> Vec<&'a mut Location> {
        let mut ids = LID::int_list(lids);
        get_multi_mut(list, &mut ids)
    }
    pub fn get_ordered<'a>(list: &'a [Location], lids: &'a [LID]) -> Vec<&'a Location> {
        let mut gotten = Self::get_multi(list, lids);
        match_order(lids, &mut gotten);
        gotten
    }
    pub fn get_ordered_mut<'a>(list: &'a mut [Location], lids: &'a [LID]) -> Vec<&'a mut Location> {
        let mut gotten = Self::get_multi_mut(list, lids);
        match_order(lids, &mut gotten);
        gotten
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct PID(pub usize);

impl PID {
    pub fn new(i: usize) -> Self {
        PID(i)
    }
    pub fn get<'a>(&self, list: &'a [Person]) -> &'a Person {
        &list[self.0]
    }
    pub fn get_mut<'a>(&self, list: &'a mut [Person]) -> &'a mut Person {
        &mut list[self.0]
    }

    pub fn get_two(list: &[Person], l1: PID, l2: PID) -> (&Person, &Person) {
        get_two(list, l1.into(), l2.into())
    }
    pub fn get_two_mut(list: &mut [Person], l1: PID, l2: PID) -> (&mut Person, &mut Person) {
        get_two_mut(list, l1.into(), l2.into())
    }
    pub fn int_list(list: &[PID]) -> Vec<usize> {
        let mut v = Vec::with_capacity(list.len());
        list.iter().for_each(|x| v.push(x.0));
        v
    }
    pub fn get_multi<'a>(list: &'a [Person], pids: &'a [PID]) -> Vec<&'a Person> {
        let mut ids = PID::int_list(pids);
        get_multi(list, &mut ids)
    }
    pub fn get_multi_mut<'a>(list: &'a mut [Person], pids: &'a [PID]) -> Vec<&'a mut Person> {
        let mut ids = PID::int_list(pids);
        get_multi_mut(list, &mut ids)
    }
    pub fn get_ordered<'a>(list: &'a [Person], lids: &'a [PID]) -> Vec<&'a Person> {
        let mut gotten = Self::get_multi(list, lids);
        match_order(lids, &mut gotten);
        gotten
    }
    pub fn get_ordered_mut<'a>(list: &'a mut [Person], lids: &'a [PID]) -> Vec<&'a mut Person> {
        let mut gotten = Self::get_multi_mut(list, lids);
        match_order(lids, &mut gotten);
        gotten
    }
}

impl From<PID> for usize {
    fn from(p: PID) -> usize {
        p.0
    }
}
impl From<LID> for usize {
    fn from(l: LID) -> usize {
        l.0
    }
}

pub fn get_two<'a, T>(v: &'a [T], mut i1: usize, mut i2: usize) -> (&'a T, &'a T) {
    let mut flag = false;
    if i2 < i1 {
        flag = true;
        std::mem::swap(&mut i1, &mut i2);
    } else if i1 == i2 {
        panic!("get two used with i1==i2")
    }
    let (v1, v2) = v.split_at(i1 + 1);
    if flag {
        (&v2[i2 - i1 - 1], &v1[i1])
    } else {
        (&v1[i1], &v2[i2 - i1 - 1])
    }
}

pub fn get_two_mut<'a, T>(v: &'a mut [T], mut i1: usize, mut i2: usize) -> (&'a mut T, &'a mut T) {
    let mut flag = false;
    if i2 < i1 {
        flag = true;
        std::mem::swap(&mut i1, &mut i2);
    } else if i1 == i2 {
        panic!("get two used with i1==i2")
    }
    let (v1, v2) = v.split_at_mut(i1 + 1);
    if flag {
        (&mut v2[i2 - i1 - 1], &mut v1[i1])
    } else {
        (&mut v1[i1], &mut v2[i2 - i1 - 1])
    }
}

// THIS WILL NOT PRESERVE LIST ORDER IN ITS RETURN VALUES
//
pub fn get_multi<'a, T>(v: &'a [T], list: &mut [usize]) -> Vec<&'a T> {
    list.sort_unstable();
    let mut targets: Vec<&T> = Vec::with_capacity(list.len());
    let vlen = v.len();
    let mut last_i = vlen;
    let mut i_buffer = 0;
    let mut working_v: &[T] = v;
    for i in list {
        let i = *i;
        if i == last_i {
            panic!("get_multi used with matching indexes!")
        } else if i >= vlen {
            panic!("get_multi_mut used with out of bound index!")
        }
        last_i = i;
        let (v1, v2) = working_v.split_at(i - i_buffer + 1);
        i_buffer += v1.len();
        targets.push(&v1[v1.len() - 1]);
        working_v = v2;
    }
    targets
}

pub fn get_multi_mut<'a, T>(v: &'a mut [T], list: &mut [usize]) -> Vec<&'a mut T> {
    list.sort_unstable();
    let mut targets: Vec<&mut T> = Vec::with_capacity(list.len());
    let vlen = v.len();
    let mut last_i = vlen;
    let mut i_buffer = 0;
    let mut working_v: &mut [T] = v;
    for i in list {
        let i = *i;
        if i == last_i {
            panic!("get_multi_mut used with matching indexes!")
        } else if i >= vlen {
            panic!("get_multi_mut used with out of bound index!")
        }
        last_i = i;
        let (v1, v2) = working_v.split_at_mut(i - i_buffer + 1);
        i_buffer += v1.len();
        targets.push(&mut v1[v1.len() - 1]);
        working_v = v2;
    }
    targets
}

pub trait ID {
    fn get_id(&self) -> usize;
}

impl ID for LID {
    fn get_id(&self) -> usize {
        self.0
    }
}
impl ID for &LID {
    fn get_id(&self) -> usize {
        self.0
    }
}
impl ID for &mut LID {
    fn get_id(&self) -> usize {
        self.0
    }
}

impl ID for PID {
    fn get_id(&self) -> usize {
        self.0
    }
}
impl ID for &PID {
    fn get_id(&self) -> usize {
        self.0
    }
}
impl ID for &mut PID {
    fn get_id(&self) -> usize {
        self.0
    }
}

impl ID for Location {
    fn get_id(&self) -> usize {
        self.id.0
    }
}
impl ID for &Location {
    fn get_id(&self) -> usize {
        self.id.0
    }
}
impl ID for &mut Location {
    fn get_id(&self) -> usize {
        self.id.0
    }
}
impl ID for Person {
    fn get_id(&self) -> usize {
        self.id.0
    }
}
impl ID for &mut Person {
    fn get_id(&self) -> usize {
        self.id.0
    }
}
impl ID for &Person {
    fn get_id(&self) -> usize {
        self.id.0
    }
}

pub fn match_order<T: ID, K: ID>(ids: &[T], list: &mut [K]) {
    list.sort_unstable_by(|item1, item2| {
        let (i1, i2) = (item1.get_id(), item2.get_id());
        for j in ids {
            let j = j.get_id();
            if j == i1 {
                return std::cmp::Ordering::Less;
            } else if j == i2 {
                return std::cmp::Ordering::Greater;
            }
        }
        panic!("match order given list that does not contain id!")
    });
}
