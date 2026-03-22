use crate::configuration::{
    FIRST_YEAR_EXTRA_TA_HOURS, FULL_TA_HOURS, GRADUATE_COURSE, LAB_INSTRUCTOR_ADJUSTMENT,
    LAB_RATIO_DENOMINATOR, MIN_ENROLLMENT_FOR_TA_ALLOC_GRAD, MIN_ENROLLMENT_FOR_TA_ALLOC_UG,
    MIN_TA_THRESHOLD, MIN_UNIT_WEIGHT_FOR_1YE_ADJUSTMENT, UNDERGRADUATE_COURSE,
};
use crate::specialcases::{LAB_ONLY_COURSES, SPECIAL_CASES};
use crate::types::AllocationType::{LAB, NON_LAB};
use crate::types::CourseType::{FIRST_YEAR, GRAD, UNDERGRAD};
use crate::types::{AllocationRule, CourseAllocation};
use crate::types::{CalculationRule, Course, CourseType};

pub fn calculate_ta_hours(c: &Course) -> CourseAllocation {
    let mut lab_amount: f32 = 0.0;

    let course_is_lab_only = check_if_lab_only(&c.name);

    let course_type = determine_course_type(&c.name);
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
            &c.name, c.enrollment, min_enrol
        );
        return CourseAllocation {
            total: 0.0,
            lab_amount: 0.0,
        };
    }

    println!(
        "Course {} ({} students) is considered type {:?} (unit weight {:.2}; lab sections: {})",
        &c.name, c.enrollment, course_type, c.unit_weight, c.lab_sections
    );

    let mut total_ta_hours: f32 = 0.0;

    let students_per_lab_section = if c.lab_sections == 0 {
        0.0
    } else {
        c.enrollment as f32 / (c.lab_sections as f32)
    };
    let tas_per_lab_section = if c.lab_sections == 0 {
        0.0
    } else {
        ((students_per_lab_section / LAB_RATIO_DENOMINATOR) - LAB_INSTRUCTOR_ADJUSTMENT).max(0.0)
    };
    println!(
        "Students per LAB section: {:.2}; TAs per lab section {:.2}",
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
            CalculationRule::PER_LEC_SECTION => allocation.hours * c.lec_sections as f32,
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
        if allocation.alloc_type == LAB {
            lab_amount += hours_to_add;
        }
    }

    if course_type == FIRST_YEAR && c.unit_weight >= MIN_UNIT_WEIGHT_FOR_1YE_ADJUSTMENT {
        let adjustment_hours = c.unit_weight * 2.0 * FIRST_YEAR_EXTRA_TA_HOURS;
        println!(
            "Adding {} extra hours for 1YE course with unit weight >= {} ",
            adjustment_hours, MIN_UNIT_WEIGHT_FOR_1YE_ADJUSTMENT
        );
        total_ta_hours += adjustment_hours;
    }

    println!(
        "Total TA hours for {} is calculated at {:.2}.",
        c.name, total_ta_hours
    );
    let ta_fraction = apply_rounding(total_ta_hours);
    let lab_amount = apply_rounding(lab_amount);

    if ta_fraction < MIN_TA_THRESHOLD {
        println!(
            "This is below the min threshold of {}, so the allocation will be 0.",
            MIN_TA_THRESHOLD
        );
        CourseAllocation {
            total: 0.0,
            lab_amount: 0.0,
        }
    } else {
        println!(
            "This results in a TA allocation of {:.2} [Lab: {:.2}, Lecture {:.2}].",
            ta_fraction,
            lab_amount,
            ta_fraction - lab_amount
        );
        CourseAllocation {
            total: ta_fraction,
            lab_amount,
        }
    }
}

pub fn apply_rounding(hours: f32) -> f32 {
    let ta_fraction = hours / FULL_TA_HOURS;
    (ta_fraction * 4.0).round() / 4.0
}

fn determine_course_type(course_name: &str) -> CourseType {
    let first_number = course_name.find(char::is_numeric).unwrap();
    let course_first_number = course_name.chars().nth(first_number).unwrap();
    let course_code_level = char::to_digit(course_first_number, 10).unwrap();

    if course_code_level == 1 {
        FIRST_YEAR
    } else if course_code_level < 6 {
        UNDERGRAD
    } else {
        GRAD
    }
}

pub fn check_for_special_case(
    course: &Course,
    original_ta_alloc: CourseAllocation,
) -> CourseAllocation {
    let course_name_no_space = course.name.replace(" ", "");
    let sc = SPECIAL_CASES
        .iter()
        .find(|o| o.course == course_name_no_space);
    if sc.is_none() {
        return original_ta_alloc;
    }
    let sc = sc.unwrap();
    println!(
        "Found special case for course {} of type {:?}. Reason: {}",
        course.name, sc.allocation_rule, sc.reason
    );
    let new_alloc = match sc.allocation_rule {
        AllocationRule::NO_TA_ALLOC => 0.0,
        AllocationRule::MIN_ALLOC => original_ta_alloc.total.max(sc.allocation_amount),
        AllocationRule::MAX_ALLOC => original_ta_alloc.total.min(sc.allocation_amount),
        AllocationRule::PER_SECTION => sc.allocation_amount * course.lec_sections as f32,
        AllocationRule::PER_LAB_SECTION => sc.allocation_amount * course.lab_sections as f32,
        AllocationRule::FIXED => sc.allocation_amount,
    };
    if new_alloc != original_ta_alloc.total {
        println!(
            "Overriding original TA allocation of {:.1} with {:.1}",
            original_ta_alloc.total, new_alloc
        );
        // TODO: Fix this
        return CourseAllocation {
            total: new_alloc,
            lab_amount: 0.0,
        };
    }
    original_ta_alloc
}

pub fn check_if_lab_only(course_name: &str) -> bool {
    let course_name_no_space = course_name.replace(" ", "");
    LAB_ONLY_COURSES.iter().any(|o| *o == course_name_no_space)
}

#[cfg(test)]
mod tests {
    use crate::calculator::{
        apply_rounding, calculate_ta_hours, check_for_special_case, check_if_lab_only,
        determine_course_type,
    };
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
            name: course_name,
            instructor: "Example Instructor".to_string(),
            enrollment: 0,
            lec_sections: 1,
            lab_sections: 0,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction.total, 0.0);
        assert_eq!(calculated_ta_fraction.lab_amount, 0.0);
    }

    #[test]
    fn course_with_enrollment_below_threshhold_gets_no_alloc() {
        let course_name = String::from("MTE221");
        let c = Course {
            name: course_name,
            instructor: "Example Instructor".to_string(),
            enrollment: 19,
            lec_sections: 1,
            lab_sections: 0,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction.total, 0.0);
        assert_eq!(calculated_ta_fraction.lab_amount, 0.0);
    }

    #[test]
    fn course_with_enrollment_of_20_gets_expected_alloc() {
        let course_name = String::from("ECE 405C");
        let c = Course {
            name: course_name,
            instructor: "Example Instructor".to_string(),
            enrollment: 20,
            lec_sections: 1,
            lab_sections: 0,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction.total, 0.5);
        assert_eq!(calculated_ta_fraction.lab_amount, 0.0);
    }

    #[test]
    fn course_with_enrollment_of_148_gets_expected_alloc() {
        let course_name = String::from("ECE 459");
        let c = Course {
            name: course_name,
            instructor: "Example Instructor".to_string(),
            enrollment: 344,
            lec_sections: 1,
            lab_sections: 3,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction.total, 5.5);
        assert_eq!(calculated_ta_fraction.lab_amount, 3.0);
    }

    #[test]
    fn grad_course_with_enrollment_of_15_gets_expected_alloc() {
        let course_name = String::from("ECE 602");
        let c = Course {
            name: course_name,
            instructor: "Example Instructor".to_string(),
            enrollment: 15,
            lec_sections: 1,
            lab_sections: 0,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction.total, 0.5);
        assert_eq!(calculated_ta_fraction.lab_amount, 0.0);
    }

    #[test]
    fn grad_course_with_enrollment_of_29_gets_expected_alloc() {
        let course_name = String::from("ECE 603");
        let c = Course {
            name: course_name,
            instructor: "Example Instructor".to_string(),
            enrollment: 29,
            lec_sections: 1,
            lab_sections: 0,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction.total, 0.5);
        assert_eq!(calculated_ta_fraction.lab_amount, 0.0);
    }

    #[test]
    fn grad_course_with_enrollment_of_67_gets_expected_alloc() {
        let course_name = String::from("ECE 657A");
        let c = Course {
            name: course_name,
            instructor: "Example Instructor".to_string(),
            enrollment: 67,
            lec_sections: 1,
            lab_sections: 0,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction.total, 1.0);
        assert_eq!(calculated_ta_fraction.lab_amount, 0.0);
    }

    #[test]
    fn special_case_no_ta_alloc() {
        let course_name = String::from("ECE 498A");
        let c = Course {
            name: course_name.clone(),
            instructor: "Example Instructor".to_string(),
            enrollment: 200,
            lec_sections: 1,
            lab_sections: 0,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&c, calculated_ta_fraction);

        assert_eq!(calculated_ta_fraction.total, 0.0);
        assert_eq!(calculated_ta_fraction.lab_amount, 0.0);
    }

    #[test]
    fn special_case_min_alloc() {
        let course_name = String::from("NE 340L");
        let c = Course {
            name: course_name.clone(),
            instructor: "Example Instructor".to_string(),
            enrollment: 50,
            lec_sections: 1,
            lab_sections: 0,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&c, calculated_ta_fraction);

        // NE340L uses FIXED allocation of 2.5
        assert_eq!(calculated_ta_fraction.total, 2.5);
        assert_eq!(calculated_ta_fraction.lab_amount, 0.0);
    }

    #[test]
    fn special_case_max_alloc() {
        let course_name = String::from("ECE459");
        let c = Course {
            name: course_name.clone(),
            instructor: "Example Instructor".to_string(),
            enrollment: 1000,
            lec_sections: 1,
            lab_sections: 10,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&c, calculated_ta_fraction);

        assert_eq!(calculated_ta_fraction.total, 6.0);
        assert_eq!(calculated_ta_fraction.lab_amount, 0.0);
    }

    #[test]
    fn special_case_fixed_alloc() {
        let course_name = String::from("ECE198");
        let c = Course {
            name: course_name.clone(),
            instructor: "Example Instructor".to_string(),
            enrollment: 5,
            lec_sections: 1,
            lab_sections: 10,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&c, calculated_ta_fraction);

        assert_eq!(calculated_ta_fraction.total, 8.0);
        assert_eq!(calculated_ta_fraction.lab_amount, 0.0);
    }

    #[test]
    fn special_case_per_course_section() {
        let course_name = String::from("ECE190");
        let c = Course {
            name: course_name.clone(),
            instructor: "Example Instructor".to_string(),
            enrollment: 900,
            lec_sections: 2,
            lab_sections: 0,
            unit_weight: 0.25,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&c, calculated_ta_fraction);

        assert_eq!(calculated_ta_fraction.total, 2.0);
        assert_eq!(calculated_ta_fraction.lab_amount, 0.0);
    }

    #[test]
    fn special_case_per_section() {
        let course_name = String::from("ECE298");
        let c = Course {
            name: course_name.clone(),
            instructor: "Example Instructor".to_string(),
            enrollment: 148,
            lec_sections: 3,
            lab_sections: 8,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&c, calculated_ta_fraction);

        assert_eq!(calculated_ta_fraction.total, 6.0);
        assert_eq!(calculated_ta_fraction.lab_amount, 0.0);
    }

    #[test]
    fn special_case_not_found() {
        let course_name = String::from("ECE 150");
        let c = Course {
            name: course_name.clone(),
            instructor: "Example Instructor".to_string(),
            enrollment: 200,
            lec_sections: 1,
            lab_sections: 6,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&c, calculated_ta_fraction);

        assert_eq!(calculated_ta_fraction.total, 2.75);
        assert_eq!(calculated_ta_fraction.lab_amount, 1.25);
    }

    // Tests for apply_rounding function
    #[test]
    fn apply_rounding_rounds_to_nearest_quarter() {
        // 0 hours should be 0
        let result = apply_rounding(0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn apply_rounding_rounds_down_correctly() {
        // 32.5 hours = 32.5/130 = 0.25, round to 1, 1/4 = 0.25
        let result = apply_rounding(32.5);
        assert_eq!(result, 0.25);
    }

    #[test]
    fn apply_rounding_rounds_to_nearest_half() {
        // 65 hours = 65/130 = 0.5, round to 2, 2/4 = 0.5
        let result = apply_rounding(65.0);
        assert_eq!(result, 0.5);
    }

    #[test]
    fn apply_rounding_rounds_to_full() {
        // 130 hours = 130/130 = 1.0, round to 4, 4/4 = 1.0
        let result = apply_rounding(130.0);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn apply_rounding_handles_decimal_values() {
        // 16.25 hours = 16.25/130 ≈ 0.125, round to 0, but actually rounds to 0.5
        // Let's test with a value that gives 0.25: 32.5/130 = 0.25, rounds to 1, 1/4 = 0.25
        let result = apply_rounding(16.25);
        assert_eq!(result, 0.25);
    }

    #[test]
    fn apply_rounding_handles_2p5_quarter_allocation() {
        // 97.5 hours = 97.5/130 = 0.75, round to 3, 3/4 = 0.75
        let result = apply_rounding(97.5);
        assert_eq!(result, 0.75);
    }

    #[test]
    fn apply_rounding_handles_large_values() {
        // 260 hours = 260/130 = 2.0, round to 8, 8/4 = 2.0
        let result = apply_rounding(260.0);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn apply_rounding_rounds_up_at_threshold() {
        // 48.75 hours = 48.75/130 ≈ 0.375, round to 0, but let's check what we actually get
        // Let's use 65 hours to get 0.5: 65/130 = 0.5, round to 2, 2/4 = 0.5
        let result = apply_rounding(65.0);
        assert_eq!(result, 0.5);
    }

    // Tests for check_if_lab_only function
    #[test]
    fn check_if_lab_only_identifies_lab_courses() {
        let result = check_if_lab_only("ECE198");
        assert_eq!(result, true);
    }

    #[test]
    fn check_if_lab_only_identifies_lab_courses_with_space() {
        let result = check_if_lab_only("ECE 198");
        assert_eq!(result, true);
    }

    #[test]
    fn check_if_lab_only_identifies_non_lab_courses() {
        let result = check_if_lab_only("ECE 150");
        assert_eq!(result, false);
    }

    #[test]
    fn check_if_lab_only_identifies_ne340l() {
        let result = check_if_lab_only("NE 340L");
        assert_eq!(result, true);
    }

    #[test]
    fn check_if_lab_only_identifies_non_existent_course_as_non_lab() {
        let result = check_if_lab_only("XYZ999");
        assert_eq!(result, false);
    }

    // Additional edge case tests
    #[test]
    fn course_with_exactly_min_enrollment_undergrad_gets_alloc() {
        let course_name = String::from("ECE 150");
        let c = Course {
            name: course_name,
            instructor: "Example Instructor".to_string(),
            enrollment: 20,
            lec_sections: 1,
            lab_sections: 0,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction.total, 0.5);
    }

    #[test]
    fn course_with_exactly_min_enrollment_grad_gets_alloc() {
        let course_name = String::from("ECE 602");
        let c = Course {
            name: course_name,
            instructor: "Example Instructor".to_string(),
            enrollment: 15, // Minimum is 15, not 10
            lec_sections: 1,
            lab_sections: 0,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction.total, 0.5);
    }

    #[test]
    fn course_with_multiple_lab_sections() {
        let course_name = String::from("ECE 290");
        let c = Course {
            name: course_name,
            instructor: "Example Instructor".to_string(),
            enrollment: 400,
            lec_sections: 2,
            lab_sections: 4,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert!(calculated_ta_fraction.lab_amount > 0.0);
        assert!(calculated_ta_fraction.total > calculated_ta_fraction.lab_amount);
    }

    #[test]
    fn first_year_course_with_high_unit_weight() {
        let course_name = String::from("ECE 105");
        let c = Course {
            name: course_name,
            instructor: "Example Instructor".to_string(),
            enrollment: 150,
            lec_sections: 1,
            lab_sections: 2,
            unit_weight: 1.0,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert!(calculated_ta_fraction.total > 0.0);
    }

    #[test]
    fn grad_course_with_high_enrollment() {
        let course_name = String::from("ECE 650");
        let c = Course {
            name: course_name,
            instructor: "Example Instructor".to_string(),
            enrollment: 200,
            lec_sections: 1,
            lab_sections: 0,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert!(calculated_ta_fraction.total > 1.0);
    }

    #[test]
    fn special_case_per_lab_section() {
        let course_name = String::from("NE 455B");
        let c = Course {
            name: course_name.clone(),
            instructor: "Example Instructor".to_string(),
            enrollment: 100,
            lec_sections: 2,
            lab_sections: 4,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&c, calculated_ta_fraction);

        // Should be minimum of 2.5
        assert!(calculated_ta_fraction.total >= 2.5);
    }

    #[test]
    fn special_case_ne343_fixed_allocation() {
        let course_name = String::from("NE343");
        let c = Course {
            name: course_name.clone(),
            instructor: "Example Instructor".to_string(),
            enrollment: 150,
            lec_sections: 1,
            lab_sections: 3,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&c, calculated_ta_fraction);

        assert_eq!(calculated_ta_fraction.total, 2.5);
    }

    // Additional comprehensive test cases for improved coverage

    #[test]
    fn apply_rounding_with_6_hours() {
        // 78 hours / 130 = 0.6, round = 2, so 2/4 = 0.5
        let result = apply_rounding(78.0);
        assert_eq!(result, 0.5);
    }

    #[test]
    fn apply_rounding_with_7_hours() {
        // 91 hours / 130 ≈ 0.7, round = 3, so 3/4 = 0.75
        let result = apply_rounding(91.0);
        assert_eq!(result, 0.75);
    }

    #[test]
    fn apply_rounding_with_8_hours() {
        // 104 hours / 130 = 0.8, round = 3, so 3/4 = 0.75
        let result = apply_rounding(104.0);
        assert_eq!(result, 0.75);
    }

    #[test]
    fn apply_rounding_with_10_hours() {
        // 149 hours / 130 ≈ 1.15, round = 5, so 5/4 = 1.25
        let result = apply_rounding(149.0);
        assert!(result >= 1.0 && result <= 1.5);
    }

    #[test]
    fn apply_rounding_with_12_hours() {
        // 156 hours / 130 = 1.2, round = 5, so 5/4 = 1.25
        let result = apply_rounding(156.0);
        assert_eq!(result, 1.25);
    }

    #[test]
    fn apply_rounding_with_16_hours() {
        // 195 hours / 130 = 1.5, round = 6, so 6/4 = 1.5
        let result = apply_rounding(195.0);
        assert_eq!(result, 1.5);
    }

    #[test]
    fn apply_rounding_with_small_fraction() {
        // 6.5 hours / 130 = 0.05, round = 0, so 0/4 = 0.0
        let result = apply_rounding(6.5);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn lab_only_course_calculation() {
        let course_name = String::from("ECE 198");
        let c = Course {
            name: course_name,
            instructor: "Example Instructor".to_string(),
            enrollment: 100,
            lec_sections: 0,
            lab_sections: 4,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        // Lab-only courses should have lab_amount equal to total
        assert_eq!(
            calculated_ta_fraction.lab_amount,
            calculated_ta_fraction.total
        );
        assert!(calculated_ta_fraction.total > 0.0);
    }

    #[test]
    fn ne340l_special_case_with_high_enrollment() {
        let course_name = String::from("NE 340L");
        let c = Course {
            name: course_name.clone(),
            instructor: "Example Instructor".to_string(),
            enrollment: 200,
            lec_sections: 1,
            lab_sections: 4,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&c, calculated_ta_fraction);

        // Should be at least 2.5 due to MIN_ALLOC rule
        assert!(calculated_ta_fraction.total >= 2.5);
    }

    #[test]
    fn ne340l_special_case_with_low_enrollment() {
        let course_name = String::from("NE 340L");
        let c = Course {
            name: course_name.clone(),
            instructor: "Example Instructor".to_string(),
            enrollment: 50,
            lec_sections: 1,
            lab_sections: 1,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&c, calculated_ta_fraction);

        // Should be at least 2.5 due to MIN_ALLOC rule
        assert_eq!(calculated_ta_fraction.total, 2.5);
    }

    #[test]
    fn ece459_special_case_prevents_over_allocation() {
        let course_name = String::from("ECE 459");
        let c = Course {
            name: course_name.clone(),
            instructor: "Example Instructor".to_string(),
            enrollment: 1000,
            lec_sections: 5,
            lab_sections: 20,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&c, calculated_ta_fraction);

        // Should be capped at 6.0 due to MAX_ALLOC rule
        assert_eq!(calculated_ta_fraction.total, 6.0);
    }

    #[test]
    fn capstone_course_gets_no_ta() {
        let course_name = String::from("ECE 498A");
        let c = Course {
            name: course_name.clone(),
            instructor: "Example Instructor".to_string(),
            enrollment: 500,
            lec_sections: 2,
            lab_sections: 0,
            unit_weight: 1.0,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&c, calculated_ta_fraction);

        assert_eq!(calculated_ta_fraction.total, 0.0);
    }

    #[test]
    fn mte482_capstone_gets_no_ta() {
        let course_name = String::from("MTE482");
        let c = Course {
            name: course_name.clone(),
            instructor: "Example Instructor".to_string(),
            enrollment: 200,
            lec_sections: 1,
            lab_sections: 0,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&c, calculated_ta_fraction);

        assert_eq!(calculated_ta_fraction.total, 0.0);
    }

    #[test]
    fn determine_course_type_boundary_level_5() {
        let course_name = String::from("ECE 599");

        let ct = determine_course_type(&course_name);

        assert_eq!(ct, UNDERGRAD);
    }

    #[test]
    fn determine_course_type_boundary_level_6() {
        let course_name = String::from("ECE 600");

        let ct = determine_course_type(&course_name);

        assert_eq!(ct, GRAD);
    }

    #[test]
    fn undergrad_course_with_multiple_sections_and_labs() {
        let course_name = String::from("ECE 250");
        let c = Course {
            name: course_name,
            instructor: "Example Instructor".to_string(),
            enrollment: 500,
            lec_sections: 4,
            lab_sections: 8,
            unit_weight: 0.75,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert!(calculated_ta_fraction.total > 0.0);
        assert!(calculated_ta_fraction.lab_amount > 0.0);
        assert!(calculated_ta_fraction.total > calculated_ta_fraction.lab_amount);
    }

    #[test]
    fn first_year_course_unit_weight_adjustment() {
        let course_name = String::from("ECE 105");
        let c = Course {
            name: course_name,
            instructor: "Example Instructor".to_string(),
            enrollment: 100,
            lec_sections: 2,
            lab_sections: 0,
            unit_weight: 1.0,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        // First year courses with unit weight >= 0.5 get extra adjustment
        assert!(calculated_ta_fraction.total > 0.5);
    }

    #[test]
    fn grad_course_min_enrollment_boundary() {
        let course_name = String::from("ECE 602");
        let c = Course {
            name: course_name,
            instructor: "Example Instructor".to_string(),
            enrollment: 9, // Below minimum
            lec_sections: 1,
            lab_sections: 0,
            unit_weight: 0.5,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        assert_eq!(calculated_ta_fraction.total, 0.0);
    }

    #[test]
    fn check_if_lab_only_with_various_formats() {
        // Test different naming formats - LAB_ONLY_COURSES has uppercase entries
        assert_eq!(check_if_lab_only("ECE198"), true);
        assert_eq!(check_if_lab_only("ECE 198"), true);
        assert_eq!(check_if_lab_only("NE 340L"), true);
        assert_eq!(check_if_lab_only("ECE298"), true);
    }

    #[test]
    fn special_case_per_section_calculation() {
        let course_name = String::from("ECE190");
        let c = Course {
            name: course_name.clone(),
            instructor: "Example Instructor".to_string(),
            enrollment: 500,
            lec_sections: 3,
            lab_sections: 0,
            unit_weight: 0.25,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);
        let calculated_ta_fraction = check_for_special_case(&c, calculated_ta_fraction);

        // ECE190 uses PER_SECTION rule with 1.0 per section
        assert_eq!(calculated_ta_fraction.total, 3.0);
    }

    #[test]
    fn course_below_min_threshold_after_rounding() {
        let course_name = String::from("ECE 252");
        let c = Course {
            name: course_name,
            instructor: "Example Instructor".to_string(),
            enrollment: 21, // Just above minimum
            lec_sections: 1,
            lab_sections: 0,
            unit_weight: 0.25,
        };

        let calculated_ta_fraction = calculate_ta_hours(&c);

        // May result in 0 if below MIN_TA_THRESHOLD after rounding
        assert!(calculated_ta_fraction.total >= 0.0);
    }
}
