use crate::definitions::*;
use std::collections::HashSet;

pub fn students() -> Vec<HashSet<usize>> {
    vec![
        HashSet::from_iter(vec![0, 1].into_iter()),
        HashSet::from_iter(vec![1, 2].into_iter()),
        HashSet::from_iter(vec![1, 2].into_iter()),
        HashSet::from_iter(vec![1, 3].into_iter()),
    ]
}

pub fn gen_people(
    n: usize,
    n_courses: usize,
    n_courses_per_person: usize,
) -> Vec<HashSet<usize>> {
    let mut rng = rand::thread_rng();

    (0..n)
        .map(|_| {
            HashSet::from_iter(
                rand::seq::index::sample(&mut rng, n_courses, n_courses_per_person)
                    .into_iter()
            )
        })
        .collect()
}

pub fn professors() -> Vec<HashSet<usize>> {
    vec![
        HashSet::from_iter(vec![0].into_iter()),
        HashSet::from_iter(vec![1].into_iter()),
        HashSet::from_iter(vec![2, 3].into_iter()),
    ]
}
