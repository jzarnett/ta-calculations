#![allow(clippy::upper_case_acronyms, non_camel_case_types)]

use crate::calculator::check_for_special_case;
use crate::types::Course;
use std::env;

mod calculator;
mod configuration;
mod specialcases;
mod types;

fn main() {
    println!("Calculating TA hours for courses...");
    let args: Vec<String> = env::args().collect();
    let courses = read_input_file(args.get(1).unwrap());
    println!("Found {} courses to evaluate.", courses.len());

    let ta_hours = calculate_ta_hours_for_courses(courses);
    write_output(ta_hours);
}

fn calculate_ta_hours_for_courses(courses: Vec<Course>) -> Vec<(Course, f32)> {
    let mut result: Vec<(Course, f32)> = Vec::new();
    for c in courses {
        let ta_allocation =
            calculator::calculate_ta_hours(&c.course_name, c.has_lab, c.enrollment, c.unit_weight);
        let ta_allocation = check_for_special_case(&c.course_name, ta_allocation);
        result.push((c, ta_allocation));
    }
    result
}

fn write_output(courses: Vec<(Course, f32)>) {
    let mut wtr = csv::Writer::from_path("TA-Allocations.csv").unwrap();
    wtr.write_record(["Course", "Enrollment", "TA Allocation"])
        .unwrap();

    for c in courses {
        wtr.write_record(&[c.0.course_name, c.0.enrollment.to_string(), c.1.to_string()])
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
            unit_weight: record.get(4).unwrap().trim().parse().unwrap(),
        };
        courses.push(course);
    }
    courses
}

#[cfg(test)]
mod tests {
    use crate::calculator::calculate_ta_hours;
    use crate::{calculate_ta_hours_for_courses, read_input_file};

    #[test]
    fn parse_example_input_file() {
        let input_file = String::from("test_files/simple.csv");
        let courses = read_input_file(&input_file);

        assert_eq!(courses.len(), 1);
        assert_eq!(courses.first().unwrap().course_name, "ECE150");
        assert_eq!(courses.first().unwrap().enrollment, 450);
        assert_eq!(courses.first().unwrap().has_lab, true);
    }

    #[test]
    fn parse_example_input_file_with_multiple_courses() {
        let input_file = String::from("test_files/two_courses.csv");
        let courses = read_input_file(&input_file);

        assert_eq!(courses.len(), 2);
        assert_eq!(courses.first().unwrap().course_name, "ECE150");
        assert_eq!(courses.first().unwrap().enrollment, 450);
        assert_eq!(courses.first().unwrap().has_lab, true);
        assert_eq!(courses.get(1).unwrap().course_name, "ECE 192");
        assert_eq!(courses.get(1).unwrap().enrollment, 300);
        assert_eq!(courses.get(1).unwrap().has_lab, false);
    }

    #[test]
    fn calculate_ta_hours_for_course_with_lab() {
        let course_name = String::from("ECE150");
        let course = crate::types::Course {
            course_name,
            enrollment: 450,
            has_lab: true,
            unit_weight: 1.0,
        };
        let outcome = calculate_ta_hours_for_courses(vec![course]);

        assert_eq!(outcome.len(), 1);
        assert_eq!(outcome.first().unwrap().1, 11.2);
        assert_eq!(outcome.first().unwrap().0.course_name, "ECE150");
        assert_eq!(outcome.first().unwrap().0.enrollment, 450);
        assert_eq!(outcome.first().unwrap().0.has_lab, true);
        assert_eq!(outcome.first().unwrap().0.unit_weight, 1.0);
    }
}
