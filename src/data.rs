use crate::definitions::*;

pub fn students() -> Vec<Student> {
    vec![
        Student::new(0, vec![Course::new(0), Course::new(1)]),
        Student::new(1, vec![Course::new(1), Course::new(2)]),
        Student::new(2, vec![Course::new(1), Course::new(2)]),
        Student::new(3, vec![Course::new(1), Course::new(3)]),
    ]
}

pub fn professors() -> Vec<Professor> {
    vec![
        Professor::new(0, vec![Course::new(0)]),
        Professor::new(1, vec![Course::new(1)]),
        Professor::new(2, vec![Course::new(2), Course::new(3)]),
    ]
}
