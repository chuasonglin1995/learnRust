// ===== Defining Modules =====
// use crate::garden::vegetables::Asparagus; Then youc can only write Asparagus to use it
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
