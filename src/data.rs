use rand::seq::SliceRandom;
use std::collections::HashSet;

pub fn gen_students(
    n: usize,
    n_courses: usize,
    n_courses_per_student: usize,
) -> Vec<HashSet<usize>> {
    let mut rng = rand::thread_rng();

    (0..n)
        .map(|_| {
            HashSet::from_iter(
                rand::seq::index::sample(&mut rng, n_courses, n_courses_per_student)
                    .into_iter()
                    .map(|x| x + 1),
            )
        })
        .collect()
}

pub fn gen_professors(n_courses: usize, n_courses_per_prof: usize) -> Vec<HashSet<usize>> {
    assert!(
        n_courses % n_courses_per_prof == 0,
        "n_courses must be divisible by n_courses_per_prof"
    );

    let mut courses = (1..=n_courses).collect::<Vec<usize>>();
    let mut rng = rand::thread_rng();
    courses.shuffle(&mut rng);

    courses
        .chunks_exact(n_courses_per_prof)
        .map(|chunk| HashSet::from_iter(chunk.iter().copied()))
        .collect()
}
