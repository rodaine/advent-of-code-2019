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
        if self.0 == 0 {
            return None;
        }
        Some(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dec01_part1() {
        let sum: usize = input_to_mass("dec1.txt")
            .map(mass_to_fuel)
            .sum();

        assert_eq!(sum, 3087896);
    }


    #[test]
    fn dec01_part2() {
        let sum: usize = input_to_mass("dec1.txt")
            .flat_map(|m| FuelIterator(m))
            .sum();

        assert_eq!(sum, 4628989);
    }
}