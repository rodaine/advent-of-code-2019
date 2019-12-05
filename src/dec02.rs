#[cfg(test)]
mod tests {
    use crate::computer::IntCodeComputer;

    fn run_noun_verb(comp: &IntCodeComputer, noun: i64, verb: i64) -> i64 {
        let mut comp = comp.clone();
        comp[1] = noun;
        comp[2] = verb;
        comp.run();
        comp[0]
    }

    #[test]
    fn part1() {
        let comp = IntCodeComputer::from_input_file("dec02.txt", 0);
        assert_eq!(run_noun_verb(&comp, 12, 2), 7_594_646);
    }

    #[test]
    fn part2() {
        const TARGET: i64 = 19_690_720;
        let comp = IntCodeComputer::from_input_file("dec02.txt", 0);

        for noun in 0..100 {
            for verb in 0..100 {
                if run_noun_verb(&comp, noun, verb) == TARGET {
                    assert_eq!(100 * noun + verb, 3376);
                    return;
                }
            }
        }

        panic!("should have found a valid noun and verb");
    }
}