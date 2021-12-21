use cached::proc_macro::cached;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Copy)]
struct Player {
    value: usize,
    pos: usize,
}

impl Player {
    fn play(&mut self, roll: usize) {
        self.pos += roll;
        self.pos %= 10;
        if self.pos == 0 {
            self.pos = 10;
        }
        self.value += self.pos;
    }

    fn clone_play(&self, roll: usize) -> Self {
        let mut pos = (self.pos + roll) % 10;
        pos %= 10;
        if pos == 0 {
            pos = 10;
        }
        Self {
            value: self.value + pos,
            pos,
        }
    }
}

#[derive(Debug)]
struct Dice {
    value: usize,
    rolls: usize,
}

impl Dice {
    fn new() -> Self {
        Self { value: 0, rolls: 0 }
    }

    fn roll(&mut self) -> usize {
        self.next().unwrap() + self.next().unwrap() + self.next().unwrap()
    }

    fn rolls(&self) -> usize {
        self.rolls
    }
}

impl Iterator for Dice {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        self.rolls += 1;
        self.value += 1;
        self.value %= 101;
        if self.value == 0 {
            self.value = 1;
        }
        Some(self.value)
    }
}

const MULTIVERSUM_WIN_SCORE: usize = 21;

#[cached]
fn run_universe(p1: Player, p2: Player, score: usize) -> (u128, u128) {
    if p2.value >= score {
        return (0, 1);
    }
    let mut wins = (0, 0);
    for r1 in 1..=3 {
        for r2 in 1..=3 {
            for r3 in 1..=3 {
                let roll = r1 + r2 + r3;
                let p1_next = p1.clone_play(roll);
                let (p2_win, p1_win) = run_universe(p2, p1_next, score);
                wins.0 += p1_win;
                wins.1 += p2_win;
            }
        }
    }
    wins
}

fn main() {
    let mut p1 = Player { value: 0, pos: 10 };
    let mut p2 = Player { value: 0, pos: 2 };
    let mut dice = Dice::new();
    let mut cnt = 0;
    while p1.value < 1000 && p2.value < 1000 {
        let roll: usize = dice.roll();
        if cnt % 2 == 0 {
            p1.play(roll);
        } else {
            p2.play(roll);
        }
        cnt += 1;
    }
    let res = p1.value.min(p2.value) * dice.rolls();
    println!("Game result {}", res);
    let p1 = Player { value: 0, pos: 10 };
    let p2 = Player { value: 0, pos: 2 };
    let wins = run_universe(p1, p2, MULTIVERSUM_WIN_SCORE);
    println!("Wins Player 1: {}  Wins Player 2: {}", wins.0, wins.1);
}
