use bitbankcc::{Bitbankcc, CandleType, CurrencyPair};

fn main() {
    let bb = bitbankcc::Bitbankcc::new();

    let ticker = bb.get_ticker(CurrencyPair::BtcJpy);
    // dbg!(ticker);

    let depth = bb.get_depth(CurrencyPair::BtcJpy);
    // dbg!(&depth.unwrap().asks[0]);

    // TODO: ts = bb.get_transaction(CurrencyPair::BTC_JPY);
    // dbg!(ts);

    let cs = bb.get_candlestick(CurrencyPair::BtcJpy, CandleType::_1day, "2017");
    // dbg!(&cs.unwrap().values[0]);
}
