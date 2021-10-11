
// Many to many
// student* --- *course

use std::rc::Rc;
use std::cell::RefCell;

/*
 * This is a bad design
 */

struct Student {
    name: String,
    courses: Vec<Rc<RefCell<Course>>>
}

impl Student {
    fn new(name: &str) -> Student {
        Student {
            name: name.into(),
            courses: Vec::new()
        }
    }
}

struct Course {
    name: String,
    students: Vec<Rc<RefCell<Student>>>
}

impl Course {
    fn new(name: &str) -> Course {
        Course {
            name: name.into(),
            students: Vec::new()
        }
    }

    fn add_student(course: Rc<RefCell<Course>>, student: Rc<RefCell<Student>>) {
        student.borrow_mut().courses.push(course.clone());
        course.borrow_mut().students.push(student);
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn circular_references() {
    let john = Rc::new(RefCell::new(Student::new("John")));
    let course = Rc::new(RefCell::new(Course::new("Rust")));

    Course::add_student(course, john);
}
