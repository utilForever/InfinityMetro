use std::cmp::Ordering;

/// # Person
///
/// Implements person in game.
/// Every person have unique `id` from `Game` and `dest`(destination)
#[derive(Debug, Clone)]
pub struct Person {
    id: u32,
    dest: u32,
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Person {}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Person {
    /// **Makes new person**
    ///
    /// - `id`: unique id for every person
    /// - `dest`: destination of this person
    pub fn new(id: u32, dest: u32) -> Person {
        Person { id, dest }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_dest(&self) -> u32 {
        self.dest
    }
}

#[cfg(test)]
mod person_test {
    use crate::core::game::person::Person;
    use std::cmp::Ordering;

    #[test]
    fn eq_test() {
        let a = Person::new(1, 2);
        let b = a.clone();
        let c = Person::new(2, 3);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn ord_test() {
        let a = Person::new(1, 2);
        let b = Person::new(2, 1);

        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
    }

    #[test]
    fn get_id_test() {
        let a = Person::new(2, 3);

        assert_eq!(a.get_id(), 2);
    }

    #[test]
    fn get_dest_test() {
        let a = Person::new(2, 3);

        assert_eq!(a.get_dest(), 3);
    }
}
