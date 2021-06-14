mod ticker_client;
mod error;

mod model {
    pub(crate) mod crypto_compare;
}

fn main() {
    let data = ticker_client::get().expect("failed to fetch !");
    println!("{:#?}", data);
}
