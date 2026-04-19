mod student;

use student::{CourseGrade, Grade, Student, StudentDatabase};

fn main() {
    println!("=== Student Management System ===\n");

    // Create some students
    let mut alice = Student::new(
        String::from("S001"),
        String::from("Alice Johnson"),
        String::from("alice@example.com"),
    );

    let mut bob = Student::new(
        String::from("S002"),
        String::from("Bob Smith"),
        String::from("bob@example.com"),
    );

    let mut charlie = Student::new(
        String::from("S003"),
        String::from("Charlie Brown"),
        String::from("charlie@example.com"),
    );

    // Add grades to Alice
    alice.add_grade(CourseGrade::new(
        String::from("IS4010"),
        String::from("App Dev with AI"),
        3,
        Grade::A,
    ));

    alice.add_grade(CourseGrade::new(
        String::from("IS3050"),
        String::from("Database Design"),
        3,
        Grade::A,
    ));

    alice.add_grade(CourseGrade::new(
        String::from("IS2000"),
        String::from("Intro to IS"),
        3,
        Grade::B,
    ));

    // Add grades to Bob
    bob.add_grade(CourseGrade::new(
        String::from("IS4010"),
        String::from("App Dev with AI"),
        3,
        Grade::B,
    ));

    bob.add_grade(CourseGrade::new(
        String::from("CS2100"),
        String::from("Data Structures"),
        4,
        Grade::B,
    ));

    // Add grades to Charlie
    charlie.add_grade(CourseGrade::new(
        String::from("IS3050"),
        String::from("Database Design"),
        3,
        Grade::C,
    ));

    // Create database
    let mut db = StudentDatabase::new();

    // Add students
    match db.add_student(alice) {
        Ok(()) => println!("✓ Added Alice"),
        Err(e) => println!("✗ Error: {}", e),
    }

    match db.add_student(bob) {
        Ok(()) => println!("✓ Added Bob"),
        Err(e) => println!("✗ Error: {}", e),
    }

    match db.add_student(charlie) {
        Ok(()) => println!("✓ Added Charlie"),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Database statistics
    println!("\n=== Database Statistics ===");
    println!("Total students: {}", db.student_count());
    println!("Average GPA: {:.2}", db.average_gpa());

    // List all students
    println!("\n=== All Students ===");
    for student in db.list_students() {
        println!(
            "  {} - {} (Credits: {}, GPA: {:.2}, Standing: {})",
            student.id,
            student.name,
            student.credits_earned,
            student.calculate_gpa(),
            student.class_standing()
        );
    }

    // Show detailed info for a specific student
    println!("\n=== Student Details (Alice) ===");
    if let Some(student) = db.find_student("S001") {
        println!("Name: {}", student.name);
        println!("Email: {}", student.email);
        println!("Credits: {}", student.credits_earned);
        println!("GPA: {:.2}", student.calculate_gpa());
        println!("Standing: {}", student.class_standing());
        println!("Can graduate? {}", student.can_graduate());

        println!("\nTranscript:");
        for course in &student.grades {
            println!(
                "  {} ({}): {:?} - {:.1} quality points",
                course.course_code,
                course.credits,
                course.grade,
                course.quality_points()
            );
        }
    }

    // Test grade parsing
    println!("\n=== Grade Parsing ===");
    match Grade::from_string("A") {
        Some(grade) => {
            println!("Parsed 'A': {:?}", grade);
            println!("GPA points: {}", grade.to_gpa_points());
            println!("Passing? {}", grade.is_passing());
        }
        None => println!("Invalid grade"),
    }
}
