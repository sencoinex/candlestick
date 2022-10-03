use super::{Asset, AssetPair, Price, Quantity};

#[derive(Debug)]
pub struct OrderFilled<A: Asset> {
    pub asset_pair: AssetPair<A>,
    pub price: Price,
    pub quantity: Quantity,
}
