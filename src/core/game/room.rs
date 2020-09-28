use crate::core::game::person::Person;
use std::collections::btree_map;
use std::collections::BTreeMap;
use std::ops::Index;

/// # Room
///
/// Implements the room for containing people.
/// It uses `BTreeSet` for better performance.
#[derive(Debug, Clone)]
pub struct Room {
    people: BTreeMap<u32, Person>,
}

impl Room {
    fn new() -> Room {
        Room {
            people: BTreeMap::new(),
        }
    }

    fn len(&self) -> usize {
        self.people.len()
    }

    fn get_person(&self, id: u32) -> &Person {
        self.people.index(&id)
    }

    fn get_all_people(&self) -> Vec<u32> {
        let mut v = Vec::new();
        v.reserve(self.people.len());

        for (i, _) in &self.people {
            v.push(*i);
        }

        v
    }

    fn add_person(&mut self, person: Person) {
        self.people.insert(person.get_id(), person);
    }

    fn add_people(&mut self, people: Vec<Person>) {
        for person in people {
            self.add_person(person);
        }
    }

    fn contains(&self, id: u32) -> bool {
        self.people.contains_key(&id)
    }

    fn remove_person(&mut self, id: u32) -> Option<Person> {
        self.people.remove(&id)
    }

    fn remove_people(&mut self, ids: Vec<u32>) -> Vec<Person> {
        let mut v = Vec::new();
        v.reserve(ids.len());

        for id in ids {
            if let Some(p) = self.remove_person(id) {
                v.push(p);
            }
        }

        v
    }

    fn iter(&self) -> Iter {
        Iter {
            cur: self.people.iter(),
        }
    }
}

pub struct Iter<'a> {
    cur: btree_map::Iter<'a, u32, Person>,
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
    use crate::core::game::person::Person;
    use crate::core::game::room::Room;

    #[test]
    fn add_test() {
        let mut room = Room::new();

        room.add_person(Person::new(2, 2));
        room.add_person(Person::new(1, 2));
        room.add_person(Person::new(3, 2));

        assert_eq!(room.len(), 3);

        let v = room.get_all_people();

        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);
    }

    #[test]
    fn remove_test() {
        let mut room = Room::new();

        room.add_people(vec![
            Person::new(2, 2),
            Person::new(1, 2),
            Person::new(3, 2),
        ]);

        let v = room.get_all_people();

        let r = room.remove_person(v[1]);
        assert_eq!(r, Some(Person::new(2, 2)));
        assert_eq!(room.len(), 2);

        let v = room.get_all_people();

        assert_eq!(v[0], 1);
        assert_eq!(v[1], 3);
    }

    #[test]
    fn iter_test() {
        let mut room = Room::new();

        room.add_people(vec![
            Person::new(2, 2),
            Person::new(1, 2),
            Person::new(3, 2),
            Person::new(5, 2),
            Person::new(4, 2),
        ]);

        room.remove_people(vec![4, 5]);

        for (i, p) in room.iter().enumerate() {
            assert_eq!(i + 1, p as usize);
            let r = room.get_person(p);
            assert_eq!(r.get_dest(), 2);
        }
    }

    #[test]
    fn contains_test() {
        let mut room = Room::new();

        room.add_people(vec![
            Person::new(2, 2),
            Person::new(1, 2),
            Person::new(3, 2),
            Person::new(4, 2),
        ]);

        assert!(room.contains(3));
        assert!(!room.contains(5));
    }
}
