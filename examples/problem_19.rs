const DAYS_IN_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

#[derive(Debug)]
struct Date {
    year: u32,
    month: u32,
    day_idx: u8,
}

impl Date {
    fn advance_month(&mut self) {
        let days_passed = DAYS_IN_MONTH[(self.month - 1) as usize]
            + if self.month == 2
                && (self.year % 400 == 0 || (self.year % 4 == 0 && self.year % 100 != 0))
            {
                1
            } else {
                0
            };
        // println!("{self:?} {days_passed}");

        if self.month == 12 {
            self.month = 1; // set to jan
            self.year += 1;
        } else {
            self.month += 1;
        }

        self.day_idx = (self.day_idx + days_passed) % 7;
    }
}

fn main() {
    let mut problem_date = Date {
        year: 1900,
        month: 1,
        day_idx: 0, // monday
    };
    let mut count = 0;

    while problem_date.year <= 2000 {
        if problem_date.day_idx == 6 && problem_date.year >= 1901 {
            count += 1;
        }
        problem_date.advance_month();
    }

    println!("{count}");
}
