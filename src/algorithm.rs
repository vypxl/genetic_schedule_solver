use crate::definitions::*;
use rand::{thread_rng, Rng};

const SCORE_PAUSE_TIMESLOT: f64 = 1.0;
const SCORE_DOUBLE_BOOKED_TIMESLOT: f64 = 3.0;
const SCORE_DOUBLE_BOOKED_TIMESLOT_PROFESSOR: f64 = 1e9;

impl TimeTable {
    pub fn evaluate(&self, is_professor: bool) -> f64 {
        let mut score = 0.0;
        let mut started = false;
        let mut trailing_pause = 0.0;

        for time in 0..self.n_timeslots {
            let mut n_courses = 0;
            for room in 0..self.n_rooms {
                if self.inner[time][room].is_some() {
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

                // if we are evaluating a professor, add an additional penalty, because professors
                // can't teach two courses at the same time
                if is_professor {
                    score += SCORE_DOUBLE_BOOKED_TIMESLOT_PROFESSOR;
                }
            }
        }

        // Do not count pauses after the last course
        score -= trailing_pause;

        println!("{}", score);
        score
    }

    pub fn mutate(&self, chance: f64) -> TimeTable {
        let mut tt = self.clone();

        for time in 0..self.n_timeslots {
            for room in 0..self.n_rooms {
                if thread_rng().gen_bool(chance) {
                    if let Some(course) = self.get(time, room) {
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
                }
            }
        }

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
            let student_tt = tt.filter(student);
            score += student_tt.evaluate(false)
        }

        for prof in self.professors().iter() {
            let prof_tt = tt.filter(prof);
            score += prof_tt.evaluate(true)
        }

        score
    }

    pub fn make_random_tt(&self) -> TimeTable {
        let mut tt = TimeTable::new(self);

        for course in self.courses() {
            tt.random_place(*course);
        }

        tt
    }

    pub fn select(&self, population: Vec<TimeTable>) -> Vec<TimeTable> {
        let mut scores = population
            .iter()
            .map(|tt| self.evaluate(tt))
            .collect::<Vec<_>>();
        scores.sort_by(f64::total_cmp);

        let cutoff = scores[population.len() / 10];

        population
            .into_iter()
            .enumerate()
            .filter(|(i, _)| scores[*i] < cutoff)
            .map(|(_, tt)| tt.clone())
            .collect::<Vec<_>>()
    }
}
