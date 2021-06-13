use bitbankcc::{CurrencyPair, OrderSide, OrderType};
use std::collections::HashMap;
use std::env;

fn main() {
    let bb = bitbankcc::Bitbankcc::with_credentials(
        env::var("API_KEY").unwrap(),
        env::var("API_SECRET").unwrap(),
    );

    // let assets = bb.get_assets();
    // dbg!(&assets.unwrap().values[0]);
    //
    // let order = bb.get_order(CurrencyPair::BtcJpy, 90956209);
    // dbg!(order);

    // let order = bb.send_order(
    //     CurrencyPair::BtcJpy,
    //     3886555.0,
    //     0.01,
    //     OrderSide::Buy,
    //     OrderType::Limit,
    //     false,
    // );
    // dbg!(order);

    // let order = bb.cancel_order(CurrencyPair::BtcJpy, 129781978);
    // dbg!(order);

    // let ids = vec![129830841, 129830734];
    // let orders = bb.cancel_orders(CurrencyPair::BtcJpy, ids);
    // dbg!(orders);

    // let ids = vec![90956209, 90951996];
    // let orders = bb.get_orders(CurrencyPair::BtcJpy, ids);
    // dbg!(orders);

    let mut option = HashMap::<String, u64>::new();
    option.insert("count".to_string(), 1);
    option.insert("since".to_string(), 1490348550380);
    // Option's parameter can be seen https://docs.bitbank.cc/#!/Order/active_orders
    let orders = bb.get_active_orders(CurrencyPair::BtcJpy, option);
    dbg!(orders);

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
