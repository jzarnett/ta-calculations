#[derive(Debug)]
pub enum CalculationRule {
    PER_TERM,
    PER_STUDENT,
    PER_LAB,
    PER_LEC_SECTION,
}

#[derive(Eq, PartialEq, Debug)]
pub enum AllocationType {
    LAB,
    NON_LAB,
    LECTURE,
}

pub struct TAHourAllocation {
    pub name: &'static str,
    pub hours: f32,
    pub calc_rule: CalculationRule,
    pub alloc_type: AllocationType,
}

#[derive(Eq, PartialEq, Debug)]
pub enum CourseType {
    FIRST_YEAR,
    UNDERGRAD,
    GRAD,
}

#[derive(Debug)]
pub struct Course {
    pub name: String,
    pub instructor: String,
    pub enrollment: i32,
    pub lec_sections: i32,
    pub lab_sections: i32,
    pub unit_weight: f32,
}

pub struct SpecialCase {
    pub course: &'static str,
    pub reason: &'static str,
    pub allocation_rule: AllocationRule,
    pub allocation_amount: f32,
}
#[derive(Debug)]
pub struct CourseAllocation {
    pub total: f32,
    pub lab_amount: f32,
}

#[derive(Eq, PartialEq, Debug)]
pub enum AllocationRule {
    NO_TA_ALLOC,
    MIN_ALLOC,
    MAX_ALLOC,
    PER_SECTION,
    PER_LAB_SECTION,
    FIXED,
}
