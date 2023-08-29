use std::io;

struct Student {
    name: String,
    scores: Vec<f32>,
}

impl Student {
    fn new(name: String) -> Student {
        Student {
            name,
            scores: Vec::new(),
        }
    }

    fn add_score(&mut self, score: f32) {
        self.scores.push(score);
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_average_score(&self) -> f32 {
        if self.scores.is_empty() {
            0.0
        } else {
            let sum: f32 = self.scores.iter().sum();
            sum / self.scores.len() as f32
        }
    }

    fn get_grade(&self) -> String {
        let average = self.get_average_score();
        if average >= 90.0 {
            String::from("A")
        } else if average >= 80.0 {
            String::from("B")
        } else if average >= 70.0 {
            String::from("C")
        } else if average >= 60.0 {
            String::from("D")
        } else {
            String::from("F")
        }
    }
}

fn main() {
    println!("Welcome, Professor!");
    println!("Please enter student names and their scores. When done, enter 'done'.");

    let mut students: Vec<Student> = Vec::new();

    loop {
        println!("Enter student name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let name = name.trim();

        if name.eq_ignore_ascii_case("done") {
            break;
        }

        let mut student = Student::new(name.to_string());

        loop {
            println!("Enter score for {} (or 'done' to finish):", student.get_name());
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();

            if input.eq_ignore_ascii_case("done") {
                break;
            }

            match input.parse::<f32>() {
                Ok(score) => {
                    student.add_score(score);
                    println!("Score added for {}", student.get_name());
                }
                Err(_) => {
                    println!("Invalid input. Please enter a valid score.");
                    continue;
                }
            }
        }

        students.push(student);
    }

    println!("Student Grades:");
    for student in &students {
        println!(
            "Student: {}, Average Score: {:.2}, Grade: {}",
            student.get_name(),
            student.get_average_score(),
            student.get_grade()
        );
    }
}
