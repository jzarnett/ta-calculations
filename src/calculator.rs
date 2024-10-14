use crate::configuration::{
    FIRST_YEAR_EXTRA_TA_HOURS, FULL_TA_HOURS, GRADUATE_COURSE, LAB_RATIO_DENOMINATOR,
    MIN_ENROLLMENT_FOR_TA_ALLOC, MIN_TA_THRESHOLD, MIN_UNIT_WEIGHT_FOR_1YE_ADJUSTMENT,
    UNDERGRADUATE_COURSE,
};
use crate::specialcases::SPECIAL_CASES;
use crate::types;
use crate::types::AllocationType::{LAB, NON_LAB};
use crate::types::{CalculationRule, CourseType};

pub fn calculate_ta_hours(
    course_name: &String,
    course_has_lab: bool,
    enrollment: i32,
    unit_weight: f32,
) -> f32 {
    if enrollment < MIN_ENROLLMENT_FOR_TA_ALLOC {
        println!(
            "Course enrollment for {} of {} is below min threshold of {}; allocation will be 0.",
            course_name, enrollment, MIN_ENROLLMENT_FOR_TA_ALLOC
        );
        return 0.0;
    }

    let enrollment = enrollment as f32; // Manual typecast because Rust insists
    let course_type = determine_course_type(course_name);
    let configuration_to_use = match course_type {
        CourseType::FIRST_YEAR => UNDERGRADUATE_COURSE,
        CourseType::UNDERGRAD => UNDERGRADUATE_COURSE,
        CourseType::GRAD => GRADUATE_COURSE,
    };

    println!(
        "Course {} is considered type {:?} (unit weight {:.1}; lab: {})",
        course_name,
        course_type,
        unit_weight,
        if course_has_lab { "yes" } else { "no" }
    );

    let mut total_ta_hours: f32 = 0.0;

    for allocation in configuration_to_use {
        if course_has_lab && allocation.alloc_type == NON_LAB {
            continue;
        }
        if !course_has_lab && allocation.alloc_type == LAB {
            continue;
        }

        let hours_to_add = match allocation.calc_rule {
            CalculationRule::PER_TERM => allocation.hours,
            CalculationRule::PER_STUDENT => allocation.hours * enrollment,
            CalculationRule::PER_LAB => allocation.hours * enrollment / LAB_RATIO_DENOMINATOR,
        };
        println!(
            "Adding {:.2} hours for {} (Calculation Rule: {:?})",
            hours_to_add, allocation.name, allocation.calc_rule
        );
        total_ta_hours += hours_to_add;
    }

    if course_type == CourseType::FIRST_YEAR && unit_weight >= MIN_UNIT_WEIGHT_FOR_1YE_ADJUSTMENT {
        let adjustment_hours = unit_weight * FIRST_YEAR_EXTRA_TA_HOURS;
        println!(
            "Adding {} extra hours for 1YE course with unit weight >= {} ",
            adjustment_hours, MIN_UNIT_WEIGHT_FOR_1YE_ADJUSTMENT
        );
        total_ta_hours += adjustment_hours;
    }

    println!(
        "Total TA hours for {} is calculated at {}.",
        course_name, total_ta_hours
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
    let or = SPECIAL_CASES
        .iter()
        .find(|o| o.course == course_name_no_space);
    if or.is_none() {
        return original_ta_alloc;
    }
    let or = or.unwrap();
    println!(
        "Found special case for course {} of type {:?}. Reason: {}",
        course_name, or.allocation_rule, or.reason
    );
    let new_alloc = match or.allocation_rule {
        types::AllocationRule::NO_TA_ALLOC => 0.0,
        types::AllocationRule::MIN_ALLOC => original_ta_alloc.max(or.allocation_amount),
        types::AllocationRule::MAX_ALLOC => original_ta_alloc.min(or.allocation_amount),
    };
    if new_alloc != original_ta_alloc {
        println!(
            "Overriding original TA allocation of {:.1} with {:.1}",
            original_ta_alloc, new_alloc
        );
    }
    new_alloc
}

#[cfg(test)]
mod tests {
    use crate::calculator::{calculate_ta_hours, check_for_special_case, determine_course_type};
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

        let calculated_ta_fraction = calculate_ta_hours(&course_name, true, 0, 1.0);

        assert_eq!(calculated_ta_fraction, 0.0);
    }

    #[test]
    fn course_with_enrollment_below_threshhold_gets_no_alloc() {
        let course_name = String::from("MTE221");

        let calculated_ta_fraction = calculate_ta_hours(&course_name, true, 14, 1.0);

        assert_eq!(calculated_ta_fraction, 0.0);
    }

    #[test]
    fn course_with_enrollment_of_15_gets_expected_alloc() {
        let course_name = String::from("ECE 405C");

        let calculated_ta_fraction = calculate_ta_hours(&course_name, true, 15, 1.0);

        assert_eq!(calculated_ta_fraction, 0.6);
    }

    #[test]
    fn course_with_enrollment_of_148_gets_expected_alloc() {
        let course_name = String::from("ECE 459");

        let calculated_ta_fraction = calculate_ta_hours(&course_name, true, 344, 1.0);

        assert_eq!(calculated_ta_fraction, 7.9);
    }

    #[test]
    fn grad_course_with_enrollment_of_15_gets_expected_alloc() {
        let course_name = String::from("ECE 602");

        let calculated_ta_fraction = calculate_ta_hours(&course_name, false, 15, 1.0);

        assert_eq!(calculated_ta_fraction, 0.4);
    }

    #[test]
    fn grad_course_with_enrollment_of_29_gets_expected_alloc() {
        let course_name = String::from("ECE 603");

        let calculated_ta_fraction = calculate_ta_hours(&course_name, false, 29, 1.0);

        assert_eq!(calculated_ta_fraction, 0.5);
    }

    #[test]
    fn grad_course_with_enrollment_of_67_gets_expected_alloc() {
        let course_name = String::from("ECE 657A");

        let calculated_ta_fraction = calculate_ta_hours(&course_name, false, 67, 1.0);

        assert_eq!(calculated_ta_fraction, 1.0);
    }

    #[test]
    fn first_year_course_gets_no_boost_if_not_at_least_1_unit() {
        let course_name = String::from("ECE 192");

        let calculated_ta_fraction = calculate_ta_hours(&course_name, false, 15, 0.5);

        assert_eq!(calculated_ta_fraction, 0.5);
    }
    #[test]
    fn first_year_course_gets_larger_boost_if_higher_weight() {
        let course_name = String::from("MTE 120");

        let calculated_ta_fraction = calculate_ta_hours(&course_name, true, 15, 1.5);

        assert_eq!(calculated_ta_fraction, 2.1);
    }

    #[test]
    fn special_case_no_ta_alloc() {
        let course_name = String::from("ECE 498A");

        let calculated_ta_fraction = calculate_ta_hours(&course_name, false, 200, 1.0);
        let calculated_ta_fraction = check_for_special_case(&course_name, calculated_ta_fraction);

        assert_eq!(calculated_ta_fraction, 0.0);
    }

    #[test]
    fn special_case_min_alloc() {
        let course_name = String::from("NE 340");

        let calculated_ta_fraction = calculate_ta_hours(&course_name, true, 50, 1.0);
        let calculated_ta_fraction = check_for_special_case(&course_name, calculated_ta_fraction);

        assert_eq!(calculated_ta_fraction, 5.0);
    }

    #[test]
    fn special_case_max_alloc() {
        let course_name = String::from("ECE459");

        let calculated_ta_fraction = calculate_ta_hours(&course_name, true, 1000, 1.0);
        let calculated_ta_fraction = check_for_special_case(&course_name, calculated_ta_fraction);

        assert_eq!(calculated_ta_fraction, 6.0);
    }

    #[test]
    fn special_case_not_found() {
        let course_name = String::from("ECE 150");

        let calculated_ta_fraction = calculate_ta_hours(&course_name, false, 200, 1.0);
        let calculated_ta_fraction = check_for_special_case(&course_name, calculated_ta_fraction);

        assert_eq!(calculated_ta_fraction, 4.4);
    }
}
