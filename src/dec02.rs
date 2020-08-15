#[cfg(test)]
mod tests {
    use crate::computer::IntCodeComputer;

    fn run_noun_verb(comp: &mut IntCodeComputer, noun: i64, verb: i64) -> i64 {
        comp.reset(vec![]);
        comp[1] = noun;
        comp[2] = verb;
        comp.run();
        comp[0]
    }

    #[test]
    fn parts_1_and_2() {
        let mut comp = IntCodeComputer::from_input_file("dec02.txt", vec![]);
        assert_eq!(run_noun_verb(&mut comp, 12, 2), 7_594_646);

        const TARGET: i64 = 19_690_720;
        assert_eq!(run_noun_verb(&mut comp, 33, 76), TARGET);
    }

    #[test]
    fn part2() {
        const TARGET: i64 = 19_690_720;
        let mut comp = IntCodeComputer::from_input_file("dec02.txt", vec![]);
        assert_eq!(run_noun_verb(&mut comp, 33, 76,), TARGET);
    }
}