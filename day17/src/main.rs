struct Rect {
    x1: isize,
    x2: isize,
    y1: isize,
    y2: isize,
}

struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn is_in_rec(&self, r: &Rect) -> bool {
        self.x >= r.x1 && self.x <= r.x2 && self.y >= r.y1 && self.y <= r.y2
    }

    fn overshot(&self, r: &Rect) -> bool {
        self.y < r.y1 || self.x > r.x2
    }
}

struct Velo {
    x: isize,
    y: isize,
}

impl Velo {
    fn calc_next(self) -> Self {
        let x = match self.x {
            _ if self.x > 0 => self.x - 1,
            _ if self.x < 0 => self.x + 1,
            _ => 0,
        };
        Self { x, y: self.y - 1 }
    }
}

fn calc_result(s: Velo, r: &Rect) -> Option<isize> {
    let mut max = 0;
    let mut p = Point { x: 0, y: 0 };
    let mut v = s;
    loop {
        p = Point {
            x: p.x + v.x,
            y: p.y + v.y,
        };
        v = v.calc_next();
        max = p.y.max(max);
        if p.is_in_rec(r) {
            break Some(max);
        }
        if p.overshot(r) {
            break None;
        }
    }
}

fn calc_values(rect: &Rect) -> (isize, usize) {
    let hits: Vec<_> = (rect.y1..=rect.y1.abs())
        .map(|y| {
            (1..=rect.x2)
                .filter_map(|x| calc_result(Velo { x, y }, rect))
                .collect::<Vec<isize>>()
        })
        .filter(|v| !v.is_empty())
        .collect();
    let max_y = hits.iter().filter_map(|r| r.iter().max()).max().unwrap();
    let hits_count: usize = hits.iter().map(Vec::len).sum();
    (*max_y, hits_count)
}

#[test]
fn test1() {
    let rect = Rect {
        x1: 20,
        x2: 30,
        y1: -10,
        y2: -5,
    };
    assert_eq!(calc_values(&rect), (45, 112));
}

fn main() {
    let rect = Rect {
        x1: 175,
        x2: 227,
        y1: -134,
        y2: -79,
    };
    let (max_y, m) = calc_values(&rect);
    println!("max y {}", max_y);
    println!("hits {}", m);
}
