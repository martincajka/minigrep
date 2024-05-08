use std::collections::VecDeque;

pub struct ContextWindow {
    before_lines: VecDeque<String>,
    matched_plus_after_lines: Vec<String>,
    before_capacity_counter: usize,
    after_capacity_counter: usize,
    is_match: bool,
    first_line: usize,
}

impl ContextWindow {
    pub fn new(before_capacity: usize, after_capacity: usize, first_line: usize) -> ContextWindow {
        ContextWindow {
            before_lines: VecDeque::with_capacity(before_capacity),
            matched_plus_after_lines: Vec::new(),
            before_capacity_counter: before_capacity,
            after_capacity_counter: after_capacity,
            is_match: false,
            first_line,
        }
    }

    pub fn add_line(&mut self) -> () {
        todo!()
    }
}
