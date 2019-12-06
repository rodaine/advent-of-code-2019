#![cfg(test)]

use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug, Default)]
struct Space<'a> {
    nodes: HashMap<&'a str, Object<'a>>,
}

impl<'a> Space<'a> {
    fn with_capacity(capacity: usize) -> Self {
        Self { nodes: HashMap::with_capacity(capacity) }
    }

    fn add_object(&mut self, name: &'a str, orbits: &'a str) {
        if let Some(ob) = self.nodes.get_mut(name) {
            ob.orbits = orbits;
        } else {
            self.nodes.insert(name, Object { orbits });
        }
    }

    fn orbits(&self, name: &str) -> usize {
        match self.nodes.get(name) {
            None => 0,
            Some(o) => 1 + self.orbits(&o.orbits),
        }
    }

    fn total_orbits(&self) -> usize {
        self.nodes.keys()
            .map(|k| self.orbits(k))
            .sum()
    }

    fn path_to_com(&self, mut name: &'a str) -> Vec<&'a str> {
        let mut path = Vec::new();

        while let Some(ob) = self.nodes.get(name) {
            path.push(ob.orbits);
            name = ob.orbits;
        }

        path
    }

    fn shortest_path(&self, from: &str, to: &str) -> usize {
        let mut from_path = self.path_to_com(from);
        let mut to_path = self.path_to_com(to);

        while from_path.last() == to_path.last() {
            from_path.pop();
            to_path.pop();
        }

        from_path.len() + to_path.len()
    }
}

impl<'a> FromIterator<&'a str> for Space<'a> {
    fn from_iter<T: IntoIterator<Item=&'a str>>(iter: T) -> Self {
        let mut space = Space::with_capacity(0);
        for orbit in iter {
            let mut it = orbit.split(')');
            let orbits = it.next().expect("could not unwrap orbits");
            let name = it.next().expect("could not unwrap name");
            space.add_object(name, orbits);
        }
        space
    }
}

#[derive(Debug)]
struct Object<'a> {
    orbits: &'a str,
}

mod tests {
    use super::*;
    use crate::read_to_string;

    #[test]
    fn test_total_orbits() {
        let mut space = Space::with_capacity(11);
        space.add_object("B", "COM");
        space.add_object("C", "B");
        space.add_object("D", "C");
        space.add_object("E", "D");
        space.add_object("F", "E");
        space.add_object("G", "B");
        space.add_object("H", "G");
        space.add_object("I", "D");
        space.add_object("J", "E");
        space.add_object("K", "J");
        space.add_object("L", "K");

        assert_eq!(space.total_orbits(), 42);
    }

    #[test]
    fn test_shortest_path() {
        let mut space = Space::with_capacity(11);
        space.add_object("B", "COM");
        space.add_object("C", "B");
        space.add_object("D", "C");
        space.add_object("E", "D");
        space.add_object("F", "E");
        space.add_object("G", "B");
        space.add_object("H", "G");
        space.add_object("I", "D");
        space.add_object("J", "E");
        space.add_object("K", "J");
        space.add_object("L", "K");
        space.add_object("YOU", "K");
        space.add_object("SAN", "I");

        assert_eq!(space.shortest_path("YOU", "SAN"), 4);
    }

    #[test]
    fn part_1_and_2() {
        let input = read_to_string("dec06.txt");
        let space = Space::from_iter(input.lines());
        assert_eq!(space.total_orbits(), 273_985);
        assert_eq!(space.shortest_path("YOU", "SAN"), 460);
    }
}