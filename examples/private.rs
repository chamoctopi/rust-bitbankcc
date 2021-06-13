use bitbankcc::{CurrencyPair, OrderSide, OrderType};
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
    // TODO: let orders = bb.cancel_orders(CurrencyPair::BtcJpy, ids);
    // dbg!(orders);

    let ids = vec![90956209, 90951996];
    let orders = bb.get_orders(CurrencyPair::BtcJpy, ids);
    dbg!(orders);

    // TODO:
    //     Map<String, Long> option = new HashMap<String, Long>();
    //     option.put("count", 1L);
    //     option.put("since", 1490348550380L);
    //     // Option's parameter can be seen https://docs.bitbank.cc/#!/Order/active_orders
    //     Orders orders3 = bb.getActiveOrders(CurrencyPair.BTC_JPY, option);
    //     for(Order o : orders3.orders) {
    //         System.out.println(o);
    //     }

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
