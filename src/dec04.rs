use self::Digit::*;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Digit {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl From<u8> for Digit {
    fn from(d: u8) -> Self {
        match d {
            0 => Zero,
            1 => One,
            2 => Two,
            3 => Three,
            4 => Four,
            5 => Five,
            6 => Six,
            7 => Seven,
            8 => Eight,
            9 => Nine,
            _ => unreachable!(),
        }
    }
}

impl From<char> for Digit {
    fn from(c: char) -> Self {
        match c {
            '0' => Zero,
            '1' => One,
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Combo([Digit; 6]);

impl Combo {
    fn increment(&mut self) {
        if self.0[5] != Nine {
            self.0[5] = (self.0[5] as u8 + 1).into();
            return;
        }

        let mut i = 5;
        while i > 0 && self.0[i - 1] == Nine {
            i -= 1;
        }

        let d = (self.0[i - 1] as u8 + 1).into();
        for j in i - 1..6 {
            self.0[j] = d;
        }
    }

    fn matches(self, from: usize) -> usize {
        let mut ct = 0;
        for i in from..5 {
            if self.0[i] != self.0[i + 1] {
                break;
            }
            ct += 1;
        }
        ct
    }
}

impl From<&str> for Combo {
    fn from(s: &str) -> Self {
        let mut digits = [Zero; 6];

        for (idx, d) in s.chars().map(Digit::from).enumerate() {
            digits[idx] = d
        }

        Self(digits)
    }
}

fn count_valid<F>(min: Combo, max: Combo, valid: F) -> usize
    where F: Fn(Combo) -> bool {
    let mut val = min;
    let mut ct = 0;

    while val <= max {
        if valid(val) {
            ct += 1;
        }
        val.increment();
    }

    ct
}

fn valid1(combo: Combo) -> bool {
    let mut matching = false;

    let mut i = 0;
    while i < 5 {
        if combo.0[i] > combo.0[i + 1] {
            return false;
        }

        let matches = combo.matches(i);
        matching = matching || matches > 0;
        i += 1 + matches;
    }

    matching
}

fn valid2(combo: Combo) -> bool {
    let mut matching = false;

    let mut i = 0;
    while i < 5 {
        use ::std::cmp::Ordering::*;
        match combo.0[i].cmp(&combo.0[i + 1]) {
            Greater => return false,
            Equal if !matching => {
                let matches = combo.matches(i);
                i += matches;
                matching = matches == 1;
            }
            Less | Equal => i += 1,
        }
    }

    matching
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "273025-767253";

    #[test]
    fn test_valid1() {
        assert!(valid1(Combo::from("111111")));
        assert!(!valid1(Combo::from("223450")));
        assert!(!valid1(Combo::from("123789")));
        assert!(valid1(Combo::from("112233")));
        assert!(valid1(Combo::from("123444")));
        assert!(valid1(Combo::from("111122")));
    }

    #[test]
    fn test_valid2() {
        assert!(!valid2(Combo::from("111111")));
        assert!(!valid2(Combo::from("223450")));
        assert!(!valid2(Combo::from("123789")));
        assert!(valid2(Combo::from("112233")));
        assert!(!valid2(Combo::from("123444")));
        assert!(valid2(Combo::from("111122")));
    }

    #[test]
    fn test_matches() {
        let c = Combo::from("111122");
        assert_eq!(c.matches(0), 3);
        assert_eq!(c.matches(4), 1);
    }

    #[test]
    fn parts_1_and_2() {
        let mut combos = INPUT.split('-').map(Combo::from);
        let min: Combo = combos.next().expect("did not get min value");
        let max: Combo = combos.next().expect("did not get max value");

        assert_eq!(count_valid(min, max, valid1), 910);
        assert_eq!(count_valid(min, max, valid2), 598);
    }
}
