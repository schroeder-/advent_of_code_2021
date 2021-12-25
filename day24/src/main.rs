mod alu;
use alu::{finde_highest_serial_no, finde_lowest_serial_no};
fn main() {
    let th1 = std::thread::spawn(|| finde_highest_serial_no());
    let th2 = std::thread::spawn(|| finde_lowest_serial_no());
    print!("Highest ");
    for c in th1.join().unwrap() {
        print!("{}", c);
    }
    println!();
    print!("Lowest ");
    for c in th2.join().unwrap() {
        print!("{}", c);
    }
    println!();
}
