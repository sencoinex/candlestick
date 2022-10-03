use crate::models::{Asset, OrderFilled, Time};

#[derive(Debug)]
pub enum CandlestickGeneratorInput<A: Asset> {
    /// start a new candlestick for live.
    Tick {
        open_time: Time,
    },
    /// filled order event.
    /// This message could be used for both live and archive data.
    OrderFilled {
        open_time: Time,
        order_filled: OrderFilled<A>,
    },
    Terminate,
}
