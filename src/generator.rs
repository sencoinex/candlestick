mod input;
pub use input::CandlestickGeneratorInput;

mod output;
pub use output::CandlestickGeneratorOutput;

use crate::CandlestickData;
use std::sync::mpsc;

use crate::models::{Asset, AssetPair, Candlestick, Interval};

pub struct CandlestickGenerator<A: Asset> {
    asset_pair: AssetPair<A>,
    interval: Interval,
    input_receiver: mpsc::Receiver<CandlestickGeneratorInput<A>>,
    output_sender: mpsc::Sender<CandlestickGeneratorOutput<A>>,
}

impl<A: Asset> CandlestickGenerator<A> {
    pub fn new(
        asset_pair: AssetPair<A>,
        interval: Interval,
        input_receiver: mpsc::Receiver<CandlestickGeneratorInput<A>>,
        output_sender: mpsc::Sender<CandlestickGeneratorOutput<A>>,
    ) -> Self {
        Self {
            asset_pair,
            interval,
            input_receiver,
            output_sender,
        }
    }

    pub fn start(self) {
        let mut output: Option<Candlestick<A>> = None;
        for input in self.input_receiver {
            match input {
                CandlestickGeneratorInput::Tick { open_time } => {
                    assert!(self.interval.is_valid_time(&open_time));
                    if let Some(output) = &output {
                        assert!(output.open_time < open_time);
                        self.output_sender
                            .send(vec![output.clone()])
                            .expect("output must be sent.");
                    }
                    output = Some(Candlestick::new(
                        self.asset_pair.clone(),
                        open_time,
                        self.interval,
                    ));
                }
                CandlestickGeneratorInput::OrderFilled {
                    open_time,
                    order_filled,
                } => {
                    assert!(self.interval.is_valid_time(&open_time));
                    assert_eq!(order_filled.asset_pair, self.asset_pair);
                    if let Some(current) = &mut output {
                        assert!(current.open_time <= open_time);
                        if current.open_time < open_time {
                            // close the output and reset output
                            let mut results = vec![current.clone()];
                            loop {
                                let next_time = self.interval.next(current.open_time);
                                if next_time >= open_time {
                                    break;
                                }
                                results.push(Candlestick::new(
                                    self.asset_pair.clone(),
                                    next_time,
                                    self.interval,
                                ));
                            }
                            self.output_sender
                                .send(results)
                                .expect("output must be sent.");
                            current.reset(
                                open_time,
                                Some(CandlestickData::new_with_price_and_quantity(
                                    order_filled.price,
                                    order_filled.quantity,
                                )),
                            );
                        } else {
                            // update exact the same open_time data
                            current.add_order_filled(order_filled);
                        }
                    } else {
                        output = Some(Candlestick::new_with_data(
                            self.asset_pair.clone(),
                            open_time,
                            self.interval,
                            Some(CandlestickData::new_with_price_and_quantity(
                                order_filled.price,
                                order_filled.quantity,
                            )),
                        ));
                    }
                }
                CandlestickGeneratorInput::Terminate => {
                    break;
                }
            }
        }
    }
}
