// Challenge: CSV Column Stats (CLI)
// Write a tool that:
// 1. Takes:
//      - A CSV file path.
//      - A column name.
// 2. Reads the CSV file.
// 3. Parses the specified column as 'f64'
// 4. Computes and prints:
//      - Count (n)
//      - Mean
//      - Min
//      - Max
//      - Standard deviation
// 5. Skips rows where the value is missing or not a valid number.
//      - Report how many rows were skipped.

// Constraints:
// 1. Only standard library can be used.
// 2. Assume the first line is a header row with comma-separated column names.
// 3. Handle:
//      - File not found.
//      - Column not found.
//      - No valid numeric values
// 4. Keep code organized into functions and/or small structs.

// Example CLI Usage:
// cargo run -- data.csv price

// Sample Output:
// Column: price
// Valid rows: 123
// Skipped rows: 7
// Min: 1.23
// Max 99.99
// Mean: 34.56
// Std dev: 12.34
fn main() {
    println!("Hello, world!");
}
