mod backend;

use crate::backend::querying::{query_crypto, query_stock};
use crate::backend::storage::{clear_queries, gather_queries, store_query};
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct StockCli {
    #[command(subcommand)]
    command: QueryOptions,
}

#[derive(Subcommand)]
enum QueryOptions {
    /// Initialises the project_data folder. This contains the files where the information is stored.
    Init,
    /// The stock price querying function. This displays both the raw and total price difference, while
    /// storing the query.
    Stock(Query),
    /// The crypto price querying function. This displays both the raw and total price difference, while
    /// storing the query.
    Crypto(Query),
    /// This command displays a variety of information about stored queries.
    ShowStock { name: String },
    /// This command displays a variety of information about stored queries.
    ShowCrypto { name: String },
    /// This command clears any stored stock information.
    ClearStock,
    /// This command clears any stored crypto information.
    ClearCrypto,
}

#[derive(Args, Debug)]
pub struct Query {
    date: String,
    name: String,
    amount: f64,
    price: f64,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = StockCli::parse();
    let api = alpha_vantage::set_api("LHBJUSM137M15SBF", reqwest::Client::new());

    match &cli.command {
        QueryOptions::Init => {
            let home = std::env::home_dir().unwrap();
            let path = format!("{}/project_data", home.to_str().unwrap());
            std::fs::create_dir_all(path)?;
        }
        QueryOptions::Stock(v) => {
            let query = Query {
                date: v.date.clone(),
                name: v.name.clone(),
                amount: v.amount,
                price: v.price,
            };

            let data = query_stock(api, &query).await;
            println!("Price diff (excluding amount purchased) => {}", data.0);
            println!("Total price diff => {}", data.1);

            store_query("stock_data.txt", query, data.2).unwrap();
        }
        QueryOptions::Crypto(v) => {
            let query = Query {
                date: v.date.clone(),
                name: v.name.clone(),
                amount: v.amount,
                price: v.price,
            };

            let data = query_crypto(api, &query).await;
            println!("Price diff (excluding amount purchased) => {}", data.0);
            println!("Total price diff => {}", data.1);

            store_query("crypto_data.txt", query, data.2).unwrap();
        }
        QueryOptions::ShowStock { name } => {
            let (queries, prices) = gather_queries("stock_data.txt", name).unwrap();
            profit_calculation(&queries, &prices)
        }
        QueryOptions::ShowCrypto { name } => {
            let (queries, prices) = gather_queries("crypto_data.txt", name).unwrap();
            profit_calculation(&queries, &prices)
        }
        QueryOptions::ClearStock => clear_queries("stock_data.txt").unwrap(),
        QueryOptions::ClearCrypto => clear_queries("cryto_data.txt").unwrap(),
    }

    Ok(())
}

fn profit_calculation(queries: &Vec<Query>, prices: &Vec<f64>) {
    if queries.len() == 0 {
        println!(
            "The data file is empty/That asset couldn't be found. Consider using the service a few more times and trying it again."
        );
        return;
    }

    let mut market_totalprice_sum: usize = 0;

    let user_baseprice_sum: usize = queries.into_iter().map(|q| q.price).sum::<f64>() as usize;
    let market_baseprice_sum: usize = prices.into_iter().sum::<f64>() as usize;
    let user_totalprice_sum: usize =
        queries.into_iter().map(|q| q.price * q.amount).sum::<f64>() as usize;

    for (i, q) in queries.into_iter().enumerate() {
        market_totalprice_sum += (q.amount * prices[i]) as usize;
    }

    let user_baseprice_mean = user_baseprice_sum / queries.len();
    let market_baseprice_mean = market_baseprice_sum / prices.len();

    let user_totalprice_mean = user_totalprice_sum / queries.len();
    let market_totalprice_mean = market_totalprice_sum / queries.len();

    println!("Average market stock price => {market_baseprice_mean}");
    println!("Average user stock price => {user_baseprice_mean}\n");

    println!("Average user total stock price (inc. amount) => {user_totalprice_mean}");
    println!("Average market total stock price (inc. amount) => {market_totalprice_mean}\n");

    println!(
        "Average profit (exc. amount) => {}",
        (market_baseprice_mean - user_baseprice_mean)
    );

    println!(
        "Average profit (inc. amount) => {}",
        (market_totalprice_mean - user_totalprice_mean)
    );
}
