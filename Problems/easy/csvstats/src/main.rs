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

#[allow(unused_imports)]
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct CsvReader {
    path: String,
    column: Option<String>,
}

impl CsvReader {
    fn new(path: String, column: Option<String>) -> Result<CsvReader, String> {
        Ok(CsvReader { path, column })
    }
}

fn read_header(config: CsvReader) -> io::Result<Vec<String>> {
    let file = File::open(&config.path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() {
            return Ok(line.split(',').map(|s| s.trim().to_string()).collect());
        }
    }

    Ok(Vec::new())
}

fn main() {
    let init = CsvReader::new("./track_data_final.csv".to_string(), None);
    let header = read_header(init.unwrap());
    println!("{:#?}", header);
}
