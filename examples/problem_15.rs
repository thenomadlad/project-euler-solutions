
const ROWS: usize = 20;
const COLS: usize = 20;


fn main() {
    let mut values = [[0 as usize; ROWS + 1]; COLS + 1];

    for row in (0..=ROWS).rev() {
        for col in (0..=COLS).rev() {
            values[row][col] = if row == ROWS || col == COLS {
                1
            } else {
                values[row][col + 1] + values[row + 1][col]
            };
        }
    }

    let solution = values[0][0];
    println!("{solution}");
}