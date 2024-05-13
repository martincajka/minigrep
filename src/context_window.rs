use std::collections::VecDeque;

pub struct ContextWindow {
    before_lines: VecDeque<String>,
    matched_plus_after_lines: Vec<(String, bool)>,
    before_capacity: usize,
    after_capacity: usize,
    before_capacity_counter: usize,
    after_capacity_counter: usize,
    has_any_matches: bool,
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
            has_any_matches: false,
        }
    }

    pub fn add_line(&mut self, line: &str, is_match: bool) -> () {
        match (is_match, self.has_any_matches) {
            (true, true) => {
                self.matched_plus_after_lines.push((line.to_string(), true));
            }
            (true, false) => {
                self.before_capacity_counter = 0;
                self.has_any_matches = true;
                self.matched_plus_after_lines.push((line.to_string(), true));
                self.after_capacity_counter = self.after_capacity;
            }
            (false, true) => {
                self.after_capacity_counter -= 1;
                self.matched_plus_after_lines
                    .push((line.to_string(), false));
            }
            (false, false) => {
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
        if self.has_any_matches {
            return self.write(writer);
        }
        Ok(())
    }

    pub fn is_ready_to_write_out(&self) -> bool {
        if self.has_any_matches
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
            writeln!(writer, "-{}", line)?;
        }
        for line in self.matched_plus_after_lines.iter() {
            if line.1 {
                writeln!(writer, "*{}", line.0)?; // mark matched lines with *
            } else {
                writeln!(writer, "-{}", line.0)?; // mark context lines with space
            }
        }

        self.before_lines.clear();
        self.matched_plus_after_lines.clear();
        self.has_any_matches = false;
        self.before_capacity_counter = self.before_capacity;
        self.after_capacity_counter = self.after_capacity;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::ContextWindow;

    #[test]
    fn context_init_for_before_0_after_0() -> Result<(), anyhow::Error> {
        let context = ContextWindow::new(0, 0);
        assert_eq!(context.after_capacity, 0);
        assert_eq!(context.before_capacity, 0);
        assert_eq!(context.before_capacity_counter, 0);
        assert_eq!(context.after_capacity_counter, 0);
        assert_eq!(context.before_lines.len(), 0);
        assert_eq!(context.matched_plus_after_lines.len(), 0);
        assert_eq!(context.has_any_matches, false);
        Ok(())
    }
}
