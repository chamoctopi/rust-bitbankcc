use bitbankcc::{CandleType, CurrencyPair};

fn main() {
    let bb = bitbankcc::Bitbankcc::new();

    match bb.get_ticker(CurrencyPair::BtcJpy) {
        Ok(ticker) => {
            dbg!(ticker);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    match bb.get_depth(CurrencyPair::BtcJpy) {
        Ok(depth) => {
            dbg!(&depth.asks[0]);
            dbg!(&depth.bids[0]);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    match bb.get_transaction(CurrencyPair::BtcJpy, "") {
        Ok(ts) => {
            dbg!(&ts.values[0]);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    match bb.get_candlestick(CurrencyPair::BtcJpy, CandleType::_1day, "2017") {
        Ok(cs) => {
            dbg!(&cs.r#type);
            dbg!(&cs.values[0]);
        }
        Err(err) => {
            dbg!(err);
        }
    }
}
