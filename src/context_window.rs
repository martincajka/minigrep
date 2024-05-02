use std::collections::VecDeque;

pub struct ContextWindow {
    context_lines: VecDeque<String>,
    max_capacity: usize,
    start_line_index: usize,
    match_line_index: Option<usize>,
}

impl ContextWindow {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            context_lines: VecDeque::with_capacity(max_capacity),
            max_capacity,
            start_line_index: 0,
            match_line_index: None,
        }
    }

    pub fn add_line(&mut self, line: String, current_index: usize, is_match: bool) {
        if self.context_lines.len() == self.max_capacity {
            self.context_lines.pop_front();
            self.start_line_index += 1;
        }
        self.context_lines.push_back(line);
        if is_match {
            self.match_line_index = Some(current_index);
        }
    }

    pub fn get_lines(&self) -> Vec<String> {
        self.context_lines.iter().cloned().collect()
    }

    pub fn get_start_line_index(&self) -> usize {
        self.start_line_index
    }

    pub fn get_match_line_index(&self) -> Option<usize> {
        self.match_line_index
    }
}
