struct Fibs {
    current: u32,
    next: u32,
}

impl Fibs {
    fn new() -> Fibs {
        Fibs {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibs {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        (self.current, self.next) = (self.next, self.current + self.next);

        Some(self.current)
    }
}

pub fn even_fib_sum() -> u32 {
    Fibs::new()
        .take_while(|x| *x <= 4_000_000)
        .filter(|x| x % 2 == 0)
        .sum()
}
