use crate::Query;

pub async fn query_crypto(api: alpha_vantage::api::ApiClient, query: &Query) -> (f64, f64, f64) {
    let crypto = api
        .crypto(
            alpha_vantage::crypto::CryptoFunction::Daily,
            &query.name,
            "EUR",
        )
        .json()
        .await
        .unwrap();

    let c_data = crypto.data();
    let last: Vec<_> = c_data
        .into_iter()
        .filter(|datum| *datum.time() == crypto.last_refreshed()[..10])
        .collect();

    let raw_diff = last[0].open() - query.price;
    let cost_diff = (last[0].open() * query.amount) - (query.price * query.amount);

    (raw_diff, cost_diff, last[0].open())
}

pub async fn query_stock(api: alpha_vantage::api::ApiClient, query: &Query) -> (f64, f64, f64) {
    let stock_time = api
        .stock_time(alpha_vantage::stock_time::StockFunction::Daily, &query.name)
        .interval(alpha_vantage::api::TimeSeriesInterval::SixtyMin)
        .output_size(alpha_vantage::api::OutputSize::Full)
        .json()
        .await
        .unwrap();

    let s_data = stock_time.data();
    let last: Vec<_> = s_data
        .into_iter()
        .filter(|datum| *datum.time() == stock_time.last_refreshed()[..10])
        .collect();

    let raw_diff = last[0].open() - query.price;
    let cost_diff = (last[0].open() * query.amount) - (query.price * query.amount);

    (raw_diff, cost_diff, last[0].open())
}
