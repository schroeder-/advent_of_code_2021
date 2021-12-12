use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct CaveGraph {
    caves: HashMap<String, Vec<String>>,
}

impl CaveGraph {
    fn from_file<P>(filename: P) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        let mut caves = HashMap::new();
        io::BufReader::new(file).lines().for_each(|l| {
            let line = l.unwrap();
            let (p1, p2) = line.split_once('-').unwrap();
            caves
                .entry(p1.to_owned())
                .or_insert_with(Vec::new)
                .push(p2.to_owned());
            caves
                .entry(p2.to_owned())
                .or_insert_with(Vec::new)
                .push(p1.to_owned());
        });

        Ok(Self { caves })
    }

    fn count_all_paths(&self) -> usize {
        self.find_paths("start", &mut BTreeSet::new(), false)
    }

    fn count_all_paths_double_visited(&self) -> usize {
        self.find_paths("start", &mut BTreeSet::new(), true)
    }

    fn find_paths<'a>(
        &'a self,
        point: &'a str,
        seen: &mut BTreeSet<&'a str>,
        double_visit: bool,
    ) -> usize {
        if point == "end" {
            1
        } else {
            let contained = seen.contains(&point);
            if contained && !double_visit {
                0
            } else {
                let new = !contained
                    && point.chars().next().unwrap().is_lowercase()
                    && seen.insert(point);
                let cnt = self.caves[point]
                    .iter()
                    .filter(|&p| p != "start")
                    .map(|p| self.find_paths(p.as_str(), seen, !contained && double_visit))
                    .sum();
                if new {
                    seen.remove(&point);
                }
                cnt
            }
        }
    }
}

fn main() {
    let g = CaveGraph::from_file("input.txt").unwrap();
    println!("Paths {}", g.count_all_paths());
    println!(
        "Paths double visited {}",
        g.count_all_paths_double_visited()
    );
}
