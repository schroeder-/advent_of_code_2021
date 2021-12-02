use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Pos{
    horizontal : i32,
    depth: i32,
    aim: i32,
}

impl Pos {
    fn new() -> Self{
        Pos { horizontal: 0, depth: 0, aim: 0 }
    }

    fn handle_input(&self, inp: &str) -> Self{
        let mut res = Pos {
            horizontal: self.horizontal,
            depth: self.depth,
            aim: self.aim
        };
        let kv: Vec<&str> = inp.splitn(2, " ").collect();
        let dis = kv[1].parse::<i32>().unwrap();
        match kv[0]{
            "forward" => {
                res.horizontal += dis;
                res.depth += self.aim * dis;
            }
            "down" => {
                //res.depth += dis;
                res.aim += dis;
            }
            "up" => {
                //res.depth -= dis;
                res.aim -= dis;
            }
            _ => {}
        }
        return res;
    }

    fn handle_input_simple(&self, inp: &str) -> Self{
        let mut res = Pos {
            horizontal: self.horizontal,
            depth: self.depth,
            aim: self.aim
        };
        let kv: Vec<&str> = inp.splitn(2, " ").collect();
        let dis = kv[1].parse::<i32>().unwrap();
        match kv[0]{
            "forward" => {
                res.horizontal += dis;
            }
            "down" => {
                res.depth += dis;
            }
            "up" => {
                res.depth -= dis;
            }
            _ => {}
        }
        return res;
    }

    fn calc_total_distance(&self) -> i32{
        self.depth * self.horizontal
    }
}


fn load_file<P>(filename: P) -> io::Result<Vec<String>> where P: AsRef<Path>{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().map(Result::unwrap).collect())
}

fn main() {
    let p1 = Pos::new();
    let moves = load_file("input.txt").unwrap();
    let res = moves.iter().fold(p1, |carry, mov| {
        carry.handle_input_simple(mov)
    });
    println!("Simple Distance calc {}", res.calc_total_distance());
    let p1 = Pos::new();
    let res = moves.iter().fold(p1, |carry, mov| {
        carry.handle_input(mov)
    });
    println!("Distance calc {}", res.calc_total_distance());
}
