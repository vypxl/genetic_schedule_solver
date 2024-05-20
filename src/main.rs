mod algorithm;
mod data;
mod definitions;

use algorithm::*;
use definitions::*;

fn main() {
    let con = Constraints::new(
        128,
        16,
        256,
        data::gen_people(128, 16, 8),
        data::gen_people(16, 16, 4),
    );

    let initial = (0..128).map(|_| con.make_random_tt()).collect::<Vec<_>>();
    let mut population = initial;

    for i_generation in 0..50 {
        let new_generation = con.generation(population, 16, 8, 0.05);
        println!(
            "Generation {}: {:?}",
            i_generation,
            con.evaluate_generation(&new_generation)
        );
        population = new_generation;
    }
}
