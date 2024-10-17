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
        course: "NE340",
        reason: "Cleanroom Lab Course",
        allocation_rule: AllocationRule::MIN_ALLOC,
        allocation_amount: 5.0,
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
];

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
