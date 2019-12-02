use std::io::BufRead;
use crate::read_input;

fn input_to_mass(path: &str) -> impl Iterator<Item=usize> {
    read_input(path).lines()
        .map(|l| l.expect("unable to read line"))
        .map(|l| l.parse::<usize>().expect("unable to parse usize from line"))
}

fn mass_to_fuel(mass: usize) -> usize {
    if mass < 9 { return 0; }
    mass / 3 - 2
}

struct FuelIterator(usize);

impl Iterator for FuelIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = mass_to_fuel(self.0);
        match self.0 {
            0 => None,
            m => Some(m)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_to_fuel() {
        assert_eq!(mass_to_fuel(12), 2);
        assert_eq!(mass_to_fuel(14), 2);
        assert_eq!(mass_to_fuel(1969), 654);
        assert_eq!(mass_to_fuel(100_756), 33_583);
    }

    #[test]
    fn test_fuel_iterator() {
        assert_eq!(FuelIterator(14).sum::<usize>(), 2);
        assert_eq!(FuelIterator(1969).sum::<usize>(), 966);
        assert_eq!(FuelIterator(100_756).sum::<usize>(), 50_346);
    }

    #[test]
    fn part1() {
        let sum: usize = input_to_mass("dec01.txt")
            .map(mass_to_fuel)
            .sum();

        assert_eq!(sum, 3_087_896);
    }


    #[test]
    fn part2() {
        let sum: usize = input_to_mass("dec01.txt")
            .flat_map(FuelIterator)
            .sum();

        assert_eq!(sum, 4_628_989);
    }
}