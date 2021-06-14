use crate::error::{Error, Kind::Client};
use crate::model::crypto_compare::Histohour;
use std::env;

pub fn get() -> Result<Histohour, Error<reqwest::Error>> {
    match reqwest() {
        Ok(map) => Ok(map),
        Err(e) => Err(Error { kind: Client, cause: Option::Some(e) })
    }
}

fn reqwest() -> Result<Histohour, reqwest::Error> {
    reqwest::blocking::Client::builder().build()?
        .get("https://min-api.cryptocompare.com/data/v2/histohour?fsym=BTC&tsym=USD&limit=10")
        .header("Apikey", env::var("CRYPTOCOMPARE_API_KEY").expect("Missing CRYPTOCOMPARE_API_KEY environment variable"))
        .send()?
        .json::<Histohour>()
}