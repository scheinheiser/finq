use crate::Query;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
};

#[derive(Debug)]
pub enum QueryErr {
    Io(std::io::Error),
    Parse(std::num::ParseFloatError),
}

pub fn clear_queries(filename: &str) -> Result<(), QueryErr> {
    let home = std::env::home_dir().unwrap();
    let path = format!("{}/project_data/{}", home.to_str().unwrap(), filename);

    let _ = std::fs::File::create(path)
        .map_err(|err| QueryErr::Io(err))
        .unwrap();

    Ok(())
}

pub fn store_query(filename: &str, query: Query, price: f64) -> Result<(), QueryErr> {
    let home = std::env::home_dir().unwrap();
    let path = format!("{}/project_data/{}", home.to_str().unwrap(), filename);

    let mut data_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .map_err(|err| QueryErr::Io(err))
        .unwrap();

    let record = format!(
        "{};{};{};{};{}\n",
        query.date,
        query.name,
        query.amount.to_string(),
        query.price.to_string(),
        price.to_string()
    );

    let _ = data_file.write_all(record.as_bytes());
    Ok(())
}

pub fn gather_queries(
    filename: &str,
    asset_name: &str,
) -> Result<(Vec<Query>, Vec<f64>), QueryErr> {
    let mut queries = Vec::new();
    let mut prices = Vec::new();

    let home = std::env::home_dir().unwrap();
    let path = format!("{}/project_data/{}", home.to_str().unwrap(), filename);

    let mut data_file = OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|err| QueryErr::Io(err))
        .unwrap();

    let mut contents = String::new();
    data_file
        .read_to_string(&mut contents)
        .map_err(|err| QueryErr::Io(err))
        .unwrap();

    for line in contents.split("\n") {
        if line == "" {
            break;
        }

        let q_values: Vec<_> = line.split(";").collect();
        if q_values[1] != asset_name {
            break;
        }

        let q_amount: f64 = q_values[2]
            .parse()
            .map_err(|err| QueryErr::Parse(err))
            .unwrap();
        let q_price: f64 = q_values[3]
            .parse()
            .map_err(|err| QueryErr::Parse(err))
            .unwrap();
        let r_price: f64 = q_values[4]
            .parse()
            .map_err(|err| QueryErr::Parse(err))
            .unwrap();

        queries.push(Query {
            date: String::from(q_values[0]),
            name: String::from(q_values[1]),
            amount: q_amount,
            price: q_price,
        });

        prices.push(r_price);
    }

    Ok((queries, prices))
}
