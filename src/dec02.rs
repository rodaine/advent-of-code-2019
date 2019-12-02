use crate::read_to_string;

fn load_memory(name: &str) -> Vec<usize> {
    read_to_string(name)
        .split(',')
        .map(|a| a.parse::<usize>())
        .map(|a| a.expect("failed to parse address"))
        .collect()
}

fn execute_ops(data: &mut [usize]) -> usize {
    let max = data.len();

    for idx in (0..max).step_by(4) {
        match data[idx] {
            1 => {
                let (lhs, rhs, target) = (data[idx + 1], data[idx + 2], data[idx + 3]);
                let sum = data[lhs] + data[rhs];
                data[target] = sum;
            }
            2 => {
                let (lhs, rhs, target) = (data[idx + 1], data[idx + 2], data[idx + 3]);
                let prod = data[lhs] * data[rhs];
                data[target] = prod;
            }
            99 => return data[0],
            _ => unreachable!(),
        }
    }

    unreachable!()
}

fn setup(data: &mut [usize], noun: usize, verb: usize) {
    data[1] = noun;
    data[2] = verb;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        assert_eq!(execute_ops(&mut [1, 0, 0, 0, 99]), 2);
        assert_eq!(execute_ops(&mut [1, 1, 1, 4, 99, 5, 6, 0, 99]), 30);
        assert_eq!(execute_ops(&mut [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]), 3500);
    }

    #[test]
    fn part1() {
        let mut data = load_memory("dec02.txt");
        setup(&mut data, 12, 2);
        assert_eq!(execute_ops(&mut data), 7_594_646);
    }

    #[test]
    fn part2() {
        const TARGET: usize = 19_690_720;
        let memory = load_memory("dec02.txt");

        for noun in 0..100 {
            for verb in 0..100 {
                let mut data = memory.clone();
                setup(&mut data, noun, verb);
                if execute_ops(&mut data) == TARGET {
                    assert_eq!(100 * noun + verb, 3376);
                    return;
                }
            }
        }

        panic!("should have found a valid noun and verb");
    }
}