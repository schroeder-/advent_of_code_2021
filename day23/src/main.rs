use hashbrown::HashSet;
use std::{
    collections::BinaryHeap,
    fs::File,
    hash::Hash,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, PartialOrd, Ord)]
enum Elements {
    Amber,
    Bronze,
    Copper,
    Dessert,
    Wall,
    Empty,
}

impl Elements {
    fn get_energy(&self) -> usize {
        match self {
            Elements::Amber => 1,
            Elements::Bronze => 10,
            Elements::Copper => 100,
            Elements::Dessert => 1000,
            Elements::Wall => 0xFFFFFF,
            Elements::Empty => 0,
        }
    }
    fn as_char(&self) -> char {
        match self {
            Elements::Amber => 'A',
            Elements::Bronze => 'B',
            Elements::Copper => 'C',
            Elements::Dessert => 'D',
            Elements::Wall => '#',
            Elements::Empty => '.',
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'A' => Elements::Amber,
            'B' => Elements::Bronze,
            'C' => Elements::Copper,
            'D' => Elements::Dessert,
            '.' => Elements::Empty,
            _ => Elements::Wall,
        }
    }
    fn get_room(&self) -> PosSz {
        match self {
            Elements::Amber => 3,
            Elements::Bronze => 5,
            Elements::Copper => 7,
            Elements::Dessert => 9,
            _ => 0,
        }
    }
}

type PosSz = u8;
type Pos = (PosSz, PosSz);

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Player {
    pos: (PosSz, PosSz),
    elm: Elements,
}

#[derive(Debug, Clone)]
struct State {
    map: Vec<Vec<Elements>>,
    costs: usize,
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.map.hash(state);
        self.costs.hash(state);
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.costs).cmp(&(self.costs))
    }
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.costs == other.costs
    }
}
impl Eq for State {}

struct Game {
    hallways: [PosSz; 7],
    start_state: State,
    room_height: usize,
}

impl Game {
    fn new(map: Vec<Vec<Elements>>) -> Self {
        let start_state = State {
            map: map.clone(),
            costs: 0,
        };
        let hallways = [1, 2, 4, 6, 8, 10, 11];
        let room_height = map.len() - 3;
        assert!(room_height == 2);
        Self {
            hallways,
            start_state,
            room_height,
        }
    }

    fn print(&self, s: &State) {
        for row in &s.map {
            for v in row {
                print!("{}", v.as_char());
            }
            println!();
        }
    }

    fn finished(&self, state: &State) -> bool {
        for y in 0..self.room_height {
            for x in [3u8, 5, 7, 9] {
                let e = &state.map[y + 2][x as usize];
                if *e == Elements::Empty || e.get_room() != x {
                    return false;
                }
            }
        }
        true
    }

    fn can_move_and_energy(&self, state: &State, start: Pos, end: Pos) -> Option<usize> {
        let mut steps = 0;
        let mut start = start;
        loop {
            if steps > 10 {
                panic!();
            }
            if start.0 == end.0 && start.1 == end.1 {
                break if steps > 0 { Some(steps) } else { None };
            }
            steps += 1;
            if start.0 == end.0 || start.1 != 1 {
                if end.1 > start.1 {
                    start.1 += 1;
                } else {
                    start.1 -= 1;
                }
            } else {
                if end.0 > start.0 {
                    start.0 += 1;
                } else {
                    start.0 -= 1;
                }
            }
            if state.map[start.1 as usize][start.0 as usize] != Elements::Empty {
                break None;
            }
        }
    }

    fn generate_next_states(&self, state: &State) -> Vec<State> {
        let mut v = Vec::new();
        // Check hallways
        for x in &self.hallways {
            let y = 1;
            let e = state.map[y][*x as usize];
            let nx = e.get_room();
            for ny in 0..self.room_height as u8 {
                let ny = 2 + ny;
                if let Some(steps) = self.can_move_and_energy(state, (*x, y as u8), (nx, ny)) {
                    let mut s = State {
                        map: state.map.clone(),
                        costs: state.costs + e.get_energy() * steps,
                    };
                    s.map[y][*x as usize] = Elements::Empty;
                    s.map[ny as usize][nx as usize] = e;
                    v.push(s);
                    break;
                }
            }
        }
        // Check rooms
        for x in [3, 5, 7, 9] {
            let mut all_ok = true;
            for y in (0..self.room_height).rev() {
                let y = y + 2;
                let e = state.map[y as usize][x as usize];
                if e.get_room() != x {
                    all_ok = false;
                    if y - 1 > 1 && state.map[y as usize][x as usize] != Elements::Empty {
                        // Try move to other room
                        let nx = e.get_room();
                        for ny in (0..self.room_height as u8).rev() {
                            let ny = 2 + ny;
                            if let Some(steps) =
                                self.can_move_and_energy(state, (x, y as u8), (nx, ny))
                            {
                                let mut s = State {
                                    map: state.map.clone(),
                                    costs: state.costs + e.get_energy() * steps,
                                };
                                s.map[y][x as usize] = Elements::Empty;
                                s.map[ny as usize][nx as usize] = e;
                                v.push(s);
                                continue;
                            }
                        }
                        // Try move to hallway
                        for nx in self.hallways {
                            if let Some(steps) =
                                self.can_move_and_energy(state, (x, y as u8), (nx, 1))
                            {
                                let mut s = State {
                                    map: state.map.clone(),
                                    costs: state.costs + e.get_energy() * steps,
                                };
                                s.map[y][x as usize] = Elements::Empty;
                                s.map[1][nx as usize] = e;
                                v.push(s);
                            }
                        }
                    }
                } else if !all_ok {
                    // Needs to move to hallway to get others out
                    for nx in self.hallways {
                        if let Some(steps) = self.can_move_and_energy(state, (x, y as u8), (nx, 1))
                        {
                            let mut s = State {
                                map: state.map.clone(),
                                costs: state.costs + e.get_energy() * steps,
                            };
                            s.map[y][x as usize] = Elements::Empty;
                            s.map[1][nx as usize] = e;
                            v.push(s);
                        }
                    }
                }
            }
        }
        v
    }

    fn play_game(&self) -> usize {
        let mut used = HashSet::<State>::new();
        let mut stack = BinaryHeap::new();
        stack.push(self.start_state.clone());
        while let Some(s) = stack.pop() {
            //self.print(&s);
            if self.finished(&s) {
                return s.costs;
            }
            for moves in self.generate_next_states(&s) {
                if used.insert(moves.clone()) {
                    stack.push(moves);
                }
            }
        }
        0
    }
}

fn load_input<P>(filename: P) -> io::Result<Vec<Vec<Elements>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().chars().map(Elements::from_char).collect())
        .collect())
}

fn main() {
    let inp = load_input("input.txt").unwrap();
    let amp = Game::new(inp);
    let res = amp.play_game();
    println!("{}", res);
    let mut inp = load_input("input.txt").unwrap();
    inp.insert(
        4,
        vec![
            Elements::Wall,
            Elements::Wall,
            Elements::Wall,
            Elements::Dessert,
            Elements::Wall,
            Elements::Copper,
            Elements::Wall,
            Elements::Bronze,
            Elements::Wall,
            Elements::Amber,
            Elements::Wall,
            Elements::Wall,
            Elements::Wall,
        ],
    );
    inp.insert(
        5,
        vec![
            Elements::Wall,
            Elements::Wall,
            Elements::Wall,
            Elements::Dessert,
            Elements::Wall,
            Elements::Bronze,
            Elements::Wall,
            Elements::Amber,
            Elements::Wall,
            Elements::Bronze,
            Elements::Wall,
            Elements::Wall,
            Elements::Wall,
        ],
    );
    let amp = Game::new(inp);
    let res = amp.play_game();
    println!("{}", res);
}
