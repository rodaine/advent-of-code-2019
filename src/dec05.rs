#[cfg(test)]
mod tests {
    use crate::computer::IntCodeComputer;

    #[test]
    fn parts_1_and_2() {
        let mut comp = IntCodeComputer::from_input_file("dec05.txt", vec![1]);
        assert_eq!(comp.last(), Some(9_219_874));
    }

    #[test]
    fn part2() {
        let comp = IntCodeComputer::from_input_file("dec05.txt", vec![5]);
        assert_eq!(comp.last(), Some(5_893_654));
    }
}