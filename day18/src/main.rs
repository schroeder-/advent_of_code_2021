use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
#[derive(Debug, Clone, PartialEq)]
enum SnailPart {
    Pair(Box<(SnailPart, SnailPart)>),
    Number(u32),
}

impl SnailPart {
    fn from_str(s: &str) -> SnailPart {
        let chars: Vec<_> = s.chars().collect();
        Self::from_chars(&chars, 0).1
    }

    fn from_chars(s: &[char], p: usize) -> (usize, SnailPart) {
        match s[p] {
            '[' => {
                let (p, s1) = Self::from_chars(s, p + 1);
                let (p, s2) = Self::from_chars(s, p + 1);
                (p + 1, SnailPart::Pair(Box::new((s1, s2))))
            }
            _ => (p + 1, SnailPart::Number(s[p].to_digit(10).unwrap())),
        }
    }

    fn add(&self, other: &SnailPart) -> SnailPart {
        SnailPart::Pair(Box::new((self.clone(), other.clone()))).reduce()
    }

    fn add_left(&mut self, n: u32) {
        match self {
            SnailPart::Pair(p) => p.0.add_left(n),
            SnailPart::Number(l) => *l += n,
        }
    }

    fn add_right(&mut self, n: u32) {
        match self {
            SnailPart::Pair(p) => p.1.add_right(n),
            SnailPart::Number(r) => *r += n,
        }
    }

    fn explode(&mut self, depth: usize) -> (bool, Option<u32>, Option<u32>) {
        match self {
            SnailPart::Pair(b) => {
                if let SnailPart::Number(l) = b.0 {
                    if let SnailPart::Number(r) = b.1 {
                        if depth >= 4 {
                            *self = SnailPart::Number(0);
                            return (true, Some(l), Some(r));
                        }
                    }
                }
                let (result, left, right) = b.0.explode(depth + 1);
                if result {
                    let right = if let Some(r) = right {
                        b.1.add_left(r);
                        None
                    } else {
                        right
                    };
                    (true, left, right)
                } else {
                    let (result, left, right) = b.1.explode(depth + 1);
                    if result {
                        let left = if let Some(l) = left {
                            b.0.add_right(l);
                            None
                        } else {
                            left
                        };
                        (true, left, right)
                    } else {
                        (false, None, None)
                    }
                }
            }
            _ => (false, None, None),
        }
    }

    fn reduce(mut self) -> Self {
        while self.explode(0).0 || self.split() {}
        self
    }

    fn split(&mut self) -> bool {
        match self {
            SnailPart::Pair(b) => b.0.split() || b.1.split(),
            SnailPart::Number(n) => {
                if *n >= 10 {
                    let a = *n / 2;
                    let b = *n - a;
                    *self = SnailPart::Pair(Box::new((SnailPart::Number(a), SnailPart::Number(b))));
                    true
                } else {
                    false
                }
            }
        }
    }

    fn magnitute(&self) -> u32 {
        match self {
            SnailPart::Pair(b) => 3 * b.0.magnitute() + 2 * b.1.magnitute(),
            SnailPart::Number(n) => *n,
        }
    }
}

fn load_input<P>(filename: P) -> io::Result<Vec<SnailPart>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|l| SnailPart::from_str(&l.unwrap()))
        .collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let s1 = SnailPart::from_str("[1,2]");
        assert_eq!(
            s1,
            SnailPart::Pair(Box::new((SnailPart::Number(1), SnailPart::Number(2))))
        );
    }
    #[test]
    fn test_add_simple() {
        let s1 = SnailPart::from_str("[1,2]");
        let s2 = SnailPart::from_str("[[3,4],5]");
        assert_eq!(s1.add(&s2), SnailPart::from_str("[[1,2],[[3,4],5]]"));
    }

    #[test]
    fn test_reduce() {
        let s1 = SnailPart::from_str("[[[[[9,8],1],2],3],4]");
        assert_eq!(s1.reduce(), SnailPart::from_str("[[[[0,9],2],3],4]"));
        let s1 = SnailPart::from_str("[7,[6,[5,[4,[3,2]]]]]");
        assert_eq!(s1.reduce(), SnailPart::from_str("[7,[6,[5,[7,0]]]]"));
        let s1 = SnailPart::from_str("[[6,[5,[4,[3,2]]]],1]");
        assert_eq!(s1.reduce(), SnailPart::from_str("[[6,[5,[7,0]]],3]"));
        let s1 = SnailPart::from_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        assert_eq!(
            s1.reduce(),
            SnailPart::from_str("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
        );
    }

    #[test]
    fn add_list() {
        let all = [
            SnailPart::from_str("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]"),
            SnailPart::from_str("[[[5,[2,8]],4],[5,[[9,9],0]]]"),
            SnailPart::from_str("[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]"),
            SnailPart::from_str("[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]"),
            SnailPart::from_str("[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]"),
            SnailPart::from_str("[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]"),
            SnailPart::from_str("[[[[5,4],[7,7]],8],[[8,3],8]]"),
            SnailPart::from_str("[[9,3],[[9,9],[6,[4,9]]]]"),
            SnailPart::from_str("[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]"),
            SnailPart::from_str("[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"),
        ];
        let mut it = all.iter().cloned();
        let s1 = it.next().unwrap();
        let sum = it.fold(s1, |c, s| c.add(&s));
        assert_eq!(
            sum,
            SnailPart::from_str("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
        );
    }

    #[test]
    fn test_magnitute() {
        let s1 = SnailPart::from_str("[[1,2],[[3,4],5]]");
        assert_eq!(s1.reduce().magnitute(), 143);
        let s1 = SnailPart::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(s1.reduce().magnitute(), 1384);
        let s1 = SnailPart::from_str("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(s1.reduce().magnitute(), 445);
        let s1 = SnailPart::from_str("[[[[3,0],[5,3]],[4,4]],[5,5]]");
        assert_eq!(s1.reduce().magnitute(), 791);
        let s1 = SnailPart::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        assert_eq!(s1.reduce().magnitute(), 1137);
        let s1 = SnailPart::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(s1.reduce().magnitute(), 3488);
        let s1 =
            SnailPart::from_str("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
        assert_eq!(s1.magnitute(), 4140);
    }
}

fn main() {
    let inp = load_input("input.txt").unwrap();
    let mut it = inp.iter();
    let s1 = it.next().unwrap().clone();
    let sum = it.fold(s1, |c, s| c.add(s));
    println!("Magnitue {}", sum.magnitute());
    let max_magnitute = inp
        .iter()
        .tuple_combinations()
        .map(|(s1, s2)| {
            let a = s1.add(s2).magnitute();
            let b = s2.add(s1).magnitute();
            a.max(b)
        })
        .max()
        .unwrap();
    println!("Max Magnitue combination {}", max_magnitute);
}
