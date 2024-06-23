#[derive(Debug, Clone)]
pub struct SearchConstraint {
    start: Option<i32>,
    end: Option<i32>,
    left: Option<i32>,
    right: Option<i32>,
}
