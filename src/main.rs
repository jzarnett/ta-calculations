#![allow(clippy::upper_case_acronyms, non_camel_case_types)]

use crate::calculator::check_for_special_case;
use crate::types::{Course, CourseAllocation};
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

fn calculate_ta_hours_for_courses(courses: Vec<Course>) -> Vec<(Course, CourseAllocation)> {
    let mut result: Vec<(Course, CourseAllocation)> = Vec::new();
    for c in courses {
        let ta_allocation = calculator::calculate_ta_hours(&c);
        let ta_allocation = check_for_special_case(&c, ta_allocation);
        result.push((c, ta_allocation));
    }
    result
}

fn write_output(courses: Vec<(Course, CourseAllocation)>) {
    let mut wtr = csv::Writer::from_path("TA-Allocations.csv").unwrap();
    wtr.write_record(["Course", "Instructor", "Enrollment", "TA Allocation", "Lab Fraction"])
        .unwrap();

    for c in courses {
        wtr.write_record(&[
            c.0.name,
            c.0.instructor,
            c.0.enrollment.to_string(),
            c.1.total.to_string(),
            c.1.lab_amount.to_string(),
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
            name: record.get(0).unwrap().trim().to_string(),
            instructor: record.get(1).unwrap().trim().to_string(),
            enrollment: record.get(2).unwrap().trim().parse().unwrap(),
            lec_sections: record.get(3).unwrap().trim().parse().unwrap(),
            lab_sections: record.get(4).unwrap().trim().parse().unwrap(),
            unit_weight: record.get(5).unwrap().trim().parse().unwrap(),
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
        let c = courses.first().unwrap();

        assert_eq!(courses.len(), 1);
        assert_eq!(c.name, "ECE150");
        assert_eq!(c.instructor, "Instructor Name");
        assert_eq!(c.enrollment, 450);
        assert_eq!(c.lec_sections, 3);
        assert_eq!(c.lab_sections, 3);
        assert_eq!(c.unit_weight, 1.0);
    }

    #[test]
    fn parse_example_input_file_with_multiple_courses() {
        let input_file = String::from("test_files/two_courses.csv");
        let courses = read_input_file(&input_file);

        assert_eq!(courses.len(), 2);
        assert_eq!(courses.first().unwrap().name, "ECE150");
        assert_eq!(courses.first().unwrap().enrollment, 450);
        assert_eq!(courses.first().unwrap().lab_sections, 3);
        assert_eq!(courses.get(1).unwrap().name, "ECE 192");
        assert_eq!(courses.get(1).unwrap().enrollment, 300);
        assert_eq!(courses.get(1).unwrap().lab_sections, 0);
    }

    #[test]
    fn calculate_ta_hours_for_course_with_lab() {
        let course_name = String::from("ECE150");
        let course = crate::types::Course {
            name: course_name,
            instructor: "Bob Example".to_string(),
            enrollment: 450,
            lec_sections: 2,
            lab_sections: 1,
            unit_weight: 1.0,
        };
        let outcome = calculate_ta_hours_for_courses(vec![course]);

        assert_eq!(outcome.len(), 1);
        assert_eq!(outcome.first().unwrap().1.total, 8.8);
        assert_eq!(outcome.first().unwrap().1.lab_amount, 4.5);
        assert_eq!(outcome.first().unwrap().0.name, "ECE150");
        assert_eq!(outcome.first().unwrap().0.enrollment, 450);
        assert_eq!(outcome.first().unwrap().0.lab_sections, 1);
        assert_eq!(outcome.first().unwrap().0.unit_weight, 1.0);
    }
}
