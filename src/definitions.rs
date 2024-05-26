use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::ops::Range;

#[derive(Clone, PartialEq, Eq)]
pub struct TimeTable {
    pub n_timeslots: usize,
    pub n_rooms: usize,
    inner: Vec<Vec<usize>>,
    lookup: HashMap<usize, (usize, usize)>,
    free_slots_cache: Option<Vec<(usize, usize)>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Constraints {
    pub n_timeslots: usize,
    pub n_rooms: usize,
    professors: Vec<HashSet<usize>>,
    students: Vec<HashSet<usize>>,
    courses: Range<usize>,
}

impl TimeTable {
    fn compute_free_slots(&self) -> Vec<(usize, usize)> {
        self.inner
            .iter()
            .enumerate()
            .flat_map(|(time, timeslot)| {
                timeslot
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| **c == 0)
                    .map(move |(room, _)| (time, room))
            })
            .collect()
    }

    fn free_slots_cached(&mut self) -> &Vec<(usize, usize)> {
        if self.free_slots_cache.is_none() {
            self.free_slots_cache = Some(self.compute_free_slots());
        }

        self.free_slots_cache.as_ref().unwrap()
    }

    fn compute_lookup(&self) -> HashMap<usize, (usize, usize)> {
        let mut lookup = HashMap::<usize, (usize, usize)>::new();

        for (time, timeslot) in self.inner.iter().enumerate() {
            for (room, c) in timeslot.iter().enumerate() {
                if *c != 0 {
                    lookup.insert(*c, (time, room));
                }
            }
        }

        lookup
    }

    pub fn new(constraints: &Constraints) -> Self {
        let inner = vec![vec![0; constraints.n_rooms]; constraints.n_timeslots];
        Self {
            n_timeslots: constraints.n_timeslots,
            n_rooms: constraints.n_rooms,
            inner,
            lookup: HashMap::new(), // empty because we have only zeros in our table
            free_slots_cache: None,
        }
    }

    pub fn make_empty_copy(&self) -> Self {
        Self {
            n_timeslots: self.n_timeslots,
            n_rooms: self.n_rooms,
            inner: vec![vec![0; self.n_rooms]; self.n_timeslots],
            lookup: HashMap::new(),
            free_slots_cache: None,
        }
    }

    /// Set the course at a given time and room
    /// If the course is 0, the slot is unset
    /// If the course is already placed somewhere else, return an Err with the location and don't
    /// do anything
    pub fn set(&mut self, time: usize, room: usize, course: usize) -> Result<(), (usize, usize)> {
        if course == 0 {
            self.unset(time, room);
            return Ok(());
        }

        if let Some(c) = self.lookup.get(&course) {
            return Err(*c);
        }

        self.inner[time][room] = course;
        self.lookup.insert(course, (time, room));

        Ok(())
    }

    pub fn unset(&mut self, time: usize, room: usize) {
        let course = self.inner[time][room];
        self.lookup.remove(&course);
        self.inner[time][room] = 0;

        assert!(self.find(course).is_none());
    }

    pub fn get(&self, time: usize, room: usize) -> usize {
        self.inner[time][room]
    }

    pub fn find(&self, course: usize) -> Option<(usize, usize)> {
        self.lookup.get(&course).copied()
    }

    pub fn courses(&self) -> HashSet<usize> {
        self.lookup.keys().copied().collect()
    }

    // randomly place a course in the timetable into a free slot
    pub fn random_place(&mut self, course: usize) -> Result<(), (usize, usize)> {
        let free_slots = self.free_slots_cached();

        assert!(!free_slots.is_empty(), "No free slots left");
        let (time, room) = free_slots[rand::thread_rng().gen_range(0..free_slots.len())];
        self.set(time, room, course)
    }

    // shift the courses in all rooms to the left (reduce fragmentation)
    pub fn defrag(&mut self) {
        for time in self.inner.iter_mut() {
            time.sort_by(|a, b| {
                // Sort zeros to the end
                if *a == 0 {
                    std::cmp::Ordering::Greater
                } else if *b == 0 {
                    std::cmp::Ordering::Less
                } else {
                    a.cmp(b)
                }
            });
        }

        self.lookup = self.compute_lookup();
        self.free_slots_cache = None;
    }
}

impl std::fmt::Debug for TimeTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+ ---")?;
        for (i, time) in self.inner.iter().enumerate() {
            write!(f, "| time {: >3} |", i + 1)?;
            for room in time.iter() {
                match room {
                    0 => write!(f, "{: >4}, ", "-")?,
                    course => write!(f, "{: >4}, ", course)?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "+ ---")?;
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
        let courses = 1..n_courses + 1;

        assert!(n_timeslots * n_rooms >= n_courses); // there must be enough space for all courses
        assert!(professors.len() * n_timeslots >= n_courses); // each prof can teach one course per timeslot

        for c in courses.clone() {
            if professors.iter().any(|p| p.contains(&c)) {
                continue;
            };

            panic!("Course {} has no professor", c);
        }

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
