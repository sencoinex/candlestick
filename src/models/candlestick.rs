use super::{Asset, AssetPair, Interval, OrderFilled, Price, Quantity, Time, Volume};

#[derive(Debug, Clone)]
pub struct CandlestickData {
    pub open: Price,
    pub high: Price,
    pub low: Price,
    pub close: Price,
    pub volume: Volume,
}

impl CandlestickData {
    pub fn new_with_price_and_quantity(price: Price, quantity: Quantity) -> Self {
        Self {
            open: price,
            high: price,
            low: price,
            close: price,
            volume: Volume::new(quantity),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Candlestick<A: Asset> {
    pub asset_pair: AssetPair<A>,
    pub open_time: Time,
    pub interval: Interval,
    pub data: Option<CandlestickData>,
}

impl<A: Asset> Candlestick<A> {
    pub fn new(asset_pair: AssetPair<A>, open_time: Time, interval: Interval) -> Self {
        Self {
            asset_pair,
            open_time,
            interval,
            data: None,
        }
    }

    pub fn new_with_data(
        asset_pair: AssetPair<A>,
        open_time: Time,
        interval: Interval,
        data: Option<CandlestickData>,
    ) -> Self {
        Self {
            asset_pair,
            open_time,
            interval,
            data,
        }
    }

    pub fn reset(&mut self, open_time: Time, data: Option<CandlestickData>) {
        self.open_time = open_time;
        self.data = data;
    }

    pub fn add_order_filled(&mut self, order_filled: OrderFilled<A>) {
        if let Some(data) = &mut self.data {
            if data.high < order_filled.price {
                data.high = order_filled.price;
            }
            if order_filled.price < data.low {
                data.low = order_filled.price;
            }
            data.close = order_filled.price;
            data.volume.add_quantity(&order_filled.quantity);
        } else {
            self.data = Some(CandlestickData::new_with_price_and_quantity(
                order_filled.price,
                order_filled.quantity,
            ));
        }
    }
}
