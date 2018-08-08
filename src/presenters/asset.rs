use models::*;

#[derive(Debug, Clone)]
pub struct AssetPresenter {
    pub asset: Asset,
    pub value_in_usd: f64,
    pub value_in_btc: f64,
}

impl AssetPresenter {
    pub fn new(asset: Asset, asset_price_in_btc: f64, btc_price_in_usd: f64) -> Self {
        Self {
            asset: asset.clone(),
            value_in_btc:   asset.amount * asset_price_in_btc,
            value_in_usd:   asset.amount * asset_price_in_btc * btc_price_in_usd,
        }
    }
}
