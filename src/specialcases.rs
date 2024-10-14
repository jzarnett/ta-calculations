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
];

mod tests {
    use crate::specialcases::SPECIAL_CASES;

    #[test]
    fn no_spaces_in_special_case_course_names() {
        for sc in SPECIAL_CASES {
            assert!(!sc.course.contains(" "));
        }
    }
}