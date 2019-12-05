#[cfg(test)]
mod tests {
    use crate::computer::IntCodeComputer;

    #[test]
    fn part1() {
        let comp = IntCodeComputer::from_input_file("dec05.txt", 1);
        assert_eq!(comp.last(), Some(9_219_874));
    }

    #[test]
    fn part2() {
        let comp = IntCodeComputer::from_input_file("dec05.txt", 5);
        assert_eq!(comp.last(), Some(5_893_654));
    }
}