const DAYS_IN_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

#[derive(Debug, Clone)]
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

        if self.month == 12 {
            self.month = 1; // set to jan
            self.year += 1;
        } else {
            self.month += 1;
        }

        self.day_idx = (self.day_idx + days_passed) % 7;
    }

    fn months_until(self, year: u32, month: u32) -> MonthIterator {
        MonthIterator {
            date: self,
            year,
            month,
        }
    }

    fn jan1900() -> Date {
        Date {
            year: 1900,
            month: 1,
            day_idx: 0, // monday
        }
    }
}

#[derive(Debug)]
struct MonthIterator {
    date: Date,
    year: u32,
    month: u32,
}

impl Iterator for MonthIterator {
    type Item = Date;

    fn next(&mut self) -> Option<Self::Item> {
        if self.date.year < self.year
            || (self.date.year == self.year && self.date.month < self.month)
        {
            self.date.advance_month();
            Some(self.date.clone())
        } else {
            None
        }
    }
}

fn main() {
    let count = Date::jan1900()
        .months_until(2001, 1)
        .filter(|date| date.day_idx == 6 && date.year >= 1901)
        .count();

    println!("{count}");
}
