use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn load_input<P>(filename: P) -> io::Result<Vec<Vec<usize>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect())
}

struct Graph {
    matrix: Vec<Vec<usize>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos(usize, usize);

#[derive(PartialEq, Eq)]
struct Item(Pos, usize);

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Graph {
    fn get_adajcents(&self, p: &Pos) -> Vec<Pos> {
        let mut aj = Vec::new();
        let Pos(xm, ym) = self.get_len();
        if p.0 > 0 {
            aj.push(Pos(p.0 - 1, p.1));
        }
        if p.1 > 0 {
            aj.push(Pos(p.0, p.1 - 1))
        }
        if p.0 + 1 < xm {
            aj.push(Pos(p.0 + 1, p.1));
        }
        if p.1 + 1 < ym {
            aj.push(Pos(p.0, p.1 + 1));
        }
        aj
    }
    fn get_len(&self) -> Pos {
        Pos(self.matrix[0].len(), self.matrix.len())
    }
    fn get_cost(&self, p: &Pos) -> usize {
        self.matrix[p.1][p.0]
    }

    fn expanded(&self, count: usize) -> Self {
        let Pos(xm, ym) = self.get_len();
        let mut m = vec![vec![0; xm * 5]; ym * 5];
        for y in 0..ym {
            for x in 0..xm {
                for i in 0..count {
                    let v = self.matrix[y][x];
                    for j in 0..count {
                        let d = v + i + j;
                        m[y + (ym * i)][x + (xm * j)] = d % 10 + d / 10;
                    }
                }
            }
        }
        Graph { matrix: m }
    }
}

fn find_shortes_path(nodes: &Graph) -> usize {
    let mut visited = HashSet::new();
    let mut stack = BinaryHeap::new();
    stack.push(Item(Pos(0, 0), 0));
    let Pos(y_max, x_max) = nodes.get_len();
    let goal = Pos(y_max - 1, x_max - 1);
    while let Some(Item(pos, cost)) = stack.pop() {
        if pos == goal {
            return cost;
        }
        if visited.contains(&pos) {
            continue;
        }
        for p in nodes.get_adajcents(&pos).iter() {
            stack.push(Item(*p, cost + nodes.get_cost(p)));
        }
        visited.insert(pos);
    }
    0
}

fn main() {
    let inp = load_input("input.txt").unwrap();
    let start = std::time::Instant::now();
    let g = Graph { matrix: inp };
    let p1 = find_shortes_path(&g);
    let ext_g = g.expanded(5);
    let p2 = find_shortes_path(&ext_g);
    let dur = start.elapsed();
    println!("Shortes Path: {}", p1);
    println!("Shortes Path: {}", p2);
    println!("duration {}ms", dur.as_millis());
}
