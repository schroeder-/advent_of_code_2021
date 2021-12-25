use std::{
    fs::File,
    io::{self, BufRead, BufWriter, Write},
    path::Path,
};

struct Alu {
    ins: Vec<Instruction>,
}
enum Target {
    W,
    Y,
    Z,
    X,
    No(isize),
    Input,
}

impl Target {
    fn from_token(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "y" => Self::Y,
            "z" => Self::Z,
            "w" => Self::W,
            _ => Self::No(s.parse().unwrap()),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::X => "x".to_owned(),
            Self::Y => "y".to_owned(),
            Self::Z => "z".to_owned(),
            Self::W => "w".to_owned(),
            Self::No(s) => s.to_string(),
            Self::Input => "input[i]".to_owned(),
        }
    }
}

enum Instruction {
    Input(Target, Target),
    Add(Target, Target),
    Mul(Target, Target),
    Div(Target, Target),
    Mod(Target, Target),
    Eq(Target, Target),
}

impl Alu {
    fn new(ins: Vec<Instruction>) -> Self {
        Self { ins }
    }

    fn generate_function(&self) -> io::Result<()> {
        let f = File::create("src/alu.rs")?;
        let mut w = BufWriter::new(f);
        writeln!(w, "pub fn finde_highest_serial_no() -> [isize; 14]{{")?;
        for i in 0..14 {
            writeln!(w, "\tfor i{} in (1..=9).rev(){{", i)?;
        }
        let strs: Vec<String> = (0..14).map(|x| format!("i{}", x)).collect();
        writeln!(w, "\tlet no = [{}];", strs.join(","))?;
        writeln!(w, "\tif calc_serial_no(no){{")?;
        writeln!(w, "\t\treturn no;")?;
        writeln!(w, "\t}}")?;
        for _ in 0..14 {
            writeln!(w, "\t}}")?;
        }
        writeln!(w, "panic!();")?;
        writeln!(w, "\t[0; 14]")?;
        writeln!(w, "}}")?;

        writeln!(w, "pub fn finde_lowest_serial_no() -> [isize; 14]{{")?;
        for i in 0..14 {
            writeln!(w, "\tfor i{} in (1..=9){{", i)?;
        }
        let strs: Vec<String> = (0..14)
            .map(|x| format!("i{}", x))
            .collect();
        writeln!(w, "\tlet no = [{}];", strs.join(","))?;
        writeln!(w, "\tif calc_serial_no(no){{")?;
        writeln!(w, "\t\treturn no;")?;
        writeln!(w, "\t}}")?;
        for _ in 0..14 {
            writeln!(w, "\t}}")?;
        }
        writeln!(w, "panic!();")?;
        writeln!(w, "\t[0; 14]")?;
        writeln!(w, "}}")?;

        writeln!(w, "fn calc_serial_no(no: [isize; 14]) -> bool {{")?;
        writeln!(w, "   let mut w: isize = 0;")?;
        writeln!(w, "   let mut x: isize = 0;")?;
        writeln!(w, "   let mut y: isize = 0;")?;
        writeln!(w, "   let mut z: isize = 0;")?;
        let mut p = 0;
        for ins in self.ins.iter() {
            match ins {
                Instruction::Input(a, _) => {
                    writeln!(w, "  {} = no[{}];", a.to_string(), p)?;
                    p += 1;
                }
                Instruction::Add(a, b) => {
                    writeln!(w, "  {0} = {0} + {1};", a.to_string(), b.to_string())?;
                }
                Instruction::Mul(a, b) => {
                    writeln!(w, "  {0} = {0} * {1};", a.to_string(), b.to_string())?;
                }
                Instruction::Div(a, b) => {
                    writeln!(w, "  {0} = {0} / {1};", a.to_string(), b.to_string())?;
                }
                Instruction::Mod(a, b) => {
                    writeln!(w, "  {0} = {0} % {1};", a.to_string(), b.to_string())?;
                }
                Instruction::Eq(a, b) => writeln!(
                    w,
                    "  {0} = ({0} == {1}) as isize;",
                    a.to_string(),
                    b.to_string()
                )?,
            };
        }
        writeln!(w, "  w == 0")?;
        writeln!(w, "}}")?;
        Ok(())
    }
}
fn load_input<P>(filename: P) -> io::Result<Vec<Instruction>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .filter_map(|l| {
            let l = l.unwrap();
            let tokens: Vec<_> = l.split(' ').collect();
            if tokens.len() == 2 {
                Some(Instruction::Input(
                    Target::from_token(tokens[1]),
                    Target::Input,
                ))
            } else {
                match tokens[0] {
                    "add" => Some(Instruction::Add(
                        Target::from_token(tokens[1]),
                        Target::from_token(tokens[2]),
                    )),
                    "mul" => Some(Instruction::Mul(
                        Target::from_token(tokens[1]),
                        Target::from_token(tokens[2]),
                    )),
                    "div" => Some(Instruction::Div(
                        Target::from_token(tokens[1]),
                        Target::from_token(tokens[2]),
                    )),
                    "mod" => Some(Instruction::Mod(
                        Target::from_token(tokens[1]),
                        Target::from_token(tokens[2]),
                    )),
                    "eql" => Some(Instruction::Eq(
                        Target::from_token(tokens[1]),
                        Target::from_token(tokens[2]),
                    )),
                    _ => None,
                }
            }
        })
        .collect())
}

fn main() {
    let inp = load_input("input.txt").unwrap();
    let alu = Alu::new(inp);
    alu.generate_function().unwrap();
}
