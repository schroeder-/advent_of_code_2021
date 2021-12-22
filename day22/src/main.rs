use hashbrown::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
#[derive(Debug, Clone)]
struct Cube {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
    state: bool,
}

impl Cube {
    fn volume(&self) -> isize {
        (self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)
    }
    fn intersect(lhs: &Self, rhs: &Self, state: bool) -> Option<Self> {
        let c = Self {
            x: (lhs.x.0.max(rhs.x.0), lhs.x.1.min(rhs.x.1)),
            y: (lhs.y.0.max(rhs.y.0), lhs.y.1.min(rhs.y.1)),
            z: (lhs.z.0.max(rhs.z.0), lhs.z.1.min(rhs.z.1)),
            state,
        };
        if c.x.0 <= c.x.1 && c.y.0 <= c.y.1 && c.z.0 <= c.z.1 {
            Some(c)
        } else {
            None
        }
    }
}

fn parse_input<P>(path: P) -> std::io::Result<Vec<Cube>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let re = regex::Regex::new(
        r"^(on|off) x=([\-0-9]*)..([\-0-9]*),y=([\-0-9]*)..([\-0-9]*),z=([\-0-9]*)..([\-0-9]*)$",
    )
    .expect("Error regex is invalid");
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .filter_map(|l| {
            if let Ok(l) = l {
                if let Some(c) = re.captures(&l) {
                    let state = &c[1] == "on";
                    let values: Vec<isize> = c
                        .iter()
                        .skip(2)
                        .filter(Option::is_some)
                        .map(|c| c.unwrap().as_str().parse().unwrap())
                        .collect();
                    if let [x1, x2, y1, y2, z1, z2] = values[..] {
                        Some(Cube {
                            x: (x1, x2),
                            y: (y1, y2),
                            z: (z1, z2),
                            state,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect())
}

fn calc_cubes(cmds: &[Cube]) -> usize {
    let mut cbs = HashSet::new();
    for Cube { x, y, z, state } in cmds {
        if !(x.1 < -50 || x.0 > 50 || y.1 < -50 || y.0 > 50 || z.1 < -50 || z.0 > 50) {
            let x = x.clamp(&(-50, -50), &(50, 50));
            let y = y.clamp(&(-50, -50), &(50, 50));
            let z = z.clamp(&(-50, -50), &(50, 50));
            for x in x.0..=x.1 {
                for y in y.0..=y.1 {
                    for z in z.0..=z.1 {
                        if *state {
                            cbs.insert((x, y, z));
                        } else {
                            cbs.remove(&(x, y, z));
                        }
                    }
                }
            }
        }
    }
    cbs.len()
}

fn calc_cubes_complex(cmds: &[Cube]) -> usize {
    let mut cubes = Vec::new();
    for c in cmds {
        let new_cubes: Vec<_> = cubes
            .iter()
            .filter_map(|oc| Cube::intersect(c, oc, !oc.state))
            .collect();
        if c.state {
            cubes.push(c.clone());
        }
        cubes.extend(new_cubes);
    }

    cubes
        .iter()
        .map(|c| {
            let v = c.volume();
            if c.state {
                v
            } else {
                -v
            }
        })
        .sum::<isize>() as usize
}

#[test]
fn expl1() {
    let mut inp = vec![Cube {
        x: (10, 12),
        y: (10, 12),
        z: (10, 12),
        state: true,
    }];
    assert_eq!(calc_cubes(&inp), 27);
    inp.push(Cube {
        x: (11, 13),
        y: (11, 13),
        z: (11, 13),
        state: true,
    });
    assert_eq!(calc_cubes(&inp), 27 + 19);
    inp.push(Cube {
        x: (9, 11),
        y: (9, 11),
        z: (9, 11),
        state: false,
    });
    assert_eq!(calc_cubes(&inp), 27 + 19 - 8);
    inp.push(Cube {
        x: (10, 10),
        y: (10, 10),
        z: (10, 10),
        state: true,
    });
    assert_eq!(calc_cubes(&inp), 39);
}

#[test]
fn simple() {
    let inp = parse_input("simple.txt").unwrap();
    assert_eq!(calc_cubes(&inp), 590784);
}

fn main() {
    let inp = parse_input("input.txt").unwrap();
    println!("cubes on part a {}", calc_cubes(&inp));
    println!("cubes on part b {}", calc_cubes_complex(&inp));
}
