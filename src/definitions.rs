use rand::Rng;
use std::collections::HashSet;
use std::ops::Range;

#[derive(Clone, PartialEq, Eq)]
pub struct TimeTable {
    pub n_timeslots: usize,
    pub n_rooms: usize,
    pub inner: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constraints {
    pub n_timeslots: usize,
    pub n_rooms: usize,
    professors: Vec<HashSet<usize>>,
    students: Vec<HashSet<usize>>,
    courses: Range<usize>,
}

impl TimeTable {
    pub fn new(constraints: &Constraints) -> Self {
        let inner = vec![vec![0; constraints.n_timeslots]; constraints.n_rooms];
        Self {
            n_timeslots: constraints.n_timeslots,
            n_rooms: constraints.n_rooms,
            inner,
        }
    }

    pub fn make_empty_copy(&self) -> Self {
        Self {
            n_timeslots: self.n_timeslots,
            n_rooms: self.n_rooms,
            inner: vec![vec![0; self.n_timeslots]; self.n_rooms],
        }
    }

    pub fn set(&mut self, time: usize, room: usize, course: usize) {
        self.inner[time][room] = course;
    }

    pub fn unset(&mut self, time: usize, room: usize) {
        self.inner[time][room] = 0;
    }

    pub fn get(&self, time: usize, room: usize) -> usize {
        self.inner[time][room]
    }

    pub fn find(&self, course: usize) -> Option<(usize, usize)> {
        assert!(course != 0);
        todo!("optimize this");
        for (time, timeslot) in self.inner.iter().enumerate() {
            for (room, c) in timeslot.iter().enumerate() {
                if *c == course {
                    return Some((time, room));
                }
            }
        }
        None
    }

    // get a new timetable that only contains the courses the given entity has
    pub fn filter(&self, entity: HashSet<usize>) -> Self {
        let mut result = self.make_empty_copy();

        for (time, timeslot) in self.inner.iter().enumerate() {
            for (room, c) in timeslot.iter().enumerate() {
                if *c != 0 && entity.contains(c) {
                    result.set(time, room, *c);
                }
            }
        }

        result
    }

    // list all courses that are in the timetable
    pub fn courses(&self) -> HashSet<usize> {
        self.inner
            .iter()
            .flat_map(|timeslot| timeslot.iter().filter_map(|c| if *c != 0 { Some(*c) } else { None }))
            .collect()
    }

    // randomly place a course in the timetable into a free slot
    pub fn random_place(&mut self, course: usize) {
        let free_slots = self
            .inner
            .iter()
            .enumerate()
            .flat_map(|(time, timeslot)| {
                timeslot
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| **c == 0)
                    .map(move |(room, _)| (time, room))
            })
            .collect::<Vec<_>>();

        let (time, room) = free_slots[rand::thread_rng().gen_range(0..free_slots.len())];
        self.set(time, room, course);
    }
}

impl std::fmt::Debug for TimeTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+ ---")?;
        for time in self.inner.iter() {
            write!(f, "| ")?;
            for room in time.iter() {
                match room {
                    0 => write!(f, "{: >4}, ", "-")?,
                    course => write!(f, "{: >4}, ", course)?,
                }
            }
            writeln!(f)?;
        }
        write!(f, "+ ---\n")?;
        Ok(())
    }
}

impl Constraints {
    pub fn new(
        n_timeslots: usize,
        n_rooms: usize,
        n_courses: usize,
        professors: Vec<HashSet<usize>>,
        students: Vec<HashSet<usize>>,
    ) -> Self {
        let courses = 1..n_courses+1;

        for c in courses {
            todo!("Check if all courses have a prof")
        }

        todo!("Check if there is a valid timetable (enough space and no prof overlap possible)");

        Self {
            n_timeslots,
            n_rooms,
            professors,
            students,
            courses,
        }
    }

    pub fn professors(&self) -> &Vec<HashSet<usize>> {
        &self.professors
    }

    pub fn students(&self) -> &Vec<HashSet<usize>> {
        &self.students
    }

    pub fn courses(&self) -> &Range<usize> {
        &self.courses
    }
}
