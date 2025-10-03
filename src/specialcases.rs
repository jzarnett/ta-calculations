use crate::types::{AllocationRule, SpecialCase};

pub const SPECIAL_CASES: &[SpecialCase] = &[
    SpecialCase {
        course: "ECE498A",
        reason: "Capstone Course",
        allocation_rule: AllocationRule::NO_TA_ALLOC,
        allocation_amount: 0.0,
    },
    SpecialCase {
        course: "ECE498B",
        reason: "Capstone Course",
        allocation_rule: AllocationRule::NO_TA_ALLOC,
        allocation_amount: 0.0,
    },
    SpecialCase {
        course: "MTE482",
        reason: "Capstone Course",
        allocation_rule: AllocationRule::NO_TA_ALLOC,
        allocation_amount: 0.0,
    },
    SpecialCase {
        course: "NE340L",
        reason: "Cleanroom Lab Course",
        allocation_rule: AllocationRule::FIXED,
        allocation_amount: 2.0,
    },
    SpecialCase {
        course: "ECE459",
        reason: "Project Course",
        allocation_rule: AllocationRule::MAX_ALLOC,
        allocation_amount: 6.0,
    },
    SpecialCase {
        course: "NE455B",
        reason: "Cleanroom Lab Course",
        allocation_rule: AllocationRule::MIN_ALLOC,
        allocation_amount: 2.0,
    },
    SpecialCase {
        course: "NE409",
        reason: "Half-Credit No TA Course",
        allocation_rule: AllocationRule::NO_TA_ALLOC,
        allocation_amount: 0.0,
    },
    SpecialCase {
        course: "ECE190",
        reason: "1 TA per Section 1st Year Course",
        allocation_rule: AllocationRule::PER_SECTION,
        allocation_amount: 1.0,
    },
    SpecialCase {
        course: "ECE298",
        reason: "Hands-On Lab Course",
        allocation_rule: AllocationRule::PER_SECTION,
        allocation_amount: 3.0,
    },
    SpecialCase {
        course: "ECE198",
        reason: "Hands-On Lab Course",
        allocation_rule: AllocationRule::FIXED,
        allocation_amount: 8.0,
    },
    SpecialCase {
        course: "ECE464",
        reason: "High Voltage Lab",
        allocation_rule: AllocationRule::MIN_ALLOC,
        allocation_amount: 1.0,
    },
    SpecialCase {
        course: "ECE474",
        reason: "Lab Safety",
        allocation_rule: AllocationRule::PER_LAB_SECTION,
        allocation_amount: 0.4,
    },
    SpecialCase {
        course: "NE216L",
        reason: "Nano Lab",
        allocation_rule: AllocationRule::FIXED,
        allocation_amount: 1.0,
    },
    SpecialCase {
        course: "NE217L",
        reason: "Nano Lab -- Combined with 216L",
        allocation_rule: AllocationRule::FIXED,
        allocation_amount: 0.0,
    },
];
// 192 (half credit)
// ECE 260, ME 260, other high power labs, every 15 students = 1 TA

pub const LAB_ONLY_COURSES: &[&str] = &["NE340L", "NE455A", "ECE198", "ECE298"];

mod tests {
    use crate::specialcases::{LAB_ONLY_COURSES, SPECIAL_CASES};

    #[test]
    fn no_spaces_in_special_case_course_names() {
        for sc in SPECIAL_CASES {
            assert!(!sc.course.contains(" "));
        }
    }

    #[test]
    fn no_spaces_in_lab_only_course_names() {
        for l in LAB_ONLY_COURSES {
            assert!(!l.contains(" "));
        }
    }
}
