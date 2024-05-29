# Genetic Course Schedule Solver

This is a project I made for a Uni course.

It is a genetic algorithm that solves the problem of scheduling courses for students, measured by an evaluation function (see report.pdf).

## How to use

You will need an up-to-date Rust toolchain.

Then you can use `cargo build --release` and `target/release/genetic_course_schedule_solver` to run the program.

It will guide you through how to use it.

To reproduce my experiments, look at the saved data in `data/` and the invocations in `justfile`.

For reference, here is the synopsis of the cli:

```plain
$ target/release/genetic_course_schedule_solver --help
Usage: genetic_schedule_solver <COMMAND>

Commands:
  generate
  compute
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

$ target/release/genetic_schedule_solver help generate
Usage: genetic_schedule_solver generate [OPTIONS] <N_STUDENTS> <N_COURSES> <N_TIMESLOTS>
<N_ROOMS> <N_COURSES_PER_STUDENT> <N_COURSES_PER_PROF>

Arguments:
  <N_STUDENTS>             Number of students
  <N_COURSES>              Number of courses
  <N_TIMESLOTS>            Number of timeslots
  <N_ROOMS>                Number of rooms
  <N_COURSES_PER_STUDENT>  Number of courses each student takes
  <N_COURSES_PER_PROF>     Number of courses each professor teaches

Options:
  -o, --out-file <OUT_FILE>  Output file
  -h, --help                 Print help

$ target/release/genetic_schedule_solver help compute
Usage: genetic_schedule_solver compute [OPTIONS] <POPULATION_SIZE> <N_GENERATIONS> <SELEC
TION_FACTOR> <MUTATION_CHANCE>

Arguments:
  <POPULATION_SIZE>   Population size
  <N_GENERATIONS>     Number of generations to simulate
  <SELECTION_FACTOR>  Share of total population used to create the next one
  <MUTATION_CHANCE>   Chance of mutation

Options:
  -f, --filename <FILENAME>  Input file
  -c, --csv-output           Output as CSV
  -h, --help                 Print help

```
