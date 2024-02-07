const NUMBERS: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&'static str; 10] = [
    "zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const AND: &'static str = "and";
const HUNDRED: &'static str = "hundred";
const THOUSAND: &'static str = "thousand";

fn main() {
    let result: usize = (1..=1000).map(textify).map(|s| s.len()).sum();
    println!("{result}");
}

fn textify(val: usize) -> String {
    let mut val = val;
    let mut result = String::new();

    if val >= 1000 {
        result += &textify(val / 1000);
        result += THOUSAND;
        val = val % 1000;
    }

    if val >= 100 {
        result += &textify(val / 100);
        result += HUNDRED;
        val = val % 100;
    }

    if val >= 20 {
        let tens = val / 10;
        let units = val % 10;

        if result.len() > 0 {
            result += AND;
        }
        result += TENS[tens];
        result += if units != 0 { NUMBERS[units] } else { "" };
    } else {
        if val != 0 {
            if result.len() > 0 {
                result += AND;
            }
            result += NUMBERS[val];
        } else {
            if result.len() == 0 {
                result += NUMBERS[val];
            }
        }
    }

    result
}
