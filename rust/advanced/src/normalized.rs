
// Many to many
// student* --- *course

struct Student {
    name: String,
}

struct Course {
    name: String,
}

impl Student {
    fn courses(&self, platform: Platform) -> Vec<String> {
        platform.enrollments.iter()
            .filter(|&e| e.student.name == self.name)
            .map(|e| e.course.name.clone())
            .collect()
    }
}

struct Enrollment<'a> {
    student: &'a Student,
    course: &'a Course,
}

impl <'a> Enrollment<'a> {
    fn new(student: &'a Student, course: &'a Course) -> Enrollment<'a> {
        Enrollment {student, course}
    }
}

struct Platform<'a> {
    enrollments: Vec<Enrollment<'a>>
}

impl <'a> Platform<'a> {
    fn new() -> Platform<'a> {
        Platform {
            enrollments: Vec::new()
        }
    }

    fn enroll(&mut self, student: &'a Student, course: &'a Course) {
        self.enrollments.push(Enrollment::new(student, course))
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn normalized() {
    let john = Student{name: "John".to_string()};
    let course = Course{name: "Rust".to_string()};

    let mut platform = Platform::new();
    platform.enroll(&john, &course);

    for c in john.courses(platform) {
        println!("John takes {}", c);
    }
}

