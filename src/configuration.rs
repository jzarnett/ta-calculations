use crate::types::AllocationType::{LAB, LECTURE, NON_LAB};
use crate::types::{CalculationRule, TAHourAllocation};

pub const FULL_TA_HOURS: f32 = 130.0;
pub const MIN_TA_THRESHOLD: f32 = 0.3;
pub const LAB_RATIO_DENOMINATOR: f32 = 40.0;
pub const FIRST_YEAR_EXTRA_TA_HOURS: f32 = 130.0;

pub const MIN_UNIT_WEIGHT_FOR_1YE_ADJUSTMENT: f32 = 1.0;

pub const MIN_ENROLLMENT_FOR_TA_ALLOC: i32 = 15;

pub const UNDERGRADUATE_COURSE: &[TAHourAllocation] = &[
    TAHourAllocation {
        name: "Midterm Marking",
        hours: 0.33,
        calc_rule: CalculationRule::PER_STUDENT,
        alloc_type: LECTURE,
    },
    TAHourAllocation {
        name: "Final Marking",
        hours: 0.67,
        calc_rule: CalculationRule::PER_STUDENT,
        alloc_type: LECTURE,
    },
    TAHourAllocation {
        name: "Tutorials",
        hours: 12.0,
        calc_rule: CalculationRule::PER_TERM,
        alloc_type: LECTURE,
    },
    TAHourAllocation {
        name: "Tutorial Prep",
        hours: 12.0,
        calc_rule: CalculationRule::PER_TERM,
        alloc_type: LECTURE,
    },
    TAHourAllocation {
        name: "Office Hours",
        hours: 12.0,
        calc_rule: CalculationRule::PER_TERM,
        alloc_type: LECTURE,
    },
    TAHourAllocation {
        name: "Lab Delivery",
        hours: 15.0,
        calc_rule: CalculationRule::PER_LAB,
        alloc_type: LAB,
    },
    TAHourAllocation {
        name: "Lab Prep",
        hours: 15.0,
        calc_rule: CalculationRule::PER_LAB,
        alloc_type: LAB,
    },
    TAHourAllocation {
        name: "Lab Marking",
        hours: 1.1,
        calc_rule: CalculationRule::PER_STUDENT,
        alloc_type: LAB,
    },
    TAHourAllocation {
        name: "Assignment Marking",
        hours: 1.0,
        calc_rule: CalculationRule::PER_STUDENT,
        alloc_type: NON_LAB,
    },
    TAHourAllocation {
        name: "Exam Proctoring",
        hours: 5.0,
        calc_rule: CalculationRule::PER_TERM,
        alloc_type: LECTURE,
    },
    TAHourAllocation {
        name: "Extra TA Hours",
        hours: 0.0,
        calc_rule: CalculationRule::PER_TERM,
        alloc_type: LECTURE,
    },
];

pub const GRADUATE_COURSE: &[TAHourAllocation] = &[
    TAHourAllocation {
        name: "Final Marking",
        hours: 0.53,
        calc_rule: CalculationRule::PER_STUDENT,
        alloc_type: LECTURE,
    },
    TAHourAllocation {
        name: "Tutorials",
        hours: 12.0,
        calc_rule: CalculationRule::PER_TERM,
        alloc_type: LECTURE,
    },
    TAHourAllocation {
        name: "Office Hours",
        hours: 12.0,
        calc_rule: CalculationRule::PER_TERM,
        alloc_type: LECTURE,
    },
    TAHourAllocation {
        name: "Assignment Marking",
        hours: 1.0,
        calc_rule: CalculationRule::PER_STUDENT,
        alloc_type: LECTURE,
    },
    TAHourAllocation {
        name: "Exam Proctoring",
        hours: 3.0,
        calc_rule: CalculationRule::PER_TERM,
        alloc_type: LECTURE,
    },
    TAHourAllocation {
        name: "Extra TA Hours",
        hours: 0.0,
        calc_rule: CalculationRule::PER_TERM,
        alloc_type: LECTURE,
    },
];
