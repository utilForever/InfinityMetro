use std::collections::btree_map;
use std::collections::BTreeMap;
use std::ops::Index;

/// # Room
///
/// Implements the room for containing people.
/// It uses `BTreeSet` for better performance.
#[derive(Debug, Clone)]
pub struct Room {
    people: BTreeMap<u32, u32>,
}

impl Room {
    pub fn new() -> Room {
        Room {
            people: BTreeMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.people.len()
    }

    pub fn get_people(&self, dest: u32) -> &u32 {
        self.people.index(&dest)
    }

    pub fn get_all_people(&self) -> Vec<u32> {
        let mut v = Vec::new();

        for i in self.people.keys() {
            v.push(*i);
        }

        v
    }

    pub fn add_people(&mut self, dest: u32, count: u32) {
        match self.people.get_mut(&dest) {
            Some(v) => *v += count,
            None => {
                self.people.insert(dest, count);
            }
        }
    }

    pub fn contains(&self, dest: u32) -> bool {
        self.people.contains_key(&dest)
    }

    pub fn remove_people(&mut self, dest: u32) -> Option<u32> {
        self.people.remove(&dest)
    }

    pub fn iter(&self) -> Iter {
        Iter {
            cur: self.people.iter(),
        }
    }
}

pub struct Iter<'a> {
    cur: btree_map::Iter<'a, u32, u32>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if let Some(next) = self.cur.next() {
            let (i, _) = next;
            Some(*i)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod room_test {
    use super::*;

    #[test]
    fn add_test() {
        let mut room = Room::new();

        room.add_people(2, 1);
        room.add_people(1, 1);
        room.add_people(3, 1);

        assert_eq!(room.len(), 3);

        let v = room.get_all_people();

        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);
    }

    #[test]
    fn remove_test() {
        let mut room = Room::new();

        room.add_people(2, 1);
        room.add_people(1, 1);
        room.add_people(3, 1);

        let v = room.get_all_people();

        let r = room.remove_people(v[1]);
        assert_eq!(r, Some(1));
        assert_eq!(room.len(), 2);

        let v = room.get_all_people();

        assert_eq!(v[0], 1);
        assert_eq!(v[1], 3);
    }

    #[test]
    fn iter_test() {
        let mut room = Room::new();

        room.add_people(2, 1);
        room.add_people(1, 1);
        room.add_people(3, 1);
        room.add_people(5, 1);
        room.add_people(4, 1);

        room.remove_people(5);
        room.remove_people(4);

        for (i, p) in room.iter().enumerate() {
            assert_eq!(i + 1, p as usize);
            let r = room.get_people(p);
            assert_eq!(*r, 1);
        }
    }

    #[test]
    fn contains_test() {
        let mut room = Room::new();

        room.add_people(2, 1);
        room.add_people(1, 1);
        room.add_people(3, 1);
        room.add_people(4, 1);

        assert!(room.contains(3));
        assert!(!room.contains(5));
    }
}
