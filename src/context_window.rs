use std::collections::VecDeque;

pub struct ContextWindow {
    before_lines: VecDeque<String>,
    matched_plus_after_lines: Vec<String>,
    before_capacity: usize,
    after_capacity: usize,
    before_capacity_counter: usize,
    after_capacity_counter: usize,
    is_match_indices: Vec<usize>,
    first_line: usize,
}

impl ContextWindow {
    pub fn new(before_capacity: usize, after_capacity: usize) -> ContextWindow {
        ContextWindow {
            before_lines: VecDeque::with_capacity(before_capacity),
            matched_plus_after_lines: Vec::new(),
            before_capacity,
            after_capacity,
            before_capacity_counter: before_capacity,
            after_capacity_counter: after_capacity,
            is_match_indices: Vec::new(),
            first_line: 0,
        }
    }

    pub fn add_line(&mut self, line: &str, is_match: bool) -> () {
        match (is_match, self.is_match_indices.len() == 0) {
            (true, true) => {
                self.before_capacity_counter = 0;
                self.is_match_indices
                    .push(self.before_lines.len() + self.is_match_indices.len());
                self.matched_plus_after_lines.push(line.to_string());
            }
            (true, false) => {
                self.is_match_indices
                    .push(self.before_lines.len() + self.is_match_indices.len());
                self.matched_plus_after_lines.push(line.to_string());
                self.after_capacity_counter = self.after_capacity;
            }
            (false, false) => {
                self.after_capacity_counter -= 1;
                self.matched_plus_after_lines.push(line.to_string());
            }
            (false, true) => {
                if self.before_capacity_counter == 0 {
                    self.before_lines.pop_front();
                } else {
                    self.before_capacity_counter -= 1;
                }
                self.before_lines.push_back(line.to_string())
            }
        }
    }

    pub fn finalize_after_last_line(&mut self, writer: impl std::io::Write) -> std::io::Result<()> {
        if !self.is_match_indices.is_empty() {
            return self.write(writer);
        }
        Ok(())
    }

    pub fn is_ready_to_write_out(&self) -> bool {
        if !self.is_match_indices.is_empty()
            && self.before_capacity_counter == 0
            && self.after_capacity_counter == 0
        {
            true
        } else {
            false
        }
    }

    pub fn write(&mut self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        for line in self.before_lines.iter() {
            writeln!(writer, "{}", line)?;
        }
        for (i, line) in self.matched_plus_after_lines.iter().enumerate() {
            if self
                .is_match_indices
                .contains(&(self.first_line + i + self.before_lines.len()))
            {
                writeln!(writer, "*{}", line)?; // mark matched lines with *
            } else {
                writeln!(writer, " {}", line)?; // mark context lines with space
            }
        }
        self.first_line =
            self.first_line + self.before_lines.len() + self.matched_plus_after_lines.len();
        self.before_lines.clear();
        self.matched_plus_after_lines.clear();
        self.is_match_indices.clear();
        self.before_capacity_counter = self.before_capacity;
        self.after_capacity_counter = self.after_capacity;
        Ok(())
    }
}
