pub struct Fibs {
    current: u64,
    next: u64,
}

impl Default for Fibs {
    fn default() -> Self {
        Self {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibs {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        (self.current, self.next) = (self.next, self.current + self.next);

        Some(self.current)
    }
}
