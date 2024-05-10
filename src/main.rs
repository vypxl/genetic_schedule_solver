mod algorithm;
mod data;
mod definitions;

use algorithm::*;
use definitions::*;

fn main() {
    let con = Constraints::new(vec![4; 4], 4, data::professors(), data::students());
    let mut tt = con.make_random_tt();

    println!("{:?}: {}", tt, con.evaluate(&tt));
}
