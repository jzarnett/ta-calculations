#[derive(Debug)]
pub enum CalculationRule {
    PER_TERM,
    PER_STUDENT,
    PER_LAB,
}

#[derive(Eq, PartialEq, Debug)]
pub enum AllocationType {
    LAB,
    NON_LAB,
    ALWAYS,
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

pub struct Course {
    pub course_name: String,
    pub enrollment: i32,
    pub has_lab: bool,
}
