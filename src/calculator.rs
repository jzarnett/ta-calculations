use crate::configuration::{
    FIRST_YEAR_EXTRA_TA_HOURS, FULL_TA_HOURS, GRADUATE_COURSE, LAB_INSTRUCTOR_ADJUSTMENT,
    LAB_RATIO_DENOMINATOR, MIN_ENROLLMENT_FOR_TA_ALLOC_GRAD, MIN_ENROLLMENT_FOR_TA_ALLOC_UG,
    MIN_TA_THRESHOLD, MIN_UNIT_WEIGHT_FOR_1YE_ADJUSTMENT, UNDERGRADUATE_COURSE,
};
use crate::specialcases::{LAB_ONLY_COURSES, SPECIAL_CASES};
use crate::types;
use crate::types::AllocationType::{LAB, NON_LAB};
use crate::types::CourseType::{FIRST_YEAR, GRAD, UNDERGRAD};
use crate::types::{CalculationRule, Course, CourseType};

pub fn calculate_ta_hours(c: &Course) -> f32 {
    let course_is_lab_only = check_if_lab_only(&c.course_name);

    let course_type = determine_course_type(&c.course_name);
    let configuration_to_use = match course_type {
        FIRST_YEAR => UNDERGRADUATE_COURSE,
        UNDERGRAD => UNDERGRADUATE_COURSE,
        GRAD => GRADUATE_COURSE,
    };
    let min_enrol = match course_type {
        UNDERGRAD => MIN_ENROLLMENT_FOR_TA_ALLOC_UG,
        GRAD => MIN_ENROLLMENT_FOR_TA_ALLOC_GRAD,
        FIRST_YEAR => MIN_ENROLLMENT_FOR_TA_ALLOC_UG,
    };

    if c.enrollment < min_enrol {
        println!(
            "Course enrollment for {} of {} is below min threshold of {}; allocation will be 0.",
            &c.course_name, c.enrollment, min_enrol
        );
        return 0.0;
    }

    println!(
        "Course {} is considered type {:?} (unit weight {:.1}; lab sections: {})",
        &c.course_name, course_type, c.unit_weight, c.lab_sections
    );

    let mut total_ta_hours: f32 = 0.0;

    let students_per_lab_section = c.enrollment as f32 / (c.lab_sections as f32);
    let tas_per_lab_section = ((students_per_lab_section / LAB_RATIO_DENOMINATOR).floor()
        - LAB_INSTRUCTOR_ADJUSTMENT)
        .max(0.0);
    println!(
        "Students per LAB section: {}; TAs per lab section {}",
        students_per_lab_section, tas_per_lab_section
    );

    for allocation in configuration_to_use {
        if c.lab_sections > 0 && allocation.alloc_type == NON_LAB {
            continue;
        }
        if c.lab_sections == 0 && allocation.alloc_type == LAB {
            continue;
        }
        if course_is_lab_only && allocation.alloc_type != LAB {
            continue;
        }

        let hours_to_add = match allocation.calc_rule {
            CalculationRule::PER_TERM => allocation.hours,
            CalculationRule::PER_STUDENT => allocation.hours * c.enrollment as f32,
            CalculationRule::PER_LAB => {
                allocation.hours * c.lab_sections as f32 * tas_per_lab_section
            }
        };
        println!(
            "Adding {:.2} hours for {} (Calculation Rule: {:?})",
            hours_to_add, allocation.name, allocation.calc_rule
        );
        total_ta_hours += hours_to_add;
    }

    if course_type == CourseType::FIRST_YEAR && c.unit_weight >= MIN_UNIT_WEIGHT_FOR_1YE_ADJUSTMENT
    {
        let adjustment_hours = c.unit_weight * FIRST_YEAR_EXTRA_TA_HOURS;
        println!(
            "Adding {} extra hours for 1YE course with unit weight >= {} ",
            adjustment_hours, MIN_UNIT_WEIGHT_FOR_1YE_ADJUSTMENT
        );
        total_ta_hours += adjustment_hours;
    }

    println!(
        "Total TA hours for {} is calculated at {}.",
        c.course_name, total_ta_hours
    );
    let ta_fraction = total_ta_hours / FULL_TA_HOURS;
    let ta_fraction = (ta_fraction * 10.0).round() / 10.0;

    if ta_fraction < MIN_TA_THRESHOLD {
        println!(
            "This is below the min threshold of {}, so the allocation will be 0.",
            MIN_TA_THRESHOLD
        );
        0.0
    } else {
        println!("This results in a TA allocation of {}.", ta_fraction);
        ta_fraction
    }
}

fn determine_course_type(course_name: &str) -> CourseType {
    let first_number = course_name.find(char::is_numeric).unwrap();
    let course_first_number = course_name.chars().nth(first_number).unwrap();
    let course_code_level = char::to_digit(course_first_number, 10).unwrap();

    if course_code_level == 1 {
        CourseType::FIRST_YEAR
    } else if course_code_level < 6 {
        CourseType::UNDERGRAD
    } else {
        CourseType::GRAD
    }
}

pub fn check_for_special_case(course_name: &String, original_ta_alloc: f32) -> f32 {
    let course_name_no_space = course_name.replace(" ", "");
    let sc = SPECIAL_CASES
        .iter()
        .find(|o| o.course == course_name_no_space);
    if sc.is_none() {
        return original_ta_alloc;
    }
    let sc = sc.unwrap();
    println!(
        "Found special case for course {} of type {:?}. Reason: {}",
        course_name, sc.allocation_rule, sc.reason
    );
    let new_alloc = match sc.allocation_rule {
        types::AllocationRule::NO_TA_ALLOC => 0.0,
        types::AllocationRule::MIN_ALLOC => original_ta_alloc.max(sc.allocation_amount),
        types::AllocationRule::MAX_ALLOC => original_ta_alloc.min(sc.allocation_amount),
    };
    if new_alloc != original_ta_alloc {
        println!(
            "Overriding original TA allocation of {:.1} with {:.1}",
            original_ta_alloc, new_alloc
        );
    }
    new_alloc
}

pub fn check_if_lab_only(course_name: &str) -> bool {
    let course_name_no_space = course_name.replace(" ", "");
    LAB_ONLY_COURSES.iter().any(|o| *o == course_name_no_space)
}

#[cfg(test)]
mod tests {
    use crate::calculator::{calculate_ta_hours, check_for_special_case, determine_course_type};
    use crate::types::Course;
    use crate::types::CourseType::{FIRST_YEAR, GRAD, UNDERGRAD};

    #[test]
    fn determine_course_type_finds_course_if_1ye() {
        let course_name = String::from("ECE 150");

        let ct = determine_course_type(&course_name);

        assert_eq!(ct, FIRST_YEAR)
    }

    #[test]
    fn determine_course_type_finds_course_if_not_1ye() {
        let course_name = String::from("ECE252");

        let ct = determine_course_type(&course_name);

        assert_eq!(ct, UNDERGRAD)
    }

    #[test]
    fn determine_course_type_finds_course_if_grad() {
        let course_name = String::from("NE-650");

        let ct = determine_course_type(&course_name);

        assert_eq!(ct, GRAD)
    }

    #[test]
    fn undergrad_course_with_zero_enrollment_gets_no_alloc() {
        let course_name = String::from("ECE 155");
        let c = Course {
            course_name,
            enrollment: 0,
            lab_sections: 0,
            unit_weight: 1.0,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction, 0.0);
    }

    #[test]
    fn course_with_enrollment_below_threshhold_gets_no_alloc() {
        let course_name = String::from("MTE221");
        let c = Course {
            course_name,
            enrollment: 19,
            lab_sections: 0,
            unit_weight: 1.0,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction, 0.0);
    }

    #[test]
    fn course_with_enrollment_of_20_gets_expected_alloc() {
        let course_name = String::from("ECE 405C");
        let c = Course {
            course_name,
            enrollment: 20,
            lab_sections: 0,
            unit_weight: 1.0,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction, 0.5);
    }

    #[test]
    fn course_with_enrollment_of_148_gets_expected_alloc() {
        let course_name = String::from("ECE 459");
        let c = Course {
            course_name,
            enrollment: 344,
            lab_sections: 3,
            unit_weight: 1.0,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction, 6.8);
    }

    #[test]
    fn grad_course_with_enrollment_of_15_gets_expected_alloc() {
        let course_name = String::from("ECE 602");
        let c = Course {
            course_name,
            enrollment: 15,
            lab_sections: 0,
            unit_weight: 1.0,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction, 0.4);
    }

    #[test]
    fn grad_course_with_enrollment_of_29_gets_expected_alloc() {
        let course_name = String::from("ECE 603");
        let c = Course {
            course_name,
            enrollment: 29,
            lab_sections: 0,
            unit_weight: 1.0,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction, 0.5);
    }

    #[test]
    fn grad_course_with_enrollment_of_67_gets_expected_alloc() {
        let course_name = String::from("ECE 657A");
        let c = Course {
            course_name,
            enrollment: 67,
            lab_sections: 0,
            unit_weight: 1.0,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction, 1.0);
    }

    #[test]
    fn first_year_course_gets_no_boost_if_not_at_least_1_unit() {
        let course_name = String::from("ECE 192");
        let c = Course {
            course_name,
            enrollment: 200,
            lab_sections: 0,
            unit_weight: 1.0,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction, 4.1);
    }
    #[test]
    fn first_year_course_gets_larger_boost_if_higher_weight() {
        let course_name = String::from("MTE 120");
        let c = Course {
            course_name,
            enrollment: 100,
            lab_sections: 4,
            unit_weight: 1.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction, 2.8);
    }

    #[test]
    fn special_case_no_ta_alloc() {
        let course_name = String::from("ECE 498A");
        let c = Course {
            course_name: course_name.clone(),
            enrollment: 200,
            lab_sections: 0,
            unit_weight: 1.0,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&course_name, calculated_ta_fraction);

        assert_eq!(calculated_ta_fraction, 0.0);
    }

    #[test]
    fn special_case_min_alloc() {
        let course_name = String::from("NE 340");
        let c = Course {
            course_name: course_name.clone(),
            enrollment: 50,
            lab_sections: 0,
            unit_weight: 1.0,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&course_name, calculated_ta_fraction);

        assert_eq!(calculated_ta_fraction, 5.0);
    }

    #[test]
    fn special_case_max_alloc() {
        let course_name = String::from("ECE459");
        let c = Course {
            course_name: course_name.clone(),
            enrollment: 1000,
            lab_sections: 10,
            unit_weight: 1.0,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&course_name, calculated_ta_fraction);

        assert_eq!(calculated_ta_fraction, 6.0);
    }

    #[test]
    fn special_case_not_found() {
        let course_name = String::from("ECE 150");
        let c = Course {
            course_name: course_name.clone(),
            enrollment: 200,
            lab_sections: 6,
            unit_weight: 1.0,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&course_name, calculated_ta_fraction);

        assert_eq!(calculated_ta_fraction, 4.3);
    }
}
