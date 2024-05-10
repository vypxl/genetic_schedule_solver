use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Course {
    pub id: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Room {
    pub id: usize,
    pub capacity: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Professor {
    pub id: usize,
    pub courses: Vec<Course>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Student {
    pub id: usize,
    pub courses: Vec<Course>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    which: usize,
}

pub type TimeSlot = Vec<Option<Course>>;

#[derive(Clone, PartialEq, Eq)]
pub struct TimeTable {
    pub n_rooms: usize,
    pub n_timeslots: usize,
    pub inner: Vec<TimeSlot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constraints {
    pub n_rooms: usize,
    pub n_timeslots: usize,
    pub room_capacities: Vec<usize>,
    professors: Vec<Professor>,
    students: Vec<Student>,
    courses: Vec<Course>,
}

impl Course {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

impl Room {
    pub fn new(id: usize, capacity: usize) -> Self {
        Self { id, capacity }
    }
}

impl Professor {
    pub fn empty(id: usize) -> Self {
        Self {
            id,
            courses: Vec::new(),
        }
    }

    pub fn new(id: usize, courses: Vec<Course>) -> Self {
        Self { id, courses }
    }
}

impl Student {
    pub fn empty(id: usize) -> Self {
        Self {
            id,
            courses: Vec::new(),
        }
    }

    pub fn new(id: usize, courses: Vec<Course>) -> Self {
        Self { id, courses }
    }
}

impl Time {
    pub fn new(which: usize) -> Self {
        Self { which }
    }
}

pub trait HasCourses {
    fn courses(&self) -> &Vec<Course>;
}

impl HasCourses for &Professor {
    fn courses(&self) -> &Vec<Course> {
        &self.courses
    }
}

impl HasCourses for &Student {
    fn courses(&self) -> &Vec<Course> {
        &self.courses
    }
}

impl TimeTable {
    pub fn new(constraints: &Constraints) -> Self {
        let inner = vec![vec![None; constraints.n_rooms]; constraints.n_timeslots];
        Self {
            n_rooms: constraints.n_rooms,
            n_timeslots: constraints.n_timeslots,
            inner,
        }
    }

    pub fn make_empty_copy(&self) -> Self {
        Self {
            n_rooms: self.n_rooms,
            n_timeslots: self.n_timeslots,
            inner: vec![vec![None; self.n_rooms]; self.n_timeslots],
        }
    }

    pub fn set(&mut self, time: usize, room: usize, course: Course) {
        self.inner[time][room] = Some(course);
    }

    pub fn unset(&mut self, time: usize, room: usize) {
        self.inner[time][room] = None;
    }

    pub fn get(&self, time: usize, room: usize) -> Option<Course> {
        self.inner[time][room]
    }

    pub fn find(&self, course: Course) -> Option<(usize, usize)> {
        for (time, timeslot) in self.inner.iter().enumerate() {
            for (room, c) in timeslot.iter().enumerate() {
                if let Some(c) = c {
                    if *c == course {
                        return Some((time, room));
                    }
                }
            }
        }
        None
    }

    // get a new timetable that only contains the courses the given entity has
    pub fn filter(&self, entity: impl HasCourses) -> Self {
        let mut result = Self {
            n_rooms: self.n_rooms,
            n_timeslots: self.n_timeslots,
            inner: vec![vec![None; self.n_rooms]; self.n_timeslots],
        };

        for (time, timeslot) in self.inner.iter().enumerate() {
            for (room, c) in timeslot.iter().enumerate() {
                if let Some(c) = c {
                    if entity.courses().contains(c) {
                        result.set(time, room, *c);
                    }
                }
            }
        }

        result
    }

    // list all courses that are in the timetable
    pub fn courses(&self) -> Vec<Course> {
        self.inner
            .iter()
            .flat_map(|timeslot| timeslot.iter().filter_map(|c| *c))
            .collect()
    }

    // randomly place a course in the timetable into a free slot
    pub fn random_place(&mut self, course: Course) {
        let free_slots = self
            .inner
            .iter()
            .enumerate()
            .flat_map(|(time, timeslot)| {
                timeslot
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| c.is_none())
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
                    Some(course) => write!(f, "{: >4}, ", course.id)?,
                    None => write!(f, "{: >4}, ", "-")?,
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
        room_capacities: Vec<usize>,
        n_timeslots: usize,
        professors: Vec<Professor>,
        students: Vec<Student>,
    ) -> Self {
        let courses = professors
            .iter()
            .flat_map(|prof| prof.courses.iter())
            .chain(students.iter().flat_map(|student| student.courses.iter()))
            .cloned()
            .collect();
        Self {
            n_rooms: room_capacities.len(),
            n_timeslots,
            room_capacities,
            professors,
            students,
            courses,
        }
    }

    pub fn professors(&self) -> &Vec<Professor> {
        &self.professors
    }

    pub fn students(&self) -> &Vec<Student> {
        &self.students
    }

    pub fn courses(&self) -> &Vec<Course> {
        &self.courses
    }
}
