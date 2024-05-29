mod algorithm;
mod cli;
mod data;
mod definitions;

use clap::Parser;
use cli::*;
use definitions::*;

fn generate_data(
    fname: Option<String>,
    n_students: usize,
    n_courses: usize,
    n_timeslots: usize,
    n_rooms: usize,
    n_courses_per_student: usize,
    n_courses_per_prof: usize,
) {
    let con = Constraints::new(
        n_timeslots,
        n_rooms,
        n_courses,
        data::gen_professors(n_courses, n_courses_per_prof),
        data::gen_students(n_students, n_courses, n_courses_per_student),
    );

    let serded = serde_json::to_string(&con).expect("Could not serialize data");

    if let Some(fname) = fname {
        std::fs::write(fname, serded).expect("Could not write file");
    } else {
        println!("{}", serded);
    }
}

fn compute(
    population_size: usize,
    n_generations: usize,
    selection_factor: f64,
    mutation_chance: f64,
    fname: Option<String>,
    csv_output: bool,
) {
    let file_contents = match fname {
        Some(f) => std::fs::read_to_string(f).expect("Could not read file"),
        None => std::io::read_to_string(std::io::stdin()).expect("Could not read from stdin"),
    };
    let con =
        serde_json::from_str::<Constraints>(&file_contents).expect("Could not deserialize data");

    let n_pairs = (population_size as f64 * selection_factor / 2.0) as usize;
    let n_children = population_size / (n_pairs * 2);

    let initial = (0..population_size)
        .map(|_| con.make_random_tt())
        .collect::<Vec<_>>();
    let mut population = initial;
    let mut overall_best = None;
    let mut overall_best_score = usize::MAX;

    for i_generation in 1..=n_generations {
        let new_generation = con.generation(population, n_pairs, n_children, mutation_chance);
        let result = con.evaluate_generation(&new_generation);

        if let Some((mean, best_score, best)) = result {
            if csv_output {
                println!("{},{}", mean, best_score);
            } else {
                println!(
                    "Generation {}: Mean: {:.2} Best: {:.2}",
                    mean, best_score, i_generation
                );
            }

            if best_score < overall_best_score {
                overall_best = Some(best);
                overall_best_score = best_score;
            }

            if best_score == 0 {
                if !csv_output {
                    println!("Found a timetable with no penalty, stopping early..");
                } else {
                    // Don't break scripts, just output invalid data for the rest
                    for _ in i_generation..n_generations {
                        println!("-2,-2");
                    }
                }
                break;
            }
        } else if !csv_output {
            println!("Generation {}: No valid timetables", i_generation);
        } else {
            println!("-1, -1");
        }
        population = new_generation;
    }

    if csv_output {
        return;
    }

    if let Some(mut best) = overall_best {
        best.defrag();
        println!(
            "Best timetable found (Penalty: {}):\n{:?}",
            overall_best_score, best
        );
    } else {
        println!("No valid timetable found.");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.action {
        Action::Generate {
            n_students,
            n_courses,
            n_timeslots,
            n_rooms,
            n_courses_per_student,
            n_courses_per_prof,
            out_file,
        } => generate_data(
            out_file.clone(),
            *n_students,
            *n_courses,
            *n_timeslots,
            *n_rooms,
            *n_courses_per_student,
            *n_courses_per_prof,
        ),
        Action::Compute {
            population_size,
            n_generations,
            selection_factor,
            mutation_chance,
            filename,
            csv_output,
        } => compute(
            *population_size,
            *n_generations,
            *selection_factor,
            *mutation_chance,
            filename.clone(),
            *csv_output,
        ),
    }

    Ok(())
}
