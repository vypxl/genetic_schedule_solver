use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    Generate {
        #[arg(help("Number of students"))]
        n_students: usize,
        #[arg(help("Number of courses"))]
        n_courses: usize,
        #[arg(help("Number of timeslots"))]
        n_timeslots: usize,
        #[arg(help("Number of rooms"))]
        n_rooms: usize,
        #[arg(help("Number of courses each student takes"))]
        n_courses_per_student: usize,
        #[arg(help("Number of courses each professor teaches"))]
        n_courses_per_prof: usize,
        #[arg(short, long, help("Output file"))]
        out_file: Option<String>,
    },
    Compute {
        #[arg(help("Population size"))]
        population_size: usize,
        #[arg(help("Number of generations to simulate"))]
        n_generations: usize,
        #[arg(help("Share of total population used to create the next one"))]
        selection_factor: f64,
        #[arg(help("Chance of mutation"))]
        mutation_chance: f64,
        #[arg(short, long, help("Input file"))]
        filename: Option<String>,
        #[arg(short, long, help("Output as CSV"))]
        csv_output: bool,
    },
}
