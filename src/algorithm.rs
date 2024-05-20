use crate::definitions::*;
use rand::{thread_rng, Rng};
use std::collections::HashSet;

const SCORE_PAUSE_TIMESLOT: usize = 1;
const SCORE_DOUBLE_BOOKED_TIMESLOT: usize = 3;

impl TimeTable {
    pub fn has_professor_overlap(&self, prof: &HashSet<usize>) -> bool {
        for time in 0..self.n_timeslots {
            let mut n_courses = 0;
            for room in 0..self.n_rooms {
                let c = self.get(time, room);
                if c != 0 && prof.contains(&c) {
                    n_courses += 1;
                }
            }

            if n_courses > 1 {
                return true;
            }
        }
        false
    }

    pub fn evaluate(&self, filter: Option<&HashSet<usize>>) -> usize {
        enum PauseState { Initial, HasBefore, HasPause }

        let mut score = 0;
        let mut pause_state = PauseState::Initial;

        for time in 0..self.n_timeslots {
            let mut n_courses = 0;
            for room in 0..self.n_rooms {
                let c = self.get(time, room);
                if c != 0 && match filter { Some(f) => f.contains(&c), _ => true } {
                    n_courses += 1;
                }
            }

            // use the state machine to count pauses (class, pause, class)
            if n_courses > 0 {
                pause_state = match pause_state {
                    PauseState::Initial => PauseState::HasBefore,
                    PauseState::HasBefore => PauseState::HasBefore,
                    PauseState::HasPause => {
                        score += SCORE_PAUSE_TIMESLOT;
                        PauseState::HasBefore
                    },
                };
            } else {
                pause_state = match pause_state {
                    PauseState::Initial => PauseState::Initial,
                    PauseState::HasBefore => PauseState::HasPause,
                    PauseState::HasPause => PauseState::Initial,
                };
            }

            if n_courses > 1 {
                // For each course after the first one, add a penalty
                score += (n_courses - 1) * SCORE_DOUBLE_BOOKED_TIMESLOT;
            }
        }

        score
    }

    pub fn mutate(&self, chance: f64) -> TimeTable {
        let mut tt = self.clone();

        for time in 0..self.n_timeslots {
            for room in 0..self.n_rooms {
                if thread_rng().gen_bool(chance) {
                    let course = tt.get(time, room);
                    if course != 0 {
                        tt.unset(time, room);
                        tt.random_place(course).unwrap(); // we unset this course right before, so it will not collide
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
                let which = (if thread_rng().gen_bool(0.5) { other } else { self }).get(time, room);
                let _ = tt.set(time, room, which); // we ignore the possible error and fix missing courses later
            }
        }

        // add back missing courses
        let self_courses = self.courses();
        let tt_courses = tt.courses();
        let missing_courses = self_courses.difference(&tt_courses); // we assume self has all courses

        for course in missing_courses {
            tt.random_place(*course).unwrap(); // we only place missing courses, so no collision
        }

        tt
    }
}

impl Constraints {
    pub fn evaluate(&self, tt: &TimeTable) -> usize {
        let mut score = 0;

        for student in self.students().iter() {
            score += tt.evaluate(Some(student))
        }

        for prof in self.professors().iter() {
            score += tt.evaluate(Some(prof)) * 1000000 // Professor inconveniences are worth more
        }

        score
    }

    pub fn make_random_tt(&self) -> TimeTable {
        let mut tt = TimeTable::new(self);

        for course in self.courses().clone() {
            tt.random_place(course).unwrap(); // we insert in order into an empty time table, so no collisions will occurr
        }

        tt
    }

    pub fn select(&self, mut population: Vec<TimeTable>, n: usize) -> Vec<TimeTable> {
        population.sort_by_cached_key(|tt| self.evaluate(tt));
        population.truncate(n);

        population
    }

    pub fn generation(
        &self,
        population: Vec<TimeTable>,
        n_pairs: usize,
        n_children: usize,
        mutation_chance: f64,
    ) -> Vec<TimeTable> {
        let fittest = self.select(population, n_pairs * 2);

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
            .filter(|tt| self.professors().iter().all(|prof| !tt.has_professor_overlap(prof)))
            .map(|tt| self.evaluate(tt));

        if valid_scores.clone().count() == 0 {
            return "No valid timetables".to_string();
        }

        let mean = (1.0 / valid_scores.clone().count() as f64) * (valid_scores.clone().sum::<usize>() as f64);

        let best = valid_scores
            .min()
            .unwrap();

        format!("Mean: {:.2}, Best: {:.2}", mean, best)
    }
}
