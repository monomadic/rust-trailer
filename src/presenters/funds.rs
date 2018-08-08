use models::*;
use presenters::*;

#[derive(Debug, Clone)]
pub struct FundsPresenter {
    pub btc:                    Option<AssetPresenter>,
    pub fiat:                   Vec<AssetPresenter>,
    pub alts:                   Vec<AssetPresenter>,
    pub total_value_in_usd:     f64,
    pub total_value_in_btc:     f64,
}

impl FundsPresenter {
    pub fn new(funds: Funds, prices: Prices) -> FundsPresenter {
        let btc_price = if let Some(btc) = funds.btc { btc.amount } else { 0.0 };

        let fiat = funds.fiat.into_iter().map(|asset| AssetPresenter::new(asset.clone(), *prices.get(&asset.symbol).unwrap_or(&0.0), btc_price));
        let alts = funds.alts.into_iter().map(|asset| AssetPresenter::new(asset.clone(), *prices.get(&asset.symbol).unwrap_or(&0.0), btc_price));

        let total_usd_price:f64 = alts.clone().map(|a| a.value_in_usd * a.asset.amount).sum();
        let total_alt_price_in_btc:f64 = alts.clone().map(|a| a.value_in_btc * a.asset.amount).sum();

        Self {
            btc:                    None,
            fiat:                   fiat.collect(),
            alts:                   alts.collect(),
            total_value_in_usd:     total_usd_price,
            total_value_in_btc:     total_alt_price_in_btc,
        }
    }
}
