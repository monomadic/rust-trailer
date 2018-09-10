use trailer::presenters::FundsPresenter;
use trailer::error::TrailerError;

fn append_log(path: &str, content: &str) -> Result<(), TrailerError> {
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path);

    match file {
        Ok(mut file) => match writeln!(file, "{}", content) {
            Ok(_) => Ok(()),
            Err(why) => Err(TrailerError::generic(&format!("{} {}", why, path)))
        },
        Err(why) => Err(TrailerError::generic(&format!("{} {}", why, path)))
    }
}

pub fn log_funds(funds: FundsPresenter) -> Result<(), TrailerError> {
    use chrono::prelude::*;
    let home_path = ::std::env::home_dir().ok_or(TrailerError::generic("cannot get homedir"))?;

    append_log(
        &format!("{}/.crypto/funds.csv", home_path.display()),
        &format!("{},{},{}", Utc::now(), funds.total_value_in_btc, funds.total_value_in_usd),
    )
}
