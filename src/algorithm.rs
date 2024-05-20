use crate::definitions::*;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

const SCORE_PAUSE_TIMESLOT: f64 = 1.0;
const SCORE_DOUBLE_BOOKED_TIMESLOT: f64 = 3.0;

impl TimeTable {
    pub fn evaluate(&self, filter: Option<&HashSet<usize>>) -> f64 {
        todo!("Professor overlap hard constraint");
        let mut score = 0.0;
        let mut started = false;
        let mut trailing_pause = 0.0;

        for time in 0..self.n_timeslots {
            let mut n_courses = 0;
            for room in 0..self.n_rooms {
                let c = self.inner[time][room];
                if c != 0 && match filter { Some(f) => f.contains(&c), _ => true } {
                    n_courses += 1;
                }
            }

            if n_courses > 0 {
                started = true;
                trailing_pause = 0.0;
            }

            if n_courses == 0 && started {
                // Add a penalty for each pause
                score += SCORE_PAUSE_TIMESLOT;
                trailing_pause += SCORE_PAUSE_TIMESLOT;
            } else if n_courses > 1 {
                // For each course after the first one, add a penalty
                score += (n_courses - 1) as f64 * SCORE_DOUBLE_BOOKED_TIMESLOT;
            }
        }

        // Do not count pauses after the last course
        score -= trailing_pause;

        score
    }

    pub fn mutate(&self, chance: f64) -> TimeTable {
        let mut tt = self.clone();

        for time in 0..self.n_timeslots {
            for room in 0..self.n_rooms {
                if thread_rng().gen_bool(chance) {
                    let course = self.get(time, room);
                    if course != 0 {
                        tt.unset(time, room);
                        tt.random_place(course);
                    }
                }
            }
        }

        tt
    }

    pub fn cross(&self, other: &TimeTable) -> TimeTable {
        let mut tt = self.clone();

        // mix the two timetables
        for time in 0..self.n_timeslots {
            for room in 0..self.n_rooms {
                if thread_rng().gen_bool(0.5) {
                    tt.inner[time][room] = other.get(time, room);
                } else {
                    tt.inner[time][room] = self.get(time, room);
                }
            }
        }

        todo!("fix this");
        // add back missing courses
        for course in self.courses().into_iter().chain(other.courses()) {
            if tt.find(course).is_none() {
                tt.random_place(course);
            }
        }

        // remove duplicate courses
        for course in tt.courses() {
            let mut found = false;
            for other_course in tt.courses() {
                if course == other_course {
                    if found {
                        let (time, room) = tt.find(course).unwrap();
                        tt.unset(time, room);
                    }
                    found = true;
                }
            }
        }

        tt
    }
}

impl Constraints {
    pub fn evaluate(&self, tt: &TimeTable) -> f64 {
        let mut score = 0.0;

        for student in self.students().iter() {
            score += tt.evaluate(Some(student))
        }

        for prof in self.professors().iter() {
            score += tt.evaluate(Some(prof)) * 3.0 // Professor inconveniences are worth more
        }

        score
    }

    pub fn make_random_tt(&self) -> TimeTable {
        let mut tt = TimeTable::new(self);

        for course in self.courses().clone() {
            tt.random_place(course);
        }

        tt
    }

    pub fn select(&self, population: Vec<TimeTable>, n: usize) -> Vec<TimeTable> {
        let mut scores = population
            .iter()
            .map(|tt| self.evaluate(tt))
            .collect::<Vec<_>>();
        scores.sort_by(f64::total_cmp);

        population
            .into_iter()
            .enumerate()
            .rev()
            .take(n)
            .map(|(_, tt)| tt)
            .collect::<Vec<_>>()
    }

    pub fn generation(
        &self,
        population: Vec<TimeTable>,
        n_fittest: usize,
        n_children: usize,
        mutation_chance: f64,
    ) -> Vec<TimeTable> {
        assert!(n_fittest % 2 == 0, "n_fittest must be even");
        let fittest = self.select(population, n_fittest);

        fittest
            .chunks(2)
            .flat_map(|pair| {
                (0..n_children).map(|_| pair[0].cross(&pair[1]).mutate(mutation_chance))
            })
            .collect::<Vec<_>>()
    }

    pub fn evaluate_generation(&self, population: &[TimeTable]) -> String {
        let valid_scores = population
            .iter()
            .map(|tt| self.evaluate(tt))
            .filter(|score| *score < 10000.0);
        let mean = (1.0 / valid_scores.clone().count() as f64) * valid_scores.sum::<f64>();
        let best = population
            .iter()
            .map(|tt| self.evaluate(tt))
            .min_by(f64::total_cmp)
            .unwrap();

        format!("Mean: {:.2}, Best: {:.2}", mean, best)
    }
}
