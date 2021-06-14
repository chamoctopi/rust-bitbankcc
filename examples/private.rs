use bitbankcc::{CurrencyPair, OrderSide, OrderType};
use std::collections::HashMap;
use std::env;

fn main() {
    let bb = bitbankcc::Bitbankcc::with_credentials(
        env::var("API_KEY").unwrap(),
        env::var("API_SECRET").unwrap(),
    );

    match bb.get_assets() {
        Ok(assets) => {
            dbg!(&assets.values[0]);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    match bb.get_order(CurrencyPair::BtcJpy, 90956209) {
        Ok(order) => {
            dbg!(&order);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    match bb.send_order(
        CurrencyPair::BtcJpy,
        3186555.0,
        0.01,
        OrderSide::Buy,
        OrderType::Limit,
        false,
    ) {
        Ok(order) => {
            dbg!(order);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    match bb.cancel_order(CurrencyPair::BtcJpy, 129781978) {
        Ok(order) => {
            dbg!(order);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    let ids = vec![129830841, 129830734];
    match bb.cancel_orders(CurrencyPair::BtcJpy, ids) {
        Ok(orders) => {
            dbg!(orders);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    let ids = vec![90956209, 90951996];
    match bb.get_orders(CurrencyPair::BtcJpy, ids) {
        Ok(orders) => {
            dbg!(orders);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    let mut option = HashMap::<String, u64>::new();
    option.insert("count".to_string(), 1);
    option.insert("since".to_string(), 1490348550380);
    // Option's parameter can be seen https://github.com/bitbankinc/bitbank-api-docs/blob/master/rest-api.md#fetch-active-orders
    match bb.get_active_orders(CurrencyPair::BtcJpy, option) {
        Ok(orders) => {
            dbg!(orders);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    let mut option = HashMap::<String, u64>::new();
    option.insert("count".to_string(), 1);
    option.insert("since".to_string(), 1490348550380);
    // Option's parameter can be seen https://github.com/bitbankinc/bitbank-api-docs/blob/master/rest-api.md#trade
    match bb.get_trade_history(CurrencyPair::BtcJpy, option) {
        Ok(trade) => {
            dbg!(&trade.values[0]);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    // TODO:
    //     Accounts accounts = bb.getWithdrawalAccounts("btc");
    //     for(Accounts.Account a : accounts.accounts) {
    //         System.out.println(a);
    //     }

    // TODO:
    //     Withdraw w = bb.requestWithdraw("btc", "XXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXX",
    //                                     BigDecimal.valueOf(0.005), "867005", "");
    //     System.out.println(w);
}
