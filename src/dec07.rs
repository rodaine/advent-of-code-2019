use crate::computer::IntCodeComputer;
use std::i64;

macro_rules! perm_range {
    ($($x:ident),+) => { (0..5)$(.filter(|x| x != &$x))* }
}

fn each_permutation<F>(mut f: F) where F: FnMut([i64;5]) {
    for a in 0..5 {
        for b in perm_range!(a) {
            for c in perm_range!(a, b) {
                for d in perm_range!(a, b,c) {
                    for e in perm_range!(a, b, c, d) {
                        f([a, b, c, d, e])
                    }
                }
            }
        }
    }
}

fn thruster_signal(comp: &mut IntCodeComputer, sequence: [i64;5]) -> i64 {
    let mut output = 0;

    for idx in 0..5 {
        comp.reset(vec![sequence[idx], output]);

        let out: Vec<i64> = comp.collect();
        assert_eq!(out.len(), 1, "unexpected extra output");

        output = out[0];
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perms() {
        let mut ct = 0;
        each_permutation(|_| ct += 1 );
        assert_eq!(ct, 120);
    }

    #[test]
    fn part_1() {
//        let mut comp = IntCodeComputer::from_input_file("dec07.txt", vec![]);
        let mut comp = IntCodeComputer::new(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0], vec![]);
//        let mut comp = IntCodeComputer::new(vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0], vec![]);
        let mut max_signal = 0;

        each_permutation(|seq| {
            println!("seq: {:?}", seq);
            let signal = thruster_signal(&mut comp, seq);
            println!("sig: {}", signal);
            max_signal = i64::max(max_signal, signal);
        });

        println!("{}", max_signal)
    }

    #[test]
    fn part_2() {}
}