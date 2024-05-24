mod algorithm;
mod data;
mod definitions;

use algorithm::*;
use definitions::*;

fn main() {
    let n_students = 128;
    let n_courses = 32;
    let n_timeslots = 16;
    let n_rooms = 8;

    let n_courses_per_student = 8;
    let n_courses_per_prof = 4;
    let population_size = 1024;
    let n_generations = 3;
    let n_pairs = 8;
    let mutation_probability = 0.01;

    assert!(
        population_size % (n_pairs * 2) == 0,
        "population size must be divisible by n_pairs * 2"
    );
    let n_children = population_size / (n_pairs * 2);

    let con = Constraints::new(
        n_timeslots,
        n_rooms,
        n_courses,
        data::gen_professors(n_courses, n_courses_per_prof),
        data::gen_students(n_students, n_courses, n_courses_per_student),
    );

    let initial = (0..population_size)
        .map(|_| con.make_random_tt())
        .collect::<Vec<_>>();
    let mut population = initial;
    let mut overall_best = None;
    let mut overall_best_score = usize::MAX;

    for i_generation in 1..=n_generations {
        let new_generation = con.generation(population, n_pairs, n_children, mutation_probability);
        let (report, best_score, best) = con.evaluate_generation(&new_generation);

        println!("Generation {}: {:?}", i_generation, report);

        if best.is_some() {
            if best_score < overall_best_score {
                overall_best = best;
                overall_best_score = best_score;
            }

            if best_score == 0 {
                println!("Found a timetable with no penalty, stopping early..");
                break;
            }
        }
        population = new_generation;
    }

    if let Some(best) = overall_best {
        println!(
            "Best timetable found (Penalty: {}):\n{:?}",
            overall_best_score, best
        );
    } else {
        println!("No valid timetable found.");
    }
}
