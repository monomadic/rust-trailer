use colored::*;
use trailer::models::*;

pub fn row(asset: Asset) -> String {
    format!("{:<20}{:<20.2}{:<20.2}",
        asset.symbol.yellow(), asset.amount, asset.locked)
}
