use serde::Deserialize;
use std::io;

#[derive(Debug, Deserialize)]
struct VentureXFormat {
    #[serde(rename = "Transaction Date")]
    transaction_date: String,
    #[serde(rename = "Posted Date")]
    posted_date: String,
    #[serde(rename = "Card No.")]
    card_number: String,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "Category")]
    category: String,
    #[serde(rename = "Debit")]
    debit: Option<f32>,
    #[serde(rename = "Credit")]
    credit: Option<f32>,
}

fn main() {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    println!("VENTURE X");
    for result in rdr.deserialize() {
        let record: VentureXFormat = result.expect("a CSV record");

        println!("{:?}", record);
    }
}
