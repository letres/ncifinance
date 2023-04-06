pub mod custom_strategy {

    use barter::data::MarketMeta;
    use barter::strategy::{Decision, Signal, SignalGenerator, SignalStrength};
    use barter_data::event::{DataKind, MarketEvent};
    use chrono::Utc;
    use serde::{Deserialize, Serialize};
    use std::boxed::Box;
    use std::clone::Clone;
    use std::collections::HashMap;
    use std::fmt::Debug;

    use ta::{
        indicators::{FastStochastic, RelativeStrengthIndex},
        Close, DataItem, Next, Open,
    };

    /// Configuration for constructing a [`WeightedStrategy`]
    /// via the new() constructor method.
    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
    pub struct WeightConfig {
        pub period: usize,
        pub weight: f64,
        pub trigger: (f64, f64),
    }

    trait DataTick: ta::Open + ta::High + ta::Low + ta::Close {}
    trait NewTrait<T>: for<'a> ta::Next<&'a T, Output = f64> + Debug + Send {}
    impl<DataItem: DataTick> NewTrait<DataItem> for RelativeStrengthIndex {}
    impl<DataItem: DataTick> NewTrait<DataItem> for FastStochastic {}
    impl DataTick for DataItem{}

    #[derive(Debug)]
    /// strategy that implements [`SignalGenerator`].
    pub struct WeightedStrategy<T> {
        config: Vec<(WeightConfig, Box<dyn NewTrait<T>>)>,
        trigger: (f64, f64),
        sell: (f64, f64),
        indicator: f64,
    }

    impl SignalGenerator for WeightedStrategy<DataItem> {
        fn generate_signal(&mut self, market: &MarketEvent<DataKind>) -> Option<Signal> {
            // Check if it's a MarketEvent with a candle
            let candle_close = match &market.kind {
                DataKind::Candle(candle) => candle.close,
                _ => return None,
            };
            let candle_open = match &market.kind {
                DataKind::Candle(candle) => candle.open,
                _ => return None,
            };
            let candle_high = match &market.kind {
                DataKind::Candle(candle) => candle.high,
                _ => return None,
            };
            let candle_low = match &market.kind {
                DataKind::Candle(candle) => candle.low,
                _ => return None,
            };
            let candle_volume = match &market.kind {
                DataKind::Candle(candle) => candle.volume,
                _ => return None,
            };
            let candle_number = match &market.kind {
                DataKind::Candle(candle) => candle.trade_count,
                _ => return None,
            };
            let item = DataItem::builder()
                .open(candle_open)
                .high(candle_high)
                .low(candle_low)
                .close(candle_close)
                .volume(candle_volume)
                .build()
                .unwrap();
            //TODO:Loop through all indicators and Weigh them???
            //Something with Constant?? eventually
            //create a better function(linear??)
            // Calculate the next indicator value using the new
            // MarketEvent Candle data
            self.indicator = 0.0;
            let mut weight;
            for x in 0..self.config.len() {
                weight = self.config[x].0.weight;
                self.indicator += weight * self.config[x].1.next(&item);
            }
            // Generate advisory signals map
            let signals = self.generate_signals_map();

            // If signals map is empty, return no SignalEvent
            if signals.is_empty() {
                return None;
            }

            Some(Signal {
                time: Utc::now(),
                exchange: market.exchange.clone(),
                instrument: market.instrument.clone(),
                market_meta: MarketMeta {
                    close: candle_close,
                    time: market.exchange_time,
                },
                signals,
            })
        }
    }

    impl<DataItem> WeightedStrategy<DataItem>
    where
        RelativeStrengthIndex: for<'a> ta::Next<&'a DataItem, Output = f64>,
        DataItem: DataTick,
    {
        /// Constructs a new [`WeightedStrategy`] component using the
        /// provided configuration struct.
        /// trigger and sell are (long,short)
        pub fn new(
            config: Vec<(i32, (usize, f64, (f64, f64)))>,
            trigger: (f64, f64),
            sell: (f64, f64),
        ) -> Self {
            //Loop through all config
            let mut test = vec![];
            for indicator in config {
                match indicator {
                    (0, vals) => {
                        test.push((
                            WeightConfig {
                                period: vals.0,
                                weight: vals.1,
                                trigger: vals.2,
                            },
                            Box::new(
                                RelativeStrengthIndex::new(vals.0)
                                    .expect("Failed to construct RSI indicator"),
                            ) as Box<dyn NewTrait<DataItem>>,
                        ));
                    }
                    (1, vals) => {
                        test.push((
                            WeightConfig {
                                period: vals.0,
                                weight: vals.1,
                                trigger: vals.2,
                            },
                            Box::new(
                                FastStochastic::new(vals.0)
                                    .expect("Failed to construct Fs indicator"),
                            ) as Box<dyn NewTrait<DataItem>>,
                        ));
                    }

                    _ => (),
                }
            }
            Self {
                config: test,
                trigger: trigger,
                sell: sell,
                indicator: 0.0,
            }
        }

        /// Given the latest Indicator value for a symbol, generates a map containing the [`SignalStrength`] for
        /// [`Decision`] under consideration.
        fn generate_signals_map(&self) -> HashMap<Decision, SignalStrength> {
            let mut signals = HashMap::with_capacity(4);
            if self.indicator < self.trigger.0 {
                signals.insert(Decision::Long, self.calculate_signal_strength());
            }
            if self.indicator > self.sell.0 {
                signals.insert(Decision::CloseLong, self.calculate_signal_strength());
            }
            // if self.indicator > self.trigger.1 {
            //     signals.insert(Decision::Short, self.calculate_signal_strength());
            // }
            // if self.indicator < self.sell.1 {
            //     signals.insert(Decision::CloseShort, self.calculate_signal_strength());
            // }
            signals
        }

        /// Calculates the [`SignalStrength`] of a particular [`Decision`].
        fn calculate_signal_strength(&self) -> SignalStrength {
            SignalStrength(1.0)
        }
    }
}
