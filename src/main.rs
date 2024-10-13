#![allow(clippy::upper_case_acronyms, non_camel_case_types)]

use crate::types::Course;
use std::env;

mod calculator;
mod configuration;
mod types;

fn main() {
    println!("Calculating TA hours for courses...");
    let args: Vec<String> = env::args().collect();
    let courses = read_input_file(args.get(1).unwrap());
    println!("Found {} courses to evaluate.", courses.len());

    calculate_ta_hours_and_write_output(courses);
}

fn calculate_ta_hours_and_write_output(courses: Vec<Course>) {
    let mut wtr = csv::Writer::from_path("TA-Allocations.csv").unwrap();
    wtr.write_record(["Course", "Enrollment", "TA Allocation"])
        .unwrap();

    for c in courses {
        let ta_allocation =
            calculator::calculate_ta_hours(&c.course_name, c.has_lab, c.enrollment, 1.0);
        wtr.write_record(&[
            c.course_name,
            c.enrollment.to_string(),
            ta_allocation.to_string(),
        ])
        .unwrap();
    }
}

fn read_input_file(path: &String) -> Vec<Course> {
    let mut courses: Vec<Course> = Vec::new();
    let mut rdr = csv::Reader::from_path(path).unwrap();
    for result in rdr.records() {
        let record = result.unwrap();
        let course = Course {
            course_name: record.get(0).unwrap().trim().to_string(),
            enrollment: record.get(2).unwrap().trim().parse().unwrap(),
            has_lab: record.get(3).unwrap().trim() == "y",
        };
        courses.push(course);
    }
    courses
}

#[cfg(test)]
mod tests {
    use crate::read_input_file;

    #[test]
    fn parse_example_input_file() {
        let input_file = String::from("test_files/simple.csv");
        let courses = read_input_file(&input_file);

        assert_eq!(courses.len(), 1);
        assert_eq!(courses.first().unwrap().course_name, "ECE150");
        assert_eq!(courses.first().unwrap().enrollment, 450);
        assert_eq!(courses.first().unwrap().has_lab, true);
    }
}
