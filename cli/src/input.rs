use trailer::error::TrailerError;

pub fn get_str(default: &str) -> Result<String, TrailerError> {
    use std::io::{stdin,stdout,Write};

    let mut s=String::new();
    let _=stdout().flush();

    let input_char_length = stdin().read_line(&mut s)?;

    if input_char_length == 1 {
        s = default.to_string();
    }

    // trim
    if let Some('\n') = s.chars().next_back() { s.pop(); }
    if let Some('\r') = s.chars().next_back() { s.pop(); }

    Ok(s)
}

pub fn get_f64(default: f64) -> Result<f64, TrailerError> {
    Ok(get_str(&format!("{}", default))?.parse::<f64>()?)
}

pub fn get_confirmation() -> Result<bool, TrailerError> {
    let result = get_str("N")?;

    // if result == "y" {
    //     println!("yes!");
    // } else {
    //     println!("no!");
    // }

    Ok(result == "y")
}
