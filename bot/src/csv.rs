use trailer::error::*;

pub fn load_backtest_data(file: &str) -> Result<Vec<f64>, TrailerError> {
    use std::io::Read;

    // let mut csvfile = ::std::fs::File::open(file).expect(&format!("csvfile to open: {}", file));
    let mut csvfile = ::std::fs::File::open(file).expect(&format!("expected csvfile to open: {}", file));
    let mut bytebuffer = Vec::new();
    csvfile.read_to_end(&mut bytebuffer).expect("expected template file to read into buffer");
    let file = String::from_utf8(bytebuffer).expect("expected csvfile to be utf8");

    let lines:Vec<&str> = file.split("\n").collect();

    Ok(lines.iter().map(|line| {
        let entry:Vec<&str> = line.split(",").collect();
        let first = entry.first().unwrap_or(&"0");
        first.parse::<f64>().unwrap_or(0.0)
    }).collect())
}
